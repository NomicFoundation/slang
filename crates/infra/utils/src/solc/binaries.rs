use std::collections::{BTreeMap, HashMap};
use std::fs::{File, OpenOptions};
use std::os::unix::prelude::PermissionsExt;
use std::path::{Path, PathBuf};
use std::time::Duration;

use anyhow::{Context, Result};
use rayon::iter::{ParallelBridge, ParallelIterator};
use semver::Version;
use serde::Deserialize;
use url::Url;

use crate::commands::Command;
use crate::paths::PathExtensions;
use crate::solc::{CliInput, CliOutput};

#[derive(Clone, Debug)]
pub struct Binary {
    pub version: Version,

    local_path: PathBuf,
}

impl Binary {
    /// Downloads any binaries for the given versions that aren't already
    /// cached on disk and returns the full set keyed by version.
    pub fn fetch_all(
        versions: impl IntoIterator<Item = Version>,
    ) -> Result<BTreeMap<Version, Self>> {
        let versions: Vec<Version> = versions.into_iter().collect();
        let binaries_dir = get_binaries_dir();

        with_lock(&binaries_dir.join("fetch.lock"), || {
            let mirror_url = get_mirror_url()?;
            let releases = fetch_releases(&mirror_url, &binaries_dir)?;

            versions
                .iter()
                .par_bridge()
                .map(|version| {
                    let local_path = binaries_dir.join(version.to_string());
                    if !local_path.exists() {
                        let release = releases.get(version).unwrap_or_else(|| {
                            panic!("Expected release '{version}' to exist at: {mirror_url}")
                        });

                        let remote_url = mirror_url.join(release)?;
                        download_file(remote_url, &local_path)?;
                        make_file_executable(&local_path)?;
                        println!("Downloaded: {local_path:?}");
                    }

                    Ok((
                        version.to_owned(),
                        Self {
                            version: version.to_owned(),
                            local_path,
                        },
                    ))
                })
                .collect()
        })
    }

    pub fn run(&self, input: &CliInput) -> Result<CliOutput> {
        let input = serde_json::to_string(input)?;

        let output = Command::new(self.local_path.unwrap_str())
            .flag("--standard-json")
            .evaluate_with_input(input)?;

        serde_json::from_str(&output)
            .with_context(|| format!("Failed to parse solc JSON output:\n{output}"))
    }
}

/// Runs `body` while holding an exclusive `flock(2)` on the given path. The
/// lock is associated with the open file description, so it serializes
/// concurrent callers across both threads and processes.
fn with_lock<T>(lock_path: &Path, body: impl FnOnce() -> Result<T>) -> Result<T> {
    let lock_file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(false)
        .open(lock_path)
        .with_context(|| format!("Failed to open lock file: {lock_path:?}"))?;

    File::lock(&lock_file).with_context(|| format!("Failed to acquire lock: {lock_path:?}"))?;

    let result = body();

    // Released implicitly on drop, but make it explicit for clarity.
    let _ = File::unlock(&lock_file);
    result
}

fn fetch_releases(mirror_url: &Url, binaries_dir: &Path) -> Result<HashMap<Version, String>> {
    #[derive(Deserialize)]
    struct MirrorList {
        releases: HashMap<Version, String>,
    }

    let list_path = binaries_dir.join("list.json");

    let should_download_list = match list_path.metadata() {
        Err(err) if err.kind() == std::io::ErrorKind::NotFound => true,
        Err(err) => return Err(err)?,
        Ok(metadata) => metadata.created()?.elapsed()? > Duration::from_secs(60 * 60 * 24),
    };

    if should_download_list {
        let list_url = mirror_url.join("list.json")?;
        download_file(list_url, &list_path)?;
    }

    let list_file = std::fs::read_to_string(list_path)?;
    let list: MirrorList = serde_json::from_str(&list_file)?;
    Ok(list.releases)
}

fn download_file(url: Url, path: &Path) -> Result<()> {
    let bytes = reqwest::blocking::get(url)?.bytes()?;
    std::fs::create_dir_all(path.unwrap_parent())?;
    std::fs::write(path, bytes)?;
    Ok(())
}

fn make_file_executable(local_path: &PathBuf) -> Result<()> {
    let mut permissions = local_path.metadata()?.permissions();
    permissions.set_mode(0o744);
    std::fs::set_permissions(local_path, permissions)?;
    Ok(())
}

fn get_binaries_dir() -> PathBuf {
    let dir = Path::repo_path("target/solc-binaries");
    std::fs::create_dir_all(&dir).expect("Failed to create solc binaries dir");
    dir
}

fn get_mirror_url() -> Result<Url> {
    use std::env::consts::{ARCH, OS};

    let platform_dir = match (OS, ARCH) {
        ("macos", "aarch64") => "macosx-amd64", // Possible using MacOS Rosetta
        ("macos", "x86_64") => "macosx-amd64",
        ("linux", "aarch64") => "linux-amd64", // Requires multiarch amd64 libs (e.g. libc6:amd64) + Rosetta/qemu
        ("linux", "x86_64") => "linux-amd64",
        ("windows", "x86_64") => "windows-amd64",
        _ => panic!(
            "Unrecognized platform ({OS}, {ARCH}). Please add it to the list defined in '{source_file}'.",
            source_file = file!(),
        ),
    };

    // See the list at <https://github.com/ethereum/solc-bin>.
    Ok(format!("https://binaries.soliditylang.org/{platform_dir}/").parse()?)
}

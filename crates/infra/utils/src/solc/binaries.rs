use std::collections::{BTreeMap, HashMap};
use std::fmt::Write;
use std::fs::{File, OpenOptions};
use std::os::unix::prelude::PermissionsExt;
use std::path::{Path, PathBuf};
use std::time::Duration;

use anyhow::{Context, Result};
use rayon::iter::{ParallelBridge, ParallelIterator};
use semver::Version;
use serde::Deserialize;
use sha3::{Digest, Sha3_256};
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

        let mirror_url = get_mirror_url()?;
        let releases = fetch_binaries_list(&mirror_url)?;

        versions
            .into_iter()
            .par_bridge()
            .map(|version| {
                let release = releases.get(&version).with_context(|| {
                    format!("Expected release '{version}' to exist at: {mirror_url}")
                })?;

                let remote_url = mirror_url.join(release)?;
                let binary = fetch_binary(version.clone(), remote_url)?;

                Ok((version, binary))
            })
            .collect()
    }

    pub fn run(&self, input: &CliInput) -> Result<CliOutput> {
        let input = serde_json::to_string(input)?;
        let entry_path = cache_entry_path(&self.version, &input);

        ensure_file_exists(&entry_path, None, || {
            let output = Command::new(self.local_path.unwrap_str())
                .flag("--standard-json")
                .evaluate_with_input(&input)?;

            std::fs::write(&entry_path, &output)
                .with_context(|| format!("Failed to write cache entry: {entry_path:?}"))
        })?;

        let output = entry_path
            .read_to_string()
            .with_context(|| format!("Failed to read cache entry: {entry_path:?}"))?;

        serde_json::from_str(&output)
            .with_context(|| format!("Failed to parse solc JSON output:\n{output}"))
    }
}

fn fetch_binaries_list(mirror_url: &Url) -> Result<HashMap<Version, String>> {
    #[derive(Deserialize)]
    struct MirrorList {
        releases: HashMap<Version, String>,
    }

    const LIST_MAX_AGE: Duration = Duration::from_secs(60 * 60 * 24);

    let list_path = solc_dir().join("list.json");

    ensure_file_exists(&list_path, Some(LIST_MAX_AGE), || {
        let list_url = mirror_url.join("list.json")?;
        download_file(list_url, &list_path)
    })?;

    let list_file = list_path.read_to_string()?;
    let list: MirrorList = serde_json::from_str(&list_file)?;
    Ok(list.releases)
}

fn fetch_binary(version: Version, remote_url: Url) -> Result<Binary> {
    let local_path = solc_dir().join("binaries").join(version.to_string());

    ensure_file_exists(&local_path, None, || {
        download_file(remote_url, &local_path)?;
        make_file_executable(&local_path)?;
        println!("Downloaded: {local_path:?}");

        Ok(())
    })?;

    Ok(Binary {
        version,
        local_path,
    })
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

fn cache_entry_path(version: &Version, input: &str) -> PathBuf {
    let mut hasher = Sha3_256::new();
    hasher.update(version.to_string().as_bytes());
    hasher.update([0u8]); // Domain separator; versions never contain NUL bytes.
    hasher.update(input.as_bytes());

    let mut name = String::new();
    for byte in hasher.finalize() {
        write!(&mut name, "{byte:02x}").unwrap();
    }

    solc_dir().join("cache").join(name)
}

fn download_file(url: Url, file_path: &Path) -> Result<()> {
    let bytes = reqwest::blocking::get(url)?.bytes()?;
    std::fs::write(file_path, bytes)?;
    Ok(())
}

fn make_file_executable(file_path: &PathBuf) -> Result<()> {
    let mut permissions = file_path.metadata()?.permissions();
    permissions.set_mode(0o744);

    std::fs::set_permissions(file_path, permissions)
        .with_context(|| format!("Failed to make file executable: {file_path:?}"))
}

fn ensure_file_exists(
    file_path: &Path,
    max_age: Option<Duration>,
    create_file: impl FnOnce() -> Result<()>,
) -> Result<()> {
    let parent_dir = file_path.unwrap_parent();
    std::fs::create_dir_all(parent_dir)
        .with_context(|| format!("Failed to create directory: {parent_dir:?}"))?;

    let file = match File::open(file_path) {
        Ok(file) => file,

        Err(error) if error.kind() == std::io::ErrorKind::NotFound => OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(false)
            .open(file_path)
            .with_context(|| format!("Failed to create file: {file_path:?}"))?,

        Err(error) => {
            return Err(error).with_context(|| format!("Failed to open file: {file_path:?}"));
        }
    };

    File::lock(&file).with_context(|| format!("Failed to acquire lock: {file_path:?}"))?;

    let up_to_date = match file.metadata() {
        Ok(metadata) if metadata.len() > 0 => match max_age {
            None => true,
            Some(max_age) => metadata
                .modified()
                .ok()
                .and_then(|time| time.elapsed().ok())
                .is_some_and(|age| age <= max_age),
        },
        // Missing, empty, or unreadable metadata: (re)create.
        _ => false,
    };

    let result = if up_to_date { Ok(()) } else { create_file() };

    File::unlock(&file).with_context(|| format!("Failed to release lock: {file_path:?}"))?;

    result
}

fn solc_dir() -> PathBuf {
    Path::repo_path("target/solc")
}

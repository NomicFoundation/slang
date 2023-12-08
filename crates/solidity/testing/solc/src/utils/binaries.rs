use std::{
    collections::HashMap, os::unix::prelude::PermissionsExt, path::Path, path::PathBuf,
    time::Duration,
};

use anyhow::Result;
use codegen_language_definition::model::Language;
use indicatif::{ProgressBar, ProgressStyle};
use infra_utils::{cargo::CargoWorkspace, commands::Command, paths::PathExtensions};
use rayon::prelude::{ParallelBridge, ParallelIterator};
use semver::Version;
use serde::Deserialize;
use url::Url;

use crate::utils::{ApiInput, ApiOutput};

#[derive(Debug)]
pub struct Binary {
    pub version: Version,

    local_path: PathBuf,
}

impl Binary {
    pub fn fetch_all(language: &Language) -> Result<Vec<Self>> {
        let binaries_dir = get_binaries_dir()?;
        let mirror_url = get_mirror_url()?;
        let releases = fetch_releases(&mirror_url, &binaries_dir)?;

        let progress_bar = ProgressBar::new(language.versions.len() as u64);

        let style = "[{elapsed_precise}] [{bar:80.cyan/blue}] {pos}/{len} │ ETA: {eta_precise}";
        progress_bar.set_style(ProgressStyle::with_template(style)?);

        let mut binaries = language
            .versions
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
                }

                progress_bar.inc(1);

                Ok(Self {
                    version: version.to_owned(),
                    local_path,
                })
            })
            .collect::<Result<Vec<_>>>()?;

        progress_bar.finish();
        println!();

        binaries.sort_by_key(|binary| binary.version.to_owned());

        Ok(binaries)
    }

    pub fn run(&self, input: &ApiInput) -> Result<ApiOutput> {
        let input = serde_json::to_string(input)?;

        let output = Command::new(self.local_path.unwrap_str())
            .flag("--standard-json")
            .evaluate_with_input(input)?;

        Ok(serde_json::from_str(&output)?)
    }
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

fn get_binaries_dir() -> Result<PathBuf> {
    Ok(CargoWorkspace::locate_source_crate("solidity_testing_solc")?.join("target/binaries"))
}

fn get_mirror_url() -> Result<Url> {
    let platform_dir = if cfg!(target_os = "macos") {
        "macosx-amd64"
    } else {
        panic!(
            "Unrecognized platform. Please add it to the list defined in '{source_file}'.",
            source_file = file!(),
        );
    };

    Ok(format!("https://binaries.soliditylang.org/{platform_dir}/").parse()?)
}

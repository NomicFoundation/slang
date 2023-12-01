use crate::utils::{ApiInput, ApiOutput};
use anyhow::Result;
use codegen_language_definition::model::Language;
use indicatif::{ProgressBar, ProgressStyle};
use infra_utils::{cargo::CargoWorkspace, commands::Command};
use rayon::prelude::{ParallelBridge, ParallelIterator};
use semver::Version;
use serde::Deserialize;
use std::{collections::HashMap, os::unix::prelude::PermissionsExt, path::Path, path::PathBuf};
use url::Url;

#[derive(Debug)]
pub struct Binary {
    pub version: Version,

    local_path: PathBuf,
}

impl Binary {
    pub fn fetch_all(language: &Language) -> Vec<Self> {
        let binaries_dir = get_binaries_dir();
        let mirror_url = get_mirror_url();
        let releases = fetch_releases(&mirror_url, &binaries_dir);

        let progress_bar = ProgressBar::new(language.versions.len() as u64);

        let style = "[{elapsed_precise}] [{bar:80.cyan/blue}] {pos}/{len} │ ETA: {eta_precise}";
        progress_bar.set_style(ProgressStyle::with_template(style).unwrap());

        let mut binaries: Vec<_> = language
            .versions
            .iter()
            .par_bridge()
            .map(|version| {
                let local_path = binaries_dir.join(version.to_string());
                if !local_path.exists() {
                    let remote_url = mirror_url.join(&releases[version]).unwrap();
                    download_file(remote_url, &local_path);
                    make_file_executable(&local_path);
                }

                progress_bar.inc(1);

                return Self {
                    version: version.to_owned(),
                    local_path,
                };
            })
            .collect();

        progress_bar.finish();
        println!();

        binaries.sort_by_key(|binary| binary.version.to_owned());

        return binaries;
    }

    pub fn run(&self, input: &ApiInput) -> Result<ApiOutput> {
        let input = serde_json::to_string(input).unwrap();

        let output = Command::new(self.local_path.to_str().unwrap())
            .flag("--standard-json")
            .evaluate_with_input(input)?;

        return Ok(serde_json::from_str(&output).unwrap());
    }
}

fn fetch_releases(mirror_url: &Url, binaries_dir: &Path) -> HashMap<Version, String> {
    #[derive(Deserialize)]
    struct MirrorList {
        releases: HashMap<Version, String>,
    }

    let list_path = binaries_dir.join("list.json");
    if !list_path.exists() {
        let list_url = mirror_url.join("list.json").unwrap();
        download_file(list_url, &list_path);
    }

    let list_file = std::fs::read_to_string(list_path).unwrap();
    let list: MirrorList = serde_json::from_str(&list_file).unwrap();
    return list.releases;
}

fn download_file(url: Url, path: &Path) {
    let bytes = reqwest::blocking::get(url).unwrap().bytes().unwrap();

    std::fs::create_dir_all(path.parent().unwrap()).unwrap();
    std::fs::write(path, bytes).unwrap();
}

fn make_file_executable(local_path: &PathBuf) {
    let mut permissions = local_path.metadata().unwrap().permissions();
    permissions.set_mode(0o744);
    std::fs::set_permissions(local_path, permissions).unwrap();
}

fn get_binaries_dir() -> PathBuf {
    return CargoWorkspace::locate_source_crate("solidity_testing_solc")
        .unwrap()
        .join("target/binaries");
}

fn get_mirror_url() -> Url {
    let platform_dir = if cfg!(target_os = "macos") {
        "macosx-amd64"
    } else {
        panic!(
            "Unrecognized platform. Please add it to the list defined in '{source_file}'.",
            source_file = file!(),
        );
    };

    return format!("https://binaries.soliditylang.org/{platform_dir}/")
        .parse()
        .unwrap();
}

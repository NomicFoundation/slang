use crate::utils::download_file;
use reqwest::Url;
use semver::Version;
use std::{fs, os::unix::prelude::PermissionsExt, path::PathBuf, process::Command};

pub struct BuildInfo {
    pub version: Version,
    pub commit: String,
    pub local_path: PathBuf,
    pub remote_url: Url,
}

pub fn download_build(build: &BuildInfo) {
    let mut version_check_path = build.local_path.clone();
    version_check_path.set_file_name(build.version.to_string() + ".version");
    if version_check_path.exists() {
        return;
    }

    if !build.local_path.exists() {
        download_file(&build.remote_url, &build.local_path)
    }

    ensure_executable(build);
    verify_version(build, &version_check_path);
}

fn ensure_executable(build: &BuildInfo) {
    let mut permissions = build.local_path.metadata().unwrap().permissions();
    permissions.set_mode(0o777);
    fs::set_permissions(&build.local_path, permissions).unwrap();
}

fn verify_version(build: &BuildInfo, version_check_path: &PathBuf) {
    let output = match Command::new(&build.local_path).arg("--version").output() {
        Ok(output) => output,
        Err(error) => panic!(
            "Cannot extract version out of {:?}\n{:?}",
            build.local_path, error
        ),
    };

    assert!(output.status.success());
    assert_eq!(output.stderr.len(), 0);

    let actual_version = String::from_utf8(output.stdout).unwrap();
    assert!(&actual_version.contains(&format!("Version: {}", build.version.to_string())));

    fs::write(version_check_path, &actual_version).unwrap();
}

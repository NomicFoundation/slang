use std::path::Path;

use anyhow::Result;
use infra_utils::commands::Command;
use infra_utils::paths::PathExtensions;
use semver::Version;
use serde::Deserialize;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct Package {
    name: String,
    version: Version,
}

pub struct Npm;

impl Npm {
    pub fn local_version(package_dir: impl AsRef<Path>) -> Result<Version> {
        let package = load_package(package_dir.as_ref())?;

        Ok(package.version)
    }

    pub fn published_version(package_dir: impl AsRef<Path>) -> Result<Version> {
        let package = load_package(package_dir.as_ref())?;

        let version = Command::new("pnpm")
            .args(["view", package.name.as_str(), "version"])
            .evaluate()?;

        Ok(Version::parse(version.trim())?)
    }
}

fn load_package(package_dir: &Path) -> Result<Package> {
    let package_json = package_dir.join("package.json").read_to_string()?;

    let package = serde_json::from_str::<Package>(&package_json)?;

    Ok(package)
}

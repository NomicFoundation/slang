use std::path::Path;

use anyhow::Result;
use infra_utils::paths::PathExtensions;
use semver::Version;
use serde::Deserialize;

#[derive(Deserialize)]
struct Package {
    version: Version,
}

pub struct Npm;

impl Npm {
    pub fn local_version(package_dir: impl AsRef<Path>) -> Result<Version> {
        let package_json = package_dir.as_ref().join("package.json").read_to_string()?;
        let package = serde_json::from_str::<Package>(&package_json)?;
        Ok(package.version)
    }
}

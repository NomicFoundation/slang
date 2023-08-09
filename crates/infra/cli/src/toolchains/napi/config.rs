use std::path::Path;

use anyhow::{Context, Result};
use infra_utils::{commands::Command, paths::PathExtensions};
use semver::Version;
use serde::Deserialize;

use crate::toolchains::napi::resolver::NapiResolver;

#[derive(Deserialize)]
struct Package {
    name: String,
    version: Version,
    napi: Option<NapiEntry>,
}

#[derive(Deserialize)]
struct NapiEntry {
    triples: NapiTriples,
}

#[derive(Deserialize)]
struct NapiTriples {
    defaults: bool,
    additional: Vec<String>,
}

pub struct NapiConfig;

impl NapiConfig {
    pub fn local_version(package_dir: impl AsRef<Path>) -> Result<Version> {
        let package = load_package(package_dir.as_ref())?;

        return Ok(package.version);
    }

    pub fn published_version(package_dir: impl AsRef<Path>) -> Result<Version> {
        let package = load_package(package_dir.as_ref())?;

        let version = Command::new("npm")
            .args(["view", package.name.as_str(), "version"])
            .evaluate()?;

        return Ok(Version::parse(version.trim())?);
    }

    pub fn list_all_targets() -> Result<Vec<String>> {
        let package = load_package(&NapiResolver::main_package_dir())?;

        let triples = package
            .napi
            .context("Failed to find NAPI config section")?
            .triples;

        assert!(
            !triples.defaults,
            "Package should explicitly list targets, instead of using defaults."
        );

        return Ok(triples.additional.to_owned());
    }
}

fn load_package(package_dir: &Path) -> Result<Package> {
    let package_json = package_dir.join("package.json").read_to_string()?;

    let package = serde_json::from_str::<Package>(&package_json)?;

    return Ok(package);
}

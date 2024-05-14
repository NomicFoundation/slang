use std::path::Path;

use anyhow::{Context, Result};
use infra_utils::commands::Command;
use infra_utils::paths::PathExtensions;
use semver::Version;
use serde::Deserialize;

use crate::toolchains::napi::glibc::ZigGlibcVersion;
use crate::toolchains::napi::resolver::NapiResolver;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct Package {
    name: String,
    version: Version,
    napi: Option<NapiEntry>,
    slang_metadata: Option<SlangMetadata>,
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

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct SlangMetadata {
    target_glibc: ZigGlibcVersion,
}

pub struct NapiConfig;

impl NapiConfig {
    pub fn local_version(package_dir: impl AsRef<Path>) -> Result<Version> {
        let package = load_package(package_dir.as_ref())?;

        Ok(package.version)
    }

    pub fn published_version(package_dir: impl AsRef<Path>) -> Result<Version> {
        let package = load_package(package_dir.as_ref())?;

        let version = Command::new("npm")
            .args(["view", package.name.as_str(), "version"])
            .evaluate()?;

        Ok(Version::parse(version.trim())?)
    }

    pub fn list_all_targets(resolver: NapiResolver) -> Result<Vec<String>> {
        let package = load_package(&resolver.main_package_dir())?;

        let triples = package
            .napi
            .context("Failed to find NAPI config section")?
            .triples;

        assert!(
            !triples.defaults,
            "Package should explicitly list targets, instead of using defaults."
        );

        Ok(triples.additional)
    }

    /// Returns the target glibc version for the GNU targets.
    pub fn target_glibc(resolver: NapiResolver) -> Result<ZigGlibcVersion> {
        let package = load_package(&resolver.main_package_dir())?;

        Ok(package
            .slang_metadata
            .context("Failed to find NAPI config metadata section")?
            .target_glibc)
    }
}

fn load_package(package_dir: &Path) -> Result<Package> {
    let package_json = package_dir.join("package.json").read_to_string()?;

    let package = serde_json::from_str::<Package>(&package_json)?;

    Ok(package)
}

use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{bail, Context, Result};
use infra_utils::hash::sha256_hex_of_file;
use infra_utils::paths::PathExtensions;
use serde::{Deserialize, Serialize};

/// Filesystem layout for build-once / publish-from-artifact handoff.
/// Files in this tree are produced by `infra publish prepare` and consumed
/// (verified + uploaded) by `infra publish npm` and `infra publish cargo`.
pub struct ArtifactPaths;

impl ArtifactPaths {
    pub fn root() -> PathBuf {
        Path::repo_path("target/publish-artifacts")
    }

    pub fn npm_dir() -> PathBuf {
        Self::root().join("npm")
    }

    pub fn cargo_dir() -> PathBuf {
        Self::root().join("cargo")
    }

    pub fn manifest_path() -> PathBuf {
        Self::root().join("manifest.json")
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Manifest {
    pub workspace_version: String,
    pub npm: Option<NpmArtifact>,
    /// Cargo crates in dependency order: each crate's deps must already be
    /// uploaded before it can be uploaded.
    pub cargo: Vec<CargoArtifact>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct NpmArtifact {
    /// Relative to `target/publish-artifacts/`.
    pub path: String,
    pub sha256: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CargoArtifact {
    pub crate_name: String,
    pub version: String,
    /// `.crate` source tarball — relative to `target/publish-artifacts/`.
    pub crate_path: String,
    pub crate_sha256: String,
    /// Registry-publish JSON metadata — relative to `target/publish-artifacts/`.
    /// Built from the rewritten `Cargo.toml` `cargo package` produces.
    pub metadata_path: String,
    pub metadata_sha256: String,
}

impl Manifest {
    pub fn load() -> Result<Self> {
        let path = ArtifactPaths::manifest_path();
        let contents = fs::read_to_string(&path)
            .with_context(|| format!("Failed to read manifest: {path:?}"))?;
        Ok(serde_json::from_str(&contents)?)
    }

    pub fn save(&self) -> Result<()> {
        let path = ArtifactPaths::manifest_path();
        fs::create_dir_all(path.unwrap_parent())?;
        let json = serde_json::to_string_pretty(self)?;
        fs::write(&path, json).with_context(|| format!("Failed to write manifest: {path:?}"))
    }

    pub fn absolute_path(relative: &str) -> PathBuf {
        ArtifactPaths::root().join(relative)
    }

    /// Recompute SHA-256 for every recorded artifact and bail if any mismatch.
    /// Called by `npm` and `cargo` publish steps before uploading.
    pub fn verify_integrity(&self) -> Result<()> {
        if let Some(npm) = &self.npm {
            let abs = Self::absolute_path(&npm.path);
            let actual = sha256_hex_of_file(&abs)?;
            if actual != npm.sha256 {
                bail!(
                    "Integrity check failed for {abs:?}: expected {expected}, got {actual}",
                    expected = npm.sha256,
                );
            }
        }
        for entry in &self.cargo {
            for (rel, expected) in [
                (&entry.crate_path, &entry.crate_sha256),
                (&entry.metadata_path, &entry.metadata_sha256),
            ] {
                let abs = Self::absolute_path(rel);
                let actual = sha256_hex_of_file(&abs)?;
                if actual != *expected {
                    bail!(
                        "Integrity check failed for {abs:?}: expected {expected}, got {actual}",
                    );
                }
            }
        }
        Ok(())
    }
}

use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{bail, Context, Result};
use infra_utils::hash::sha256_hex_of_file;
use infra_utils::paths::PathExtensions;
use serde::{Deserialize, Serialize};

/// Filesystem layout for build-once / publish-from-artifact handoff.
/// Files in this tree are produced by `infra publish prepare` and consumed
/// (verified + uploaded) by `infra publish npm`.
///
/// The cargo side does *not* go through this artifact pipeline: `cargo publish
/// --no-verify` re-packages from the workspace source at publish time. That's
/// the handover doc's "Middle tier" — we accept that the `.crate` is regenerated
/// in exchange for not maintaining any custom cargo metadata / wire-format code.
pub struct ArtifactPaths;

impl ArtifactPaths {
    pub fn root() -> PathBuf {
        Path::repo_path("target/publish-artifacts")
    }

    pub fn npm_dir() -> PathBuf {
        Self::root().join("npm")
    }

    pub fn manifest_path() -> PathBuf {
        Self::root().join("manifest.json")
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Manifest {
    pub workspace_version: String,
    pub npm: Option<NpmArtifact>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct NpmArtifact {
    /// Relative to `target/publish-artifacts/`.
    pub path: String,
    pub sha256: String,
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
    /// Called by `infra publish npm` before uploading.
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
        Ok(())
    }
}

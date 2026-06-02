use std::env::var;
use std::path::{Path, PathBuf};

use anyhow::Result;
use infra_utils::cargo::CargoWorkspace;
use infra_utils::paths::PathExtensions;

/// Staging directory the `prepare` step packs publish artifacts into, and that the
/// review/publish jobs restore them to. Single source of truth for the location,
/// shared by `prepare` and the workflow's upload/download steps.
pub const DIR: &str = "target/publish-artifacts";

pub fn dir() -> PathBuf {
    Path::repo_path(DIR)
}

/// Marker written when nothing was packed; satisfies the artifact upload's
/// `if-no-files-found: error` so an all-skipped run still uploads something.
pub fn skipped_marker() -> PathBuf {
    Path::repo_path(format!("{DIR}/SKIPPED"))
}

/// Versioned + commit-tagged name for the uploaded workflow artifact.
pub fn workflow_artifact_name() -> Result<String> {
    let version = CargoWorkspace::local_version()?;
    let sha = var("GITHUB_SHA").unwrap_or_default();
    let short_sha = &sha[..sha.len().min(12)];

    Ok(format!("slang-publish-artifacts-{version}-{short_sha}"))
}

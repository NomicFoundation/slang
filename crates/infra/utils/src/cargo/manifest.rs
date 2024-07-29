use std::collections::HashMap;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use semver::Version;
use serde::Deserialize;
use url::Url;

use crate::paths::PathExtensions;

#[derive(Deserialize)]
pub struct WorkspaceManifest {
    pub workspace: Workspace,
}

#[derive(Deserialize)]
pub struct Workspace {
    pub package: WorkspacePackage,
    pub dependencies: HashMap<String, Dependency>,
}

#[derive(Deserialize)]
pub struct WorkspacePackage {
    pub version: Version,
}

#[derive(Deserialize)]
#[serde(untagged)]
pub enum Dependency {
    Local { path: PathBuf, version: Version },
    CratesIO { version: Version },
    GitBranch { git: Url, branch: String },
    GitRevision { git: Url, rev: String },
}

impl WorkspaceManifest {
    pub fn load() -> Result<Self> {
        let manifest_path = Path::repo_path("Cargo.toml");

        toml::from_str(&manifest_path.read_to_string()?)
            .with_context(|| format!("Failed to deserialize manifest: {manifest_path:?}"))
    }
}

use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

use anyhow::{Context, Result};
use semver::Version;
use serde::Deserialize;

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
pub struct Dependency {
    pub path: Option<PathBuf>,
    pub version: Option<Version>,
}

impl WorkspaceManifest {
    pub fn load() -> Result<Self> {
        let manifest_path = Path::repo_path("Cargo.toml");

        toml::from_str(&manifest_path.read_to_string()?)
            .with_context(|| format!("Failed to deserialize manifest: {manifest_path:?}"))
    }
}

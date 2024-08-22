use std::collections::HashMap;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use semver::Version;
use serde::Deserialize;
use url::Url;

use crate::paths::PathExtensions;

#[derive(Deserialize)]
pub struct WorkspaceManifest {
    workspace: Workspace,
}

#[derive(Deserialize)]
struct Workspace {
    package: WorkspacePackage,
    dependencies: HashMap<String, Dependency>,
}

#[derive(Deserialize)]
struct WorkspacePackage {
    version: Version,
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

    pub fn version(&self) -> &Version {
        &self.workspace.package.version
    }

    pub fn dependency(&self, dependency_name: impl AsRef<str>) -> Result<&Dependency> {
        let dependency_name = dependency_name.as_ref();

        self.workspace
            .dependencies
            .get(dependency_name)
            .with_context(|| format!("Cannot find dependency '{dependency_name}' in workspace."))
    }
}

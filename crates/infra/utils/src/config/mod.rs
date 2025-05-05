use std::fs;
use std::path::Path;

use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::paths::PathExtensions;

#[derive(Serialize, Deserialize, Debug)]
pub struct Project {
    pub hash: String,
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct File {
    pub hash: String,
    pub file: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Configuration {
    pub projects: Vec<Project>,
    pub files: Vec<File>,
}

pub fn read_config() -> Result<Configuration> {
    // Read hashes from a configuration file
    let config_path = Path::repo_path("crates/infra/cli/src/commands/perf/projects.json");
    let config_content = fs::read_to_string(config_path)?;
    let config: Configuration = serde_json::from_str(&config_content)?;
    Ok(config)
}

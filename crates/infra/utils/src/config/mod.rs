use std::fs;
use std::path::Path;

use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::paths::PathExtensions;

/*
 Reader for the configuration file that contians the projects and files to benchmark
*/

pub const WORKING_DIR: &str = "target/benchmarks-inputs";

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Configuration {
    pub projects: Vec<Project>,
    pub files: Vec<File>,
}

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

pub fn read_config() -> Result<Configuration> {
    let config_path = Path::repo_path("crates/infra/cli/src/commands/perf/projects.json");
    let config_content = fs::read_to_string(config_path)?;
    let config: Configuration = serde_json::from_str(&config_content)?;
    Ok(config)
}

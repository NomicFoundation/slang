use std::fs;
use std::path::{Path, PathBuf};

use anyhow::Result;
use infra_utils::cargo::CargoWorkspace;
use infra_utils::paths::PathExtensions;
use serde::{Deserialize, Serialize};

/*
 Reader for the configuration file that contians the projects and files to benchmark
*/
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
    pub name: String,
}

pub fn working_dir_path() -> PathBuf {
    Path::repo_path("target/benchmarks-inputs")
}

pub fn read_config() -> Result<Configuration> {
    let config_path = CargoWorkspace::locate_source_crate("solidity_testing_perf_cargo")?;
    let config_path = config_path.join("../projects.json");
    let config_content = fs::read_to_string(config_path)?;
    let config: Configuration = serde_json::from_str(&config_content)?;
    Ok(config)
}

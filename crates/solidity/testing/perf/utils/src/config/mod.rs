use std::fs;
use std::path::PathBuf;

use anyhow::Result;
use infra_utils::cargo::CargoWorkspace;
use serde::{Deserialize, Serialize};

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

pub fn config_file_path() -> Result<PathBuf> {
    let config_path = CargoWorkspace::locate_source_crate("solidity_testing_perf_cargo")?;
    Ok(config_path.join("../projects.json"))
}

pub fn read_config() -> Result<Configuration> {
    let path = config_file_path()?;
    let config_content = fs::read_to_string(path)?;
    let config: Configuration = serde_json::from_str(&config_content)?;
    Ok(config)
}

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
    /// Benchmark inputs that are not Sourcify-verified deployments, but raw
    /// `solc` standard JSON *inputs* fetched from a URL.
    #[serde(default)]
    pub standard_json_inputs: Vec<StandardJsonInput>,
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

/// A `solc` standard JSON input downloaded from a URL.
///
/// Unlike Sourcify metadata, a standard JSON input carries neither a compiler
/// version nor a fully qualified contract name, so both are supplied here.
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct StandardJsonInput {
    pub name: String,
    /// Must address immutable content (e.g. a raw GitHub URL pinned to a commit
    /// hash, never a branch): the fetched file is cached forever under
    /// `target/`, and a corpus that changes underneath us silently invalidates
    /// every past measurement.
    pub url: String,
    /// The version to compile with. Standard JSON inputs don't name one, so
    /// pick the lowest that satisfies every `pragma solidity` in the corpus.
    pub compiler_version: String,
    /// The source the v1 benchmarks start their import crawl from. The v2
    /// benchmarks ignore it and feed in every source.
    pub entrypoint: String,
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

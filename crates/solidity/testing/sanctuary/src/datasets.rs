use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use anyhow::Result;
use infra_utils::cargo::CargoWorkspace;
use infra_utils::commands::Command;
use infra_utils::paths::PathExtensions;
use serde::Deserialize;

use crate::chains::Chain;

/// This type prepares a 'sparse-checkout' of the repository, rather than cloning the whole thing (100k-1M files).
/// Then reads the 'contracts.json' index file from the repository and parses it to populate [`DataSet::directories`].
/// Contracts are grouped into directories from "00" to "ff" (first two chars in the contract's address):
///
/// ```yml
/// - "{repo_dir}/contracts/{network}":
///     - "00": ...
///     - "01": ...
///     - "02":
///         - "{address}_{name}.sol"
///         - "{address}_{name}.sol"
///         - "{address}_{name}.sol"
///     - ...
///     - ...
///     - "ff": ...
///     - "contracts.json"
/// ```
///
/// So we leverage this fact to check out each directory separately lazily, to allow for test sharding.
/// This is necessary for CI, where disk space is limited. Caller should call [`DataSet::directories`]
/// to get the list of directories, and [`DataSet::checkout_directory`] each in turn when needed.
pub(crate) struct DataSet {
    repo_dir: PathBuf,
    network: String,

    // Using a [`BTreeMap`] to maintain order of iteration over keys.
    directories: BTreeMap<String, Vec<SourceFile>>,
}

pub struct SourceFile {
    pub path: PathBuf,
    pub compiler: String,
}

impl DataSet {
    pub fn initialize(chain: &Chain) -> Result<Self> {
        let repo_dir = clone_repository(chain)?;
        let network = chain.network_name().to_owned();
        let directories = list_contracts(&repo_dir, &network)?;

        Ok(Self {
            repo_dir,
            network,
            directories,
        })
    }

    pub fn directories(&self) -> &BTreeMap<String, Vec<SourceFile>> {
        &self.directories
    }

    pub fn checkout_directory(&self, directory: &str) -> Result<&Vec<SourceFile>> {
        // Make sure to reset any local changes, in case some were made during local development/debugging:
        Command::new("git")
            .arg("reset")
            .flag("--hard")
            .current_dir(&self.repo_dir)
            .run()?;

        let relative_path = format!("contracts/{network}/{directory}", network = self.network);

        Command::new("git")
            .arg("sparse-checkout")
            .property("set", relative_path)
            .current_dir(&self.repo_dir)
            .run()?;

        Ok(&self.directories[directory])
    }
}

fn clone_repository(chain: &Chain) -> Result<PathBuf> {
    let repo_dir = CargoWorkspace::locate_source_crate("solidity_testing_sanctuary")?
        .join("target/datasets")
        .join(chain.name());

    if !repo_dir.exists() {
        Command::new("git")
            .arg("clone")
            .arg(chain.github_url()?)
            .arg(repo_dir.unwrap_str())
            .property("--depth", "1")
            .property("--filter", "blob:none")
            .flag("--no-checkout")
            .run()?;
    }

    Command::new("git")
        .arg("sparse-checkout")
        .property("set", "--cone")
        .current_dir(&repo_dir)
        .run()?;

    Command::new("git")
        .arg("checkout")
        .arg("origin/HEAD")
        .current_dir(&repo_dir)
        .run()?;

    Command::new("git")
        .arg("pull")
        .current_dir(&repo_dir)
        .run()?;

    Ok(repo_dir)
}

fn list_contracts(repo_dir: &Path, network: &str) -> Result<BTreeMap<String, Vec<SourceFile>>> {
    #[derive(Deserialize)]
    struct Contract {
        address: String,
        name: String,
        compiler: String,
    }

    // This is a newline-delimited stream of JSON objects:
    // Unfortunately 'sparse-checkout' doesn't support single files, only folders.
    // So we cannot checkout 'contracts.json' without all sibling contract folders.
    // So let's just read it from git index instead.
    let contracts_file = Command::new("git")
        .arg("show")
        .arg(format!(":contracts/{network}/contracts.json"))
        .current_dir(repo_dir)
        .evaluate()?;

    let mut directories = BTreeMap::new();

    for line in contracts_file.lines() {
        let Contract {
            mut address,
            name,
            compiler,
        } = serde_json::from_str(line)?;

        // Strip the inconsistent prefix if it exists:
        if address.starts_with("0x") || address.starts_with("0X") {
            address = address[2..].to_string();
        }

        assert!(
            address.len() == 40 && address.chars().all(|c| c.is_ascii_hexdigit()),
            "Invalid address: '{address}'"
        );

        // Contracts are grouped into directories from "00" to "ff" (first two chars in the address):
        let directory = address[0..2].to_lowercase();

        let path = repo_dir
            .join("contracts")
            .join(network)
            .join(&directory)
            .join(format!("{address}_{name}.sol"));

        directories
            .entry(directory)
            .or_insert_with(Vec::new)
            .push(SourceFile {
                path,
                compiler: compiler.to_owned(),
            });
    }

    Ok(directories)
}

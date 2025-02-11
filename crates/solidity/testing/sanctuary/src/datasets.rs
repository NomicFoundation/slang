use std::collections::{BTreeMap, HashSet};
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use infra_utils::cargo::CargoWorkspace;
use infra_utils::commands::Command;
use infra_utils::paths::PathExtensions;
use once_cell::sync::Lazy;
use regex::Regex;
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

    pub fn checkout_directory(&self, directory: &str) -> &Vec<SourceFile> {
        // Make sure to reset any local changes, in case some were made during local development/debugging:
        Command::new("git")
            .arg("reset")
            .flag("--hard")
            .current_dir(&self.repo_dir)
            .run();

        let relative_path = format!("contracts/{network}/{directory}", network = self.network);

        Command::new("git")
            .arg("sparse-checkout")
            .property("set", relative_path)
            .current_dir(&self.repo_dir)
            .run();

        &self.directories[directory]
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
            .run();
    }

    Command::new("git")
        .arg("sparse-checkout")
        .property("set", "--cone")
        .current_dir(&repo_dir)
        .run();

    Command::new("git")
        .arg("checkout")
        .arg("origin/HEAD")
        .current_dir(&repo_dir)
        .run();

    Command::new("git").arg("pull").current_dir(&repo_dir).run();

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
    let contracts_file_path = format!("contracts/{network}/contracts.json");
    let contracts_file = Command::new("git")
        .arg("show")
        .arg(format!(":{contracts_file_path}"))
        .current_dir(repo_dir)
        .evaluate()?;

    let mut directories = BTreeMap::new();

    let skip_lines = &[
        // git conflict markers:
        Regex::new(r#"^<<<<<<< HEAD$"#)?,
        Regex::new(r#"^=======$"#)?,
        Regex::new(r#"^>>>>>>> [0-9a-f]{40}$"#)?,
    ];

    for line in contracts_file.lines() {
        if skip_lines.iter().any(|pattern| pattern.is_match(line)) {
            continue;
        }

        let Contract {
            mut address,
            name,
            mut compiler,
        } = serde_json::from_str(line).with_context(|| {
            format!("Failed to parse contract JSON line from {contracts_file_path:?}: {line:?}")
        })?;

        // Strip the inconsistent prefix if it exists:
        if address.starts_with("0x") || address.starts_with("0X") {
            address = address[2..].to_string();
        }

        // Patch the version for the contract if it's known to be wrong:
        patch_possibly_buggy_version_from_etherscan(network, &address, &mut compiler);

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
                compiler: compiler.clone(),
            });
    }

    Ok(directories)
}

// For some contracts, the version of the Solidity compiler used to compile them
// is wrong, i.e. it declares a version that will not compile the contract.
//
// This function patches the version for the contract if it's known to be wrong.
fn patch_possibly_buggy_version_from_etherscan(network: &str, address: &str, version: &mut String) {
    // We only know of buggy contracts on the mainnet:
    if network != "mainnet" {
        return;
    }

    #[allow(clippy::items_after_statements)]
    // These all use `pure`/`view`, which was introduced in 0.4.16:
    // TODO(#934): Remove this once the sanctuary index update is propagated.
    static PURE_VIEW_CONTRACTS: Lazy<HashSet<&str>> = Lazy::new(|| {
        HashSet::from_iter([
            /* 0.4.11 */ "0ff7599a9e2c9eb63ddb42a0e8b475b579a13e08",
            /* 0.4.13 */ "123ab195dd38b1b40510d467a6a359b201af056f",
            /* 0.4.11 */ "18ecc2461dfd84c5ce9da581aca58919a8750ae5",
            /* 0.4.11 */ "40da24a66f729d0bd0453681c5a6506bdc2a7a6a",
            /* 0.4.15 */ "49ec146e6385777c41a8b4637fa4416eb667549b",
            /* 0.4.11 */ "52442275f6cd49bd2ce9a920ea13e2618a19b071",
            /* 0.4.13 */ "6b9d422cc05029d1324fc5b007aff49e2ab54882",
            /* 0.4.11 */ "c0c45cbb1dce225cf620c36807a1cdecb85feda5",
            /* 0.4.15 */ "cabdff9789c92ac0f8a02b820c3148f15b61ea9b",
            /* 0.4.11 */ "fe70ba4ed491316870a8e6ad1ea095fbe19bc0b2",
        ])
    });

    if PURE_VIEW_CONTRACTS.contains(address) {
        *version = "0.4.16".to_string();
    }
}

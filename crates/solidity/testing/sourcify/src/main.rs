use std::iter::Flatten;
use std::io::ErrorKind;
use std::path::PathBuf;
use std::fs::{self, ReadDir};

use anyhow::{bail, Error, Result};
use clap::{Parser, Subcommand, ValueEnum};
use reqwest::blocking::Client;
use semver::{BuildMetadata, Prerelease, Version};
use serde::Deserialize;
use slang_solidity::parser::Parser as SlangParser;
use strum_macros::AsRefStr;
use tar::Archive;

#[derive(Debug, Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Test(TestCommand),
}

#[derive(Debug, Parser)]
struct TestCommand {
    #[command(subcommand)]
    chain: Chain
}

#[derive(Subcommand, Debug, AsRefStr, PartialEq, Copy, Clone)]
#[strum(serialize_all = "lowercase")]
enum Chain {
    Ethereum{ network: EthereumNetwork },
}

impl Chain {
    fn id(self) -> u64 {
        match self {
            Chain::Ethereum{ network } => match network {
                EthereumNetwork::Mainnet => 1,
            }
        }
    }

    fn from_id(id: u64) -> Option<Chain> {
        match id {
            1 => Some(Chain::Ethereum { network: EthereumNetwork::Mainnet }),
            _ => None
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, ValueEnum, AsRefStr)]
enum EthereumNetwork {
    Mainnet,
}

fn main() -> Result<()> {
    let Cli { command } = Cli::parse();

    match command {
        Commands::Test(test_command) => run_test_command(&test_command),
    }
}

fn run_test_command(cmd: &TestCommand) -> Result<()> {
    let repo = Repository::new(cmd.chain)?;
    let repo_entries = repo.fetch_entries()?;

    for repo_entry in repo_entries {
        let archive = repo.fetch_archive(&repo_entry)?;

        for contract in archive {
            if !contract.metadata.is_solidity() {
                continue;
            }

            if let Ok(version) = contract.metadata.version() {
                // let contract_sources = vec![];
                for source in contract {
                    run_parser_tests(&source, &version)?;
                }
            } else {
                println!("{} is not a valid version", contract.metadata.compiler.version);
            }

        }
    }

    Ok(())
}

fn run_parser_tests(source: &str, version: &semver::Version) -> Result<()> {
    let parser = SlangParser::create(version.to_owned())?;

    let output = parser.parse_file_contents(source);

    if output.is_valid() {
        Ok(())
    } else {
        bail!("Parsing failed");
    }
}

#[derive(Deserialize)]
struct Manifest {
    files: Vec<ManifestFile>
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct ManifestFile {
    path: String,
}

#[derive(Deserialize, Clone)]
struct ContractVersion {
    version: String,
}

#[derive(Deserialize, Clone)]
struct ContractMetadata {
    compiler: ContractVersion,
    language: String,
}

impl ContractMetadata {
    fn version(&self) -> Result<Version> {
        let mut version = Version::parse(&self.compiler.version)?;
        // Slang doesn't use prereleases or build IDs, but Sourcify will include them when it logs
        // the solidity version used to compile the contract.
        version.pre = Prerelease::EMPTY;
        version.build = BuildMetadata::EMPTY;
        Ok(version)
    }

    fn is_solidity(&self) -> bool {
        self.language.eq_ignore_ascii_case("solidity")
    }
}

struct RepoEntry {
    path: String,
    chain: Chain,
    prefix: u16,
}

impl RepoEntry {
    fn is_chain(&self, chain: Chain) -> bool {
        self.chain == chain
    }
}

impl TryFrom<&ManifestFile> for RepoEntry {
    type Error = anyhow::Error;

    fn try_from(file: &ManifestFile) -> Result<RepoEntry> {
        // File path should come in this format: 
        // /sourcify-repository-2025-03-24T03-00-26/full_match.1.00.tar.gz
        //                                                     - --
        //                                                     | | shard prefix
        //                                                     | chain ID
        let mut parts = file.path.split('.');
        let name_prefix = parts.next().unwrap();
        if !name_prefix.ends_with("match") {
            bail!("Invalid entry name format: {}", file.path);
        }

        let chain_id_part = parts.next().ok_or(Error::msg("Failed to get chain ID"))?;
        let chain_id: u64 = chain_id_part.parse()?;
        let chain = Chain::from_id(chain_id).ok_or(Error::msg(format!("Invalid chain id: {chain_id}")))?;

        let prefix_part = parts.next().ok_or(Error::msg("Failed to get shard prefix"))?;
        let prefix = u16::from_str_radix(prefix_part, 16)?;

        Ok(RepoEntry{
            path: file.path.clone(),
            chain,
            prefix,
        })
    }
}

struct Repository {
    path: PathBuf,
    chain: Chain
}

impl Repository {
    fn new(chain: Chain) -> Result<Repository> {
        let temp_dir_path = std::env::temp_dir().join(format!("sourcify_{}", chain.id()));

        if let Err(err) = fs::create_dir_all(&temp_dir_path) {
            if err.kind() == ErrorKind::AlreadyExists {
                fs::remove_dir_all(&temp_dir_path)?;
                fs::create_dir_all(&temp_dir_path)?;
            } else {
                return Err(Error::new(err));
            }
        }

        Ok(Repository{
            chain,
            path: temp_dir_path,
        })
    }

    fn fetch_entries(&self) -> Result<Vec<RepoEntry>> {
        let client = Client::new();
        let res = client.get("https://repo-backup.sourcify.dev/manifest.json").send()?;

        let status = res.status();
        if !status.is_success() {
            bail!("Error fetching manifest.json");
        }

        let body = res.text()?;
        let manifest: Manifest = serde_json::from_str(&body)?;

        Ok(manifest.files.iter().filter_map(|f| {
            if let Ok(repo_entry) = RepoEntry::try_from(f) {
                if repo_entry.is_chain(self.chain) {
                    Some(repo_entry)
                } else {
                    None
                }
            } else {
                None
            }
        }).collect())
    }

    fn fetch_archive(&self, repo_entry: &RepoEntry) -> Result<ContractArchive> {
        let client = Client::new();
        let res = client.get(format!("https://repo-backup.sourcify.dev{}", repo_entry.path)).send()?;

        let status = res.status();
        if !status.is_success() {
            bail!("Could not fetch source tarball");
        }

        let repo_dir = self.path.join(format!("{}", repo_entry.prefix));
        fs::create_dir(&repo_dir).inspect_err(|e| eprintln!("Failed to create directory {}: {e}", repo_dir.to_str().unwrap()))?;

        let mut archive = Archive::new(res);
        archive.unpack(&repo_dir)?;

        let contracts_path = repo_dir.join(format!("repository/full_match/{}", repo_entry.chain.id()));

        Ok(ContractArchive{
            path: contracts_path,
        })
    }
}

impl Drop for Repository {
    fn drop(&mut self) {
        fs::remove_dir_all(&self.path).expect("Failed to remove Repository directory.");
    }
}

struct ContractArchive {
    path: PathBuf,
}

impl IntoIterator for ContractArchive {
    type IntoIter = ContractArchiveIterator;
    type Item = <ContractArchiveIterator as Iterator>::Item;

    fn into_iter(self) -> Self::IntoIter {
        ContractArchiveIterator::new(&self).unwrap()
    }
}

struct ContractArchiveIterator {
    contracts: Flatten<ReadDir>,
}

impl ContractArchiveIterator {
    fn new(archive: &ContractArchive) -> Result<ContractArchiveIterator> {
        let contracts = fs::read_dir(&archive.path)?.flatten();

        Ok(ContractArchiveIterator{
            contracts,
        })
    }

    fn next_item(&mut self) -> Result<Option<ContractArchiveIteratorItem>> {
        if let Some(next_contract) = self.contracts.next() {
            let contract_path = next_contract.path();
            let metadata_file = fs::File::open(contract_path.join("metadata.json"))?;
            let metadata = serde_json::from_reader(metadata_file)?;

            Ok(Some(ContractArchiveIteratorItem{
                metadata,  
                path: contract_path,
            }))
        } else {
            Ok(None)
        }
    }
}

impl Iterator for ContractArchiveIterator {
    type Item = ContractArchiveIteratorItem;

    fn next(&mut self) -> Option<Self::Item> {
        self.next_item().inspect_err(|e| eprintln!("Failed to get next contract source: {e}")).unwrap_or(None)
    }
}

struct ContractArchiveIteratorItem {
    path: PathBuf,
    metadata: ContractMetadata,
}

impl IntoIterator for ContractArchiveIteratorItem {
    type IntoIter = SourceArchiveIterator;
    type Item = <SourceArchiveIterator as Iterator>::Item;

    fn into_iter(self) -> Self::IntoIter {
        SourceArchiveIterator::new(&self).expect("Failed to create SourceArchiveIterator")
    }
}

struct SourceArchiveIterator {
    sources: Flatten<ReadDir>,
}

impl SourceArchiveIterator {
    fn new(contract_archive: &ContractArchiveIteratorItem) -> Result<SourceArchiveIterator> {
        let sources = fs::read_dir(&contract_archive.path)?.flatten();
        Ok(SourceArchiveIterator { sources })
    }

    fn next_item(&mut self) -> Result<Option<String>> {
        if let Some(source_file) = self.sources.next() {
            let source = fs::read_to_string(source_file.path())?;
            Ok(Some(source))
        } else {
            Ok(None)
        }
    }
}

impl Iterator for SourceArchiveIterator {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        self.next_item().inspect_err(|e| eprintln!("Could not get next source file: {e}")).unwrap_or(None)
    }
}

impl Drop for ContractArchive {
    fn drop(&mut self) {
        fs::remove_dir_all(&self.path).expect("Failed to remove unpacked contract archive.");
    }
}

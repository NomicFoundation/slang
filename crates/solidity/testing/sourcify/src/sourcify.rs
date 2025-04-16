use std::fs;
use std::io::{BufReader, ErrorKind, Read};
use std::path::{Path, PathBuf};

use anyhow::{bail, Error, Result};
use reqwest::blocking::Client;
use semver::Version;
use serde::Deserialize;
use slang_solidity::compilation::CompilationUnit;
use tar::Archive;

use crate::chains::Chain;
use crate::command::ShardingOptions;
use crate::compilation_builder::CompilationBuilder;
use crate::metadata::ContractMetadata;

/// A `Repository` is used to fetch contracts from a certain chain. In addition to providing
/// an API for fetching contracts, it also manages a directory where fetched contract archives will
/// be written.
pub struct Repository {
    /// The chain that this Repository will fetch from
    chain: Chain,
    /// The parent path where fetched `ContractArchives` will be unpacked to.
    path: PathBuf,
    /// If `true`, then the files for this repository will not be deleted after the test is complete.
    should_save: bool,
}

impl Repository {
    pub fn new(chain: Chain, should_save: bool) -> Result<Repository> {
        let root = if should_save {
            PathBuf::from("target")
        } else {
            std::env::temp_dir()
        };

        let path = root.join(format!("sourcify_{}", chain.id()));
        create_or_replace_dir(&path)?;

        Ok(Repository {
            chain,
            path,
            should_save,
        })
    }

    /// Fetch shard info for the current chain.
    pub fn fetch_shards(&self, options: &ShardingOptions) -> Result<Vec<Shard>> {
        let client = Client::new();
        let res = client
            .get("https://repo-backup.sourcify.dev/manifest.json")
            .send()?;

        let status = res.status();
        if !status.is_success() {
            bail!("Error fetching manifest.json");
        }

        let manifest: Manifest = res.json()?;
        Ok(manifest.get_chain_shards(self.chain, options))
    }

    /// Fetch the `ContractArchive` data for this shard. Data is recieved as a tar archive, and is unpacked
    /// into the directory created by this `Repository`.
    pub fn fetch_archive(&self, shard: &Shard, chain: Chain) -> Result<ContractArchive> {
        let client = Client::new();
        let res = client
            .get(format!("https://repo-backup.sourcify.dev{}", shard.path))
            .send()?;

        let status = res.status();
        if !status.is_success() {
            bail!("Could not fetch source tarball");
        }

        let repo_dir = self.path.join(format!("{}", shard.id));
        create_or_replace_dir(&repo_dir).inspect_err(|e| {
            eprintln!(
                "Failed to create directory {}: {e}",
                repo_dir.to_str().unwrap()
            );
        })?;

        let mut archive = Archive::new(res);
        archive.unpack(&repo_dir)?;

        Ok(ContractArchive {
            id: shard.id,
            match_type: shard.match_type,
            contracts_path: repo_dir.join(format!(
                "repository/{}/{}",
                shard.match_type.dir_name(),
                chain.id()
            )),
            should_save: self.should_save,
        })
    }

    fn clean(&self) {
        if !self.should_save {
            fs::remove_dir_all(&self.path).unwrap();
        }
    }
}

impl Drop for Repository {
    fn drop(&mut self) {
        self.clean();
    }
}

#[derive(Deserialize)]
struct Manifest {
    files: Vec<ManifestFile>,
}

impl Manifest {
    fn get_chain_shards(&self, chain: Chain, options: &ShardingOptions) -> Vec<Shard> {
        let mut shards: Vec<_> = self
            .files
            .iter()
            .filter_map(|manifest| add_from_manifest(manifest, options, chain).ok())
            .collect();

        shards.sort_by(|a, b| a.id.cmp(&b.id));
        shards
    }
}

fn add_from_manifest(
    file: &ManifestFile,
    options: &ShardingOptions,
    chain: Chain,
) -> Result<Shard> {
    // File path should come in this format:
    // /sourcify-repository-2025-03-24T03-00-26/full_match.1.00.tar.gz
    //                                                     - --
    //                                                     | | shard prefix
    //                                                     | chain ID
    let mut parts = file.path.split('.');
    let name_prefix = parts.next().unwrap();
    let match_type = if name_prefix.ends_with("full_match") {
        MatchType::Full
    } else if name_prefix.ends_with("partial_match") {
        MatchType::Partial
    } else {
        bail!("Invalid match type in archive path: {}", file.path);
    };

    if match_type == MatchType::Partial && options.exclude_partials {
        bail!("Partials are excluded");
    }

    let chain_id_part = parts.next().ok_or(Error::msg("Failed to get chain ID"))?;
    let chain_id: u64 = chain_id_part.parse()?;

    if chain_id != chain.id() {
        bail!("Wrong chain");
    }

    let id_part = parts
        .next()
        .ok_or(Error::msg("Failed to get shard prefix"))?;
    let id = u16::from_str_radix(id_part, 16)?;

    if !options.get_id_range().contains(&id) {
        bail!("Shard not part of selected range.");
    }

    Ok(Shard {
        path: file.path.clone(),
        id,
        match_type,
    })
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct ManifestFile {
    path: String,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum MatchType {
    Full,
    Partial,
}

impl MatchType {
    pub fn dir_name(self) -> &'static str {
        match self {
            MatchType::Full => "full_match",
            MatchType::Partial => "partial_match",
        }
    }
}

pub struct Shard {
    pub id: u16,
    pub match_type: MatchType,
    /// A URL path used to fetch the `ContractArchive` for this shard.
    pub path: String,
}

#[derive(Clone)]
pub struct ContractArchive {
    pub id: u16,
    pub match_type: MatchType,
    contracts_path: PathBuf,
    should_save: bool,
}

impl ContractArchive {
    pub fn contracts(&self) -> impl Iterator<Item = Contract> {
        let dir = fs::read_dir(&self.contracts_path).expect("Could not open contract directory.");
        dir.flatten().flat_map(|dir_entry| -> Result<Contract> {
            let contract_path = dir_entry.path();
            let contract = Contract::new(&contract_path)?;

            Ok(contract)
        })
    }

    pub fn get_contract(&self, contract_id: &str) -> Result<Contract> {
        let contract_path = self.contracts_path.join(contract_id);
        Contract::new(&contract_path)
    }

    pub fn contract_count(&self) -> usize {
        fs::read_dir(&self.contracts_path)
            .map(|i| i.count())
            .unwrap_or(0)
    }

    pub fn clean(&self) {
        if !self.should_save {
            let _ = fs::remove_dir_all(&self.contracts_path);
        }
    }
}

impl Drop for ContractArchive {
    fn drop(&mut self) {
        self.clean();
    }
}

/// A single contract, found inside a `ContractArchive`. Source files for this contract have not been read or
/// processed, but can be found in the directory at `sources_path`.
pub struct Contract {
    pub name: String,
    pub metadata: ContractMetadata,
    sources_path: PathBuf,
}

impl Contract {
    fn new(contract_path: &Path) -> Result<Contract> {
        let name = contract_path
            .file_name()
            .unwrap()
            .to_str()
            .ok_or(Error::msg("Could not get contract directory name"))?;

        let metadata_file = fs::File::open(contract_path.join("metadata.json"))?;
        let reader = BufReader::new(metadata_file);

        let metadata = serde_json::from_reader(reader)?;

        let sources_path = contract_path.join("sources");

        Ok(Contract {
            metadata,
            sources_path,
            name: name.into(),
        })
    }

    /// Create a `CompilationUnit` for this contract. This includes all available source files and resolves
    /// imports, accounting for file remapping/renaming. The resulting `CompilationUnit` is ready to check for
    /// errors.
    pub fn create_compilation_unit(&self) -> Result<CompilationUnit> {
        let mut builder = CompilationBuilder::new(self)?;
        builder.create_compilation_unit()
    }

    pub fn entrypoint(&self) -> Result<String> {
        self.metadata.get_real_name(&self.metadata.target)
    }

    // We want to use `file.read_to_string` instead of `File::read_to_string` because the
    // former allows us to reuse a read buffer, meaning fewer allocations while processing
    // contracts.
    #[allow(clippy::verbose_file_reads)]
    pub fn read_file(&self, name: &str, buffer: &mut String) -> Result<usize> {
        let mut file = fs::File::open(self.sources_path.join(name))?;
        file.read_to_string(buffer).map_err(Error::new)
    }

    pub fn sources_count(&self) -> usize {
        fs::read_dir(&self.sources_path)
            .map(|i| i.count())
            .unwrap_or(0)
    }

    pub fn version(&self) -> Version {
        self.metadata.version.clone()
    }
}

/// Helper function - create a directory, and if the directory already exits, delete the existing
/// one and recreate it.
fn create_or_replace_dir(path: &PathBuf) -> std::io::Result<()> {
    if let Err(err) = fs::create_dir_all(path) {
        if err.kind() == ErrorKind::AlreadyExists {
            fs::remove_dir_all(path)?;
            fs::create_dir_all(path)?;
        } else {
            return Err(err);
        }
    }

    Ok(())
}

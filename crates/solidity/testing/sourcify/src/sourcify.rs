use std::iter::Flatten;
use std::io::{BufReader, ErrorKind, Read};
use std::path::PathBuf;
use std::fs::{self, File, ReadDir};

use crate::chains::Chain;
use crate::command::ShardingOptions;
use crate::compilation_builder::CompilationBuilder;
use crate::metadata::ContractMetadata;

use anyhow::{bail, Error, Result};
use reqwest::blocking::Client;
use semver::Version;
use serde::Deserialize;
use slang_solidity::compilation::{AddFileResponse, CompilationUnit, InternalCompilationBuilder};
use tar::Archive;

/// A `Repository` is used to fetch contracts from a certain chain. In addition to providing
/// an API for fetching contracts, it also manages a directory where fetched contract archives will
/// be written.
pub struct Repository {
    /// The chain that this Repository will fetch from
    chain: Chain,
    /// The parent path where fetched `ContractArchives` will be unpacked to.
    path: PathBuf,
}

impl Repository {
    pub fn new(chain: Chain) -> Result<Repository> {
        let temp_dir_path = std::env::temp_dir().join(format!("sourcify_{}", chain.id()));
        create_or_overwrite_dir(&temp_dir_path)?;

        Ok(Repository{
            chain,
            path: temp_dir_path,
        })
    }

    pub fn fetch_entries(&self, options: &ShardingOptions) -> Result<Vec<Shard>> {
        let client = Client::new();
        let res = client.get("https://repo-backup.sourcify.dev/manifest.json").send()?;

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
        let res = client.get(format!("https://repo-backup.sourcify.dev{}", shard.path)).send()?;

        let status = res.status();
        if !status.is_success() {
            bail!("Could not fetch source tarball");
        }

        let repo_dir = self.path.join(format!("{}", shard.id));
        create_or_overwrite_dir(&repo_dir).inspect_err(|e| eprintln!("Failed to create directory {}: {e}", repo_dir.to_str().unwrap()))?;

        let mut archive = Archive::new(res);
        archive.unpack(&repo_dir)?;

        Ok(ContractArchive{
            id: shard.id,
            contracts_path: repo_dir.join(format!("repository/{}/{}", shard.match_type.dir_name(), chain.id())),
        })
    }
}

impl Drop for Repository {
    fn drop(&mut self) {
        fs::remove_dir_all(&self.path).expect("Failed to remove Repository directory.");
    }
}

#[derive(Deserialize)]
struct Manifest {
    files: Vec<ManifestFile>,
}

impl Manifest {
    fn get_chain_shards(&self, chain: Chain, options: &ShardingOptions) -> Vec<Shard> {
        let mut shard_size = if let Some(shard_count) = options.shard_count {
            self.files.len() / shard_count
        } else {
            self.files.len()
        };

        if options.exclude_partials {
            shard_size /= 2;
        }

        let mut shards = Vec::with_capacity(shard_size);

        for file in &self.files {
            add_from_manifest(&mut shards, file, options, chain);
        }

        shards.sort_by(|a, b| a.id.cmp(&b.id));
        shards
    }
}

fn add_from_manifest(shards: &mut Vec<Shard>, file: &ManifestFile, options: &ShardingOptions, chain: Chain) -> Result<()> {
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
    
    if match_type.is_partial() && options.exclude_partials {
        return Ok(());
    }

    let chain_id_part = parts.next().ok_or(Error::msg("Failed to get chain ID"))?;
    let chain_id: u64 = chain_id_part.parse()?;

    if chain_id != chain.id() {
        return Ok(());
    }

    let id_part = parts.next().ok_or(Error::msg("Failed to get shard prefix"))?;
    let id = u16::from_str_radix(id_part, 16)?;

    if !options.get_id_range().contains(&id) {
        return Ok(())
    }

    let shard = Shard {
        path: file.path.clone(),
        id,
        chain,
        match_type,
    };

    shards.push(shard);

    Ok(())
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
    pub fn is_full(&self) -> bool {
        *self == MatchType::Full
    }

    pub fn is_partial(&self) -> bool {
        *self == MatchType::Partial
    }

    pub fn dir_name(&self) -> &'static str {
        match self {
            MatchType::Full => "full_match",
            MatchType::Partial => "partial_match",
        }
    }
}

pub struct Shard {
    /// The chain that this shard is part of.
    pub chain: Chain,
    /// A URL path used to fetch the `ContractArchive` for this shard.
    pub path: String,
    pub id: u16,
    pub match_type: MatchType,
}

#[derive(Clone)]
pub struct ContractArchive {
    pub id: u16,
    contracts_path: PathBuf,
}

impl ContractArchive {
    pub fn get_contract(&self, contract_id: &str) -> Result<Contract> {
        let contract_path = self.contracts_path.join(contract_id);
        Contract::new(&contract_path)
    }

    pub fn contracts_path(&self) -> String {
        self.contracts_path.to_str().unwrap().into()
    }

    pub fn contract_count(&self) -> usize {
        fs::read_dir(&self.contracts_path).map(|i| i.count()).unwrap_or(0)
    }

    pub fn clean(&self) -> Result<()> {
        Ok(fs::remove_dir_all(&self.contracts_path)?)
    }
}

impl IntoIterator for &ContractArchive {
    type IntoIter = ContractArchiveIterator;
    type Item = <ContractArchiveIterator as Iterator>::Item;

    fn into_iter(self) -> Self::IntoIter {
        ContractArchiveIterator::new(self).expect("Failed to create ContractArchiveIterator.")
    }
}

impl Drop for ContractArchive {
    fn drop(&mut self) {
        self.clean().unwrap();
    }
}

pub struct ContractArchiveIterator {
    contracts: Flatten<ReadDir>,
}

impl Iterator for ContractArchiveIterator {
    type Item = Result<Contract>;

    fn next(&mut self) -> Option<Self::Item> {
        self.next_contract()
    }
}

impl ContractArchiveIterator {
    fn new(archive: &ContractArchive) -> Result<ContractArchiveIterator> {
        let contracts = fs::read_dir(&archive.contracts_path)?.flatten();

        Ok(ContractArchiveIterator{
            contracts,
        })
    }

    pub fn next_contract(&mut self) -> Option<Result<Contract>> {
        match self._next_contract() {
            Ok(c) => match c {
                Some(contract) => Some(Ok(contract)),
                None => None,
            },
            Err(e) => Some(Err(e)),
        }
    }

    pub fn _next_contract(&mut self) -> Result<Option<Contract>> {
        if let Some(next_contract) = self.contracts.next() {
            let contract_path = next_contract.path();
            let contract = Contract::new(&contract_path)?;

            Ok(Some(contract))
        } else {
            Ok(None)
        }
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
    fn new(contract_path: &PathBuf) -> Result<Contract> {
        let name = contract_path.file_name().unwrap().to_str().ok_or(Error::msg("Could not get contract directory name"))?;

        let metadata_file = fs::File::open(contract_path.join("metadata.json"))?;
        let reader = BufReader::new(metadata_file);

        let metadata = serde_json::from_reader(reader)?;

        let sources_path = contract_path.join("sources");

        Ok(Contract{
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

    pub fn read_file(&self, name: &str, buffer: &mut String) -> Result<usize> {
        let mut file = fs::File::open(self.sources_path.join(name))?;
        file.read_to_string(buffer).map_err(Error::new)
    }

    pub fn sources_count(&self) -> usize {
        fs::read_dir(&self.sources_path).map(|i| i.count()).unwrap_or(0)
    }

    pub fn version(&self) -> Version {
        self.metadata.version.clone()
    }
}

impl IntoIterator for &Contract {
    type IntoIter = SourceArchiveIterator;
    type Item = <SourceArchiveIterator as Iterator>::Item;

    fn into_iter(self) -> Self::IntoIter {
        SourceArchiveIterator::new(self).expect("Failed to create SourceArchiveIterator.")
    }
}

pub struct SourceArchiveIterator {
    sources: Flatten<ReadDir>,
}

impl Iterator for SourceArchiveIterator {
    type Item = Result<SourceFile>;

    fn next(&mut self) -> Option<Self::Item> {
        self.next_source()        
    }
}

impl SourceArchiveIterator {
    fn new(contract_archive: &Contract) -> Result<SourceArchiveIterator> {
        let sources = fs::read_dir(&contract_archive.sources_path)?.flatten();
        Ok(SourceArchiveIterator { sources })
    }

    fn next_source(&mut self) -> Option<Result<SourceFile>> {
        match self._next_source() {
            Ok(s) => match s {
                Some(source) => Some(Ok(source)),
                None => None,
            },
            Err(e) => Some(Err(e)),
        }
    }

    fn _next_source(&mut self) -> Result<Option<SourceFile>> {
        if let Some(source_file) = self.sources.next() {
            let name = source_file.file_name().into_string().map_err(|_| Error::msg("Failed to get file name"))?; 

            let source = fs::File::open(source_file.path()).inspect_err(|e| eprintln!("Could not open file {name}: {e}"))?;
            Ok(Some(SourceFile{
                name,
                file: source,
            }))
        } else {
            Ok(None)
        }
        
    }
}

pub struct SourceFile {
    pub name: String,
    file: File,
}

/// Helper function - create a directory, and if the directory already exits, delete the existing
/// one and recreate it.
fn create_or_overwrite_dir(path: &PathBuf) -> std::io::Result<()> {
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

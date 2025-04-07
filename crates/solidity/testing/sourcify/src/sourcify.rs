use std::iter::Flatten;
use std::io::{BufReader, ErrorKind, Read};
use std::path::PathBuf;
use std::fs::{self, ReadDir};

use crate::chains::Chain;
use crate::metadata::ContractMetadata;

use anyhow::{bail, Error, Result};
use reqwest::blocking::Client;
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

    pub fn fetch_entries(&self) -> Result<Vec<Shard>> {
        let client = Client::new();
        let res = client.get("https://repo-backup.sourcify.dev/manifest.json").send()?;

        let status = res.status();
        if !status.is_success() {
            bail!("Error fetching manifest.json");
        }

        let manifest: Manifest = res.json()?;
        Ok(manifest.get_chain_shards(self.chain))
    }

    /// Fetch the `ContractArchive` data for this shard. Data is recieved as a tar archive, and is unpacked
    /// into the directory created by this `Repository`.
    pub fn fetch_archive(&self, shard: &Shard) -> Result<ContractArchive> {
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
            contracts_path: repo_dir.join(format!("repository/full_match/{}", shard.chain.id())),
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
    fn get_chain_shards(&self, chain: Chain) -> Vec<Shard> {
        self.files.iter().filter_map(|file| {
            if let Ok(shard) = Shard::try_from(file) {
                if shard.is_chain(chain) {
                    return Some(shard);
                }
            }

            None
        }).collect()
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct ManifestFile {
    path: String,
}

pub struct Shard {
    /// A URL path used to fetch the `ContractArchive` for this shard.
    path: String,
    /// The chain that this shard is part of.
    chain: Chain,
    id: u16,
}

impl Shard {
    fn is_chain(&self, chain: Chain) -> bool {
        self.chain == chain
    }
}

impl TryFrom<&ManifestFile> for Shard {
    type Error = anyhow::Error;

    fn try_from(file: &ManifestFile) -> Result<Shard> {
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

        let id_part = parts.next().ok_or(Error::msg("Failed to get shard prefix"))?;
        let id = u16::from_str_radix(id_part, 16)?;

        Ok(Shard{
            id,
            path: file.path.clone(),
            chain,
        })
    }
}

#[derive(Clone)]
pub struct ContractArchive {
    contracts_path: PathBuf,
}

impl ContractArchive {
    pub fn contracts_path(&self) -> String {
        self.contracts_path.to_str().unwrap().into()
    }

    pub fn contract_count(&self) -> usize {
        fs::read_dir(&self.contracts_path).map(|i| i.count()).unwrap_or(0)
    }

    pub fn contract_iter(&self) -> ContractArchiveIterator {
        ContractArchiveIterator::new(self).expect("Failed to create ContractArchiveIterator")
    }

    pub fn clean(&self) -> Result<()> {
        Ok(fs::remove_dir_all(&self.contracts_path)?)
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

pub struct Buffers {
    source: String,
    metadata: String,
}

impl Buffers {
    pub fn new() -> Buffers {
        Buffers {
            source: String::new(),
            metadata: String::new(),
        }
    }

    fn clear(&mut self) {
        self.source.clear();
        self.metadata.clear();
    }
}

impl ContractArchiveIterator {
    fn new(archive: &ContractArchive) -> Result<ContractArchiveIterator> {
        let contracts = fs::read_dir(&archive.contracts_path)?.flatten();

        Ok(ContractArchiveIterator{
            contracts,
        })
    }

    pub fn next_contract(&mut self, buffers: &mut Buffers) -> Option<Result<Contract>> {
        match self._next_contract(buffers) {
            Ok(c) => match c {
                Some(contract) => Some(Ok(contract)),
                None => None,
            },
            Err(e) => Some(Err(e)),
        }
    }

    pub fn _next_contract(&mut self, buffers: &mut Buffers) -> Result<Option<Contract>> {
        if let Some(next_contract) = self.contracts.next() {
            buffers.clear();

            let contract_path = next_contract.path();
            let name = next_contract.file_name().into_string().map_err(|_| Error::msg("Could not get contract directory name"))?;

            let mut metadata_file = fs::File::open(contract_path.join("metadata.json"))?;
            metadata_file.read_to_string(&mut buffers.metadata);

            let metadata = serde_json::from_str(&buffers.metadata)?;

            let sources_path = contract_path.join("sources");

            Ok(Some(Contract{
                metadata, 
                name,
                sources_path,
            }))
        } else {
            Ok(None)
        }
    }
}

/// A single contract, found inside a `ContractArchive`. Source files for this contract have not been read or 
/// processed, but can be found in the directory at `sources_path`.
pub struct Contract {
    pub name: String,
    sources_path: PathBuf,
    metadata: ContractMetadata,
}

impl Contract {
    /// Create a `CompilationUnit` for this contract. This includes all available source files and resolves
    /// imports, accounting for file remapping/renaming. The resulting `CompilationUnit` is ready to check for
    /// errors.
    pub fn create_compilation_unit(&self, buffers: &mut Buffers) -> Result<CompilationUnit> {
        let mut builder = InternalCompilationBuilder::create(self.metadata.version.clone())?;

        let mut source_iter = self.source_iter(&mut buffers.source);

        while let Some(s) = source_iter.next_source() {
            match s {
                Ok(source) => {
                    let AddFileResponse { import_paths } = builder.add_file(source.name.clone(), &source.source);
                    for import in import_paths {
                        let path = import.node().unparse();
                        let path = path
                            .strip_prefix(|c| matches!(c, '"' | '\''))
                            .unwrap()
                            .strip_suffix(|c| matches!(c, '"' | '\''))
                            .unwrap();

                        let target_filename = self.metadata.get_absolute_filename(&path);

                        builder.resolve_import(&source.name, &import, target_filename)?;
                    }
                },
                Err(e) => {
                    println!("Failed getting source: {e}");
                }
            }
        }

        Ok(builder.build())
    }

    pub fn sources_count(&self) -> usize {
        fs::read_dir(&self.sources_path).map(|i| i.count()).unwrap_or(0)
    }

    pub fn source_iter<'a>(&self, buffer: &'a mut String) -> SourceArchiveIterator<'a> {
        SourceArchiveIterator::new(self, buffer).expect("Failed to create SourceArchiveIterator")
    }
}

pub struct SourceArchiveIterator<'a> {
    sources: Flatten<ReadDir>,
    source_buffer: &'a mut String,
}

impl <'a> SourceArchiveIterator<'a> {
    fn new(contract_archive: &Contract, buffer: &'a mut String) -> Result<SourceArchiveIterator<'a>> {
        let sources = fs::read_dir(&contract_archive.sources_path)?.flatten();
        Ok(SourceArchiveIterator { sources, source_buffer: buffer, })
    }

    fn next_source(&mut self) -> Option<Result<SourceArchiveIteratorItem<'_>>> {
        match self._next_source() {
            Ok(s) => match s {
                Some(source) => Some(Ok(source)),
                None => None,
            },
            Err(e) => Some(Err(e)),
        }
    }

    fn _next_source(&mut self) -> Result<Option<SourceArchiveIteratorItem<'_>>> {
        if let Some(source_file) = self.sources.next() {
            let name = source_file.file_name().into_string().map_err(|_| Error::msg("Failed to get file name"))?; 

            self.source_buffer.clear();

            let mut source = fs::File::open(source_file.path()).inspect_err(|e| eprintln!("Could not open file {name}: {e}"))?;
            source.read_to_string(&mut self.source_buffer).inspect_err(|e| eprintln!("Failed to read file: {e}"))?;
            Ok(Some(SourceArchiveIteratorItem{
                name,
                source: &self.source_buffer,
            }))
        } else {
            Ok(None)
        }
        
    }
}

pub struct SourceArchiveIteratorItem<'it> {
    name: String,
    source: &'it str,
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

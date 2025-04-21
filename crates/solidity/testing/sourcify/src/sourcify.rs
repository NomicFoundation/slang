use std::fs;
use std::io::{BufReader, Read};
use std::path::PathBuf;

use anyhow::{bail, Error, Result};
use reqwest::blocking::Client;
use semver::{BuildMetadata, Prerelease, Version};
use slang_solidity::compilation::CompilationUnit;
use tar::Archive;

use crate::chains::{Chain, ChainId};
use crate::command::ShardingOptions;
use crate::compilation_builder::CompilationBuilder;
use crate::import_resolver::ImportResolver;

pub struct Manifest {
    /// Description of the archives that are available to fetch.
    archive_descriptors: Vec<ArchiveDescriptor>,
}

impl Manifest {
    pub fn new(chain: Chain, options: &ShardingOptions) -> Result<Manifest> {
        let client = Client::new();
        let res = client
            .get("https://repo-backup.sourcify.dev/manifest.json")
            .send()?;

        let status = res.status();
        if !status.is_success() {
            bail!("Error fetching manifest.json");
        }

        let obj: serde_json::Value = res.json()?;
        let archive_descs: Option<Vec<ArchiveDescriptor>> = obj.get("files")
            .and_then(|files| files.as_array())
            .map(|files| files
                .iter()
                .filter_map(|file| file.get("path").and_then(|val| val.as_str()))
                .filter_map(|path| ArchiveDescriptor::new(path).ok())
                .filter(|desc| desc.matches_chain_and_shard(chain, options))
                .collect()
            );

        if let Some(mut archive_descs) = archive_descs {
            if !archive_descs.is_empty() {
                archive_descs.sort_by(|a, b| a.shard_prefix.cmp(&b.shard_prefix));

                return Ok(Manifest{
                    archive_descriptors: archive_descs,
                });
            }
        }

        bail!("No valid archives found in manifest");
    }

    /// Search for a specific contract and return it if found. Returns `None` if the contract can not
    /// be fetched for any reason (including if the `contract_id` is not parseable).
    pub fn get_contract(&self, contract_id: &str) -> Option<Contract> {
        if let Ok(contract_shard_id) = u16::from_str_radix(contract_id.get(2..4).unwrap(), 16) {
            let archives = self.archive_descriptors
                .iter()
                .filter(|shard| shard.shard_prefix == contract_shard_id)
                .map(|desc| ContractArchive::new(desc))
                .flatten();

            for archive in archives {
                if let Ok(contract) = archive.get_contract(contract_id) {
                    return Some(contract);
                }
            }
        }

        None
    }

    pub fn archives(self) -> impl Iterator<Item = ContractArchive> {
        self.archive_descriptors.into_iter().map(|desc| ContractArchive::new(&desc)).flatten()
    }

    pub fn archive_count(&self) -> usize {
        self.archive_descriptors.len()
    }
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

/// Describes an archive that's available in the Sourcify repository.
/// Can be used by `ContractArchive::fetch()` to download this archive.
pub struct ArchiveDescriptor {
    pub shard_prefix: u16,
    pub chain_id: ChainId,
    pub match_type: MatchType,
    /// A URL path used to fetch the `ContractArchive` for this shard.
    pub path: String,
}

impl ArchiveDescriptor {
    fn new(path_str: &str) -> Result<ArchiveDescriptor> {
        // File path should come in this format:
        // /sourcify-repository-2025-03-24T03-00-26/full_match.1.00.tar.gz
        //                                                     - --
        //                                                     | | shard prefix
        //                                                     | chain ID
        let mut parts = path_str.split('.');
        let name_prefix = parts.next().unwrap();
        let match_type = if name_prefix.ends_with("full_match") {
            MatchType::Full
        } else if name_prefix.ends_with("partial_match") {
            MatchType::Partial
        } else {
            bail!("Invalid match type in archive path: {}", path_str);
        };

        let chain_id_part = parts.next().ok_or(Error::msg("Failed to get chain ID"))?;
        let chain_id: u64 = chain_id_part.parse()?;

        let shard_prefix_part = parts
            .next()
            .ok_or(Error::msg("Failed to get shard prefix"))?;
        let shard_prefix = u16::from_str_radix(shard_prefix_part, 16)?;

        Ok(ArchiveDescriptor {
            path: path_str.into(),
            shard_prefix,
            chain_id: ChainId(chain_id),
            match_type,
        })
    }

    fn matches_chain_and_shard(&self, chain: Chain, options: &ShardingOptions) -> bool {
        if self.match_type == MatchType::Partial && options.exclude_partial_matches {
            return false;
        }

        if self.chain_id != chain.id() {
            return false;
        }
        
        if !options.get_id_range().contains(&self.shard_prefix) {
            return false
        }

        true
    }

    /// Get a path that should be used as the target when unpacking the archive 
    /// represented by this `ArchiveDescriptor`.
    fn archive_dir(&self) -> PathBuf {
        PathBuf::from(format!(
            "target/sourcify_{chain_id}/{shard_prefix:02x}",
            chain_id = self.chain_id,
            shard_prefix = self.shard_prefix,
        ))
    }

    /// Get the path inside `self.archive_dir()` that contains all of the contracts. 
    /// This path is defined by the archives fetched from Sourcify, and should be updated 
    /// in case Sourcify ever changes its repository format.
    fn contracts_dir(&self) -> PathBuf {
        self.archive_dir().join(format!(
            "repository/{match_type}/{chain_id}",
            match_type = self.match_type.dir_name(),
            chain_id = self.chain_id,
        ))
    }
}

#[derive(Clone)]
pub struct ContractArchive {
    /// Path to the directory inside this archive which contains all of the contracts.
    /// Iterate over the entries at this path to read the contracts.
    contracts_path: PathBuf,
}

impl ContractArchive {
    pub fn new(desc: &ArchiveDescriptor) -> Result<ContractArchive> {
        let client = Client::new();
        let res = client
            .get(format!("https://repo-backup.sourcify.dev{}", desc.path))
            .send()?;

        let status = res.status();
        if !status.is_success() {
            bail!("Could not fetch source tarball");
        }

        let archive_dir = desc.archive_dir();

        let mut archive = Archive::new(res);
        archive.unpack(&archive_dir)?;

        Ok(ContractArchive {
            contracts_path: desc.contracts_dir(),
        })
    }

    pub fn contracts(&self) -> impl Iterator<Item = Contract> {
        let dir = fs::read_dir(&self.contracts_path).expect("Could not open contract directory.");
        dir.flatten().flat_map(|dir_entry| -> Result<Contract> {
            let contract_path = dir_entry.path();
            let contract = Contract::new(&contract_path)
                .inspect_err(|e| println!("Failed to create contract: {e}"))?;

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
        fs::remove_dir_all(&self.contracts_path).unwrap();
    }

    pub fn display_path(&self) -> String {
        self.contracts_path.to_str().unwrap().into()
    }
}

/// A single contract, found inside a `ContractArchive`. Source files for this contract have not been read or
/// processed, but can be found in the directory at `sources_path`.
pub struct Contract {
    pub name: String,
    pub version: Version,
    pub target: String,
    pub import_resolver: ImportResolver,

    sources_path: PathBuf,
}

impl Contract {
    fn new(contract_path: &PathBuf) -> Result<Contract> {
        let name = contract_path
            .file_name()
            .unwrap()
            .to_str()
            .ok_or(Error::msg("Could not get contract directory name"))?;

        let metadata_file = fs::File::open(contract_path.join("metadata.json"))?;
        let reader = BufReader::new(metadata_file);

        let metadata_val: serde_json::Value = serde_json::from_reader(reader)?;

        let version = metadata_val.get("compiler")
            .and_then(|compiler| compiler.get("version"))
            .and_then(|version_val| version_val.as_str())
            .ok_or(Error::msg("Could not get compiler.version from contract metadata"))
            .and_then(|version_str| Version::parse(version_str).map_err(Error::new))
            .map(|mut version| {
                version.pre = Prerelease::EMPTY;
                version.build = BuildMetadata::EMPTY;
                version
            })?;

        let target = metadata_val.get("settings")
            .and_then(|settings| settings.get("compilationTarget"))
            .and_then(|target| target.as_object())
            .and_then(|target_obj| target_obj.keys().next())
            .ok_or(Error::msg("Could not get settings.compilationTarget from contract metadata"))?
            .clone();

        let import_resolver: ImportResolver = metadata_val.try_into()?;

        Ok(Contract {
            target,
            version,
            import_resolver,
            sources_path: contract_path.join("sources"),
            name: name.into(),
        })
    }

    /// Create a `CompilationUnit` for this contract. This includes all available source files and resolves
    /// imports, accounting for file remapping/renaming. The resulting `CompilationUnit` is ready to check for
    /// errors.
    pub fn create_compilation_unit(&self) -> Result<CompilationUnit> {
        CompilationBuilder::new(self)?.build()
    }

    pub fn entrypoint(&self) -> Result<String> {
        self.import_resolver.get_real_name(&self.target)
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
}

use std::fs;
use std::io::BufReader;
use std::path::{Path, PathBuf};

use anyhow::{bail, Error, Result};
use infra_utils::cargo::CargoWorkspace;
use infra_utils::paths::PathExtensions;
use reqwest::blocking::Client;
use semver::{BuildMetadata, Prerelease, Version};
use slang_solidity::compilation::CompilationUnit;
use tar::Archive;

use crate::command::{ChainId, ShardingOptions};
use crate::compilation_builder::CompilationBuilder;
use crate::import_resolver::ImportResolver;

pub struct Manifest {
    /// Description of the archives that are available to fetch.
    archive_descriptors: Vec<ArchiveDescriptor>,
}

impl Manifest {
    pub fn new(chain_id: ChainId, options: &ShardingOptions) -> Result<Manifest> {
        let client = Client::new();
        let res = client
            .get("https://repo-backup.sourcify.dev/manifest.json")
            .send()?;

        let status = res.status();
        if !status.is_success() {
            bail!("Error fetching manifest.json");
        }

        let obj: serde_json::Value = res.json()?;
        let mut archive_descriptors: Vec<_> = obj
            .get("files")
            .and_then(|files| files.as_array())
            .map(|files| {
                files
                    .iter()
                    .filter_map(|file| file.get("path").and_then(|val| val.as_str()))
                    .filter_map(|path| {
                        ArchiveDescriptor::new("https://repo-backup.sourcify.dev", path).ok()
                    })
                    .filter(|desc| desc.matches_chain_and_shard(chain_id, options))
                    .collect()
            })
            .unwrap_or_default();

        archive_descriptors.sort_by(|a, b| a.prefix.cmp(&b.prefix));

        if archive_descriptors.is_empty() {
            return Err(Error::msg(format!(
                "No valid archive found for chain {chain_id}"
            )));
        }

        Ok(Manifest {
            archive_descriptors,
        })
    }

    /// Search for a specific contract and return it if found. Returns `None` if the contract can not
    /// be fetched for any reason (including if the `contract_id` is not parseable).
    pub fn fetch_contract(&self, contract_id: &str) -> Option<Contract> {
        u8::from_str_radix(contract_id.get(2..4).unwrap(), 16)
            .ok()
            .and_then(|contract_prefix| {
                self.archives()
                    .filter(|desc| desc.prefix == contract_prefix)
                    .flat_map(ContractArchive::fetch)
                    .find_map(|archive| archive.get_contract(contract_id).ok())
            })
    }

    pub fn archives(&self) -> impl Iterator<Item = &ArchiveDescriptor> {
        self.archive_descriptors.iter()
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
    pub prefix: u8,
    pub chain_id: ChainId,
    pub match_type: MatchType,
    /// GET this url to fetch the `ContractArchive` for this `ArchiveDescriptor`.
    pub url: String,
}

impl ArchiveDescriptor {
    fn new(base_url: &str, path_str: &str) -> Result<ArchiveDescriptor> {
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
        let chain_id: ChainId = chain_id_part.parse()?;

        let prefix_part = parts
            .next()
            .ok_or(Error::msg("Failed to get shard prefix"))?;
        let prefix = u8::from_str_radix(prefix_part, 16)?;

        Ok(ArchiveDescriptor {
            url: format!("{base_url}{path_str}"),
            prefix,
            chain_id,
            match_type,
        })
    }

    fn matches_chain_and_shard(&self, chain_id: ChainId, options: &ShardingOptions) -> bool {
        if self.match_type == MatchType::Partial && options.exclude_partial_matches {
            return false;
        }

        if self.chain_id != chain_id {
            return false;
        }

        if !options.get_id_range().contains(&self.prefix) {
            return false;
        }

        true
    }

    /// Get a path that should be used as the target when unpacking the archive
    /// represented by this `ArchiveDescriptor`.
    fn archive_dir(&self) -> PathBuf {
        CargoWorkspace::locate_source_crate("solidity_testing_sourcify")
            .unwrap_or_default()
            .join(format!(
                "target/sourcify_{chain_id}/{prefix:02x}",
                chain_id = self.chain_id,
                prefix = self.prefix,
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
    pub fn fetch(desc: &ArchiveDescriptor) -> Result<ContractArchive> {
        let client = Client::new();
        let res = client.get(&desc.url).send()?;

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

    pub fn clean(self) {
        fs::remove_dir_all(&self.contracts_path).unwrap();
    }

    pub fn display_path(&self) -> String {
        self.contracts_path
            .strip_repo_root()
            .unwrap_or(&self.contracts_path)
            .to_str()
            .unwrap()
            .into()
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
    fn new(contract_path: &Path) -> Result<Contract> {
        let name = contract_path
            .file_name()
            .unwrap()
            .to_str()
            .ok_or(Error::msg("Could not get contract directory name"))?;

        let metadata_file = fs::File::open(contract_path.join("metadata.json"))?;
        let reader = BufReader::new(metadata_file);

        let metadata_val: serde_json::Value = serde_json::from_reader(reader)?;

        let version = metadata_val
            .get("compiler")
            .and_then(|compiler| compiler.get("version"))
            .and_then(|version_val| version_val.as_str())
            .ok_or(Error::msg(
                "Could not get compiler.version from contract metadata",
            ))
            .and_then(|version_str| Version::parse(version_str).map_err(Error::new))
            .map(|mut version| {
                version.pre = Prerelease::EMPTY;
                version.build = BuildMetadata::EMPTY;
                version
            })?;

        let target = metadata_val
            .get("settings")
            .and_then(|settings| settings.get("compilationTarget"))
            .and_then(|target| target.as_object())
            .and_then(|target_obj| target_obj.keys().next())
            .ok_or(Error::msg(
                "Could not get settings.compilationTarget from contract metadata",
            ))?
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

    pub fn entrypoint(&self) -> Option<String> {
        self.import_resolver.get_source_id(&self.target)
    }

    pub fn read_file(&self, name: &str) -> Result<String> {
        self.sources_path.join(name).read_to_string()
    }

    pub fn sources_count(&self) -> usize {
        self.import_resolver.sources_count()
    }
}

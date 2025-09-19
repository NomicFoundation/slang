use std::fs::{self, File};
use std::path::{Path, PathBuf};
use std::time::{Duration, SystemTime};

use anyhow::{anyhow, bail, Error, Result};
use infra_utils::cargo::CargoWorkspace;
use infra_utils::paths::PathExtensions;
use reqwest::blocking::Client;
use semver::{BuildMetadata, Prerelease, Version};
use slang_solidity::compilation::{CompilationBuilder, CompilationBuilderConfig, CompilationUnit};
use solidity_testing_utils::import_resolver::{ImportRemap, ImportResolver, SourceMap};
use tar::Archive;

use crate::command::{ArchiveOptions, ChainId, ShardingOptions};

pub struct Manifest {
    /// Description of the archives that are available to fetch.
    archive_descriptors: Vec<ArchiveDescriptor>,
}

impl Manifest {
    pub fn new(
        chain_id: ChainId,
        options: &ShardingOptions,
        archive_options: &ArchiveOptions,
    ) -> Result<Manifest> {
        let obj = Self::obtain_manifest(archive_options)?;
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

    fn obtain_manifest(options: &ArchiveOptions) -> Result<serde_json::Value> {
        let manifest_path = CargoWorkspace::locate_source_crate("solidity_testing_sourcify")
            .unwrap_or_default()
            .join("target/manifest.json");

        if !options.offline && !file_exists_and_is_fresh(&manifest_path) {
            let manifest_url = "https://repo-backup.sourcify.dev/manifest.json";
            println!("Downloading manifest from {manifest_url}");
            let client = Client::new();
            let mut res = client.get(manifest_url).send()?;

            let status = res.status();
            if !status.is_success() {
                bail!("Error fetching manifest.json");
            }
            let mut writer = File::create(&manifest_path)?;
            res.copy_to(&mut writer)?;
        }

        let manifest_contents = fs::read_to_string(manifest_path)?;
        Ok(serde_json::from_str(&manifest_contents)?)
    }

    /// Search for a specific contract and return it if found. Returns `None` if the contract can not
    /// be fetched for any reason (including if the `contract_id` is not parseable).
    pub fn fetch_contract(&self, contract_id: &str) -> Option<Contract> {
        let options = ArchiveOptions {
            offline: false,
            save: true,
        };
        u8::from_str_radix(contract_id.get(2..4).unwrap(), 16)
            .ok()
            .and_then(|contract_prefix| {
                self.archives()
                    .filter(|desc| desc.prefix == contract_prefix)
                    .flat_map(|path| ContractArchive::fetch(path, &options))
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

    /// Should remove the archive and contracts at the end
    delete_files: bool,
}

impl ContractArchive {
    pub fn fetch(desc: &ArchiveDescriptor, options: &ArchiveOptions) -> Result<ContractArchive> {
        let contracts_path = desc.contracts_dir();
        if !options.offline && !file_exists_and_is_fresh(&contracts_path) {
            println!("Downloading shard archive from {url}", url = desc.url);
            let client = Client::new();
            let res = client.get(&desc.url).send()?;

            let status = res.status();
            if !status.is_success() {
                bail!("Could not fetch source tarball");
            }

            let archive_dir = desc.archive_dir();

            let mut archive = Archive::new(res);
            archive.unpack(&archive_dir)?;

            // explicitly touch the contracts path mtime to avoid downloading
            // again if sufficiently up-to-date
            fs::File::open(&contracts_path)?.set_modified(SystemTime::now())?;
        }

        Ok(ContractArchive {
            contracts_path,
            delete_files: !options.save && !options.offline,
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
        if self.delete_files {
            fs::remove_dir_all(&self.contracts_path).unwrap();
        }
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

        let metadata_contents = fs::read_to_string(contract_path.join("metadata.json"))?;
        let metadata_val: serde_json::Value = serde_json::from_str(&metadata_contents)?;

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

        let import_resolver = import_resolver_from(&metadata_val)?;

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
        let name = self.name.clone();
        let entrypoint = self.entrypoint().ok_or(Error::msg(format!(
            "Entrypoint not found in contract {name}"
        )))?;

        let mut builder =
            CompilationBuilder::new(self.version.clone(), ContractConfig { contract: self })?;

        builder.add_file(&entrypoint).map_err(|e| anyhow!(e))?;
        Ok(builder.build())
    }

    pub fn entrypoint(&self) -> Option<String> {
        self.import_resolver.get_source_id(&self.target)
    }

    pub fn sources_count(&self) -> usize {
        self.import_resolver.sources_count()
    }

    pub fn read_file(&self, file_id: &str) -> Result<String> {
        let source = self.sources_path.join(file_id).read_to_string()?;
        Ok(source)
    }
}

struct ContractConfig<'a> {
    contract: &'a Contract,
}

impl CompilationBuilderConfig for ContractConfig<'_> {
    type Error = anyhow::Error;

    fn read_file(&mut self, file_id: &str) -> Result<Option<String>> {
        self.contract.read_file(file_id).map(Some)
    }

    fn resolve_import(
        &mut self,
        source_file_id: &str,
        import_path_cursor: &slang_solidity::cst::Cursor,
    ) -> Result<Option<String>> {
        let import_path = import_path_cursor.node().unparse();
        let import_path = import_path
            .strip_prefix(|c| matches!(c, '"' | '\''))
            .unwrap()
            .strip_suffix(|c| matches!(c, '"' | '\''))
            .unwrap()
            .trim();

        Ok(self
            .contract
            .import_resolver
            .resolve_import(source_file_id, import_path))
    }
}

fn file_exists_and_is_fresh<P: AsRef<Path>>(path: P) -> bool {
    if let Ok(metadata) = fs::metadata(path) {
        if let Ok(modified) = metadata.modified() {
            // Sourcify updates archives once a day, so anything updated updated
            // in the last 24 hours is considered fresh
            if SystemTime::now()
                .duration_since(modified)
                .unwrap_or(Duration::MAX)
                <= Duration::from_secs(24 * 3600)
            {
                return true;
            }
        }
    }
    false
}

fn import_resolver_from(value: &serde_json::Value) -> Result<ImportResolver> {
    let import_remaps: Vec<ImportRemap> = value
        .get("settings")
        .and_then(|settings| settings.get("remappings"))
        .and_then(|remappings| remappings.as_array())
        .map_or(vec![], |mappings| {
            mappings
                .iter()
                .filter_map(|mapping| mapping.as_str())
                .filter_map(|m| ImportRemap::new(m).ok())
                .filter(|remap| !remap.has_known_bug())
                .collect()
        });

    let source_maps: Vec<SourceMap> = value
        .get("sources")
        .and_then(|sources| sources.as_object())
        .ok_or(Error::msg(
            "Could not get sources entry in contract metadata.",
        ))
        .map(|sources| {
            sources
                .iter()
                .filter_map(|(key, value)| {
                    value
                        .get("keccak256")
                        .and_then(|k| k.as_str())
                        .map(|source_id| SourceMap {
                            source_id: source_id.into(),
                            virtual_path: key.clone(),
                        })
                })
                .collect()
        })?;

    Ok(ImportResolver {
        import_remaps,
        source_maps,
    })
}

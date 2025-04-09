use std::{collections::HashMap, path::{Component, PathBuf}, str::FromStr};

use anyhow::{bail, Error, Result};
use semver::{BuildMetadata, Prerelease, Version};
use serde::Deserialize;

#[derive(Deserialize)]
#[serde(try_from = "ContractMetadataResponse")]
pub struct ContractMetadata {
    pub version: Version,
    pub target: String,   
    remappings: Vec<ImportRemap>,
    sources: Vec<FileMapping>,
}

impl ContractMetadata {
    /// Given an import path from a source file, traverse the remappings to find the real filename of
    /// the target source file.
    pub fn resolve_import(&self, source_name: &str, import_path: &str) -> Result<String> {
        let import_path = self.remap_import(import_path);

        if import_path.starts_with("http") || import_path.starts_with("ipfs") {
            // URL imports don't need special resolution
            return self.get_real_filename(&import_path);
        }

        let source_file_path = self.get_virtual_path(source_name)?;
        let source_dir = source_file_path.parent().ok_or(Error::msg(format!("Could not get directory of source file {}", source_name)))?;

        let import_path = PathBuf::from_str(&import_path)?;

        let source_dir_parts: Vec<_> = source_dir.components().collect();
        let mut import_path_components = import_path.components();

        let mut resolved_parts = vec![];

        match import_path_components.next().unwrap() {
            norm @ Component::Normal(_) => resolved_parts.push(norm),
            Component::CurDir => resolved_parts.extend(source_dir_parts),
            Component::ParentDir => {
                resolved_parts.extend(source_dir_parts);
               resolved_parts.pop();
            },
            Component::RootDir => {},
            Component::Prefix(_) => bail!("Found prefix component in import path, which is not supported"),
        }

        for component in import_path_components {
            match component {
                norm @ Component::Normal(_) => resolved_parts.push(norm),
                Component::ParentDir => {
                    resolved_parts.pop();
                },
                Component::CurDir => {},
                invalid => bail!("Invalid path component found inside import path: {invalid:?}"),
            }
        }

        let resolved_import_path: PathBuf = resolved_parts.iter().collect();
        let resolved_import = resolved_import_path.to_str().unwrap();

        self.get_real_filename(&resolved_import)
    }

    pub fn get_real_filename(&self, virtual_path: &str) -> Result<String> {
        for source in &self.sources {
            if source.virtual_path == virtual_path {
                return Ok(source.real_name.clone());
            }
        }
        Err(Error::msg(format!("Could not get real name for import {virtual_path}")))
    }

    pub fn get_virtual_path(&self, real_name: &str) -> Result<PathBuf> {
        for source in &self.sources {
            if source.real_name == real_name {
                return Ok(PathBuf::from(&source.virtual_path));
            }
        }
        Err(Error::msg(format!("Could not get virtual path for source file {real_name}")))
    }

    fn remap_import(&self, import_path: &str) -> String {
        let mut longest_match: Option<&ImportRemap> = None;
        for remap in self.remappings.iter() {
            if import_path.starts_with(&remap.mapped) {
                match longest_match {
                    Some(r) => {
                        if r.mapped.len() < remap.mapped.len() {
                            longest_match = Some(&remap);
                        }
                    },
                    None => {
                        longest_match = Some(&remap);
                    }
                }
            }
        }

        if let Some(r) = longest_match {
            import_path.replacen(&r.mapped, &r.original, 1)
        } else {
            import_path.into()
        }
    }
}

struct FileMapping {
    /// The actual filename for the source file, as found in the archive. This name can
    /// be used to read the content of a source file.
    real_name: String,
    /// The path to the source file in the contract's "virtual filesystem". This is the
    /// path to the source file as the contract was originally constructed. This value 
    /// should be used when resolving imports to the real source files.
    virtual_path: String,
}

struct ImportRemap {
    /// A stand-in value for the full path (`original`) to the imported file.
    mapped: String,
    /// The full path to the imported file.
    original: String,
}

#[derive(Deserialize)]
struct ContractMetadataResponse {
    compiler: ContractVersion,
    settings: ContractSettings,
    language: String,
    sources: HashMap<String, ContractSource>,
}

#[derive(Deserialize)]
struct ContractVersion {
    version: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct ContractSettings {
    compilation_target: HashMap<String, String>,
    remappings: Vec<String>,
}

#[derive(Deserialize)]
struct ContractSource {
    keccak256: String,
}

impl TryFrom<ContractMetadataResponse> for ContractMetadata {
    type Error = anyhow::Error;

    fn try_from(response: ContractMetadataResponse) -> Result<ContractMetadata> {
        if !response.language.eq_ignore_ascii_case("solidity") {
            bail!("Only Solidity contracts are supported");
        }

        let mut version = Version::parse(&response.compiler.version)?;
        version.pre = Prerelease::EMPTY;
        version.build = BuildMetadata::EMPTY;

        let target = response.settings.compilation_target.keys().take(1).collect::<Vec<_>>()[0].to_owned();

        let mut remappings = vec![];
        for remap in response.settings.remappings {
            if let Some((mapped, original)) = remap.split_once('=') {
                remappings.push(ImportRemap{
                    original: original.into(),
                    mapped: mapped.trim_start_matches(':').into(),
                });
            }
        }

        let mut sources = vec![];
        for (virtual_path, source) in response.sources {
            sources.push(FileMapping{
                virtual_path,
                real_name: source.keccak256,
            });
        }

        Ok(ContractMetadata {
            version,
            target,
            remappings,
            sources,
        })
    }
}

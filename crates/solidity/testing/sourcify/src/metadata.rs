use std::{collections::HashMap, path::{Component, PathBuf}, str::FromStr};

use anyhow::{bail, Error, Result};
use semver::{BuildMetadata, Prerelease, Version};
use serde::Deserialize;

#[derive(Deserialize)]
#[serde(try_from = "ContractMetadataResponse")]
pub struct ContractMetadata {
    pub version: Version,
    pub target: String,   
    pub remappings: Vec<ImportRemap>,
    pub sources: Vec<FileMapping>,
}

impl ContractMetadata {
    /// Given an import path from a source file, traverse the remappings to find the real filename of
    /// the target source file.
    pub fn resolve_import(&self, source_name: &str, import_path: &str) -> Result<String> {
        if let Some(remapped_import) = self.remap_import(import_path) {
            // Paths that have been remapped don't need to go through path resolution.
            return self.get_real_name(&remapped_import);
        }

        if path_is_url(import_path) {
            // URL imports don't need path resolution
            return self.get_real_name(import_path);
        }

        let source_file_path = self.get_virtual_path(source_name)?;

        let resolved_path = if path_is_url(&source_file_path) {
            self.resolve_relative_url_import(&source_file_path, import_path)?
        } else {
            self.resolve_relative_import(&source_file_path, import_path)?
        };

        self.get_real_name(&resolved_path)
    }

    pub fn get_real_name(&self, virtual_path: &str) -> Result<String> {
        for source in &self.sources {
            if source.virtual_path == virtual_path {
                return Ok(source.real_name.clone());
            }
        }
        Err(Error::msg(format!("Could not get real name for import {virtual_path}")))
    }

    pub fn get_virtual_path(&self, real_name: &str) -> Result<String> {
        for source in &self.sources {
            if source.real_name == real_name {
                return Ok(source.virtual_path.clone());
            }
        }
        Err(Error::msg(format!("Could not get virtual path for source file {real_name}")))
    }

    fn remap_import(&self, import_path: &str) -> Option<String> {
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
            Some(import_path.replacen(&r.mapped, &r.original, 1))
        } else {
            None
        }
    }

    /// Resolve an import path that is relative to `source_path`.
    fn resolve_relative_import(&self, source_path: &str, import_path: &str) -> Result<String> {
        let source_file_path = PathBuf::from_str(source_path)?;
        let source_dir = source_file_path.parent().ok_or(Error::msg(format!("Could not get directory of source file {source_path}")))?;

        let import_path = PathBuf::from_str(&import_path)?;

        let source_dir_parts: Vec<_> = source_dir.components().collect();
        let mut resolved_parts = vec![];

        // We're basically doing what `Path::canonicalize()` would do, but we can't use that because
        // the path must be a real path in the filesystem, and we're operating on "virtual" paths.
        // We check the first import path component to initialize `resolved_parts`
        let mut import_path_components = import_path.components();
        match import_path_components.next().unwrap() {
            // a/b.sol - an absolute path, so we can ignore `source_path`
            norm @ Component::Normal(_) => resolved_parts.push(norm),
            // /a/b.sol
            root @ Component::RootDir => resolved_parts.push(root),
            // ./a/b.sol - relative to `source_path`
            Component::CurDir => resolved_parts.extend(source_dir_parts),
            // ../a/b.sol - relative, but one dir above `source_dir`
            Component::ParentDir => {
                resolved_parts.extend(source_dir_parts);
               resolved_parts.pop();
            },
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
        Ok(resolved_import_path.to_str().unwrap().into())
    }

    /// Resolve an import from a source file which was imported using a URL. These need a bit of special handling
    /// because the resolved path needs to also be a URL.
    fn resolve_relative_url_import(&self, source_path: &str, import_path: &str) -> Result<String> {
        let (proto, rest) = source_path.split_once("://").ok_or(Error::msg("Cannot parse url"))?;
        let (host, path) = rest.split_once("/").ok_or(Error::msg("Cannot parse path"))?;

        let resolved_path = self.resolve_relative_import(path, import_path)?;

        let mut res = String::new();
        res.push_str(proto);
        res.push_str("://");
        res.push_str(host);
        res.push_str("/");
        res.push_str(&resolved_path);

        Ok(res)
    }
}

pub struct FileMapping {
    /// The actual filename for the source file, as found in the archive. This name can
    /// be used to read the content of a source file.
    real_name: String,
    /// The path to the source file in the contract's "virtual filesystem". This is the
    /// path to the source file as the contract was originally constructed. This value 
    /// should be used when resolving imports to the real source files.
    virtual_path: String,
}

pub struct ImportRemap {
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

fn path_is_url(path: &str) -> bool {
    path.starts_with("http") || path.starts_with("ipfs")
}

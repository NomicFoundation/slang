use std::collections::HashMap;

use anyhow::{bail, Result};
use semver::{BuildMetadata, Prerelease, Version};
use serde::Deserialize;

#[derive(Deserialize)]
#[serde(try_from = "ContractMetadataResponse")]
pub struct ContractMetadata {
    pub version: Version,
    pub target: String,   
    remappings: Vec<FileMapping>,
    sources: Vec<FileMapping>,
}

impl ContractMetadata {
    /// Given an import path from a source file, traverse the Sourcify remappings to find the real filename of
    /// the target source file.
    pub fn get_absolute_filename(&self, import_path: &str) -> String {
        let mut target = import_path;
        for remapping in &self.remappings {
            if import_path == &remapping.mapped {
                target = &remapping.original;
            }
        }

        for source in &self.sources {
            if &source.original == target {
                target = &source.mapped;
            }
        }

        target.into()
    }
}

struct FileMapping {
    mapped: String,
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
                remappings.push(FileMapping{
                    original: original.into(),
                    mapped: mapped.trim_start_matches(':').into(),
                });
            }
        }

        let mut sources = vec![];
        for (original_name, source) in response.sources {
            sources.push(FileMapping{
                original: original_name,
                mapped: source.keccak256,
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

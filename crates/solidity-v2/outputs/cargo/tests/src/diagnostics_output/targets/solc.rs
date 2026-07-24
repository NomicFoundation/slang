use anyhow::{Result, anyhow};
use inflector::Inflector;
use infra_utils::solc::{
    Binary, CliInput, CliSettings, InputSource, LanguageSelector, render_solc_error,
};
use semver::Version;
use slang_solidity_v2::compilation::FileId;
use slang_solidity_v2_common::collections::SortedMap;
use slang_solidity_v2_common::evm_targets::EvmTarget;
use slang_solidity_v2_common::versions::LanguageVersion;

use crate::diagnostics_output::targets::TestTarget;

pub(crate) struct SolcTarget {
    binaries: SortedMap<Version, Binary>,
}

impl SolcTarget {
    pub(crate) fn new(versions: impl IntoIterator<Item = Version>) -> Result<Self> {
        Ok(Self {
            binaries: Binary::fetch_all(versions)?,
        })
    }
}

impl TestTarget for SolcTarget {
    fn name(&self) -> &'static str {
        "solc"
    }

    fn collect_diagnostics(
        &self,
        files: &SortedMap<FileId, String>,
        version: LanguageVersion,
        evm_target: EvmTarget,
    ) -> Result<Vec<String>> {
        let semver_version: Version = version.into();
        let binary = self
            .binaries
            .get(&semver_version)
            .ok_or_else(|| anyhow!("no solc binary fetched for version {semver_version}"))?;

        // `solc` works with string file names, so convert from `FileId` keys.
        let sources: SortedMap<String, String> = files
            .iter()
            .map(|(file_id, content)| (file_id.to_string(), content.clone()))
            .collect();

        let input = CliInput {
            language: LanguageSelector::Solidity,
            sources: sources
                .iter()
                .map(|(name, content)| {
                    (
                        name.clone(),
                        InputSource {
                            content: content.clone(),
                        },
                    )
                })
                .collect(),
            settings: CliSettings {
                evm_version: Some(evm_target.to_string().to_camel_case()),
                experimental: if version < LanguageVersion::V0_8_35 {
                    // 'experimental' flag was introduced in '0.8.35'
                    None
                } else {
                    // Allow experimental features, like 'Amsterdam' hardfork on '0.8.36'
                    Some(true)
                },
            },
        };

        let output = binary.run(&input)?;
        let errors = output.errors.unwrap_or_default();

        let rendered = errors
            .into_iter()
            .map(|error| render_solc_error(&error, &sources).unwrap())
            .collect();

        Ok(rendered)
    }
}

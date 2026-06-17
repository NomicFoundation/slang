use anyhow::{anyhow, Result};
use inflector::Inflector;
use infra_utils::solc::{
    render_solc_error, Binary, CliInput, CliSettings, InputSource, LanguageSelector,
};
use semver::Version;
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
        files: &SortedMap<String, String>,
        version: LanguageVersion,
        evm_target: EvmTarget,
    ) -> Result<Vec<String>> {
        let semver_version: Version = version.into();
        let binary = self
            .binaries
            .get(&semver_version)
            .ok_or_else(|| anyhow!("no solc binary fetched for version {semver_version}"))?;

        let input = CliInput {
            language: LanguageSelector::Solidity,
            sources: files
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
            },
        };

        let output = binary.run(&input)?;
        let errors = output.errors.unwrap_or_default();

        let rendered = errors
            .into_iter()
            .map(|error| render_solc_error(&error, files).unwrap())
            .collect();

        Ok(rendered)
    }
}

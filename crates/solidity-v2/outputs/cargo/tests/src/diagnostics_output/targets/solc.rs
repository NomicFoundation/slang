use std::collections::BTreeMap;

use anyhow::{anyhow, Result};
use infra_utils::solc::{render_solc_error, Binary, CliInput, InputSource, LanguageSelector};
use semver::Version;

use crate::diagnostics_output::targets::TestTarget;

pub(crate) struct SolcTarget {
    binaries: BTreeMap<Version, Binary>,
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

    fn get_errors(
        &self,
        files: &BTreeMap<String, String>,
        version: &Version,
    ) -> Result<Vec<String>> {
        let binary = self
            .binaries
            .get(version)
            .ok_or_else(|| anyhow!("no solc binary fetched for version {version}"))?;

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
        };

        let output = binary.run(&input)?;
        let raw_errors = output.errors.unwrap_or_default();
        let mut errors = Vec::with_capacity(raw_errors.len());

        for error in &raw_errors {
            let mut rendered = String::new();
            render_solc_error(error, files, &mut rendered)?;
            errors.push(rendered);
        }

        Ok(errors)
    }
}

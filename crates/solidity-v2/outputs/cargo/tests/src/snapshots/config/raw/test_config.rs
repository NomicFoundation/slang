use anyhow::{Context, Result, ensure};
use serde::Deserialize;

use super::evm_target_override::RawEvmTargetOverride;
use super::expected_solc_divergence::RawExpectedSolcDivergence;
use super::language_version_override::RawLanguageVersionOverride;
use crate::snapshots::config::TestConfig;

/// The config as written in a file. Every field is optional: values not
/// provided here are inherited from the closest parent directory that does
/// provide them.
#[derive(Deserialize, Eq, PartialEq)]
#[serde(deny_unknown_fields)]
pub(crate) struct RawTestConfig {
    override_language_versions: Option<RawLanguageVersionOverride>,
    override_evm_target: Option<RawEvmTargetOverride>,

    /// An empty list is not the same as a missing one: it stops a parent's
    /// entries from being inherited.
    expected_solc_divergence: Option<Vec<RawExpectedSolcDivergence>>,
}

impl TryFrom<RawTestConfig> for TestConfig {
    type Error = anyhow::Error;

    fn try_from(raw: RawTestConfig) -> Result<Self> {
        let RawTestConfig {
            override_language_versions,
            override_evm_target,
            expected_solc_divergence,
        } = raw;

        ensure!(
            override_language_versions.is_some()
                || override_evm_target.is_some()
                || expected_solc_divergence.is_some(),
            "Test config has no fields set, so it is redundant and should be removed."
        );

        Ok(Self {
            override_language_versions: override_language_versions
                .map(|versions| {
                    versions
                        .try_into()
                        .context("Invalid `override_language_versions` field.")
                })
                .transpose()?,

            override_evm_target: override_evm_target
                .map(|target| {
                    target
                        .try_into()
                        .context("Invalid `override_evm_target` field.")
                })
                .transpose()?,

            expected_solc_divergence: expected_solc_divergence
                .map(|entries| {
                    entries
                        .into_iter()
                        .enumerate()
                        .map(|(index, entry)| {
                            entry.try_into().with_context(|| {
                                format!("Invalid `expected_solc_divergence[{index}]` entry.")
                            })
                        })
                        .collect::<Result<_>>()
                })
                .transpose()?,
        })
    }
}

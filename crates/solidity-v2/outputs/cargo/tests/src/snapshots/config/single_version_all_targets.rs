use anyhow::{Context, Result};
use serde::Deserialize;
use slang_solidity_v2_common::evm_targets::EvmTargetSpecifier;
use slang_solidity_v2_common::versions::LanguageVersion;

use super::expected_solc_divergence::RawExpectedSolcDivergence;
use super::selected_version::RawSelectedVersion;

/// Pins the language version, and iterates over all `EvmTarget` variants.
#[derive(Clone, Debug)]
pub struct SingleVersionAllTargets {
    pub version: LanguageVersion,

    /// The EVM targets where slang and solc are expected to disagree on the
    /// status (success/failure) of a snapshot. Used for diagnostics where slang
    /// intentionally diverges from solc.
    pub expected_solc_divergence: Vec<EvmTargetSpecifier>,
}

/// The matrix as written in a config file. Both of its fields are optional, so
/// that a nested config can override one of them while inheriting the other
/// from a parent directory.
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct RawSingleVersionAllTargets {
    version: Option<RawSelectedVersion>,
    expected_solc_divergence: Option<Vec<RawExpectedSolcDivergence<EvmTargetSpecifier>>>,
}

impl RawSingleVersionAllTargets {
    /// Absorbs fields from `parent`, a matrix declared further up the traversal,
    /// for every field that `self` doesn't already provide.
    pub(super) fn absorb(&mut self, parent: Self) {
        let Self {
            version: parent_version,
            expected_solc_divergence: parent_expected_solc_divergence,
        } = parent;

        self.version = self.version.take().or(parent_version);

        self.expected_solc_divergence = self
            .expected_solc_divergence
            .take()
            .or(parent_expected_solc_divergence);
    }
}

impl TryFrom<RawSingleVersionAllTargets> for SingleVersionAllTargets {
    type Error = anyhow::Error;

    fn try_from(raw: RawSingleVersionAllTargets) -> Result<Self> {
        let RawSingleVersionAllTargets {
            version,
            expected_solc_divergence,
        } = raw;

        Ok(Self {
            version: version
                .context("No config file provides the `version` field.")?
                .try_into()
                .context("Invalid `version` field.")?,

            expected_solc_divergence: expected_solc_divergence
                .unwrap_or_default()
                .into_iter()
                .enumerate()
                .map(|(index, entry)| {
                    entry.try_into().with_context(|| {
                        format!("Invalid `expected_solc_divergence[{index}]` entry.")
                    })
                })
                .collect::<Result<_>>()?,
        })
    }
}

use anyhow::{Context, Result};
use serde::Deserialize;
use slang_solidity_v2_common::evm_targets::EvmTarget;
use slang_solidity_v2_common::versions::LanguageVersionSpecifier;

use super::expected_solc_divergence::RawExpectedSolcDivergence;
use super::selected_target::RawSelectedTarget;

/// Pins the EVM target, and iterates over all `LanguageVersion` variants.
#[derive(Clone, Debug)]
pub struct SingleTargetAllVersions {
    pub target: EvmTarget,

    /// The language versions where slang and solc are expected to disagree on
    /// status (success/failure) of a snapshot. Used for diagnostics where slang
    /// intentionally diverges from solc.
    pub expected_solc_divergence: Vec<LanguageVersionSpecifier>,
}

/// The matrix as written in a config file. Both of its fields are optional, so
/// that a nested config can override one of them while inheriting the other
/// from a parent directory.
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct RawSingleTargetAllVersions {
    target: Option<RawSelectedTarget>,
    expected_solc_divergence: Option<Vec<RawExpectedSolcDivergence<LanguageVersionSpecifier>>>,
}

impl RawSingleTargetAllVersions {
    /// Absorbs fields from `other`, a matrix declared further up the traversal,
    /// for every field that `self` doesn't already provide.
    pub(super) fn absorb(&mut self, parent: Self) {
        let Self {
            target: parent_target,
            expected_solc_divergence: parent_expected_solc_divergence,
        } = parent;

        self.target = self.target.take().or(parent_target);

        self.expected_solc_divergence = self
            .expected_solc_divergence
            .take()
            .or(parent_expected_solc_divergence);
    }
}

impl TryFrom<RawSingleTargetAllVersions> for SingleTargetAllVersions {
    type Error = anyhow::Error;

    fn try_from(raw: RawSingleTargetAllVersions) -> Result<Self> {
        let RawSingleTargetAllVersions {
            target,
            expected_solc_divergence,
        } = raw;

        Ok(Self {
            target: target
                .context("No config file provides the `target` field.")?
                .try_into()
                .context("Invalid `target` field.")?,

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

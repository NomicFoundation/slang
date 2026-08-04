use anyhow::{Result, ensure};
use serde::Deserialize;
use slang_solidity_v2_common::evm_targets::EvmTargetSpecifier;
use slang_solidity_v2_common::versions::LanguageVersionSpecifier;

/// A single expected status divergence, as written in a config file.
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct RawExpectedSolcDivergence<S> {
    /// A language version or EVM target specifier, depending on the axis the test varies.
    specifier: S,

    /// Reason for declaring this divergence (for documentation purposes).
    reason: String,
}

impl TryFrom<RawExpectedSolcDivergence<EvmTargetSpecifier>> for EvmTargetSpecifier {
    type Error = anyhow::Error;

    fn try_from(raw: RawExpectedSolcDivergence<EvmTargetSpecifier>) -> Result<Self> {
        let RawExpectedSolcDivergence { specifier, reason } = raw;

        ensure!(!reason.trim().is_empty(), "Reason must be non-empty");

        if let Self::Range { from, till } = &specifier {
            ensure!(
                from < till,
                "Empty range: '{from}' must come before '{till}'."
            );
        }

        Ok(specifier)
    }
}

impl TryFrom<RawExpectedSolcDivergence<LanguageVersionSpecifier>> for LanguageVersionSpecifier {
    type Error = anyhow::Error;

    fn try_from(raw: RawExpectedSolcDivergence<LanguageVersionSpecifier>) -> Result<Self> {
        let RawExpectedSolcDivergence { specifier, reason } = raw;

        ensure!(!reason.trim().is_empty(), "Reason must be non-empty");

        if let Self::Range { from, till } = &specifier {
            ensure!(
                from < till,
                "Empty range: '{from}' must come before '{till}'."
            );
        }

        Ok(specifier)
    }
}

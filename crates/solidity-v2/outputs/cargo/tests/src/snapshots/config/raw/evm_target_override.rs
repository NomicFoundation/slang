use anyhow::{Result, ensure};
use serde::Deserialize;
use slang_solidity_v2_common::evm_targets::EvmTarget;

/// The EVM target a config file pins its test to, as written in the file.
#[derive(Deserialize, Eq, PartialEq)]
#[serde(deny_unknown_fields)]
pub(crate) struct RawEvmTargetOverride {
    /// The target every language version runs at.
    value: EvmTarget,

    /// Reason for pinning the target (for documentation purposes).
    reason: String,
}

impl TryFrom<RawEvmTargetOverride> for EvmTarget {
    type Error = anyhow::Error;

    fn try_from(raw: RawEvmTargetOverride) -> Result<Self> {
        let RawEvmTargetOverride { value, reason } = raw;

        ensure!(!reason.trim().is_empty(), "Reason must be non-empty");

        Ok(value)
    }
}

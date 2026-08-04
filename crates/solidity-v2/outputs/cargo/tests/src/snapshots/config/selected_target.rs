use anyhow::{Result, ensure};
use serde::Deserialize;
use slang_solidity_v2_common::evm_targets::EvmTarget;

/// The EVM target a config file pins its matrix to.
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct RawSelectedTarget {
    /// The EVM target the config pins to.
    value: EvmTarget,

    /// Reason for pinning this target (for documentation purposes).
    reason: String,
}

impl TryFrom<RawSelectedTarget> for EvmTarget {
    type Error = anyhow::Error;

    fn try_from(raw: RawSelectedTarget) -> Result<Self> {
        let RawSelectedTarget { value, reason } = raw;

        ensure!(!reason.trim().is_empty(), "Reason must be non-empty");

        Ok(value)
    }
}

use anyhow::{Result, ensure};
use serde::Deserialize;
use slang_solidity_v2_common::versions::LanguageVersion;

/// The language version a config file pins its matrix to.
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct RawSelectedVersion {
    /// The language version the config pins to.
    value: LanguageVersion,

    /// Reason for pinning this version (for documentation purposes).
    reason: String,
}

impl TryFrom<RawSelectedVersion> for LanguageVersion {
    type Error = anyhow::Error;

    fn try_from(raw: RawSelectedVersion) -> Result<Self> {
        let RawSelectedVersion { value, reason } = raw;

        ensure!(!reason.trim().is_empty(), "Reason must be non-empty");

        Ok(value)
    }
}

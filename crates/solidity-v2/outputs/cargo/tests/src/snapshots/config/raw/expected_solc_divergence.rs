use anyhow::{Result, ensure};
use serde::Deserialize;
use slang_solidity_v2_common::versions::{LanguageVersion, LanguageVersionSpecifier};

/// A single expected status divergence, as written in a config file.
#[derive(Deserialize, Eq, PartialEq)]
#[serde(deny_unknown_fields)]
pub(crate) struct RawExpectedSolcDivergence {
    /// The language versions where slang and solc are expected to disagree.
    specifier: LanguageVersionSpecifier,

    /// Reason for declaring this divergence (for documentation purposes).
    reason: String,
}

impl TryFrom<RawExpectedSolcDivergence> for LanguageVersionSpecifier {
    type Error = anyhow::Error;

    fn try_from(raw: RawExpectedSolcDivergence) -> Result<Self> {
        let RawExpectedSolcDivergence { specifier, reason } = raw;

        ensure!(!reason.trim().is_empty(), "Reason must be non-empty");

        // A specifier that matches no version declares nothing.
        ensure!(
            LanguageVersion::ALL
                .iter()
                .any(|version| specifier.contains(*version)),
            "'{specifier:?}' selects no language version."
        );

        Ok(specifier)
    }
}

use anyhow::{Result, ensure};
use serde::Deserialize;
use slang_solidity_v2_common::versions::{LanguageVersion, LanguageVersionSpecifier};

/// The language versions a config file narrows its test down to, as written in
/// the file.
#[derive(Deserialize, Eq, PartialEq)]
#[serde(deny_unknown_fields)]
pub(crate) struct RawLanguageVersionOverride {
    /// The versions the test runs at.
    specifier: LanguageVersionSpecifier,

    /// Reason for narrowing the versions (for documentation purposes).
    reason: String,
}

impl TryFrom<RawLanguageVersionOverride> for LanguageVersionSpecifier {
    type Error = anyhow::Error;

    fn try_from(raw: RawLanguageVersionOverride) -> Result<Self> {
        let RawLanguageVersionOverride { specifier, reason } = raw;

        ensure!(!reason.trim().is_empty(), "Reason must be non-empty");

        // A specifier that matches no version would silently retire the test.
        ensure!(
            LanguageVersion::ALL
                .iter()
                .any(|version| specifier.contains(*version)),
            "'{specifier:?}' selects no language version."
        );

        Ok(specifier)
    }
}

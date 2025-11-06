use language_v2_internal_macros::{derive_spanned_type, ParseInputTokens, WriteOutputTokens};
use semver::Version;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[derive_spanned_type(Clone, Debug, ParseInputTokens, WriteOutputTokens)]
#[serde(tag = "type")]
pub enum VersionSpecifier {
    Never,
    From { from: Version },
    Till { till: Version },
    Range { from: Version, till: Version },
}

impl VersionSpecifier {
    pub fn contains(&self, version: &Version) -> bool {
        match self {
            VersionSpecifier::Never => false,
            VersionSpecifier::From { from } => from <= version,
            VersionSpecifier::Till { till } => version < till,
            VersionSpecifier::Range { from, till } => from <= version && version < till,
        }
    }

    /// Returns an iterator over the versions specified as the upper and lower bound.
    pub fn versions(&self) -> impl Iterator<Item = &Version> {
        match self {
            VersionSpecifier::Never => [None, None],
            VersionSpecifier::From { from } => [Some(from), None],
            VersionSpecifier::Till { till } => [None, Some(till)],
            VersionSpecifier::Range { from, till } => [Some(from), Some(till)],
        }
        .into_iter()
        .flatten()
    }
}

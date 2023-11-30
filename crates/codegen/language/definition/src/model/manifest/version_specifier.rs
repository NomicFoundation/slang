use semver::Version;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[codegen_language_internal_macros::derive_spanned_type(
    codegen_language_internal_macros::ParseInputTokens,
    codegen_language_internal_macros::WriteOutputTokens
)]
pub enum VersionSpecifier {
    Never,
    From { from: Version },
    Till { till: Version },
    Range { from: Version, till: Version },
}

impl VersionSpecifier {
    pub fn contains(&self, version: &Version) -> bool {
        match self {
            VersionSpecifier::Never => {
                return false;
            }
            VersionSpecifier::From { from } => {
                return from <= version;
            }
            VersionSpecifier::Till { till } => {
                return version < till;
            }
            VersionSpecifier::Range { from, till } => {
                return from <= version && version < till;
            }
        };
    }
}

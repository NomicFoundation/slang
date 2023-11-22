use crate::model::Spanned;
use codegen_language_internal_macros::{ParseInputTokens, WriteOutputTokens};
use semver::Version;
use serde::Serialize;
use std::ops::Deref;

#[derive(Debug, Eq, ParseInputTokens, PartialEq, Serialize, WriteOutputTokens)]
pub enum VersionSpecifier {
    Never,
    From {
        from: Spanned<Version>,
    },
    Till {
        till: Spanned<Version>,
    },
    Range {
        from: Spanned<Version>,
        till: Spanned<Version>,
    },
}

impl VersionSpecifier {
    pub fn contains(&self, version: &Version) -> bool {
        match self {
            VersionSpecifier::Never => {
                return false;
            }
            VersionSpecifier::From { from } => {
                return from.deref() <= version;
            }
            VersionSpecifier::Till { till } => {
                return version < till.deref();
            }
            VersionSpecifier::Range { from, till } => {
                return from.deref() <= version && version < till.deref();
            }
        };
    }
}

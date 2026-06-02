use serde::Serialize;

use crate::versions::LanguageVersion;

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub enum LanguageVersionSpecifier {
    From {
        from: LanguageVersion,
    },
    Till {
        till: LanguageVersion,
    },
    Range {
        from: LanguageVersion,
        till: LanguageVersion,
    },
}

impl LanguageVersionSpecifier {
    pub fn contains(&self, other: LanguageVersion) -> bool {
        match self {
            LanguageVersionSpecifier::From { from } => other >= *from,
            LanguageVersionSpecifier::Till { till } => other < *till,
            LanguageVersionSpecifier::Range { from, till } => other >= *from && other < *till,
        }
    }
}

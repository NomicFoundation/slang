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
    #[inline]
    pub const fn from(from: LanguageVersion) -> Self {
        Self::From { from }
    }

    #[inline]
    pub const fn till(till: LanguageVersion) -> Self {
        Self::Till { till }
    }

    #[inline]
    pub const fn range(from: LanguageVersion, till: LanguageVersion) -> Self {
        Self::Range { from, till }
    }
}

impl LanguageVersionSpecifier {
    pub fn contains(&self, other: LanguageVersion) -> bool {
        match self {
            Self::From { from } => *from <= other,
            Self::Till { till } => other < *till,
            Self::Range { from, till } => *from <= other && other < *till,
        }
    }
}

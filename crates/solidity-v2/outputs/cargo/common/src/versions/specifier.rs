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
    #[inline]
    pub const fn contains(&self, other: LanguageVersion) -> bool {
        match self {
            Self::From { from } => (*from as u32) <= (other as u32),
            Self::Till { till } => (other as u32) < (*till as u32),
            Self::Range { from, till } => {
                (*from as u32) <= (other as u32) && (other as u32) < (*till as u32)
            }
        }
    }
}

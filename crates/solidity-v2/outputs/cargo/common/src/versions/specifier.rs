use serde::{Deserialize, Serialize};

use crate::versions::LanguageVersion;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, PartialOrd, Ord, Serialize)]
#[serde(tag = "type")]
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

    pub fn intersect(&self, other: &Self) -> Option<Self> {
        let (self_from, self_till) = self.bounds();
        let (other_from, other_till) = other.bounds();

        let from = self_from.max(other_from);
        let till = match (self_till, other_till) {
            (Some(self_till), Some(other_till)) => self_till.min(other_till),
            (Some(till), None) | (None, Some(till)) => till,
            (None, None) => {
                return Some(Self::From { from });
            }
        };

        if from < till {
            Some(Self::Range { from, till })
        } else {
            None
        }
    }

    fn bounds(&self) -> (LanguageVersion, Option<LanguageVersion>) {
        match self {
            Self::From { from } => (*from, None),
            Self::Till { till } => (LanguageVersion::EARLIEST, Some(*till)),
            Self::Range { from, till } => (*from, Some(*till)),
        }
    }
}

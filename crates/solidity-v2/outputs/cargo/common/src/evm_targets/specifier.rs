use serde::Serialize;

use crate::evm_targets::EvmTarget;

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub enum EvmTargetSpecifier {
    From { from: EvmTarget },
    Till { till: EvmTarget },
    Range { from: EvmTarget, till: EvmTarget },
}

impl EvmTargetSpecifier {
    #[inline]
    pub const fn from(from: EvmTarget) -> Self {
        Self::From { from }
    }

    #[inline]
    pub const fn till(till: EvmTarget) -> Self {
        Self::Till { till }
    }

    #[inline]
    pub const fn range(from: EvmTarget, till: EvmTarget) -> Self {
        Self::Range { from, till }
    }
}

impl EvmTargetSpecifier {
    pub fn contains(&self, other: EvmTarget) -> bool {
        match self {
            Self::From { from } => *from <= other,
            Self::Till { till } => other < *till,
            Self::Range { from, till } => *from <= other && other < *till,
        }
    }
}

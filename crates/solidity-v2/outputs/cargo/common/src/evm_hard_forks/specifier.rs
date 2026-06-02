use serde::Serialize;

use crate::evm_hard_forks::EvmHardFork;

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub enum EvmHardForkSpecifier {
    From {
        from: EvmHardFork,
    },
    Till {
        till: EvmHardFork,
    },
    Range {
        from: EvmHardFork,
        till: EvmHardFork,
    },
}

impl EvmHardForkSpecifier {
    #[inline]
    pub const fn from(from: EvmHardFork) -> Self {
        Self::From { from }
    }

    #[inline]
    pub const fn till(till: EvmHardFork) -> Self {
        Self::Till { till }
    }

    #[inline]
    pub const fn range(from: EvmHardFork, till: EvmHardFork) -> Self {
        Self::Range { from, till }
    }
}

impl EvmHardForkSpecifier {
    #[inline]
    pub const fn contains(&self, other: EvmHardFork) -> bool {
        match self {
            Self::From { from } => (*from as u32) <= (other as u32),
            Self::Till { till } => (other as u32) < (*till as u32),
            Self::Range { from, till } => {
                (*from as u32) <= (other as u32) && (other as u32) < (*till as u32)
            }
        }
    }
}

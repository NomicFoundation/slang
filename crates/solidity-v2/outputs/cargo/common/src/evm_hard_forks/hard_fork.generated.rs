// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use std::fmt::{Display, Formatter};
use std::str::FromStr;

use serde::Serialize;
use thiserror::Error;

/// All supported EVM hard forks of `Solidity`, in chronological order.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize)]
pub enum EvmHardFork {
    Frontier = 0,
    Homestead = 1,
    TangerineWhistle = 2,
    SpuriousDragon = 3,
    Byzantium = 4,
    Constantinople = 5,
    Petersburg = 6,
    Istanbul = 7,
    Berlin = 8,
    London = 9,
    Paris = 10,
    Shanghai = 11,
    Cancun = 12,
    Prague = 13,
    Osaka = 14,
}

impl EvmHardFork {
    /// The earliest supported EVM hard fork of `Solidity`.
    pub const EARLIEST: Self = Self::Frontier;

    /// The latest supported EVM hard fork of `Solidity`.
    pub const LATEST: Self = Self::Osaka;

    /// All supported EVM hard forks of `Solidity`, in order.
    pub const ALL: &'static [EvmHardFork; 15] = &[
        EvmHardFork::Frontier,
        EvmHardFork::Homestead,
        EvmHardFork::TangerineWhistle,
        EvmHardFork::SpuriousDragon,
        EvmHardFork::Byzantium,
        EvmHardFork::Constantinople,
        EvmHardFork::Petersburg,
        EvmHardFork::Istanbul,
        EvmHardFork::Berlin,
        EvmHardFork::London,
        EvmHardFork::Paris,
        EvmHardFork::Shanghai,
        EvmHardFork::Cancun,
        EvmHardFork::Prague,
        EvmHardFork::Osaka,
    ];
}

impl Display for EvmHardFork {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            EvmHardFork::Frontier => write!(f, "Frontier"),
            EvmHardFork::Homestead => write!(f, "Homestead"),
            EvmHardFork::TangerineWhistle => write!(f, "TangerineWhistle"),
            EvmHardFork::SpuriousDragon => write!(f, "SpuriousDragon"),
            EvmHardFork::Byzantium => write!(f, "Byzantium"),
            EvmHardFork::Constantinople => write!(f, "Constantinople"),
            EvmHardFork::Petersburg => write!(f, "Petersburg"),
            EvmHardFork::Istanbul => write!(f, "Istanbul"),
            EvmHardFork::Berlin => write!(f, "Berlin"),
            EvmHardFork::London => write!(f, "London"),
            EvmHardFork::Paris => write!(f, "Paris"),
            EvmHardFork::Shanghai => write!(f, "Shanghai"),
            EvmHardFork::Cancun => write!(f, "Cancun"),
            EvmHardFork::Prague => write!(f, "Prague"),
            EvmHardFork::Osaka => write!(f, "Osaka"),
        }
    }
}

#[derive(Debug, Error, PartialEq)]
pub enum FromStrError {
    #[error("Provided value is not recognized as a supported EVM hard fork.")]
    UnrecognizedEvmHardFork,
}

impl FromStr for EvmHardFork {
    type Err = FromStrError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "Frontier" => Ok(EvmHardFork::Frontier),
            "Homestead" => Ok(EvmHardFork::Homestead),
            "TangerineWhistle" => Ok(EvmHardFork::TangerineWhistle),
            "SpuriousDragon" => Ok(EvmHardFork::SpuriousDragon),
            "Byzantium" => Ok(EvmHardFork::Byzantium),
            "Constantinople" => Ok(EvmHardFork::Constantinople),
            "Petersburg" => Ok(EvmHardFork::Petersburg),
            "Istanbul" => Ok(EvmHardFork::Istanbul),
            "Berlin" => Ok(EvmHardFork::Berlin),
            "London" => Ok(EvmHardFork::London),
            "Paris" => Ok(EvmHardFork::Paris),
            "Shanghai" => Ok(EvmHardFork::Shanghai),
            "Cancun" => Ok(EvmHardFork::Cancun),
            "Prague" => Ok(EvmHardFork::Prague),
            "Osaka" => Ok(EvmHardFork::Osaka),
            _ => Err(FromStrError::UnrecognizedEvmHardFork),
        }
    }
}

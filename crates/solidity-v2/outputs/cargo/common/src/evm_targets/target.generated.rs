// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use std::fmt::{Display, Formatter};
use std::str::FromStr;

use serde::Serialize;
use thiserror::Error;

/// All supported EVM targets of `Solidity`, in chronological order.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize)]
pub enum EvmTarget {
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

impl EvmTarget {
    /// The earliest supported EVM target of `Solidity`.
    pub const EARLIEST: Self = Self::Frontier;

    /// The latest supported EVM target of `Solidity`.
    pub const LATEST: Self = Self::Osaka;

    /// All supported EVM targets of `Solidity`, in order.
    pub const ALL: &'static [EvmTarget; 15] = &[
        EvmTarget::Frontier,
        EvmTarget::Homestead,
        EvmTarget::TangerineWhistle,
        EvmTarget::SpuriousDragon,
        EvmTarget::Byzantium,
        EvmTarget::Constantinople,
        EvmTarget::Petersburg,
        EvmTarget::Istanbul,
        EvmTarget::Berlin,
        EvmTarget::London,
        EvmTarget::Paris,
        EvmTarget::Shanghai,
        EvmTarget::Cancun,
        EvmTarget::Prague,
        EvmTarget::Osaka,
    ];
}

impl Display for EvmTarget {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            EvmTarget::Frontier => write!(f, "Frontier"),
            EvmTarget::Homestead => write!(f, "Homestead"),
            EvmTarget::TangerineWhistle => write!(f, "TangerineWhistle"),
            EvmTarget::SpuriousDragon => write!(f, "SpuriousDragon"),
            EvmTarget::Byzantium => write!(f, "Byzantium"),
            EvmTarget::Constantinople => write!(f, "Constantinople"),
            EvmTarget::Petersburg => write!(f, "Petersburg"),
            EvmTarget::Istanbul => write!(f, "Istanbul"),
            EvmTarget::Berlin => write!(f, "Berlin"),
            EvmTarget::London => write!(f, "London"),
            EvmTarget::Paris => write!(f, "Paris"),
            EvmTarget::Shanghai => write!(f, "Shanghai"),
            EvmTarget::Cancun => write!(f, "Cancun"),
            EvmTarget::Prague => write!(f, "Prague"),
            EvmTarget::Osaka => write!(f, "Osaka"),
        }
    }
}

#[derive(Debug, Error, PartialEq)]
pub enum FromStrError {
    #[error("Provided value is not recognized as a supported EVM target.")]
    UnrecognizedEvmTarget,
}

impl FromStr for EvmTarget {
    type Err = FromStrError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "Frontier" => Ok(EvmTarget::Frontier),
            "Homestead" => Ok(EvmTarget::Homestead),
            "TangerineWhistle" => Ok(EvmTarget::TangerineWhistle),
            "SpuriousDragon" => Ok(EvmTarget::SpuriousDragon),
            "Byzantium" => Ok(EvmTarget::Byzantium),
            "Constantinople" => Ok(EvmTarget::Constantinople),
            "Petersburg" => Ok(EvmTarget::Petersburg),
            "Istanbul" => Ok(EvmTarget::Istanbul),
            "Berlin" => Ok(EvmTarget::Berlin),
            "London" => Ok(EvmTarget::London),
            "Paris" => Ok(EvmTarget::Paris),
            "Shanghai" => Ok(EvmTarget::Shanghai),
            "Cancun" => Ok(EvmTarget::Cancun),
            "Prague" => Ok(EvmTarget::Prague),
            "Osaka" => Ok(EvmTarget::Osaka),
            _ => Err(FromStrError::UnrecognizedEvmTarget),
        }
    }
}

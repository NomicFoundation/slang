// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use std::fmt::{Display, Formatter};

use serde::Serialize;
use thiserror::Error;

/// All supported EVM targets of `Solidity`, in chronological order.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "camelCase")]
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
    Amsterdam = 15,
}

impl EvmTarget {
    /// The earliest supported EVM target of `Solidity`.
    pub const EARLIEST: Self = Self::Frontier;

    /// The latest supported EVM target of `Solidity`.
    pub const LATEST: Self = Self::Amsterdam;

    /// All supported EVM targets of `Solidity`, in order.
    pub const ALL: &'static [EvmTarget; 16] = &[
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
        EvmTarget::Amsterdam,
    ];
}

/// Formats the target matching `solc`'s format (camelCase).
impl Display for EvmTarget {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            EvmTarget::Frontier => write!(f, "frontier"),
            EvmTarget::Homestead => write!(f, "homestead"),
            EvmTarget::TangerineWhistle => write!(f, "tangerineWhistle"),
            EvmTarget::SpuriousDragon => write!(f, "spuriousDragon"),
            EvmTarget::Byzantium => write!(f, "byzantium"),
            EvmTarget::Constantinople => write!(f, "constantinople"),
            EvmTarget::Petersburg => write!(f, "petersburg"),
            EvmTarget::Istanbul => write!(f, "istanbul"),
            EvmTarget::Berlin => write!(f, "berlin"),
            EvmTarget::London => write!(f, "london"),
            EvmTarget::Paris => write!(f, "paris"),
            EvmTarget::Shanghai => write!(f, "shanghai"),
            EvmTarget::Cancun => write!(f, "cancun"),
            EvmTarget::Prague => write!(f, "prague"),
            EvmTarget::Osaka => write!(f, "osaka"),
            EvmTarget::Amsterdam => write!(f, "amsterdam"),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Error, Hash)]
pub enum EvmTargetConversionError {
    #[error("Provided value is not recognized as a supported EVM target.")]
    UnrecognizedEvmTarget,
}

impl TryFrom<&str> for EvmTarget {
    type Error = EvmTargetConversionError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "frontier" => Ok(EvmTarget::Frontier),
            "homestead" => Ok(EvmTarget::Homestead),
            "tangerineWhistle" => Ok(EvmTarget::TangerineWhistle),
            "spuriousDragon" => Ok(EvmTarget::SpuriousDragon),
            "byzantium" => Ok(EvmTarget::Byzantium),
            "constantinople" => Ok(EvmTarget::Constantinople),
            "petersburg" => Ok(EvmTarget::Petersburg),
            "istanbul" => Ok(EvmTarget::Istanbul),
            "berlin" => Ok(EvmTarget::Berlin),
            "london" => Ok(EvmTarget::London),
            "paris" => Ok(EvmTarget::Paris),
            "shanghai" => Ok(EvmTarget::Shanghai),
            "cancun" => Ok(EvmTarget::Cancun),
            "prague" => Ok(EvmTarget::Prague),
            "osaka" => Ok(EvmTarget::Osaka),
            "amsterdam" => Ok(EvmTarget::Amsterdam),
            _ => Err(EvmTargetConversionError::UnrecognizedEvmTarget),
        }
    }
}

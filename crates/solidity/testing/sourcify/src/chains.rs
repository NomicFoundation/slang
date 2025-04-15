#![allow(clippy::unreadable_literal)]

use clap::{Subcommand, ValueEnum};
use strum_macros::AsRefStr;

#[derive(Subcommand, Debug, AsRefStr, PartialEq, Copy, Clone)]
#[strum(serialize_all = "lowercase")]
pub enum Chain {
    Arbitrum { network: ArbitrumNetwork },
    Avalanche { network: AvalancheNetwork },
    Ethereum { network: EthereumNetwork },
    Fantom { network: FantomNetwork },
    Polygon { network: PolygonNetwork },
}

impl Chain {
    pub fn id(self) -> u64 {
        match self {
            Chain::Arbitrum { network } => network as u64,
            Chain::Avalanche { network } => network as u64,
            Chain::Ethereum { network } => network as u64,
            Chain::Fantom { network } => network as u64,
            Chain::Polygon { network } => network as u64,
        }
    }

    pub fn name(&self) -> &str {
        self.as_ref()
    }

    pub fn network_name(&self) -> &str {
        match self {
            Chain::Arbitrum { network } => network.as_ref(),
            Chain::Avalanche { network } => network.as_ref(),
            Chain::Ethereum { network } => network.as_ref(),
            Chain::Fantom { network } => network.as_ref(),
            Chain::Polygon { network } => network.as_ref(),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, ValueEnum, AsRefStr)]
#[strum(serialize_all = "lowercase")]
pub enum ArbitrumNetwork {
    One = 42161,
    Nova = 42170,
}

#[derive(Clone, Copy, Debug, PartialEq, ValueEnum, AsRefStr)]
#[strum(serialize_all = "lowercase")]
pub enum AvalancheNetwork {
    CChain = 43114,
    Testnet = 43113,
}

#[derive(Clone, Copy, Debug, PartialEq, ValueEnum, AsRefStr)]
pub enum EthereumNetwork {
    Mainnet = 1,
    Sepolia = 11155111,
}

#[derive(Clone, Copy, Debug, PartialEq, ValueEnum, AsRefStr)]
#[strum(serialize_all = "lowercase")]
pub enum FantomNetwork {
    Opera = 250,
}

#[derive(Clone, Copy, Debug, PartialEq, ValueEnum, AsRefStr)]
#[strum(serialize_all = "lowercase")]
pub enum PolygonNetwork {
    Mainnet = 137,
    Amoy = 80002,
}

use clap::{Subcommand, ValueEnum};
use strum_macros::AsRefStr;

#[derive(Subcommand, Debug, AsRefStr, PartialEq, Copy, Clone)]
#[strum(serialize_all = "lowercase")]
pub enum Chain {
    Ethereum { network: EthereumNetwork },
}

impl Chain {
    pub fn id(self) -> u64 {
        match self {
            Chain::Ethereum { network } => match network {
                EthereumNetwork::Mainnet => 1,
            },
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            Chain::Ethereum { .. } => "ethereum",
        }
    }

    pub fn network_name(&self) -> &'static str {
        match self {
            Chain::Ethereum { network } => match network {
                EthereumNetwork::Mainnet => "mainnet",
            },
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, ValueEnum, AsRefStr)]
enum EthereumNetwork {
    Mainnet,
}

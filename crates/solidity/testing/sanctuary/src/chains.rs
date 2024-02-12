use anyhow::Result;
use clap::{Subcommand, ValueEnum};
use strum_macros::AsRefStr;
use url::Url;

/// Full list defined here:
/// <https://github.com/tintinweb/smart-contract-sanctuary>
#[derive(Debug, Subcommand, AsRefStr)]
#[strum(serialize_all = "lowercase")]
pub enum Chain {
    Arbitrum { network: ArbitrumNetwork },
    Avalanche { network: AvalancheNetwork },
    Bsc { network: BscNetwork },
    Celo { network: CeloNetwork },
    Ethereum { network: EthereumNetwork },
    Fantom { network: FantomNetwork },
    Optimism { network: OptimismNetwork },
    Polygon { network: PolygonNetwork },
    Tron { network: TronNetwork },
}

impl Chain {
    pub fn name(&self) -> &str {
        self.as_ref()
    }

    pub fn github_url(&self) -> Result<Url> {
        Ok(Url::parse(&format!(
            "https://github.com/tintinweb/smart-contract-sanctuary-{name}",
            name = self.name()
        ))?)
    }

    pub fn network_name(&self) -> &str {
        match self {
            Chain::Arbitrum { network } => network.as_ref(),
            Chain::Avalanche { network } => network.as_ref(),
            Chain::Bsc { network } => network.as_ref(),
            Chain::Celo { network } => network.as_ref(),
            Chain::Ethereum { network } => network.as_ref(),
            Chain::Fantom { network } => network.as_ref(),
            Chain::Optimism { network } => network.as_ref(),
            Chain::Polygon { network } => network.as_ref(),
            Chain::Tron { network } => network.as_ref(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, ValueEnum, AsRefStr)]
#[strum(serialize_all = "lowercase")]
pub enum ArbitrumNetwork {
    Mainnet,
    Testnet,
}

#[derive(Clone, Debug, PartialEq, ValueEnum, AsRefStr)]
#[strum(serialize_all = "lowercase")]
pub enum AvalancheNetwork {
    Mainnet,
    Testnet,
}

#[derive(Clone, Debug, PartialEq, ValueEnum, AsRefStr)]
#[strum(serialize_all = "lowercase")]
pub enum BscNetwork {
    Mainnet,
    Testnet,
}

#[derive(Clone, Debug, PartialEq, ValueEnum, AsRefStr)]
#[strum(serialize_all = "lowercase")]
pub enum CeloNetwork {
    Mainnet,
}

#[derive(Clone, Debug, PartialEq, ValueEnum, AsRefStr)]
#[strum(serialize_all = "lowercase")]
pub enum EthereumNetwork {
    Goerli,
    Kovan,
    Mainnet,
    Rinkeby,
    Ropsten,
    Sepolia,
}

#[derive(Clone, Debug, PartialEq, ValueEnum, AsRefStr)]
#[strum(serialize_all = "lowercase")]
pub enum FantomNetwork {
    Mainnet,
}

#[derive(Clone, Debug, PartialEq, ValueEnum, AsRefStr)]
#[strum(serialize_all = "lowercase")]
pub enum OptimismNetwork {
    Mainnet,
}

#[derive(Clone, Debug, PartialEq, ValueEnum, AsRefStr)]
#[strum(serialize_all = "lowercase")]
pub enum PolygonNetwork {
    Mainnet,
    Mumbai,
}

#[derive(Clone, Debug, PartialEq, ValueEnum, AsRefStr)]
#[strum(serialize_all = "lowercase")]
pub enum TronNetwork {
    Mainnet,
}

use anyhow::Result;
use clap::{Parser, Subcommand, ValueEnum};
use reqwest::blocking::Client;
use serde::Deserialize;
use strum_macros::AsRefStr;

#[derive(Debug, Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Test(TestCommand),
}

#[derive(Debug, Parser)]
struct TestCommand {
    #[command(subcommand)]
    chain: Chain
}

#[derive(Subcommand, Debug, AsRefStr)]
#[strum(serialize_all = "lowercase")]
enum Chain {
    Ethereum{ network: EthereumNetwork },
}

impl Chain {
    fn id(&self) -> u64 {
        match self {
            Chain::Ethereum{ network } => match network {
                EthereumNetwork::Mainnet => 1,
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq, ValueEnum, AsRefStr)]
enum EthereumNetwork {
    Mainnet,
}

fn main() -> Result<()> {
    let Cli { command } = Cli::parse();

    match command {
        Commands::Test(test_command) => run_test_command(&test_command),
    }
}

fn run_test_command(cmd: &TestCommand) -> Result<()> {
    let contracts = get_chain_contracts(&cmd.chain, None)?;
    for contract in contracts {
        println!("Contract: {}\nMatch ID: {}", contract.address, contract.match_id);
    }

    Ok(())
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct ContractMatch {
    address: String,
    match_id: String,
}

#[derive(Deserialize)]
struct ContractsResponse {
    results: Vec<ContractMatch>,
}

fn get_chain_contracts(chain: &Chain, last_match_id: Option<String>) -> Result<Vec<ContractMatch>> {
    let chain_id = chain.id();

    let client = Client::new();
    let mut builder = client.get(format!("https://sourcify.dev/server/v2/contracts/{chain_id}?limit=10")).header("Accept", "application/json");
    if let Some(last_match_id) = last_match_id {
        builder = builder.query(&[("lastMatchId", last_match_id)]);
    }

    let res = builder.send()?;
    let status = res.status();
    let body = res.text()?;
    println!("{status} - {body}");
    let contracts: ContractsResponse = serde_json::from_str(&body)?;

    Ok(contracts.results)
}


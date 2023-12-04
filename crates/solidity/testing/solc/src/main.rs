mod keywords;
mod utils;

use crate::keywords::check_solidity_keywords;
use anyhow::Result;
use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
pub struct CLI {
    #[command(subcommand)]
    command: AppCommand,
}

#[derive(Debug, Subcommand)]
enum AppCommand {
    /// Makes sure all Solidity keywords have the corrent metadata.
    CheckSolidityKeywords,
}

fn main() -> Result<()> {
    match CLI::parse().command {
        AppCommand::CheckSolidityKeywords => check_solidity_keywords(),
    }
}

#[test]
fn verify_clap_cli() {
    // Catches problems earlier in the development cycle:
    <CLI as clap::CommandFactory>::command().debug_assert();
}

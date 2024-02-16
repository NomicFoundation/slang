mod commands;
mod utils;

use anyhow::Result;
use clap::Parser;

use crate::commands::AppCommand;

#[derive(Debug, Parser)]
struct Cli {
    #[command(subcommand)]
    command: AppCommand,
}

pub fn main() -> Result<()> {
    Cli::parse().command.execute()
}

#[test]
fn verify_clap_cli() {
    // Catches problems earlier in the development cycle:
    <Cli as clap::CommandFactory>::command().debug_assert();
}

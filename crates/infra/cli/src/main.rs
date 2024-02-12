pub mod commands;
pub mod toolchains;
pub mod utils;

use clap::Parser;
use infra_utils::terminal::Terminal;

use crate::commands::AppCommand;

#[derive(Debug, Parser)]
pub struct CLI {
    #[command(subcommand)]
    command: AppCommand,
}

#[allow(dead_code)] // as it is referenced from 'build.rs' of the same crate.
fn main() {
    Terminal::wrap_execution(|| CLI::parse().command.execute());
}

#[test]
fn verify_clap_cli() {
    // Catches problems earlier in the development cycle:
    <CLI as clap::CommandFactory>::command().debug_assert();
}

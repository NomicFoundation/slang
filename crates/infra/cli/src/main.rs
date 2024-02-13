mod commands;
mod toolchains;
mod utils;

use clap::Parser;
use infra_utils::terminal::Terminal;

use crate::commands::Cli;

fn main() {
    Terminal::wrap_execution(|| Cli::parse().execute());
}

#[test]
fn verify_clap_cli() {
    // Catches problems earlier in the development cycle:
    <Cli as clap::CommandFactory>::command().debug_assert();
}

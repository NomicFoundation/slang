mod parse;

use clap::{Parser, Subcommand};

use crate::parse::ParseCommand;

#[derive(Parser, Debug)]
#[command(next_line_help = true)]
#[command(author, about)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Parses a source file, and outputs any syntax errors, or a JSON concrete syntax tree
    Parse(ParseCommand),
}

fn main() {
    match Cli::parse().command {
        Commands::Parse(command) => command.execute(),
    };
}

#[test]
fn verify_clap_cli() {
    // Catches problems earlier in the development cycle:
    <Cli as clap::CommandFactory>::command().debug_assert();
}

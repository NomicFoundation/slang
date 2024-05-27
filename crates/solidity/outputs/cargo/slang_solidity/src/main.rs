use std::process::ExitCode;

use clap::{Parser as ClapParser, Subcommand};
use semver::Version;
use slang_solidity::commands::{execute_build_graph_command, execute_parse_command};

// Below are dependencies used by the API `lib.rs`, but not the CLI "main.rs".
// However, we need to add a fake usage to suppress Cargo warnings about unused dependencies.
// This is a known issue, and we should remove this hack once there is a better solution from Cargo.
// https://github.com/rust-lang/cargo/issues/1982
mod supress_api_dependencies {
    use {
        ariadne as _, metaslang_cst as _, metaslang_graph_builder as _, serde as _,
        serde_json as _, strum as _, strum_macros as _, thiserror as _,
    };
}

#[derive(ClapParser, Debug)]
#[command(next_line_help = true)]
#[command(author, about)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Parses a Solidity (*.sol) source file, and outputs any syntax errors, or a JSON concrete syntax tree
    Parse {
        /// File path to the Solidity (*.sol) source file to parse
        file_path: String,

        /// The Solidity language version to use for parsing
        #[arg(short, long)]
        version: Version,

        /// Print the concrete syntax tree as JSON
        #[clap(long)]
        json: bool,
    },

    /// Parses a Solidity (*.sol) source file and builds a graph executing the instructions from the builder file (*.msgb)
    BuildGraph {
        /// File path to the Solidity (*.sol) source file to parse
        file_path: String,

        /// The Solidity language version to use for parsing
        #[arg(short, long)]
        version: Version,

        /// The graph buider (.msgb) file to use
        msgb_path: String,

        /// Print the graph as JSON
        #[clap(long)]
        json: bool,
    },
}

fn main() -> ExitCode {
    let command_result = match Cli::parse().command {
        Commands::Parse {
            file_path,
            version,
            json,
        } => execute_parse_command(&file_path, version, json),
        Commands::BuildGraph {
            file_path,
            version,
            msgb_path,
            json,
        } => execute_build_graph_command(&file_path, version, &msgb_path, json),
    };
    match command_result {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("{error}");
            ExitCode::FAILURE
        }
    }
}

#[test]
fn verify_clap_cli() {
    // Catches problems earlier in the development cycle:
    <Cli as clap::CommandFactory>::command().debug_assert();
}

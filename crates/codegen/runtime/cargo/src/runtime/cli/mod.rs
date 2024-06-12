use std::process::ExitCode;

use clap::Subcommand;
use semver::Version;

pub mod commands;

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Parses a source file, and outputs any syntax errors, or a JSON concrete syntax tree
    Parse {
        /// File path to the source file to parse
        file_path: String,

        /// The language version to use for parsing
        #[arg(short, long)]
        version: Version,

        /// Print the concrete syntax tree as JSON
        #[clap(long)]
        json: bool,
    },

    // This is only intended for internal development
    #[cfg(feature = "__experimental_bindings_api")]
    /// Parses a source file and builds a graph executing the instructions from the builder file (*.msgb)
    BuildGraph {
        /// File path to the source file to parse
        file_path: String,

        /// The language version to use for parsing
        #[arg(short, long)]
        version: Version,

        /// The graph buider (.msgb) file to use
        msgb_path: String,

        /// Print the graph as JSON
        #[clap(long)]
        json: bool,

        /// Include debug info (location, variable and match) in the built graph
        #[clap(long)]
        debug: bool,
    },
}

impl Commands {
    pub fn execute(self) -> ExitCode {
        let command_result = match self {
            Commands::Parse {
                file_path,
                version,
                json,
            } => commands::parse::execute(&file_path, version, json),
            #[cfg(feature = "__experimental_bindings_api")]
            Commands::BuildGraph {
                file_path,
                version,
                msgb_path,
                json,
                debug,
            } => commands::build_graph::execute(&file_path, version, &msgb_path, json, debug),
        };
        match command_result {
            Ok(()) => ExitCode::SUCCESS,
            Err(error) => {
                eprintln!("{error}");
                ExitCode::FAILURE
            }
        }
    }
}

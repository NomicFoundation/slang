// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

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

    /// This is only intended for internal development
    #[cfg(feature = "__experimental_bindings_api")]
    Bindings {
        /// File path to the source file to parse
        file_path: String,

        /// The language version to use for parsing
        #[arg(short, long)]
        version: Version,
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
            Commands::Bindings { file_path, version } => {
                commands::bindings::execute(&file_path, version)
            }
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

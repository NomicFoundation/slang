use std::process::ExitCode;

use clap::Subcommand;
#[allow(unused_imports)]
use slang_solidity::cli::commands::CommandError;

#[derive(Subcommand, Debug)]
pub enum CustomCommands {
    #[cfg(feature = "__experimental_bindings_api")]
    CheckAssertions {
        /// File path to the source file to parse
        file_path: String,

        /// The language version to use for parsing
        #[arg(short, long)]
        version: semver::Version,
    },
}

impl CustomCommands {
    #[cfg(not(feature = "__experimental_bindings_api"))]
    pub fn execute(self) -> ExitCode {
        unreachable!()
    }

    #[cfg(feature = "__experimental_bindings_api")]
    pub fn execute(self) -> ExitCode {
        let result: Result<(), CommandError> = match self {
            Self::CheckAssertions { file_path, version } => {
                super::assertions::execute_check_assertions(&file_path, version)
            }
        };
        match result {
            Ok(()) => ExitCode::SUCCESS,
            Err(error) => {
                eprintln!("{error}");
                ExitCode::FAILURE
            }
        }
    }
}

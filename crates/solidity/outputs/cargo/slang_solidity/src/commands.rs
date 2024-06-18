use std::process::ExitCode;

use clap::Subcommand;
use semver::Version;
use slang_solidity::cli::commands;
use slang_solidity::cli::commands::CommandError;
use thiserror::Error;

#[derive(Subcommand, Debug)]
pub enum LocalCommands {
    #[cfg(feature = "__experimental_bindings_api")]
    CheckAssertions {
        /// File path to the source file to parse
        file_path: String,

        /// The language version to use for parsing
        #[arg(short, long)]
        version: semver::Version,
    },
}

#[derive(Error, Debug)]
pub enum LocalCommandError {
    #[error(transparent)]
    Command(#[from] CommandError),

    #[cfg(feature = "__experimental_bindings_api")]
    #[error(transparent)]
    Assertion(#[from] crate::assertions::AssertionError),
}

impl LocalCommands {
    #[cfg(not(feature = "__experimental_bindings_api"))]
    pub fn execute(self) -> ExitCode {
        unreachable!()
    }

    #[cfg(feature = "__experimental_bindings_api")]
    pub fn execute(self) -> ExitCode {
        let result: Result<(), LocalCommandError> = match self {
            Self::CheckAssertions { file_path, version } => {
                check_assertions_command::execute(&file_path, version)
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

#[cfg(feature = "__experimental_bindings_api")]
mod check_assertions_command {
    use slang_solidity::bindings;

    use super::{commands, LocalCommandError, Version};
    use crate::assertions;

    pub fn execute(file_path_string: &str, version: Version) -> Result<(), LocalCommandError> {
        let mut bindings = bindings::create(version.clone());
        let parse_output = commands::parse::parse_source_file(file_path_string, version, |_| ())?;
        let tree_cursor = parse_output.create_tree_cursor();

        bindings.add_file(file_path_string, tree_cursor.clone());
        let assertions = assertions::collect_assertions(tree_cursor)?;

        assertions::check_assertions(&bindings, &assertions)?;

        Ok(())
    }
}

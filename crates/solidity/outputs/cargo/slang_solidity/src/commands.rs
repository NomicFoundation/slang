use std::process::ExitCode;

#[allow(unused_imports)]
use anyhow::Error;
use clap::Subcommand;

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
        let result: Result<(), Error> = match self {
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

use std::process::ExitCode;

use clap::Subcommand;

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

impl LocalCommands {
    #[cfg(not(feature = "__experimental_bindings_api"))]
    pub fn execute(self) -> ExitCode {
        unreachable!()
    }

    #[cfg(feature = "__experimental_bindings_api")]
    pub fn execute(self) -> ExitCode {
        let result: anyhow::Result<()> = match self {
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
    use anyhow::Result;
    use semver::Version;
    use slang_solidity::cli::commands;
    use slang_solidity::{assertions, bindings};

    pub fn execute(file_path_string: &str, version: Version) -> Result<()> {
        let mut bindings = bindings::create(version.clone());
        let parse_output = commands::parse::parse_source_file(file_path_string, version, |_| ())?;
        let tree_cursor = parse_output.create_tree_cursor();

        bindings.add_file(file_path_string, tree_cursor.clone());
        let assertions = assertions::collect_assertions(tree_cursor)?;

        assertions::check_assertions(&bindings, &assertions)?;

        Ok(())
    }
}

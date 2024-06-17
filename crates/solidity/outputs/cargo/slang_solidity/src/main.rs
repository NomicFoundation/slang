use std::process::ExitCode;

use clap::error::{Error, ErrorKind};
use clap::{ArgMatches, Command, FromArgMatches, Parser as ClapParser, Subcommand};
use slang_solidity::cli;

// Below are dependencies used by the API `lib.rs`, but not the CLI "main.rs".
// However, we need to add a fake usage to suppress Cargo warnings about unused dependencies.
// This is a known issue, and we should remove this hack once there is a better solution from Cargo.
// https://github.com/rust-lang/cargo/issues/1982
mod supress_api_dependencies {
    #[cfg(feature = "__experimental_bindings_api")]
    use metaslang_bindings as _;
    use {
        ariadne as _, metaslang_cst as _, semver as _, serde as _, serde_json as _, strum as _,
        strum_macros as _, thiserror as _,
    };
}

#[cfg(feature = "__experimental_bindings_api")]
mod assertions;
mod commands;

use commands::CustomCommands;

#[derive(ClapParser, Debug)]
#[command(next_line_help = true)]
#[command(author, about)]
struct Cli {
    #[command(subcommand)]
    command: CliCommand,
}

#[derive(Debug)]
enum CliCommand {
    Common(cli::Commands),
    Custom(CustomCommands),
}

impl CliCommand {
    fn execute(self) -> ExitCode {
        match self {
            Self::Common(command) => command.execute(),
            Self::Custom(command) => command.execute(),
        }
    }
}

impl FromArgMatches for CliCommand {
    fn from_arg_matches(matches: &ArgMatches) -> Result<Self, Error> {
        if let Ok(common) = cli::Commands::from_arg_matches(matches) {
            Ok(Self::Common(common))
        } else if let Ok(custom) = CustomCommands::from_arg_matches(matches) {
            Ok(Self::Custom(custom))
        } else {
            Err(Error::new(ErrorKind::MissingSubcommand))
        }
    }
    fn update_from_arg_matches(&mut self, matches: &ArgMatches) -> Result<(), Error> {
        if let Ok(common) = cli::Commands::from_arg_matches(matches) {
            *self = Self::Common(common);
        } else if let Ok(custom) = CustomCommands::from_arg_matches(matches) {
            *self = Self::Custom(custom);
        } else {
            return Err(Error::new(ErrorKind::MissingSubcommand));
        }
        Ok(())
    }
}

impl Subcommand for CliCommand {
    fn augment_subcommands(cmd: Command) -> Command {
        let cmd = cli::Commands::augment_subcommands(cmd);
        CustomCommands::augment_subcommands(cmd)
    }
    fn augment_subcommands_for_update(cmd: Command) -> Command {
        let cmd = cli::Commands::augment_subcommands(cmd);
        CustomCommands::augment_subcommands(cmd)
    }
    fn has_subcommand(name: &str) -> bool {
        cli::Commands::has_subcommand(name) || CustomCommands::has_subcommand(name)
    }
}

fn main() -> ExitCode {
    Cli::parse().command.execute()
}

#[test]
fn verify_clap_cli() {
    // Catches problems earlier in the development cycle:
    <Cli as clap::CommandFactory>::command().debug_assert();
}

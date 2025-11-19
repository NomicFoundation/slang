mod dissect;
mod keyword_versioning;

use anyhow::Result;
use clap::Subcommand;

use crate::commands::dissect::DissectCommand;
use crate::commands::keyword_versioning::KeywordVersioningCommand;

#[derive(Debug, Subcommand)]
pub enum AppCommand {
    /// Makes sure all Solidity definition keywords have the correct version specifiers.
    KeywordVersioning(KeywordVersioningCommand),

    /// Compiles a Solidity file with all versions of `solc`, listing which versions succeeded/failed.
    Dissect(DissectCommand),
}

impl AppCommand {
    pub fn execute(self) -> Result<()> {
        match self {
            AppCommand::KeywordVersioning(command) => command.execute(),
            AppCommand::Dissect(command) => command.execute(),
        }
    }
}

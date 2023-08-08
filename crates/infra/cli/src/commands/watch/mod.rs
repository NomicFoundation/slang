use anyhow::Result;
use clap::{Parser, ValueEnum};

use crate::{
    toolchains::mkdocs::Mkdocs,
    utils::{ClapExtensions, OrderedCommand, Terminal},
};

#[derive(Clone, Debug, Parser)]
pub struct WatchController {
    #[clap(trailing_var_arg = true)]
    commands: Vec<WatchCommand>,
}

impl WatchController {
    pub fn execute(&self) -> Result<()> {
        return WatchCommand::execute_all(&self.commands);
    }
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd, ValueEnum)]
pub enum WatchCommand {
    /// Build and serve documentation locally, watching for changes.
    Mkdocs,
}

impl OrderedCommand for WatchCommand {
    fn execute(&self) -> Result<()> {
        Terminal::step(self.clap_name());

        return match self {
            WatchCommand::Mkdocs => watch_mkdocs(),
        };
    }
}

fn watch_mkdocs() -> Result<()> {
    return Mkdocs::watch();
}

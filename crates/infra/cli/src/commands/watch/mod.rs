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
        WatchCommand::execute_in_order(&self.commands)
    }
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd, ValueEnum)]
enum WatchCommand {
    /// Build and serve documentation locally, watching for changes.
    Mkdocs,
}

impl OrderedCommand for WatchCommand {
    fn execute(&self) -> Result<()> {
        Terminal::step(format!("watch {name}", name = self.clap_name()));

        match self {
            WatchCommand::Mkdocs => watch_mkdocs(),
        }
    }
}

fn watch_mkdocs() -> Result<()> {
    Mkdocs::watch()
}

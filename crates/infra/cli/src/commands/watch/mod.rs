use anyhow::Result;
use clap::{Parser, ValueEnum};
use infra_utils::terminal::Terminal;

use crate::toolchains::mkdocs::Mkdocs;
use crate::utils::{ClapExtensions, OrderedCommand};

#[derive(Clone, Debug, Parser)]
pub struct WatchController {
    command: WatchCommand,
}

impl WatchController {
    pub fn execute(&self) -> Result<()> {
        self.command.execute()
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
            | WatchCommand::Mkdocs => watch_mkdocs(),
        }
    }
}

fn watch_mkdocs() -> Result<()> {
    Mkdocs::watch()
}

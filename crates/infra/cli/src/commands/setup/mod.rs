mod cargo;
mod npm;
mod pipenv;
mod workspace;

use anyhow::Result;
use clap::{Parser, ValueEnum};

use crate::{
    commands::setup::{
        cargo::setup_cargo, npm::setup_npm, pipenv::setup_pipenv, workspace::setup_workspace,
    },
    utils::{ClapExtensions, OrderedCommand, Terminal},
};

#[derive(Clone, Debug, Default, Parser)]
pub struct SetupController {
    #[clap(trailing_var_arg = true)]
    commands: Vec<SetupCommand>,
}

impl SetupController {
    pub fn execute(&self) -> Result<()> {
        return SetupCommand::execute_in_order(&self.commands);
    }
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd, ValueEnum)]
enum SetupCommand {
    /// Configure binaries used by editors for local development..
    Workspace,
    /// Install Cargo dependencies.
    Cargo,
    /// Install NPM dependencies.
    Npm,
    /// Install Pipenv dependencies.
    Pipenv,
}

impl OrderedCommand for SetupCommand {
    fn execute(&self) -> Result<()> {
        Terminal::step(format!("setup {name}", name = self.clap_name()));

        return match self {
            SetupCommand::Workspace => setup_workspace(),
            SetupCommand::Cargo => setup_cargo(),
            SetupCommand::Npm => setup_npm(),
            SetupCommand::Pipenv => setup_pipenv(),
        };
    }
}

mod cargo;
mod git;
mod npm;
mod pipenv;
mod shell_completions;

use anyhow::Result;
use clap::{Parser, ValueEnum};
use infra_utils::terminal::Terminal;

use crate::commands::setup::cargo::setup_cargo;
use crate::commands::setup::git::setup_git;
use crate::commands::setup::npm::setup_npm;
use crate::commands::setup::pipenv::setup_pipenv;
use crate::commands::setup::shell_completions::setup_shell_completions;
use crate::utils::{ClapExtensions, OrderedCommand};

#[derive(Clone, Debug, Default, Parser)]
pub struct SetupController {
    #[clap(trailing_var_arg = true)]
    commands: Vec<SetupCommand>,
}

impl SetupController {
    pub fn execute(&self) -> Result<()> {
        SetupCommand::execute_in_order(&self.commands)
    }
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd, ValueEnum)]
enum SetupCommand {
    /// Prepare the local git client/repository.
    Git,
    /// Install Cargo dependencies.
    Cargo,
    /// Install NPM dependencies.
    Npm,
    /// Install Pipenv dependencies.
    Pipenv,
    /// Generate shell completions for this CLI.
    ShellCompletions,
}

impl OrderedCommand for SetupCommand {
    fn execute(&self) -> Result<()> {
        Terminal::step(format!("setup {name}", name = self.clap_name()));

        match self {
            SetupCommand::Git => setup_git(),
            SetupCommand::Cargo => setup_cargo(),
            SetupCommand::Npm => setup_npm()?,
            SetupCommand::Pipenv => setup_pipenv()?,
            SetupCommand::ShellCompletions => setup_shell_completions()?,
        };

        Ok(())
    }
}

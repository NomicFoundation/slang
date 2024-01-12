mod cargo;
mod npm;
mod pipenv;

use std::path::PathBuf;

use anyhow::Result;
use clap::{CommandFactory, Parser, ValueEnum};
use clap_complete::Shell;
use infra_utils::paths::PathExtensions;

use crate::commands::setup::cargo::setup_cargo;
use crate::commands::setup::npm::setup_npm;
use crate::commands::setup::pipenv::setup_pipenv;
use crate::utils::{ClapExtensions, OrderedCommand, Terminal};

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
    /// Generate shell completions for this CLI.
    ShellCompletions,
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

        match self {
            SetupCommand::ShellCompletions => generate_shell_completions(),
            SetupCommand::Cargo => setup_cargo(),
            SetupCommand::Npm => setup_npm(),
            SetupCommand::Pipenv => setup_pipenv(),
        }
    }
}

fn generate_shell_completions() -> Result<()> {
    println!("Generating shell completions...\n");

    let target_dir = std::env::var("CARGO_TARGET_DIR").unwrap_or_else(|_| "target".to_string());
    let shell_completions_dir = PathBuf::from(target_dir).join("shell-completions");

    std::fs::create_dir_all(&shell_completions_dir)?;

    let mut cmd = crate::commands::Cli::command();
    let bin_name = cmd.get_bin_name().expect("Set by #[command]").to_owned();

    for shell in [Shell::Bash, Shell::Zsh] {
        let file = clap_complete::generate_to(shell, &mut cmd, &bin_name, &shell_completions_dir)?;

        let canonicalized = file.canonicalize()?;
        let stripped = canonicalized.strip_repo_root()?;
        println!("Generated file at {}", stripped.display());
    }

    Ok(())
}

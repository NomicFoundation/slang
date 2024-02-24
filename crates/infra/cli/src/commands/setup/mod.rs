mod cargo;
mod npm;
mod pipenv;

use std::path::Path;

use anyhow::{Context, Result};
use clap::{CommandFactory, Parser, ValueEnum};
use clap_complete::Shell;
use infra_utils::paths::PathExtensions;
use infra_utils::terminal::Terminal;

use crate::commands::setup::cargo::setup_cargo;
use crate::commands::setup::npm::setup_npm;
use crate::commands::setup::pipenv::setup_pipenv;
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
            | SetupCommand::ShellCompletions => generate_shell_completions(),
            | SetupCommand::Cargo => setup_cargo(),
            | SetupCommand::Npm => setup_npm(),
            | SetupCommand::Pipenv => setup_pipenv(),
        }
    }
}

fn generate_shell_completions() -> Result<()> {
    println!("Generating shell completions...\n");

    let shell_completions_dir = Path::repo_path("target/shell-completions");
    std::fs::create_dir_all(&shell_completions_dir)
        .context("Failed to create shell completions directory")?;

    let mut cmd = crate::commands::Cli::command();
    let bin_name = cmd.get_bin_name().expect("Set by #[command]").to_owned();

    for shell in [Shell::Bash, Shell::Zsh] {
        let mut out_buf = vec![];
        clap_complete::generate(shell, &mut cmd, &bin_name, &mut out_buf);

        let file = shell_completions_dir.join(format!("infra.{shell}"));
        std::fs::write(&file, out_buf).context("Couldn't write a shell completion file")?;

        let canonicalized = file.canonicalize()?;
        let stripped = canonicalized.strip_repo_root()?;
        println!("Generated {shell} completions at {}", stripped.display());
    }

    Ok(())
}

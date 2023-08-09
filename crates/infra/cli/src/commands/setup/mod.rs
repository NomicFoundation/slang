use std::{collections::HashMap, path::Path};

use anyhow::{Context, Result};
use clap::{Parser, ValueEnum};
use infra_utils::{
    commands::Command,
    github::GitHub,
    paths::{FileWalker, PathExtensions},
};
use serde::Deserialize;

use crate::utils::{ClapExtensions, OrderedCommand, Terminal};

#[derive(Clone, Debug, Parser)]
pub struct SetupController {
    #[clap(trailing_var_arg = true)]
    commands: Vec<SetupCommand>,
}

impl SetupController {
    pub fn execute(&self) -> Result<()> {
        return SetupCommand::execute_all(&self.commands);
    }
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd, ValueEnum)]
pub enum SetupCommand {
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
        Terminal::step(self.clap_name());

        return match self {
            SetupCommand::Workspace => setup_workspace(),
            SetupCommand::Cargo => setup_cargo(),
            SetupCommand::Npm => setup_npm(),
            SetupCommand::Pipenv => setup_pipenv(),
        };
    }
}

fn setup_workspace() -> Result<()> {
    if GitHub::is_running_in_ci() {
        return Ok(()); // Nothing to do here.
    }

    // Warm up IDE tools in case it was a fresh installation:
    Command::new("rust-analyzer").flag("--version").run()?;
    Command::new("rust-src").flag("--version").run()?;

    return Ok(());
}

fn setup_cargo() -> Result<()> {
    return if GitHub::is_running_in_ci() {
        Command::new("cargo").arg("fetch").flag("--locked").run()
    } else {
        Command::new("cargo").arg("fetch").run()
    };
}

fn setup_npm() -> Result<()> {
    return if GitHub::is_running_in_ci() {
        Command::new("npm").arg("install").flag("--ci").run()
    } else {
        Command::new("npm").arg("install").run()
    };
}

fn setup_pipenv() -> Result<()> {
    #[derive(Deserialize)]
    struct Pipfile {
        packages: HashMap<String, String>,
    }

    // Use the top-level `Pipfile` to find the correct version of `pipenv` to install.
    let pip_file_toml = Path::repo_path("Pipfile").read_to_string()?;
    let pip_file: Pipfile = toml::from_str(&pip_file_toml)?;

    // This should be a value like "==YYYY.MM.DD"
    let version = pip_file
        .packages
        .get("pipenv")
        .context("Failed to find 'pipenv' in 'Pipfile' packages.")?;

    // pip3 install "pipenv==YYYY.MM.DD"
    Command::new("pip3")
        .arg("install")
        .arg(format!("pipenv{version}"))
        .run()?;

    // Each Python project has its own Pipfile at the root of the project.
    // Find all of them, and run 'pipenv install' in each directory.
    for pip_file in FileWalker::from_repo_root().find(["**/Pipfile"])? {
        let project_directory = pip_file.unwrap_parent();

        let mut command = Command::new("python3")
            .property("-m", "pipenv")
            .arg("install")
            .current_dir(project_directory);

        if GitHub::is_running_in_ci() {
            command = command.flag("--deploy");
        }

        command.run()?;
    }

    return Ok(());
}

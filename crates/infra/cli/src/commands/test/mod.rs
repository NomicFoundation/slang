use std::iter::empty;

use anyhow::Result;
use clap::{Parser, Subcommand};
use infra_utils::cargo::{CargoWorkspace, CargoWorkspaceCommands};
use infra_utils::commands::Command;
use infra_utils::terminal::Terminal;

#[derive(Clone, Debug, Default, Parser)]
pub struct TestController {
    #[clap(subcommand)]
    command: Option<TestCommand>,
}

impl TestController {
    pub fn execute(&self) -> Result<()> {
        match &self.command {
            Some(TestCommand::Cargo { passthrough }) => test_cargo(passthrough)?,
            Some(TestCommand::Npm { passthrough }) => test_npm(passthrough),
            None => {
                test_cargo(empty::<String>())?;
                test_npm(empty::<String>());
            }
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Subcommand)]
enum TestCommand {
    /// Run 'cargo test' for all crates, features, and targets.
    Cargo {
        #[arg(
            trailing_var_arg = true,
            help = "Passthrough arguments to cargo nextest."
        )]
        passthrough: Vec<String>,
    },
    /// Run 'test' scripts in each NPM package in the repository.
    Npm {
        #[arg(trailing_var_arg = true, help = "Passthrough arguments to jest.")]
        passthrough: Vec<String>,
    },
}

fn test_cargo(passthrough: impl IntoIterator<Item = impl Into<String>>) -> Result<()> {
    Terminal::step("test Cargo");

    CargoWorkspace::install_binary("cargo-nextest")?;

    Command::new("cargo")
        .args(["nextest", "run"])
        .flag("--workspace")
        .flag("--all-features")
        .flag("--tests")
        .flag("--lib")
        .flag("--bins")
        .flag("--examples")
        .flag("--no-fail-fast")
        .add_build_rustflags()
        .args(passthrough)
        .run();

    Ok(())
}

fn test_npm(passthrough: impl IntoIterator<Item = impl Into<String>>) {
    Terminal::step("test Npm");

    Command::new("jest")
        .env("NODE_OPTIONS", "--experimental-vm-modules") // because we are executing ESM tests
        .env("NODE_NO_WARNINGS", "1") // disable warnings about experimental feature above (too much noise)
        .args(passthrough)
        .run();
}

use anyhow::Result;
use clap::{Parser, ValueEnum};
use infra_utils::cargo::{CargoWorkspace, CargoWorkspaceCommands};
use infra_utils::commands::Command;
use infra_utils::terminal::Terminal;

use crate::utils::{ClapExtensions, OrderedCommand};

#[derive(Clone, Debug, Default, Parser)]
pub struct TestController {
    #[clap(trailing_var_arg = true)]
    commands: Vec<TestCommand>,
}

impl TestController {
    pub fn execute(&self) -> Result<()> {
        TestCommand::execute_in_order(&self.commands)
    }
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd, ValueEnum)]
enum TestCommand {
    /// Run 'cargo test' for all crates, features, and targets.
    Cargo,
    /// Run 'test' scripts in each NPM package in the repository.
    Npm,
}

impl OrderedCommand for TestCommand {
    fn execute(&self) -> Result<()> {
        Terminal::step(format!("test {name}", name = self.clap_name()));

        match self {
            TestCommand::Cargo => test_cargo(),
            TestCommand::Npm => test_npm(),
        }
    }
}

fn test_cargo() -> Result<()> {
    CargoWorkspace::install_binary("cargo-nextest")?;

    Command::new("cargo")
        .args(["nextest", "run"])
        .flag("--workspace")
        .flag("--all-features")
        .flag("--lib")
        .flag("--bins")
        .flag("--examples")
        .flag("--no-fail-fast")
        .add_build_rustflags()
        .run()
}

fn test_npm() -> Result<()> {
    Command::new("jest").run()
}

use anyhow::Result;
use clap::{Parser, ValueEnum};
use infra_utils::{commands::Command, github::GitHub};

use crate::{
    toolchains::{
        mkdocs::Mkdocs,
        napi::{NapiCompiler, NapiProfile},
    },
    utils::{ClapExtensions, OrderedCommand, Terminal},
};

#[derive(Clone, Debug, Parser)]
pub struct CheckController {
    #[clap(trailing_var_arg = true)]
    commands: Vec<CheckCommand>,
}

impl CheckController {
    pub fn execute(&self) -> Result<()> {
        return CheckCommand::execute_all(&self.commands);
    }
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd, ValueEnum)]
pub enum CheckCommand {
    /// Run 'cargo check' for all crates, features, and targets.
    Cargo,
    /// Check NPM packages for any outdated codegen steps.
    Npm,
    /// Check mkdocs documentation for any build issues or broken links.
    Mkdocs,
}

impl OrderedCommand for CheckCommand {
    fn execute(&self) -> Result<()> {
        Terminal::step(self.clap_name());

        return match self {
            CheckCommand::Cargo => check_cargo(),
            CheckCommand::Npm => check_npm(),
            CheckCommand::Mkdocs => check_mkdocs(),
        };
    }
}

fn check_cargo() -> Result<()> {
    let mut command = Command::new("cargo")
        .arg("check")
        .flag("--offline")
        .flag("--all")
        .flag("--all-targets")
        .flag("--all-features");

    if GitHub::is_running_in_ci() {
        command = command.property(
            "--config",
            format!(
                "build.rustflags = {rustflags}",
                rustflags = serde_json::to_string(&["--deny", "warnings"])?
            ),
        );
    }

    return command.run();
}

fn check_npm() -> Result<()> {
    return NapiCompiler::run(NapiProfile::Debug);
}

fn check_mkdocs() -> Result<()> {
    return Mkdocs::build();
}

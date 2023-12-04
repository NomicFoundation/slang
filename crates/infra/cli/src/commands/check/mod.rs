use anyhow::Result;
use clap::{Parser, ValueEnum};
use infra_utils::cargo::CargoWorkspace;

use crate::{
    toolchains::{
        mkdocs::Mkdocs,
        napi::{NapiCompiler, NapiProfile},
    },
    utils::{ClapExtensions, OrderedCommand, Terminal},
};

#[derive(Clone, Debug, Default, Parser)]
pub struct CheckController {
    #[clap(trailing_var_arg = true)]
    commands: Vec<CheckCommand>,
}

impl CheckController {
    pub fn execute(&self) -> Result<()> {
        CheckCommand::execute_in_order(&self.commands)
    }
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd, ValueEnum)]
enum CheckCommand {
    /// Run 'cargo check' for all crates, features, and targets.
    Cargo,
    /// Check NPM packages for any outdated codegen steps.
    Npm,
    /// Check mkdocs documentation for any build issues or broken links.
    Mkdocs,
}

impl OrderedCommand for CheckCommand {
    fn execute(&self) -> Result<()> {
        Terminal::step(format!("check {name}", name = self.clap_name()));

        match self {
            CheckCommand::Cargo => check_cargo(),
            CheckCommand::Npm => check_npm(),
            CheckCommand::Mkdocs => check_mkdocs(),
        }
    }
}

fn check_cargo() -> Result<()> {
    CargoWorkspace::get_command("check")?.run()
}

fn check_npm() -> Result<()> {
    NapiCompiler::run(NapiProfile::Debug)
}

fn check_mkdocs() -> Result<()> {
    Mkdocs::build()
}

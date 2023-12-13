use anyhow::Result;
use clap::{Parser, ValueEnum};
use infra_utils::cargo::CargoWorkspace;

use crate::toolchains::mkdocs::Mkdocs;
use crate::toolchains::napi::{NapiCompiler, NapiProfile};
use crate::utils::{ClapExtensions, OrderedCommand, Terminal};

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
    /// Run `cargo doc` to generate Rustdoc documentation and check for any broken links.
    Rustdoc,
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
            CheckCommand::Rustdoc => check_rustdoc(),
            CheckCommand::Npm => check_npm(),
            CheckCommand::Mkdocs => check_mkdocs(),
        }
    }
}

fn check_cargo() -> Result<()> {
    // 'cargo clippy' will run both 'cargo check', and 'clippy' lints:
    CargoWorkspace::get_command("clippy")?
        .flag("--all-targets")
        .run()
}

fn check_rustdoc() -> Result<()> {
    CargoWorkspace::get_command("doc")?
        .flag("--no-deps")
        .flag("--document-private-items")
        .flag("--lib")
        .flag("--bins")
        .flag("--examples")
        .run()
}

fn check_npm() -> Result<()> {
    NapiCompiler::run(NapiProfile::Debug)
}

fn check_mkdocs() -> Result<()> {
    Mkdocs::build()
}

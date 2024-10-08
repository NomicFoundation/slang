use anyhow::Result;
use clap::{Parser, ValueEnum};
use infra_utils::cargo::CargoWorkspaceCommands;
use infra_utils::commands::Command;
use infra_utils::terminal::Terminal;
use strum::IntoEnumIterator;

use crate::toolchains::mkdocs::Mkdocs;
use crate::toolchains::wasm::WasmPackage;
use crate::utils::{ClapExtensions, OrderedCommand};

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
            CheckCommand::Npm => check_npm()?,
            CheckCommand::Mkdocs => check_mkdocs(),
        };

        Ok(())
    }
}

fn check_cargo() {
    // 'cargo clippy' will run both 'cargo check', and 'clippy' lints:
    Command::new("cargo")
        .arg("clippy")
        .flag("--workspace")
        .flag("--all-features")
        .flag("--all-targets")
        .flag("--no-deps")
        .add_build_rustflags()
        .run();
}

fn check_rustdoc() {
    Command::new("cargo")
        .arg("doc")
        .flag("--workspace")
        .flag("--all-features")
        .flag("--no-deps")
        .flag("--document-private-items")
        .add_build_rustflags()
        .run();
}

fn check_npm() -> Result<()> {
    for package in WasmPackage::iter() {
        package.build()?;
    }

    Ok(())
}

fn check_mkdocs() {
    Mkdocs::check();
}

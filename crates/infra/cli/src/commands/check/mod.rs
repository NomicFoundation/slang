use anyhow::Result;
use clap::{Parser, ValueEnum};
use infra_utils::commands::Command;
use infra_utils::terminal::Terminal;
use rayon::iter::{ParallelBridge, ParallelIterator};
use strum::IntoEnumIterator;

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
    /// Check NPM packages for any outdated codegen steps.
    Npm,
    /// Use rustdoc to generate public api documentation.
    PublicApi,
}

impl OrderedCommand for CheckCommand {
    fn execute(&self) -> Result<()> {
        Terminal::step(format!("check {name}", name = self.clap_name()));

        match self {
            CheckCommand::Cargo => check_cargo(),
            CheckCommand::Npm => check_npm(),
            CheckCommand::PublicApi => check_public_api(),
        };

        Ok(())
    }
}

fn check_cargo() {
    Command::cargo("check")
        .flag("--workspace")
        .flag("--all-features")
        .flag("--all-targets")
        .run();
}

fn check_npm() {
    WasmPackage::iter()
        .par_bridge()
        .for_each(|package| package.build().unwrap());
}

fn check_public_api() {
    crate::toolchains::public_api_snapshots();
}

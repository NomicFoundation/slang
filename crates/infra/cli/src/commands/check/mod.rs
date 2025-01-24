use anyhow::Result;
use clap::{Parser, ValueEnum};
use infra_utils::cargo::CargoWorkspaceCommands;
use infra_utils::commands::Command;
use infra_utils::paths::PathExtensions;
use infra_utils::terminal::Terminal;
use rayon::iter::{ParallelBridge, ParallelIterator};
use strum::IntoEnumIterator;

use crate::toolchains::wasm::WasmPackage;
use crate::utils::{ClapExtensions, OrderedCommand};

use std::path::Path;

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
    Ldw,
}

impl OrderedCommand for CheckCommand {
    fn execute(&self) -> Result<()> {
        Terminal::step(format!("check {name}", name = self.clap_name()));

        match self {
            CheckCommand::Cargo => check_cargo(),
            CheckCommand::Npm => check_npm(),
            CheckCommand::Ldw => check_ldw(),
        };

        Ok(())
    }
}

fn check_cargo() {
    Command::new("cargo")
        .arg("check")
        .flag("--workspace")
        .flag("--all-features")
        .flag("--all-targets")
        .add_build_rustflags()
        .run();
}

fn check_npm() {
    WasmPackage::iter()
        .par_bridge()
        .for_each(|package| package.build().unwrap());
}

/// Process ldw models
fn check_ldw() {
    // TODO:
    // 1. Iterate over all languages
    // 2. We need a way to provide the fully qualified names (FQNs) that each grammar exposes
    //    - We could probably compute these from the paths themselves, since that's all the FQNs represent. However, finding the
    //      correct files to look at is probably the exact problem that the FQN is trying to solve, so that might not be so easy.
    // 3. Questions about input file structure: Right now we already have crates/<lang>/outputs, and since
    //    the models are going to be generated it probably makes sense to put them there, even though for this use-case they're
    //    technically inputs. `outputs/ldw/inputs`?
    // 4. Questions about output file structure: We'll need to make sure that the generated passes are put somewhere where they
    //    can be used by the next stage in the build pipeline, but for now we'll just dump them somewhere convenient.
    Command::new("ts-node")
        .current_dir(Path::repo_path("crates/codegen/ldw"))
        .args(["-P", "./tsconfig.json"])
        .arg("src-ts/cli/ldw.ts")
        // .args(["--in-dir", "../../crates/testlang/outputs/ldw/"])
        // .args(["--out-dir", "../../crates/testlang/outputs/ldw/generated"])
        .flag("--help")
        .run();
}

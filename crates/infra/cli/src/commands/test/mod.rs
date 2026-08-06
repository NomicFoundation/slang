use std::iter::empty;

use clap::{Parser, Subcommand};
use infra_utils::cargo::CargoWorkspaceCommands;
use infra_utils::commands::Command;
use infra_utils::terminal::Terminal;

const SOLC_COMPARISON_CRATE: &str = "solidity_testing_solc_comparison";

#[derive(Clone, Debug, Default, Parser)]
pub struct TestController {
    #[clap(subcommand)]
    command: Option<TestCommand>,
}

impl TestController {
    pub fn execute(&self) {
        match &self.command {
            Some(TestCommand::Cargo { passthrough }) => test_cargo(passthrough),
            Some(TestCommand::Npm { passthrough }) => test_npm(passthrough),
            Some(TestCommand::SolcSemantic { passthrough }) => test_solc_semantic(passthrough),
            None => {
                test_cargo(empty::<String>());
                test_solc_semantic(empty::<String>());
                test_npm(empty::<String>());
            }
        }
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
    /// Run slang against solc's 'libsolidity' semantic test suite (every
    /// supported version), checking that all of this (valid) Solidity still
    /// compiles without slang emitting errors.
    ///
    /// Runs as its own step since it fetches an external dataset.
    SolcSemantic {
        #[arg(
            trailing_var_arg = true,
            help = "Passthrough arguments to cargo nextest."
        )]
        passthrough: Vec<String>,
    },
}

/// The flags shared by every `nextest` run below, so that the two steps build
/// with the same feature resolution and target selection.
fn cargo_nextest() -> Command {
    Command::new("cargo")
        .args(["nextest", "run"])
        .flag("--all-features")
        .flag("--tests")
        .flag("--lib")
        .flag("--bins")
        .flag("--examples")
        .flag("--no-fail-fast")
        .add_build_rustflags()
}

fn test_cargo(passthrough: impl IntoIterator<Item = impl Into<String>>) {
    Terminal::step("test Cargo");

    cargo_nextest().flag("--workspace").args(passthrough).run();
}

fn test_solc_semantic(passthrough: impl IntoIterator<Item = impl Into<String>>) {
    Terminal::step("test solc-semantic");

    cargo_nextest()
        .property("--package", SOLC_COMPARISON_CRATE)
        // The suite itself is '#[ignore]'d, so that it never runs off a plain
        // 'cargo test'/'cargo nextest run' and reaches for the network. This is
        // the explicit opt-in.
        .property("--run-ignored", "all")
        .args(passthrough)
        .run();
}

fn test_npm(passthrough: impl IntoIterator<Item = impl Into<String>>) {
    Terminal::step("test Npm");

    Command::new("jest")
        .env("NODE_OPTIONS", "--experimental-vm-modules") // because we are executing ESM tests
        .env("NODE_NO_WARNINGS", "1") // disable warnings about experimental feature above (too much noise)
        .args(passthrough)
        .run();
}

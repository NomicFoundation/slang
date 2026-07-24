use anyhow::Result;
use clap::{Parser, Subcommand};
use infra_utils::cargo::CargoWorkspaceCommands;
use infra_utils::commands::Command;
use infra_utils::terminal::Terminal;

const SOLC_COMPARISON_CRATE: &str = "solidity_testing_solc_comparison";

#[derive(Clone, Debug, Default, Parser)]
pub struct VerifyController {
    #[clap(subcommand)]
    command: Option<VerifyCommand>,
}

impl VerifyController {
    // Returns `Result` for symmetry with the other command controllers, which
    // are all invoked with `?`.
    #[allow(clippy::unnecessary_wraps)]
    pub fn execute(&self) -> Result<()> {
        match &self.command {
            Some(VerifyCommand::SolcSemanticSuite { passthrough }) => {
                verify_solc_semantic_suite(passthrough);
            }
            None => verify_solc_semantic_suite(std::iter::empty::<String>()),
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Subcommand)]
enum VerifyCommand {
    /// Run slang against solc's 'libsolidity' semantic test suite (every
    /// supported version), checking that all of this (valid) Solidity still
    /// compiles without slang emitting errors.
    ///
    /// Downloads an external dataset (solc's own semantic tests) and guards
    /// against new validations in slang accidentally rejecting valid code.
    SolcSemanticSuite {
        #[arg(
            trailing_var_arg = true,
            help = "Passthrough arguments forwarded to `cargo test`."
        )]
        passthrough: Vec<String>,
    },
}

fn verify_solc_semantic_suite(passthrough: impl IntoIterator<Item = impl Into<String>>) {
    Terminal::step("verify solc-semantic-suite");

    // The suite is the `datatest-stable` harness (one case per (version, test)),
    // run via `cargo test` — i.e. in-process with threads. We deliberately don't
    // use nextest here: nextest is process-per-test by design (see
    // <https://nexte.st/docs/design/why-process-per-test/>), and spawning ~50k
    // processes is both slow and overwhelms nextest's list phase. In-process
    // execution runs the whole matrix in seconds.
    Command::new("cargo")
        .arg("test")
        .property("--package", SOLC_COMPARISON_CRATE)
        .add_build_rustflags()
        .arg("--")
        .flag("--quiet")
        .args(passthrough)
        .run();
}

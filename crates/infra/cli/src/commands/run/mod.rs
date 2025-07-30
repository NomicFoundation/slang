use clap::{Parser, ValueEnum};
use infra_utils::commands::Command;
use infra_utils::github::GitHub;
use infra_utils::terminal::Terminal;

use crate::utils::ClapExtensions;

#[derive(Clone, Debug, Parser)]
pub struct RunController {
    /// Name of the binary to run.
    #[arg(long)]
    bin: BinaryName,

    /// Run the release build of the binary (with optimizations turned on).
    #[arg(long)]
    release: bool,

    #[clap(trailing_var_arg = true)]
    args: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, ValueEnum)]
enum BinaryName {
    /// Runs the public `slang_solidity` crate shipped to Cargo users.
    #[clap(name = "slang_solidity")]
    SlangSolidity,
    /// Runs compatibility tests between our language definition and 'solc' actual output.
    #[clap(name = "solidity_testing_solc")]
    SolidityTestingSolc,
    /// Tests our parser/binding graph against contracts fetched from the Sourcify dataset.
    #[clap(name = "solidity_testing_sourcify")]
    SolidityTestingSourcify,
}

impl RunController {
    pub fn execute(&self) {
        let bin = self.bin.clap_name();

        Terminal::step(format!("run {bin}"));

        let mut command = Command::new("cargo").arg("run");

        if self.release || GitHub::is_running_in_ci() {
            command = command.flag("--release");
        }

        command
            .property("--bin", bin)
            .arg("--")
            .args(&self.args)
            .run();
    }
}

use anyhow::Result;
use clap::{Parser, ValueEnum};
use infra_utils::commands::Command;

use crate::utils::{ClapExtensions, Terminal};

#[derive(Clone, Debug, Parser)]
pub struct RunController {
    command: RunCommand,

    #[clap(trailing_var_arg = true)]
    args: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, ValueEnum)]
enum RunCommand {
    /// Runs the public 'slang_solidity' crate shipped to Cargo users.
    #[clap(name = "slang_solidity")]
    SlangSolidity,
    /// Runs the Solidity parser against source files from the Sanctuary repositories.
    #[clap(name = "solidity_testing_sanctuary")]
    SolidityTestingSanctuary,
    /// Runs compatability tests between our language definition and 'solc' actual output.
    #[clap(name = "solidity_testing_solc")]
    SolidityTestingSolc,
}

impl RunCommand {
    fn should_run_in_release_mode(&self) -> bool {
        match self {
            Self::SolidityTestingSanctuary => {
                // This crate parses tens of thousands of Solidity files:
                // It is worth spending the extra time to recompiling its dependencies.
                true
            }
            Self::SlangSolidity | Self::SolidityTestingSolc => {
                // These run during local development. Just build in debug mode.
                false
            }
        }
    }
}

impl RunController {
    pub fn execute(&self) -> Result<()> {
        let crate_name = self.command.clap_name();

        Terminal::step(format!("run {crate_name}"));

        let mut command = Command::new("cargo").arg("run");

        if self.command.should_run_in_release_mode() {
            command = command.flag("--release");
        }

        command
            .property("--bin", &crate_name)
            .arg("--")
            .args(&self.args)
            .run()
    }
}

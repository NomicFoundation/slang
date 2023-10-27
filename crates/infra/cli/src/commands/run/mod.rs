use anyhow::Result;
use clap::{Parser, ValueEnum};
use infra_utils::{cargo::CargoWorkspace, commands::Command};

use crate::utils::{ClapExtensions, Terminal};

#[derive(Clone, Debug, Parser)]
pub struct RunController {
    command: RunCommand,

    #[clap(trailing_var_arg = true)]
    args: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, ValueEnum)]
enum RunCommand {
    /*
     *
     * User-facing:
     *
     */
    /// Runs the public 'slang_solidity' crate shipped to Cargo users.
    #[clap(name = "slang_solidity")]
    SlangSolidity,
    /// Runs the Solidity parser against source files from the Sanctuary repositories.
    #[clap(name = "solidity_testing_sanctuary")]
    SolidityTestingSanctuary,
    /// Runs compatability tests between our language definition and 'solc' actual output.
    #[clap(name = "solidity_testing_solc")]
    SolidityTestingSolc,

    /*
     *
     * Hidden (for automation only):
     *
     */
    /// Runs codegen for the Rust parser crate.
    #[clap(name = "solidity_cargo_build", hide = true)]
    SolidityCargoBuild,
    /// Runs codegen for the NAPI-exposed parser crate.
    #[clap(name = "solidity_npm_build", hide = true)]
    SolidityNpmBuild,
}

impl RunCommand {
    fn should_run_in_release_mode(&self) -> bool {
        match self {
            Self::SolidityTestingSanctuary => {
                // This crate parses tens of thousands of Solidity files:
                // It is worth spending the extra time to recompiling its dependencies.
                return true;
            }
            Self::SlangSolidity
            | Self::SolidityCargoBuild
            | Self::SolidityNpmBuild
            | Self::SolidityTestingSolc => {
                // These run during local development. Just build in debug mode.
                return false;
            }
        };
    }
}

impl RunController {
    pub fn execute(&self) -> Result<()> {
        let crate_name = self.command.clap_name();
        let crate_dir = CargoWorkspace::locate_source_crate(&crate_name)?;

        Terminal::step(format!("run {crate_name}"));

        let mut command = Command::new("cargo").arg("run");

        if self.command.should_run_in_release_mode() {
            command = command.flag("--release");
        }

        command
            .property("--bin", &crate_name)
            .flag("--offline")
            .arg("--")
            .args(&self.args)
            // Execute in the crate dir, to make use of a local './target' dir if it exists:
            .current_dir(crate_dir)
            .run()
    }
}

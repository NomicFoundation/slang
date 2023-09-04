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

#[derive(Clone, Debug, ValueEnum)]
enum RunCommand {
    /*
     *
     * User-facing:
     *
     */
    /// Run the public 'slang_solidity' crate shapped to Cargo users.
    #[clap(name = "slang_solidity")]
    SlangSolidity,

    /// Run smoke tests of the Solidity parser against source files from the Sanctuary repositories.
    #[clap(name = "solidity_testing_smoke")]
    SolidityTestingSmoke,

    /*
     *
     * Hidden (for automation only):
     *
     */
    #[clap(name = "solidity_cargo_build", hide = true)]
    SolidityCargoBuild,

    #[clap(name = "solidity_npm_build", hide = true)]
    SolidityNpmBuild,
}

impl RunController {
    pub fn execute(&self) -> Result<()> {
        let crate_name = self.command.clap_name();
        let crate_dir = CargoWorkspace::locate_source_crate(&crate_name)?;

        Terminal::step(format!("run {crate_name}"));

        return Command::new("cargo")
            .arg("run")
            .arg("--release")
            .property("--bin", &crate_name)
            .arg("--")
            .args(&self.args)
            // Execute in the crate dir, to make use of a local './target' dir if it exists:
            .current_dir(crate_dir)
            .run();
    }
}

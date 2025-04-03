use anyhow::Result;
use clap::{Parser, ValueEnum};
use infra_utils::commands::Command;

#[derive(Clone, Debug, ValueEnum, PartialEq, Eq, PartialOrd, Ord)]
pub enum Cases {
    VsSolc,
    All,
}

#[derive(Clone, Debug, Parser)]
pub struct NpmController {
    #[arg(value_enum, default_value_t = Cases::All)]
    cases: Cases,

    #[arg(trailing_var_arg = true)]
    extra_args: Vec<String>, // Collects all arguments after `--`
}

impl NpmController {
    fn install_deps() {
        let command = Command::new("npm").arg("install").arg("tsx");
        command.run();
    }

    #[allow(clippy::unnecessary_wraps)]
    pub fn execute(&self) -> Result<()> {
        Self::install_deps();

        if self.cases == Cases::VsSolc || self.cases == Cases::All {
            let command = Command::new("npx")
                .arg("tsx")
                .flag("--trace-uncaught")
                .arg("crates/solidity/testing/perf/npm/src/slang.vs.solc.mts")
                .args(&self.extra_args);

            command.run();
        }
        Ok(())
    }
}

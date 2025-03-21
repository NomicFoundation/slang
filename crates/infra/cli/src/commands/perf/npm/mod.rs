use anyhow::Result;
use clap::Parser;
use infra_utils::commands::Command;

#[derive(Clone, Debug, Parser)]
pub struct NpmController {}

impl NpmController {
    fn install_deps() {
        let command = Command::new("npm").arg("install").arg("tsx");
        command.run();
    }

    pub fn execute(&self) -> Result<()> {
        Self::install_deps();

        let command = Command::new("npx")
            .arg("tsx")
            .arg("crates/solidity/testing/perf/npm/src/benchmark.mts");

        command.run();
        Ok(())
    }
}

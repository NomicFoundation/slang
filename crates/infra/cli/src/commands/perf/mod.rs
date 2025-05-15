mod bencher;
mod cargo;
mod fetch;
mod npm;

use anyhow::Result;
use clap::{Parser, Subcommand};

use crate::commands::perf::cargo::CargoController;
use crate::commands::perf::fetch::FetchController;
use crate::commands::perf::npm::NpmController;

#[derive(Clone, Debug, Parser)]
pub struct PerfController {
    #[command(subcommand)]
    command: PerfCommand,
}

#[derive(Clone, Debug, Subcommand)]
enum PerfCommand {
    /// Run benchmark tests for the rust api, and report the results to <https://bencher.dev/console>
    Cargo(CargoController),
    /// Run benchmark tests for the typescript api
    Npm(NpmController),
    /// Fetch the sources of contracts
    Fetch(FetchController),
}

impl PerfController {
    pub fn execute(&self) -> Result<()> {
        match &self.command {
            PerfCommand::Cargo(controller) => controller.execute(),
            PerfCommand::Npm(controller) => controller.execute(),
            PerfCommand::Fetch(controller) => controller.execute(),
        }
    }
}

mod archive;
mod cargo;
mod npm;

use anyhow::Result;
use clap::{Parser, Subcommand};

use crate::commands::perf::archive::{ArchiveController, UnarchiveController};
use crate::commands::perf::cargo::CargoController;
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
    /// Run benchmark tests for the typescript api, and report the results to <https://bencher.dev/console>
    Npm(NpmController),
    /// Archive bencher PR branches (cleanup after PR close)
    Archive(ArchiveController),
    /// Restore archived bencher PR branches
    Unarchive(UnarchiveController),
}

impl PerfController {
    pub fn execute(&self) -> Result<()> {
        match &self.command {
            PerfCommand::Cargo(controller) => controller.execute(),
            PerfCommand::Npm(controller) => controller.execute(),
            PerfCommand::Archive(controller) => controller.execute(),
            PerfCommand::Unarchive(controller) => controller.execute(),
        }
    }
}

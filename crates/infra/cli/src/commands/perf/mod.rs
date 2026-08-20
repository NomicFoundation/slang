mod archive;
mod binaries;
mod cargo;
mod cargo_wall_clock;
mod npm;

use anyhow::Result;
use clap::{Parser, Subcommand};

use crate::commands::perf::archive::{ArchiveController, UnarchiveController};
use crate::commands::perf::cargo::CargoController;
use crate::commands::perf::cargo_wall_clock::CargoWallClockController;
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
    /// Run wall-clock benchmarks for the slang v2 pipeline, for local comparisons
    ///
    /// Also a cargo benchmark suite, but kept separate from 'perf cargo'
    /// because, for now, it is only intended for development use: it measures
    /// wall time rather than Valgrind's deterministic counters, and reports
    /// nothing to the Bencher dashboard. Fold it into 'perf cargo' if it ever
    /// gains a CI-reported role.
    CargoWallClock(CargoWallClockController),
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
            PerfCommand::CargoWallClock(controller) => controller.execute(),
            PerfCommand::Archive(controller) => controller.execute(),
            PerfCommand::Unarchive(controller) => controller.execute(),
        }
    }
}

mod benchmark;

use anyhow::Result;
use clap::{Parser, Subcommand};

use crate::commands::perf::benchmark::BenchmarkController;

#[derive(Clone, Debug, Parser)]
pub struct PerfController {
    #[command(subcommand)]
    command: PerfCommand,
}

#[derive(Clone, Debug, Subcommand)]
enum PerfCommand {
    /// Run benchmark tests, and report the results to <https://bencher.dev/console>
    Benchmark(BenchmarkController),
}

impl PerfController {
    pub fn execute(&self) -> Result<()> {
        match &self.command {
            PerfCommand::Benchmark(controller) => controller.execute(),
        }
    }
}

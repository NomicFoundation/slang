use anyhow::Result;
use clap::Parser;
use infra_utils::cargo::CargoWorkspace;
use infra_utils::commands::Command;

use crate::toolchains::bencher::run_bench;
use crate::utils::DryRun;

const DEFAULT_BENCHER_PROJECT: &str = "slang-dashboard-npm";

#[derive(Clone, Debug, Parser)]
pub struct NpmController {
    #[arg(short, long, default_value_t = String::from(".*"))]
    /// A regex pattern to select which project(s) to run
    pattern: String,

    #[command(flatten)]
    dry_run: DryRun,
    /// Run as a PR benchmark with regression detection via bencher start points.
    #[arg(long)]
    pr_benchmark: bool,

    #[arg(long, default_value_t = 2)]
    /// The number of cold runs
    cold: usize,

    #[arg(long, default_value_t = 5)]
    /// The number of hot runs
    hot: usize,

    /// Install deps and build the perf binary, but skip running benchmarks.
    #[arg(long, conflicts_with = "pr_benchmark")]
    smoke: bool,
}

impl NpmController {
    fn install_deps() -> Result<()> {
        CargoWorkspace::install_binary("bencher_cli")?;
        Ok(())
    }

    fn execute_npm_benchmarks(&self) {
        let test_runner = format!(
            "cargo run --package {package} -- --pattern=\"{pattern}\" --cold={cold} --hot={hot}",
            package = "solidity_testing_perf_npm",
            pattern = &self.pattern,
            cold = &self.cold,
            hot = &self.hot,
        );

        run_bench(
            self.dry_run.get(),
            self.pr_benchmark,
            DEFAULT_BENCHER_PROJECT,
            "json",
            &test_runner,
        );
    }

    #[allow(clippy::unnecessary_wraps)]
    pub fn execute(&self) -> Result<()> {
        Self::install_deps()?;

        if self.smoke {
            Command::new("cargo")
                .args(["build", "--package", "solidity_testing_perf_npm"])
                .run();
            return Ok(());
        }

        self.execute_npm_benchmarks();
        Ok(())
    }
}

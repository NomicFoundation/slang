use anyhow::Result;
use clap::Parser;
use infra_utils::cargo::CargoWorkspace;

use crate::toolchains::bencher::run_bench;
use crate::utils::DryRun;

#[derive(Clone, Debug, Parser)]
pub struct NpmController {
    #[arg(short, long, default_value_t = String::from(".*"))]
    /// A regex pattern to select which project(s) to run
    pattern: String,

    #[command(flatten)]
    dry_run: DryRun,

    #[arg(long, default_value_t = 2)]
    /// The number of cold runs
    cold: usize,

    #[arg(long, default_value_t = 5)]
    /// The number of hot runs
    hot: usize,
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

        run_bench(self.dry_run.get(), "json", &test_runner);
    }

    #[allow(clippy::unnecessary_wraps)]
    pub fn execute(&self) -> Result<()> {
        Self::install_deps()?;

        self.execute_npm_benchmarks();
        Ok(())
    }
}

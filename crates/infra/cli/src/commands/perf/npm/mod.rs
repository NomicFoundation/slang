use anyhow::Result;
use clap::Parser;
use infra_utils::cargo::CargoWorkspace;

use super::bencher::run_bench;
use crate::utils::DryRun;

#[derive(Clone, Debug, Parser)]
pub struct NpmController {
    #[arg(short, long, default_value_t = String::from(".*"))]
    pattern: String,

    #[command(flatten)]
    dry_run: DryRun,

    #[arg(trailing_var_arg = true)]
    extra_args: Vec<String>, // Collects all arguments after `--`
}

impl NpmController {
    fn install_deps() -> Result<()> {
        CargoWorkspace::install_binary("bencher_cli")?;
        Ok(())
    }

    fn execute_npm_benchmarks(&self) {
        let extra_args = if self.extra_args.is_empty() {
            String::new()
        } else {
            "-- ".to_owned() + &self.extra_args.join(" ")
        };
        let test_runner =
            format!("cargo run --package solidity_testing_perf_npmbenches -- --pattern={pattern} {extra_args}", pattern = &self.pattern);

        run_bench(self.dry_run.get(), "json", &test_runner);
    }

    #[allow(clippy::unnecessary_wraps)]
    pub fn execute(&self) -> Result<()> {
        Self::install_deps()?;

        self.execute_npm_benchmarks();
        Ok(())
    }
}

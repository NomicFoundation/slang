use std::path::Path;

use anyhow::{bail, Result};
use clap::Parser;
use infra_utils::cargo::CargoWorkspace;
use infra_utils::commands::Command;
use infra_utils::github::GitHub;
use infra_utils::paths::PathExtensions;

use crate::utils::DryRun;

// Source: https://github.com/bencherdev/bencher/blob/aa31a002842cfb0da9d6c60569396fc5261f5111/tasks/test_api/src/task/test/smoke_test.rs#L20
const BENCHER_TEST_TOKEN: &str = "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJhdWQiOiJhcGlfa2V5IiwiZXhwIjo1OTkzNjQyMTU2LCJpYXQiOjE2OTg2NzQ4NjEsImlzcyI6Imh0dHBzOi8vZGV2ZWwtLWJlbmNoZXIubmV0bGlmeS5hcHAvIiwic3ViIjoibXVyaWVsLmJhZ2dlQG5vd2hlcmUuY29tIiwib3JnIjpudWxsfQ.9z7jmM53TcVzc1inDxTeX9_OR0PQPpZAsKsCE7lWHfo";

#[derive(Clone, Debug, Parser)]
pub struct BenchmarkController {
    #[command(flatten)]
    dry_run: DryRun,
}

impl BenchmarkController {
    pub fn execute(&self) -> Result<()> {
        Self::install_perf_tools()?;

        // Bencher supports multiple languages/frameworks: https://bencher.dev/docs/explanation/adapters/
        // We currently only have one benchmark suite (Rust/iai), but we can add more here in the future.
        self.run_iai_bench("solidity_testing_perf", "iai");

        Ok(())
    }

    fn install_perf_tools() -> Result<()> {
        match Command::new("valgrind").flag("--version").evaluate() {
            Ok(output) if output.starts_with("valgrind-") => {
                // Valgrind is available
            }
            other => {
                bail!(
                    "valgrind needs to be installed to run perf tests.
            It is installed by default inside our devcontainer.
            Supported Platforms: https://valgrind.org/downloads/current.html
            {other:?}"
                );
            }
        };

        CargoWorkspace::install_binary("iai-callgrind-runner")?;

        CargoWorkspace::install_binary("bencher_cli")?;

        Ok(())
    }

    fn run_iai_bench(&self, package_name: &str, bench_name: &str) {
        let token = if self.dry_run.get() {
            // Use a dummy test token for dry runs:
            // https://github.com/bencherdev/bencher/issues/468
            BENCHER_TEST_TOKEN.to_string()
        } else {
            std::env::var("BENCHER_API_TOKEN").expect(
              "BENCHER_API_TOKEN is not set. Either perform a '--dry-run', or set it to your Bencher API token: https://bencher.dev/console"
            )
        };

        let cargo_command = format!("cargo bench --package {package_name} --bench {bench_name}");

        let testbed = if GitHub::is_running_in_ci() {
            "ci"
        } else {
            "dev"
        };

        Command::new("bencher")
            .arg("run")
            .property("--project", "slang")
            .property("--adapter", "rust_iai_callgrind")
            .property("--testbed", testbed)
            .property("--token", token)
            .arg(cargo_command)
            .run();

        let reports_dir = Path::repo_path("target/iai")
            .join(package_name)
            .join(bench_name);

        println!("

Bencher Run is complete...
Test Results: [https://bencher.dev/console/projects/slang/reports]

Reports/Logs: {reports_dir:?}
- Callgrind flamegraphs (callgrind.*.svg) can be viewed directly in the browser.
- DHAT traces (dhat.*.out) can be viewed using the [dhat/dh_view.html] tool from the Valgrind release [https://valgrind.org/downloads/].

");
    }
}

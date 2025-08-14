use std::path::Path;

use anyhow::Result;
use clap::Parser;
use infra_utils::cargo::CargoWorkspace;
use infra_utils::commands::Command;
use infra_utils::github::GitHub;
use infra_utils::paths::PathExtensions;

use crate::toolchains::bencher::run_bench;
use crate::utils::DryRun;

const DEFAULT_BENCHER_PROJECT: &str = "slang-dashboard-cargo";

#[derive(Clone, Debug, Parser)]
pub struct CargoController {
    #[command(flatten)]
    dry_run: DryRun,
}

impl CargoController {
    pub fn execute(&self) -> Result<()> {
        Self::install_valgrind();

        CargoWorkspace::install_binary("iai-callgrind-runner")?;
        CargoWorkspace::install_binary("bencher_cli")?;

        // Bencher supports multiple languages/frameworks: https://bencher.dev/docs/explanation/adapters/
        // We currently only have one benchmark suite (Rust/iai), but we can add more here in the future.
        self.run_iai_bench("solidity_testing_perf", "iai");

        Ok(())
    }

    fn install_valgrind() {
        if GitHub::is_running_in_ci() {
            Command::new("sudo").args(["apt", "update"]).run();

            Command::new("sudo")
                .args(["apt", "install", "valgrind"])
                .flag("--yes")
                .run();

            return;
        }

        if GitHub::is_running_in_devcontainer() {
            Command::new("sudo").args(["apt-get", "update"]).run();

            Command::new("sudo")
                .args(["apt-get", "install", "valgrind"])
                .flag("--yes")
                .run();

            return;
        }

        match Command::new("valgrind").flag("--version").evaluate() {
            Ok(output) if output.starts_with("valgrind-") => {
                // Valgrind is available
            }
            other => {
                panic!(
                    "valgrind needs to be installed on this machine to run perf tests.
                    Supported Platforms: https://valgrind.org/downloads/current.html
                    {other:?}"
                );
            }
        }
    }

    fn run_iai_bench(&self, package_name: &str, bench_name: &str) {
        let test_runner = format!("cargo bench --package {package_name} --bench {bench_name}");

        run_bench(
            self.dry_run.get(),
            DEFAULT_BENCHER_PROJECT,
            "rust_iai_callgrind",
            &test_runner,
        );

        let reports_dir = Path::repo_path("target/iai")
            .join(package_name)
            .join(bench_name);

        println!("

Reports/Logs: {reports_dir:?}
- Callgrind flamegraphs (callgrind.*.svg) can be viewed directly in the browser.
- DHAT traces (dhat.*.out) can be viewed using the [dhat/dh_view.html] tool from the Valgrind release [https://valgrind.org/downloads/].

");
    }
}

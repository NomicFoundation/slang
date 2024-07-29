use std::path::Path;

use anyhow::{bail, Result};
use clap::{Parser, ValueEnum};
use infra_utils::cargo::CargoWorkspace;
use infra_utils::commands::Command;
use infra_utils::github::GitHub;
use infra_utils::paths::PathExtensions;
use infra_utils::terminal::Terminal;

use crate::utils::ClapExtensions;

#[derive(Clone, Debug, Parser)]
pub struct PerfController {
    command: PerfCommand,
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd, ValueEnum)]
enum PerfCommand {
    /// Run benchmark tests, and report the results to <https://bencher.dev/console>
    Benchmark,
}

impl PerfController {
    pub fn execute(&self) -> Result<()> {
        Terminal::step(format!("perf {name}", name = self.command.clap_name()));

        install_perf_tools()?;

        match self.command {
            PerfCommand::Benchmark => {
                // Bencher supports multiple languages/frameworks: https://bencher.dev/docs/explanation/adapters/
                // We currently only have one benchmark suite (Rust/iai), but we can add more here in the future.

                run_iai_bench("solidity_testing_perf", "iai")?;
            }
        };

        Ok(())
    }
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

fn run_iai_bench(package_name: &str, bench_name: &str) -> Result<()> {
    if std::env::var("BENCHER_API_TOKEN").is_err() {
        bail!("BENCHER_API_TOKEN is not set. Please set it to your Bencher API token: https://bencher.dev/console");
    }

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
        .flag("--html")
        .arg(cargo_command)
        .run()?;

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

    Ok(())
}

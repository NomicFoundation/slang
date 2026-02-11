use std::path::Path;

use anyhow::Result;
use clap::{Parser, Subcommand};
use infra_utils::cargo::CargoWorkspace;
use infra_utils::commands::Command;
use infra_utils::github::GitHub;
use infra_utils::paths::{FileWalker, PathExtensions};

use crate::toolchains::bencher::run_bench;
use crate::toolchains::pipenv::PipEnv;
use crate::utils::DryRun;

const DEFAULT_BENCHER_PROJECT_CMP: &str = "slang-dashboard-cargo-cmp";
const DEFAULT_BENCHER_PROJECT_SLANG: &str = "slang-dashboard-cargo-slang";

#[derive(Clone, Debug, Parser)]
pub struct CargoController {
    #[command(subcommand)]
    bench: Benches,
    #[command(flatten)]
    dry_run: DryRun,
    #[arg(long)]
    no_deps: bool,
}

#[derive(Clone, Debug, Subcommand)]
enum Benches {
    /// Performs the slang-specific benchmarks
    Slang,
    /// Performs a comparison with different crates for solidity parsing
    Comparison,
}

impl CargoController {
    pub fn execute(&self) -> Result<()> {
        if !self.no_deps {
            Self::install_valgrind();
            CargoWorkspace::install_binary("gungraun-runner")?;
            CargoWorkspace::install_binary("bencher_cli")?;

            Self::install_graphviz();
        }

        // Bencher supports multiple languages/frameworks: https://bencher.dev/docs/explanation/adapters/
        // We currently only have one benchmark suite (Rust/iai), but we can add more here in the future.
        match self.bench {
            Benches::Slang => self.run_iai_bench(
                "solidity_testing_perf_cargo",
                "slang",
                DEFAULT_BENCHER_PROJECT_SLANG,
            ),
            Benches::Comparison => self.run_iai_bench(
                "solidity_testing_perf_cargo",
                "comparison",
                DEFAULT_BENCHER_PROJECT_CMP,
            ),
        }
        Ok(())
    }

    fn install_valgrind() {
        Self::install_from_apt("valgrind", "1:3.18.1-1ubuntu2");

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

    fn install_graphviz() {
        Self::install_from_apt("graphviz", "2.42.2-6ubuntu0.1");

        // dot prints its version to stderr, using the help page instead
        match Command::new("dot").flag("-?").evaluate() {
            Ok(output) if output.starts_with("Usage: dot") => {
                // graphviz is available
            }
            other => {
                panic!(
                    "graphviz needs to be installed on this machine to generate callgraphs.
                    Supported Platforms: https://graphviz.org/download/
                    {other:?}"
                );
            }
        }
    }

    fn install_from_apt(package_name: &str, version: &str) {
        if GitHub::is_running_in_ci() {
            Command::new("sudo").args(["apt", "update"]).run();

            Command::new("sudo")
                .args(["apt", "install"])
                .arg(format!("{package_name}={version}"))
                .flag("--yes")
                .run();

            return;
        }

        if GitHub::is_running_in_devcontainer() {
            Command::new("sudo").args(["apt-get", "update"]).run();

            Command::new("sudo")
                .args(["apt-get", "install"])
                .arg(format!("{package_name}={version}"))
                .flag("--yes")
                .run();
        }
    }

    fn run_iai_bench(&self, package_name: &str, bench_name: &str, bencher_project: &str) {
        let test_runner = format!(
            "cargo bench --package {package_name} --bench {bench_name} --message-format json"
        );

        run_bench(
            self.dry_run.get(),
            bencher_project,
            "rust_gungraun",
            &test_runner,
        );

        let reports_dir = Path::repo_path("target/gungraun")
            .join(package_name)
            .join(bench_name);

        Self::generate_callgraph(reports_dir.clone());

        println!("

Reports/Logs: {reports_dir:?}
- Callgrind flamegraphs (callgrind.*.svg) can be viewed directly in the browser.
- DHAT traces (dhat.*.out) can be viewed using the [dhat/dh_view.html] tool from the Valgrind release [https://valgrind.org/downloads/].

");
    }

    fn generate_callgraph(reports_dir: std::path::PathBuf) {
        let callgrind_outputs =
            FileWalker::from_directory(reports_dir).find(["**/callgrind.*.out"]);

        for callgrind_output in callgrind_outputs.unwrap() {
            let callgrind_output_name = callgrind_output.unwrap_name();

            let dot_file = callgrind_output
                .unwrap_parent()
                .join(format!("{callgrind_output_name}.callgraph.dot"));

            let svg_file = callgrind_output
                .unwrap_parent()
                .join(format!("{callgrind_output_name}.callgraph.svg"));

            //gprof2dot -f callgrind callgrind.slang_merkle_proof.test.out | dot -Tsvg -o output.svg
            PipEnv::run("gprof2dot")
                .property("-f", "callgrind")
                .property("-o", dot_file.unwrap_str())
                .arg(callgrind_output.unwrap_str())
                .run();

            Command::new("dot")
                .arg("-Tsvg")
                .property("-o", svg_file.unwrap_str())
                .arg(dot_file.unwrap_str())
                .run();
        }
    }
}

use std::path::Path;

use anyhow::Result;
use clap::{Parser, Subcommand};
use infra_utils::commands::Command;
use infra_utils::paths::{FileWalker, PathExtensions};

use crate::commands::perf::binaries;
use crate::toolchains::bencher::{run_bench, BencherThreshold};
use crate::toolchains::pipenv::PipEnv;
use crate::utils::DryRun;

const DEFAULT_BENCHER_PROJECT_CMP: &str = "slang-dashboard-cargo-cmp";
const DEFAULT_BENCHER_PROJECT_SLANG: &str = "slang-dashboard-cargo-slang";
const DEFAULT_BENCHER_PROJECT_SLANG_V2: &str = "slang-dashboard-cargo-slang-v2";

#[allow(clippy::struct_excessive_bools)]
#[derive(Clone, Debug, Parser)]
pub struct CargoController {
    #[command(subcommand)]
    bench: Benches,
    #[command(flatten)]
    dry_run: DryRun,
    /// Run as a PR benchmark with regression detection via bencher start points.
    #[arg(long)]
    pr_benchmark: bool,
    #[arg(long)]
    no_deps: bool,
    /// Skip installing apt-managed deps (valgrind, graphviz).
    #[arg(long)]
    no_apt_deps: bool,
    /// Install deps and build bench binaries, but skip running benchmarks.
    #[arg(long, conflicts_with = "pr_benchmark")]
    smoke: bool,
}

#[derive(Clone, Debug, Subcommand)]
enum Benches {
    /// Performs the slang-specific benchmarks
    Slang,
    /// Performs a comparison with different crates for solidity parsing
    Comparison,
    /// Performs the slang v2 benchmarks
    SlangV2,
}

impl CargoController {
    pub fn execute(&self) -> Result<()> {
        if !self.no_deps {
            if !self.no_apt_deps {
                binaries::install_valgrind()?;
                binaries::install_graphviz()?;
            }
            binaries::install_iai_callgrind_runner()?;
            binaries::install_bencher_cli()?;
        }

        if self.smoke {
            let (package, bench_name) = match self.bench {
                Benches::Slang => ("solidity_testing_perf_cargo", "slang"),
                Benches::Comparison => ("solidity_testing_perf_cargo", "comparison"),
                Benches::SlangV2 => ("solidity_testing_perf_cargo", "slang_v2"),
            };
            Command::new("cargo")
                .args(["build", "--package", package, "--bench", bench_name])
                .run();
            // Verify gprof2dot is installed (used by generate_callgraph after full benchmarks).
            PipEnv::run("gprof2dot").arg("--help").run();
            return Ok(());
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
            Benches::SlangV2 => self.run_iai_bench(
                "solidity_testing_perf_cargo",
                "slang_v2",
                DEFAULT_BENCHER_PROJECT_SLANG_V2,
            ),
        }
        Ok(())
    }

    fn run_iai_bench(&self, package_name: &str, bench_name: &str, bencher_project: &str) {
        let test_runner = format!("cargo bench --package {package_name} --bench {bench_name}");

        // iai-callgrind uses deterministic hardware counters (not wall clock),
        // so any change reflects a real code change, not noise.
        // We also keep the window small (only 1 measurement), for the same reason.
        let threshold = |measure, upper_boundary| {
            BencherThreshold::new(measure, upper_boundary).with_max_sample_size("1")
        };

        run_bench(
            self.dry_run.get(),
            self.pr_benchmark,
            bencher_project,
            "rust_iai_callgrind",
            &[
                // Most measures use a tight 1% threshold:
                threshold("estimated-cycles", "0.01"),
                threshold("instructions", "0.01"),
                threshold("total-read-write", "0.01"),
                threshold("total-bytes", "0.01"),
                threshold("total-blocks", "0.01"),
                threshold("at-t-gmax-bytes", "0.01"),
                threshold("at-t-gmax-blocks", "0.01"),
                threshold("at-t-end-bytes", "0.01"),
                threshold("at-t-end-blocks", "0.01"),
                threshold("reads-bytes", "0.01"),
                threshold("writes-bytes", "0.01"),
                // l1-hits, l2-hits, and ram-hits instead use 100%, since there's not a simple
                // rule that could catch all cases  (ie more l1-hits is better if total bytes read remains the same,
                // but less l1-hits is  also better if it decreases total bytes read).
                // We still track them, but only alert on drastic (2x) regressions.
                threshold("l1-hits", "1"),
                threshold("l2-hits", "1"),
                threshold("ram-hits", "1"),
            ],
            &test_runner,
        );

        let reports_dir = Path::repo_path("target/iai")
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

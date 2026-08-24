use std::path::Path;

use anyhow::{Result, ensure};
use clap::{Parser, Subcommand, ValueEnum};
use infra_utils::commands::Command;
use infra_utils::paths::{FileWalker, PathExtensions};

use crate::commands::perf::binaries;
use crate::toolchains::bencher::{BencherProject, BencherThreshold, run_bench};
use crate::toolchains::pipenv::PipEnv;
use crate::utils::DryRun;

#[allow(clippy::struct_excessive_bools)]
#[derive(Clone, Debug, Parser)]
pub struct CargoController {
    #[command(subcommand)]
    bench: Benches,
    #[command(flatten)]
    dry_run: DryRun,
    /// Run as a PR benchmark with regression detection via bencher start points.
    ///
    /// Defaults to "fast" mode (DHAT skipped, Callgrind only). Pass
    /// "--pr-benchmark=full" to also run DHAT.
    #[arg(
        long,
        value_name = "MODE",
        num_args = 0..=1,
        require_equals = true,
        default_missing_value = "fast",
        conflicts_with = "dry_run"
    )]
    pr_benchmark: Option<PrBenchmarkMode>,
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
    /// Performs the wall-clock ('divan') benchmarks of the slang v2 pipeline
    ///
    /// Unlike its sibling suites, this one is not measured under Valgrind, so
    /// its numbers are wall time rather than deterministic counters, and
    /// nothing is reported to the Bencher dashboard: wall time on shared
    /// runners is too noisy to alert on.
    SlangV2WallClock {
        /// Arguments forwarded to 'divan', after '--'.
        ///
        /// Accepts a substring filter and flags such as '--sample-count' or
        /// '--max-time'. Pass '--help' to see everything 'divan' supports.
        #[arg(last = true, allow_hyphen_values = true)]
        divan_args: Vec<String>,
    },
}

#[derive(Clone, Debug, Default, PartialEq, Eq, ValueEnum)]
enum PrBenchmarkMode {
    /// Skip DHAT (run Callgrind only) to keep PRs fast.
    #[default]
    Fast,
    /// Run DHAT too, matching what `main` measures.
    Full,
}

/// The package holding every cargo benchmark suite.
const PACKAGE: &str = "solidity_testing_perf_cargo";

impl CargoController {
    pub fn execute(&self) -> Result<()> {
        match &self.bench {
            Benches::Comparison => self.run_gungraun_suite("comparison", BencherProject::CargoCmp),
            Benches::Slang => self.run_gungraun_suite("slang", BencherProject::CargoSlang),
            Benches::SlangV2 => self.run_gungraun_suite("slang_v2", BencherProject::CargoSlangV2),
            Benches::SlangV2WallClock { divan_args } => {
                ensure!(
                    self.pr_benchmark.is_none(),
                    "'--pr-benchmark' does not apply to 'slang-v2-wall-clock', which reports nothing to the Bencher dashboard."
                );

                Self::run_divan_suite("slang_v2_wall_clock", divan_args, self.smoke);

                Ok(())
            }
        }
    }

    fn run_gungraun_suite(&self, bench_name: &str, bencher_project: BencherProject) -> Result<()> {
        if !self.no_deps {
            if !self.no_apt_deps {
                binaries::install_valgrind()?;
                binaries::install_graphviz()?;
            }
            binaries::install_gungraun_runner()?;
            binaries::install_bencher_cli()?;
        }

        if self.smoke {
            Command::new("cargo")
                .args(["build", "--package", PACKAGE, "--bench", bench_name])
                .run();
            // Verify gprof2dot is installed (used by generate_callgraph after full benchmarks).
            PipEnv::run("gprof2dot").arg("--help").run();
            return Ok(());
        }

        // Bencher supports multiple languages/frameworks: https://bencher.dev/docs/explanation/adapters/
        // We currently only have one benchmark suite (Rust/gungraun), but we can add more here in the future.
        self.run_gungraun_bench(bench_name, bencher_project);
        Ok(())
    }

    /// Runs a wall-clock suite: no Valgrind, no external deps to install, and
    /// no upload step, so none of the bencher-related flags apply here.
    fn run_divan_suite(bench_name: &str, divan_args: &[String], smoke: bool) {
        if smoke {
            Command::new("cargo")
                .args(["build", "--package", PACKAGE, "--bench", bench_name])
                .run();

            return;
        }

        // 'cargo bench' builds with the 'bench' profile (optimized), which is
        // what wall-clock measurements need.
        Command::new("cargo")
            .args(["bench", "--package", PACKAGE, "--bench", bench_name])
            .arg("--")
            .args(divan_args)
            .run();
    }

    fn run_gungraun_bench(&self, bench_name: &str, bencher_project: BencherProject) {
        let test_runner = format!("cargo bench --package {PACKAGE} --bench {bench_name}");

        // gungraun's metrics come from valgrind's simulation, so they're deterministic: any change reflects a
        // real code change, not noise.
        // We also keep the window small (only 1 measurement), for the same reason.
        let threshold = |measure, upper_boundary| {
            BencherThreshold::new(measure, upper_boundary).with_max_sample_size("1")
        };

        // DHAT is much slower than Callgrind, so fast PR benchmarks skip it and
        // run Callgrind only.
        // __SLANG_PERF_SKIP_DHAT_ENV__ (keep in sync)
        let skip_dhat = matches!(self.pr_benchmark, Some(PrBenchmarkMode::Fast));
        let bench_env: &[(&str, &str)] = if skip_dhat {
            &[("SLANG_PERF_SKIP_DHAT", "1")]
        } else {
            &[]
        };

        run_bench(
            self.dry_run.get(),
            self.pr_benchmark.is_some(),
            bencher_project,
            "rust_gungraun",
            &[
                // Most measures use a tight 1% threshold:
                threshold("estimated-cycles", "0.01"),
                threshold("instructions", "0.01"),
                threshold("total-read-write", "0.01"),
                threshold("total-bytes", "0.01"),
                threshold("total-blocks", "0.01"),
                threshold("at-t-end-bytes", "0.01"),
                threshold("at-t-end-blocks", "0.01"),
                // The following metrics are alerted at 100% rather than 1%,
                // we still track them, but only alert on drastic (2x) regressions.
                //
                // These metrics are not particularly meaningful to us:
                //  - `t-gmax` is a whole-process instant; if the global heap peak happens outside our
                //    benchmarking function (e.g. during `setup`), none of our blocks are alive then and
                //    the filtered value is 0.
                //  - `reads-bytes` and `writes-bytes` are attributed by allocation site, not access site,
                //    so they miss reads/writes to memory allocated outside the benchmark and include those
                //    done in `setup`/`teardown` to memory the benchmark allocated.
                threshold("at-t-gmax-bytes", "1"),
                threshold("at-t-gmax-blocks", "1"),
                threshold("reads-bytes", "1"),
                threshold("writes-bytes", "1"),
                // l1-hits, ll-hits, and ram-hits have no simple
                // rule that could catch all cases (ie more l1-hits is better if total bytes read remains the same,
                // but less l1-hits is also better if it decreases total bytes read).
                threshold("l1-hits", "1"),
                threshold("ll-hits", "1"),
                threshold("ram-hits", "1"),
            ],
            bench_env,
            &test_runner,
        );

        let reports_dir = Path::repo_path("target/gungraun")
            .join(PACKAGE)
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

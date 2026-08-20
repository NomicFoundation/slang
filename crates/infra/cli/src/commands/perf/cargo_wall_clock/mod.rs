use anyhow::Result;
use clap::Parser;
use infra_utils::commands::Command;

/// The `divan` suite in `crates/solidity/testing/perf/cargo`.
const PACKAGE: &str = "solidity_testing_perf_cargo";
const BENCH: &str = "slang_v2_wall_clock";

/// Runs the wall-clock (`divan`) benchmarks of the Slang v2 pipeline.
///
/// This is a cargo benchmark suite like the ones behind `infra perf cargo`, and
/// it is deliberately a separate subcommand rather than another `perf cargo`
/// bench, because for now it is only meant for development use:
///
/// - It is not measured under Valgrind, so its numbers are wall time rather
///   than deterministic counters.
/// - Nothing is reported to the Bencher dashboard, since wall time on shared
///   runners is too noisy to alert on. `perf cargo` exists to feed that
///   dashboard; this command has no upload step at all.
///
/// What it is for is comparing two local builds of the pipeline — most notably
/// before and after a stage starts using multiple threads, which Valgrind's
/// instruction counts cannot show. If it ever grows a CI-reported role, it
/// belongs under `perf cargo` alongside the others.
#[derive(Clone, Debug, Parser)]
pub struct CargoWallClockController {
    /// Build the benchmark binary, but skip running the benchmarks.
    #[arg(long)]
    smoke: bool,

    /// Arguments forwarded to `divan`, after `--`.
    ///
    /// Accepts a substring filter and flags such as `--sample-count` or
    /// `--max-time`. Pass `--help` to see everything `divan` supports.
    #[arg(last = true, allow_hyphen_values = true)]
    divan_args: Vec<String>,
}

impl CargoWallClockController {
    // Returns `Result` for consistency with the sibling `perf` controllers.
    #[allow(clippy::unnecessary_wraps)]
    pub fn execute(&self) -> Result<()> {
        if self.smoke {
            Command::new("cargo")
                .args(["build", "--package", PACKAGE, "--bench", BENCH])
                .run();

            return Ok(());
        }

        // 'cargo bench' builds with the 'bench' profile (optimized), which is
        // what wall-clock measurements need.
        Command::new("cargo")
            .args(["bench", "--package", PACKAGE, "--bench", BENCH])
            .arg("--")
            .args(&self.divan_args)
            .run();

        Ok(())
    }
}

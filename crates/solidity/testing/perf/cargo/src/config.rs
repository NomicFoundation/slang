use gungraun::{Callgrind, Dhat, Direction, FlamegraphConfig, LibraryBenchmarkConfig};

/// Env var that, when set (to any value), makes
/// [`benchmark_config_with_num_callers`] skip the DHAT tool and run Callgrind
/// only.
///
/// DHAT is expensive (especially at high `num_callers`), so in certain
/// cases we want to skip it.
// __SLANG_PERF_SKIP_DHAT_ENV__ (keep in sync)
pub const SKIP_DHAT_ENV: &str = "SLANG_PERF_SKIP_DHAT";

/// Shared `LibraryBenchmarkConfig` used by every perf benchmark in this crate.
/// Centralised so the bench `main!` calls can't drift apart.
///
/// `num_callers` sets Valgrind's `--num-callers`: the maximum depth of the call
/// stack Valgrind records (and unwinds) at each allocation. Allocations are
/// attributed to a stack trace truncated to this depth, so a larger value
/// distinguishes allocation sites that share their top frames (e.g. a `malloc`
/// reached through several layers of generic/`Vec` code) and gives more
/// precise per-site attribution — at the cost of slower runs, since DHAT
/// unwinds that many frames on every allocation. Smaller values are coarser
/// but cheaper. Must be between 1 and 500 (DHAT's maximum).
pub fn benchmark_config_with_num_callers(num_callers: usize) -> LibraryBenchmarkConfig {
    assert!(
        0 < num_callers && num_callers <= 500,
        "num_callers must be between 1 and 500"
    );

    let mut config = LibraryBenchmarkConfig::default();
    config
        // 'valgrind' supports many tools. We run 'callgrind', which reports these metrics:
        // https://kcachegrind.github.io/html/Home.html
        //
        // Instructions:            Total CPU instructions executed.
        // LL Hits:                 Total (simulated) number of times the LL cache was hit.
        // L2 Hits:                 Total (simulated) number of times the L2 cache was hit.
        // RAM Hits:                Total (simulated) number of times the RAM was hit.
        // Total read+write:        Total memory reads/writes during the entire execution.
        // Estimated Cycles:        Number of CPU cycles (estimated) that went by during the entire execution.
        //
        // We also enable flame graphs into Cargo's 'target' directory.
        // They will be listed by 'infra perf' at the end of the run:
        .tool(
            Callgrind::default()
                .flamegraph(FlamegraphConfig::default().direction(Direction::BottomToTop)),
        )
        // 'valgrind' executes tests without any environment variables set by default.
        // Let's disable this behavior to be able to execute our infra utilities:
        .env_clear(false);

    // The 'DHAT' tool is much slower than Callgrind, so it's skipped on PR
    // benchmarks for the slower suites (see `SKIP_DHAT_ENV`). When enabled, it
    // reports these metrics: https://valgrind.org/docs/manual/dh-manual.html
    //
    // Total bytes:             How many bytes were allocated over the entire execution.
    // Total blocks:            How many heap blocks were allocated over the entire execution.
    // At t-gmax bytes:         How many bytes were alive when the heap size reached its global maximum (as measured in bytes).
    // At t-gmax blocks:        How many heap blocks were alive when the heap size reached its global maximum (as measured in bytes).
    // At t-end bytes:          How many bytes were alive at the end of execution (were not explicitly freed).
    // At t-end blocks:         How many heap blocks were alive at the end of execution (were not explicitly freed).
    // Reads bytes:             How many bytes within heap blocks were read during the entire execution.
    // Writes bytes:            How many bytes within heap blocks were written during the entire execution.
    if std::env::var_os(SKIP_DHAT_ENV).is_none() {
        // We set the DHAT arguments to whatever the user provided.
        let dhat_args = [format!("--num-callers={num_callers}")];
        config.tool(Dhat::with_args(dhat_args));
    }

    config
}

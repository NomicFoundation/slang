#![allow(clippy::exit)]

use std::hint::black_box;

use iai_callgrind::{
    library_benchmark, library_benchmark_group, main, Direction, FlamegraphConfig,
    LibraryBenchmarkConfig, Tool, ValgrindTool,
};
use paste::paste;
use solidity_testing_perf_cargo::dataset::SolidityProject;
use solidity_testing_perf_cargo::tests;

mod __dependencies_used_in_lib__ {
    use {
        anyhow as _, infra_utils as _, semver as _, serde as _, serde_json as _,
        slang_solidity as _, slang_solidity_v2_common as _, slang_solidity_v2_parser as _,
        solar as _, solidity_testing_utils as _, streaming_iterator as _, tree_sitter as _,
        tree_sitter_solidity as _,
    };
}

macro_rules! slang_v2_define_tests {
    ($prj:ident) => {
        /*
         * WARNING:
         * The reported `iai` benchmark ID is constructed from: `{file_name}::{group_name}::{function_name}`
         * Changing any of the above would change the resulting benchmark ID, and disconnect it from previous results.
         *
         * __SLANG_V2_INFRA_BENCHMARKS_LIST__ (keep in sync)
         */

        paste! {
            #[library_benchmark(setup = tests::slang_v2_parser::setup)]
            #[bench::test(stringify!($prj))]
            pub fn [< $prj _parser >](project: &SolidityProject) {
                black_box(tests::slang_v2_parser::run(project))
            }

            library_benchmark_group!(
                name = [< $prj _full >];

                // __SLANG_V2_INFRA_BENCHMARKS_LIST__ (keep in sync)
                benchmarks =
                    [< $prj _parser >],
            );
        }
    };
}

// We test a few projects in full for slang v2 benchmarks.
// Only 0.8.x-compatible projects can be used here.
// __SLANG_INFRA_PROJECT_LIST__ (keep in sync)
// TODO(v2) Using ui_pool_data_provider_v3 as a placeholder for now, we should look for relevant
// contracts that are compatible with slang v2.
slang_v2_define_tests!(ui_pool_data_provider_v3);
slang_v2_define_tests!(merkle_proof);

main!(
    config = LibraryBenchmarkConfig::default()
        // 'valgrind' supports many tools. By default, it runs 'callgrind', which reports these metrics:
        // https://kcachegrind.github.io/html/Home.html
        //
        // Instructions:            Total CPU instructions executed.
        // L1 Hits:                 Total (simulated) number of times the L1 cache was hit.
        // L2 Hits:                 Total (simulated) number of times the L2 cache was hit.
        // RAM Hits:                Total (simulated) number of times the RAM was hit.
        // Total read+write:        Total memory reads/writes during the entire execution.
        // Estimated Cycles:        Number of CPU cycles (estimated) that went by during the entire execution.
        //
        // We also enable the 'DHAT' tool below, which reports these metrics:
        // https://valgrind.org/docs/manual/dh-manual.html
        //
        // Total bytes:             How many bytes were allocated over the entire execution.
        // Total blocks:            How many heap blocks were allocated over the entire execution.
        // At t-gmax bytes:         How many bytes were alive when the heap size reached its global maximum (as measured in bytes).
        // At t-gmax blocks:        How many heap blocks were alive when the heap size reached its global maximum (as measured in bytes).
        // At t-end bytes:          How many bytes were alive at the end of execution (were not explicitly freed).
        // At t-end blocks:         How many heap blocks were alive at the end of execution (were not explicitly freed).
        // Reads bytes:             How many bytes within heap blocks were read during the entire execution.
        // Writes bytes:            How many bytes within heap blocks were written during the entire execution.
        .tool(Tool::new(ValgrindTool::DHAT))

        // This enables generating flame graphs into Cargo's 'target' directory.
        // They will be listed by 'infra perf' at the end of the run:
        .flamegraph(FlamegraphConfig::default().direction(Direction::BottomToTop))

        // 'valgrind' executes tests without any environment variables set by default.
        // Let's disable this behavior to be able to execute our infra utilities:
        .env_clear(false);

    // NOTE: the trailing comma is required: without it, it won't test the last one
    // __SLANG_INFRA_PROJECT_LIST__ (keep in sync)
    library_benchmark_groups = ui_pool_data_provider_v3_full,merkle_proof_full,
);

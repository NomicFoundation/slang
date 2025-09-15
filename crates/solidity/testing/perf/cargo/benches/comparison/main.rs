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
        slang_solidity as _, solar as _, solidity_testing_utils as _, streaming_iterator as _,
        tree_sitter as _, tree_sitter_solidity as _,
    };
}

macro_rules! comparison_tests {
    ($prj:ident) => {
        /*
         * WARNING:
         * The reported `iai` benchmark ID is constructed from: `{file_name}::{group_name}::{function_name}`
         * Changing any of the above would change the resulting benchmark ID, and disconnect it from previous results.
         */

        paste! {
          #[library_benchmark(setup = tests::setup::setup)]
          #[bench::first(stringify!($prj))]
          pub fn [<slang_ $prj>](project: &SolidityProject) {
              black_box(tests::parser::run(project));
          }

          #[library_benchmark(setup = tests::setup::setup)]
          #[bench::first(stringify!($prj))]
          pub fn [< solar_ $prj >](project: &SolidityProject) {
              black_box(tests::solar_parser::run(project));
          }

          #[library_benchmark(setup = tests::setup::setup)]
          #[bench::first(stringify!($prj))]
          pub fn [< tree_sitter_ $prj >](project: &SolidityProject) {
              black_box(tests::tree_sitter_parser::run(project));
          }

          library_benchmark_group!(
              name = [< $prj _group >];
              benchmarks = [< slang_ $prj >],[< solar_ $prj >],[< tree_sitter_ $prj >],
          );
        }
    };
}

// __SLANG_INFRA_PROJECT_LIST__ (keep in sync)
// flat_imports_mooniswap is version incompatible with solar
comparison_tests!(weighted_pool);
// uniswap is incompatible in tree-sitter
comparison_tests!(multicall3);
// create_x is incompatible in tree-sitter
comparison_tests!(ui_pool_data_provider_v3);
comparison_tests!(cooldogs);
comparison_tests!(one_step_leverage_f);
comparison_tests!(pointer_libraries);
comparison_tests!(merkle_proof);

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
    library_benchmark_groups =
        weighted_pool_group,
        multicall3_group,
        ui_pool_data_provider_v3_group,
        cooldogs_group,
        one_step_leverage_f_group,
        pointer_libraries_group,
        merkle_proof_group,
);

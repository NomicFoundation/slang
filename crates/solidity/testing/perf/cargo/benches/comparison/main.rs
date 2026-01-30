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

macro_rules! slang_test {
    ($prj:ident) => {
        paste! {
            #[library_benchmark(setup = tests::setup::setup)]
            #[bench::test(stringify!($prj))]
            pub fn [<slang_ $prj>](project: &SolidityProject) {
                black_box(tests::parser::run(project));
            }
        }
    };
}

macro_rules! solar_test {
    ($prj:ident) => {
        paste! {
            #[library_benchmark(setup = tests::setup::setup)]
            #[bench::test(stringify!($prj))]
            pub fn [<solar_ $prj>](project: &SolidityProject) {
                black_box(tests::solar_parser::run(project));
            }
        }
    };
}

macro_rules! tree_sitter_test {
    ($prj:ident) => {
        paste! {
            #[library_benchmark(setup = tests::setup::setup)]
            #[bench::test(stringify!($prj))]
            pub fn [<tree_sitter_ $prj>](project: &SolidityProject) {
                black_box(tests::tree_sitter_parser::run(project));
            }
        }
    };
}

macro_rules! v2_parser_test {
    ($prj:ident) => {
        paste! {
            #[library_benchmark(setup = tests::setup::setup)]
            #[bench::test(stringify!($prj))]
            pub fn [<v2_parser_ $prj>](project: &SolidityProject) {
                black_box(tests::v2_parser::run(project));
            }
        }
    };
}

// Some projects can't be parsed by tree-sitter, so we test them only in slang and solar.
// This macro abstracts that logic.
macro_rules! slang_and_solar_tests {
    ($prj:ident) => {
        solar_test!($prj);
        slang_test!($prj);
        paste! {
            library_benchmark_group!(
              name = [< $prj _group >];
              benchmarks = [< solar_ $prj >],[< slang_ $prj >],
          );
        }
    };
}

/*
 * WARNING:
 * The reported `iai` benchmark ID is constructed from: `{file_name}::{group_name}::{function_name}`
 * Changing any of the above would change the resulting benchmark ID, and disconnect it from previous results.
 */
macro_rules! comparison_tests {
    (mooniswap) => {
        // Incompatible with solar
        slang_test!(mooniswap);
        tree_sitter_test!(mooniswap);
        library_benchmark_group!(
            name = mooniswap_group;
            benchmarks = slang_mooniswap,tree_sitter_mooniswap,
        );
    };
    (uniswap) => {
        slang_test!(uniswap);
        solar_test!(uniswap);
        v2_parser_test!(uniswap);
        paste! {
            library_benchmark_group!(
                name = [< uniswap _group >];
                benchmarks = [< slang_ uniswap >],[< solar_ uniswap >],[< v2_parser_ uniswap >],
            );
        }
    };
    (uniswap_v2) => {
        // Stripped PoolManager.sol for v2 parser comparison (no pragma, no imports).
        slang_test!(uniswap_v2);
        solar_test!(uniswap_v2);
        v2_parser_test!(uniswap_v2);
        paste! {
            library_benchmark_group!(
                name = [< uniswap_v2 _group >];
                benchmarks = [< slang_ uniswap_v2 >],[< solar_ uniswap_v2 >],[< v2_parser_ uniswap_v2 >],
            );
        }
    };
    (weighted_pool_v2) => {
        // Stripped WeightedPool.sol for v2 parser comparison (no pragma, no imports).
        slang_test!(weighted_pool_v2);
        solar_test!(weighted_pool_v2);
        v2_parser_test!(weighted_pool_v2);
        paste! {
            library_benchmark_group!(
                name = [< weighted_pool_v2 _group >];
                benchmarks = [< slang_ weighted_pool_v2 >],[< solar_ weighted_pool_v2 >],[< v2_parser_ weighted_pool_v2 >],
            );
        }
    };
    (create_x_v2) => {
        // Stripped CreateX.sol for v2 parser comparison (no pragma, no assembly).
        slang_test!(create_x_v2);
        solar_test!(create_x_v2);
        v2_parser_test!(create_x_v2);
        paste! {
            library_benchmark_group!(
                name = [< create_x_v2 _group >];
                benchmarks = [< slang_ create_x_v2 >],[< solar_ create_x_v2 >],[< v2_parser_ create_x_v2 >],
            );
        }
    };
    (create_x) => {
        slang_and_solar_tests!(create_x);
    };
    (pointer_libraries) => {
        slang_and_solar_tests!(pointer_libraries);
    };
    ($prj:ident) => {
        slang_test!($prj);
        solar_test!($prj);
        tree_sitter_test!($prj);
        paste! {
          library_benchmark_group!(
              name = [< $prj _group >];
              benchmarks = [< slang_ $prj >],[< solar_ $prj >],[< tree_sitter_ $prj >],
          );
        }
    };
}

// __SLANG_INFRA_PROJECT_LIST__ (keep in sync)
comparison_tests!(mooniswap);
comparison_tests!(weighted_pool);
comparison_tests!(uniswap);
comparison_tests!(uniswap_v2);
comparison_tests!(weighted_pool_v2);
comparison_tests!(create_x_v2);
comparison_tests!(multicall3);
comparison_tests!(create_x);
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
        // mooniswap_group,
        // weighted_pool_group,
        uniswap_group,
        // uniswap_v2_group,
        // weighted_pool_v2_group,
        // create_x_v2_group,
        // multicall3_group,
        // create_x_group,
        // ui_pool_data_provider_v3_group,
        // cooldogs_group,
        // one_step_leverage_f_group,
        // pointer_libraries_group,
        // merkle_proof_group,
);

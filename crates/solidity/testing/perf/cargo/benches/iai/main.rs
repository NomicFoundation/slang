#![allow(clippy::exit)]
#![allow(clippy::needless_pass_by_value)]

use std::hint::black_box;
use std::rc::Rc;

use iai_callgrind::{
    library_benchmark, library_benchmark_group, main, Direction, FlamegraphConfig,
    LibraryBenchmarkConfig, Tool, ValgrindTool,
};
use paste::paste;
use slang_solidity::compilation::CompilationUnit;
use solidity_testing_perf_cargo::dataset::SolidityProject;
use solidity_testing_perf_cargo::tests;
use solidity_testing_perf_cargo::tests::bindings_resolve::BuiltBindingGraph;

mod __dependencies_used_in_lib__ {
    use {
        anyhow as _, infra_utils as _, semver as _, serde as _, serde_json as _,
        slang_solidity as _, solar as _, solidity_testing_perf_utils as _, streaming_iterator as _,
        tree_sitter as _, tree_sitter_solidity as _,
    };
}

// NOTE: Unfortunately, the bencher macros doesn't allow for modularization: it creates modules that are private, so
// they are inaccessible to the main function below when created inside a module. So we have to create in this file
// every benchmark for every library.

//
// Slang only benchmarks
//
macro_rules! slang_define_payload_benchmark {
    ($section_name:ident, $prj: ident, $prj_name: expr, $payload:ty) => {
        paste! {
          #[library_benchmark(setup = tests::$section_name::setup)]
          #[bench::first($prj_name)]
          pub fn [<$prj _ $section_name>](payload: $payload) {
              black_box(tests::$section_name::run(payload));
          }
        }
    };
}

macro_rules! slang_define_full_tests {
    ($prj:ident, $prj_name:tt) => {
        /*
         * WARNING:
         * The reported `iai` benchmark ID is constructed from: `{file_name}::{group_name}::{function_name}`
         * Changing any of the above would change the resulting benchmark ID, and disconnect it from previous results.
         *
         * __SLANG_INFRA_BENCHMARKS_LIST__ (keep in sync)
         */

        //NOTE: parsing is tested for the comparisons, so there's no need to add it here
        slang_define_payload_benchmark!(cursor, $prj, $prj_name, Rc<CompilationUnit>);
        slang_define_payload_benchmark!(query, $prj, $prj_name, Rc<CompilationUnit>);
        slang_define_payload_benchmark!(bindings_build, $prj, $prj_name, Rc<CompilationUnit>);
        slang_define_payload_benchmark!(bindings_resolve, $prj, $prj_name, BuiltBindingGraph);

        paste! {
        library_benchmark_group!(
            name = [< slang_ $prj _full>];

            // __SLANG_INFRA_BENCHMARKS_LIST__ (keep in sync)
            benchmarks =
              [< $prj _cursor>],
              [< $prj _query>],
              [< $prj _bindings_build>],
              [< $prj _bindings_resolve>],
          );
        }
    };
}

// We test a few projects in full for slang-only benchmarks
// __SLANG_INFRA_PROJECT_LIST__ (keep in sync)
slang_define_full_tests!(
    circular_imports_weighted_pool,
    "circular_imports_weighted_pool"
);
slang_define_full_tests!(
    three_quarters_file_merkle_proof,
    "three_quarters_file_merkle_proof"
);

// Slang comparison: we only test what can be compared: at the moment, just the parsing
macro_rules! slang_comparison_tests {
    ($prj:ident, $prj_name:tt) => {
        /*
         * WARNING:
         * The reported `iai` benchmark ID is constructed from: `{file_name}::{group_name}::{function_name}`
         * Changing any of the above would change the resulting benchmark ID, and disconnect it from previous results.
         */
        slang_define_payload_benchmark!(parser, $prj, $prj_name, &'static SolidityProject);

        paste! {
        library_benchmark_group!(
            name = [< slang_ $prj _group >];

            benchmarks = [< $prj _parser>],
          );
        }
    };
}

//
// Solar benchmarks
//
macro_rules! solar_comparison_tests {
    ($prj: ident, $prj_name: expr) => {
        paste! {
          #[library_benchmark(setup = tests::setup::setup)]
          #[bench::first($prj_name)]
          pub fn [< solar_ $prj >](project: &SolidityProject) {
              black_box(tests::solar_parser::run(project, false));
          }

          library_benchmark_group!(
            name = [< solar_ $prj _group >];
            benchmarks = [< solar_ $prj >]
          );
        }
    };
}

//
// Tree-sitter benchmarks
//
macro_rules! tree_sitter_comparison_tests {
    ($prj: ident, $prj_name: expr) => {
        paste! {
          #[library_benchmark(setup = tests::setup::setup)]
          #[bench::first(stringify!($prj))]
          pub fn [< tree_sitter_ $prj >](project: &SolidityProject) {
              black_box(tests::tree_sitter_parser::run(project, false));
          }

          library_benchmark_group!(
            name = [< tree_sitter_ $prj _group >];
            benchmarks = [< tree_sitter_ $prj >]
          );
        }
    };
}

// __SLANG_INFRA_PROJECT_LIST__ (keep in sync)
macro_rules! build_comparison_tests_for {
  ($library: ident) => {
    paste! {
      [< $library _comparison_tests>]!(flat_imports_mooniswap, "flat_imports_mooniswap");
      [< $library _comparison_tests>]!(circular_imports_weighted_pool, "circular_imports_weighted_pool");
      [< $library _comparison_tests>]!(protocol_uniswap, "protocol_uniswap");
      [< $library _comparison_tests>]!(protocol_multicall3, "protocol_multicall3");
      [< $library _comparison_tests>]!(protocol_create_x, "protocol_create_x");
      [< $library _comparison_tests>]!(protocol_ui_pool_data_provider_v3, "protocol_ui_pool_data_provider_v3");
      [< $library _comparison_tests>]!(deep_nesting_cooldogs, "deep_nesting_cooldogs");
      [< $library _comparison_tests>]!(largest_file_trivia_oslf, "largest_file_trivia_oslf");
      [< $library _comparison_tests>]!(median_file_safe_math, "median_file_safe_math");
      [< $library _comparison_tests>]!(three_quarters_file_merkle_proof, "three_quarters_file_merkle_proof");
    }
  };
}

build_comparison_tests_for!(slang);
build_comparison_tests_for!(solar);
build_comparison_tests_for!(tree_sitter);

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
    library_benchmark_groups = slang_circular_imports_weighted_pool_group,slang_three_quarters_file_merkle_proof_group,//slang_flat_imports_mooniswap,slang_circular_imports_weighted_pool,slang_protocol_uniswap,slang_protocol_multicall3,slang_protocol_create_x,slang_protocol_ui_pool_data_provider_v3,slang_deep_nesting_cooldogs,slang_largest_file_trivia_oslf,slang_median_file_safe_math,slang_three_quarters_file_merkle_proof,//solar_circular_imports_weighted_pool_group,solar_protocol_uniswap_group,solar_protocol_multicall3_group,solar_protocol_create_x_group,solar_protocol_ui_pool_data_provider_v3_group,solar_deep_nesting_cooldogs_group,solar_largest_file_trivia_oslf_group,solar_median_file_safe_math_group,solar_three_quarters_file_merkle_proof_group,tree_sitter_flat_imports_mooniswap_group,tree_sitter_circular_imports_weighted_pool_group,tree_sitter_protocol_uniswap_group,tree_sitter_protocol_multicall3_group,tree_sitter_protocol_ui_pool_data_provider_v3_group,tree_sitter_deep_nesting_cooldogs_group,tree_sitter_largest_file_trivia_oslf_group,tree_sitter_median_file_safe_math_group,tree_sitter_three_quarters_file_merkle_proof_group,
);

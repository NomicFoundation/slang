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

//
// Slang only benchmarks
//
macro_rules! slang_define_payload_benchmark {
    ($section_name:ident, $prj: ident, $payload:ty) => {
        paste! {
          #[library_benchmark(setup = tests::$section_name::setup)]
          #[bench::first(stringify!($prj))]
          pub fn [<$prj _ $section_name>](payload: $payload) {
              black_box(tests::$section_name::run(payload));
          }
        }
    };
}

macro_rules! slang_define_full_tests {
    ($prj:ident) => {
        /*
         * WARNING:
         * The reported `iai` benchmark ID is constructed from: `{file_name}::{group_name}::{function_name}`
         * Changing any of the above would change the resulting benchmark ID, and disconnect it from previous results.
         *
         * __SLANG_INFRA_BENCHMARKS_LIST__ (keep in sync)
         */

        slang_define_payload_benchmark!(parser, $prj, &'static SolidityProject);
        slang_define_payload_benchmark!(cursor, $prj, Rc<CompilationUnit>);
        slang_define_payload_benchmark!(query, $prj, Rc<CompilationUnit>);
        slang_define_payload_benchmark!(bindings_build, $prj, Rc<CompilationUnit>);
        slang_define_payload_benchmark!(bindings_resolve, $prj, BuiltBindingGraph);

        paste! {
        library_benchmark_group!(
            name = [< slang_ $prj _full>];

            // __SLANG_INFRA_BENCHMARKS_LIST__ (keep in sync)
            benchmarks =
              [< $prj _parser>],
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
slang_define_full_tests!(circular_imports_weighted_pool);
slang_define_full_tests!(three_quarters_file_merkle_proof);

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
    library_benchmark_groups = slang_circular_imports_weighted_pool_full,slang_three_quarters_file_merkle_proof_full,
);

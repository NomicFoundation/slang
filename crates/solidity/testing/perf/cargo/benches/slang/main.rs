#![allow(clippy::exit)]

use std::hint::black_box;
use std::rc::Rc;

use iai_callgrind::{
    library_benchmark, library_benchmark_group, main, Direction, FlamegraphConfig,
    LibraryBenchmarkConfig, Tool, ValgrindTool,
};
use paste::paste;
use slang_solidity::backend::BinderOutput;
use slang_solidity::compilation::CompilationUnit;
use solidity_testing_perf_cargo::dataset::SolidityProject;
use solidity_testing_perf_cargo::tests;
use solidity_testing_perf_cargo::tests::bindings_build::BuiltBindingGraph;

mod __dependencies_used_in_lib__ {
    use {
        anyhow as _, infra_utils as _, semver as _, serde as _, serde_json as _,
        slang_solidity as _, solar as _, solidity_testing_utils as _, streaming_iterator as _,
        tree_sitter as _, tree_sitter_solidity as _,
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

        paste! {
            #[library_benchmark(setup = tests::parser::setup)]
            #[bench::test(stringify!($prj))]
            pub fn [< $prj _parser >](project: &SolidityProject) -> Rc<CompilationUnit> {
                black_box(tests::parser::run(project))
            }

            #[library_benchmark(setup = tests::cursor::setup)]
            #[bench::test(stringify!($prj))]
            pub fn [< $prj _cursor >](unit: Rc<CompilationUnit>) -> Rc<CompilationUnit> {
                black_box(tests::cursor::run(unit))
            }

            #[library_benchmark(setup = tests::query::setup)]
            #[bench::test(stringify!($prj))]
            pub fn [< $prj _query >](unit: Rc<CompilationUnit>) -> Rc<CompilationUnit> {
                black_box(tests::query::run(unit))
            }

            #[library_benchmark(setup = tests::bindings_build::setup)]
            #[bench::test(stringify!($prj))]
            pub fn [< $prj _bindings_build >](unit: Rc<CompilationUnit>) -> BuiltBindingGraph {
                black_box(tests::bindings_build::run(unit))
            }

            #[library_benchmark(setup = tests::bindings_resolve::setup)]
            #[bench::test(stringify!($prj))]
            pub fn [< $prj _bindings_resolve >](unit: BuiltBindingGraph) -> BuiltBindingGraph {
                black_box(tests::bindings_resolve::run(unit))
            }

            // We add a cleanup phase to measure the destruction of the AST and the binding structures
            #[library_benchmark(setup = tests::bindings_resolve::setup)]
            #[bench::test(stringify!($prj))]
            pub fn [< $prj _cleanup >](unit: BuiltBindingGraph) {
                black_box(unit);
            }

            #[library_benchmark(setup = tests::binder::setup)]
            #[bench::test(stringify!($prj))]
            fn [< $prj _binder >](unit: CompilationUnit) -> BinderOutput {
                black_box(tests::binder::run(unit))
            }

            #[library_benchmark(setup = tests::binder_cleanup::setup)]
            #[bench::test(stringify!($prj))]
            fn [< $prj _binder_cleanup >](output: BinderOutput) {
                black_box(output);
            }

            library_benchmark_group!(
                name = [< $prj _full >];

                // __SLANG_INFRA_BENCHMARKS_LIST__ (keep in sync)
                benchmarks =
                    [< $prj _parser >],
                    [< $prj _cursor >],
                    [< $prj _query >],
                    [< $prj _bindings_build >],
                    [< $prj _bindings_resolve >],
                    [< $prj _cleanup >],
                    [< $prj _binder >],
                    [< $prj _binder_cleanup >],
            );
        }
    };
}

// We test a few projects in full for slang-only benchmarks
// __SLANG_INFRA_PROJECT_LIST__ (keep in sync)
slang_define_full_tests!(weighted_pool);
slang_define_full_tests!(merkle_proof);

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
    library_benchmark_groups = weighted_pool_full,merkle_proof_full,
);

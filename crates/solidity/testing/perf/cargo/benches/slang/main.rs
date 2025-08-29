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
          #[bench::first(stringify!($prj))]
          pub fn [< $prj _parser >](project: &'static SolidityProject) -> Rc<CompilationUnit> {
              black_box(tests::parser::run(project))
          }

          #[library_benchmark(setup = tests::cursor::setup)]
          #[bench::first(stringify!($prj))]
          pub fn [<$prj _cursor >](unit: Rc<CompilationUnit>) -> Rc<CompilationUnit> {
              black_box(tests::cursor::run(Rc::clone(&unit)));
              black_box(unit)
          }

          #[library_benchmark(setup = tests::query::setup)]
          #[bench::first(stringify!($prj))]
          pub fn [<$prj _query >](unit: Rc<CompilationUnit>) -> Rc<CompilationUnit> {
              black_box(tests::query::run(Rc::clone(&unit)));
              black_box(unit)
          }

          #[library_benchmark(setup = tests::bindings_build::setup)]
          #[bench::first(stringify!($prj))]
          pub fn [<$prj _bindings_build >](unit: Rc<CompilationUnit>) -> BuiltBindingGraph {
              fn run(unit: Rc<CompilationUnit>) -> BuiltBindingGraph {
                let binding_graph = tests::bindings_build::run(Rc::clone(&unit));
                BuiltBindingGraph {
                    unit,
                    binding_graph,
                }
              }

              black_box(run(unit))
          }

          #[library_benchmark(setup = tests::bindings_resolve::setup)]
          #[bench::first(stringify!($prj))]
          pub fn [<$prj _bindings_resolve >](unit: BuiltBindingGraph) -> BuiltBindingGraph {
              black_box(tests::bindings_resolve::run(unit.clone()));
              black_box(unit)
          }

          // We add a cleanup phase to measure the destruction of the AST and the binding structures
          #[library_benchmark(setup = tests::bindings_resolve::setup)]
          #[bench::first(stringify!($prj))]
          pub fn [< $prj _cleanup >](unit: BuiltBindingGraph) {
              black_box(unit);
          }

          library_benchmark_group!(
              name = [< $prj _full>];

              // __SLANG_INFRA_BENCHMARKS_LIST__ (keep in sync)
              benchmarks =
                [< $prj _parser>],
                [< $prj _cursor>],
                [< $prj _query>],
                [< $prj _bindings_build>],
                [< $prj _bindings_resolve>],
                [< $prj _cleanup>],
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
    library_benchmark_groups = circular_imports_weighted_pool_full,three_quarters_file_merkle_proof_full,
);

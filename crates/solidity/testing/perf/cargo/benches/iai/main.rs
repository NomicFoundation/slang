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
use solidity_testing_perf_cargo::tests::bindings_resolve::BuiltBindingGraph;

mod __dependencies_used_in_lib__ {
    use {
        anyhow as _, infra_utils as _, semver as _, serde_json as _, slang_solidity as _,
        solidity_testing_perf_utils as _,
    };
}

macro_rules! define_payload_benchmark {
    ($name:ident, $prj: ident, $file: expr, $payload:ty) => {
        paste! {
          #[library_benchmark]
          #[bench::first(args = ($file), setup = solidity_testing_perf_cargo::tests::$name::setup)]
          pub fn [<$prj _ $name>](payload: $payload) {
              black_box(solidity_testing_perf_cargo::tests::$name::run(payload));
          }
        }
    };
}

macro_rules! define_payload_tests {
    ($prj:ident, $name:tt) => {
        /*
         * WARNING:
         * The reported `iai` benchmark ID is constructed from: `{file_name}::{group_name}::{function_name}`
         * Changing any of the above would change the resulting benchmark ID, and disconnect it from previous results.
         *
         * __SLANG_INFRA_BENCHMARKS_LIST__ (keep in sync)
         */
        define_payload_benchmark!(parser, $prj, $name, &'static SolidityProject);
        define_payload_benchmark!(cursor, $prj, $name, Rc<CompilationUnit>);
        define_payload_benchmark!(query, $prj, $name, Rc<CompilationUnit>);
        define_payload_benchmark!(bindings_build, $prj, $name, Rc<CompilationUnit>);
        define_payload_benchmark!(bindings_resolve, $prj, $name, BuiltBindingGraph);

        paste! {
        library_benchmark_group!(
              name = $prj;

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

include!("../../src/benches_list.rs");

macro_rules! do_main {
    ($($groups:ident),+ $(,)?) => {
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

    library_benchmark_groups = $($groups,)+
);
}
}

// manually listed because I found no way to successfully do it dynamically
do_main!(
    mooniswap_flat_imports,
    weighted_pool_circular_imports,
    uniswap_lib,
    multicall3_lib,
    create_x_lib,
    ui_pool_data_provider_v3_lib,
    pointer_librarires_largest_file,
    one_step_leverage_f_largest_file_with_trivia,
    safe_math_median_file,
    merkle_proof_three_quarters_file,
);

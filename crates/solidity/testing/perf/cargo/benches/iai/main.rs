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
        anyhow as _,
        infra_utils as _,
        semver as _,
        serde_json as _,
        slang_solidity as _,
        // solang as _, solang_parser as _,
        solar as _,
        solidity_testing_perf_utils as _,
    };
}

// *Note:* Unfortunately, the bencher macros doesn't allow for modularization: it creates modules that are private, so
// they are inaccessible to the main function below when created insida a module. So we have to create here every
// benchmark for every library.

//
// Slang benchmarks
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

macro_rules! slang_define_payload_tests {
    ($prj:ident, $prj_name:tt) => {
        /*
         * WARNING:
         * The reported `iai` benchmark ID is constructed from: `{file_name}::{group_name}::{function_name}`
         * Changing any of the above would change the resulting benchmark ID, and disconnect it from previous results.
         *
         * __SLANG_INFRA_BENCHMARKS_LIST__ (keep in sync)
         */
        slang_define_payload_benchmark!(parser, $prj, $prj_name, &'static SolidityProject);
        slang_define_payload_benchmark!(cursor, $prj, $prj_name, Rc<CompilationUnit>);
        slang_define_payload_benchmark!(query, $prj, $prj_name, Rc<CompilationUnit>);
        slang_define_payload_benchmark!(bindings_build, $prj, $prj_name, Rc<CompilationUnit>);
        slang_define_payload_benchmark!(bindings_resolve, $prj, $prj_name, BuiltBindingGraph);

        paste! {
        library_benchmark_group!(
            name = [< slang_ $prj >];

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

include!("../../src/slang_benches_list.rs");

//
// Solang benchmarks
//
// macro_rules! solang_define_payload_test {
//     ($prj: ident, $prj_name: expr) => {
//         paste! {
// TODO: This is not right, the setup shouldn't be part of the measurement
//           fn [< setup_and_test_ $prj >] () {
//               let payload = solidity_testing_perf_cargo::tests::solang::setup($prj_name);
//               solidity_testing_perf_cargo::tests::solang::run(payload)
//           }

//           #[library_benchmark]
//           pub fn [< solang_ $prj >]() {
//               black_box([< setup_and_test_ $prj >]());
//           }

//           library_benchmark_group!(
//             name = [< solang_ $prj _group >];
//             benchmarks = [< solang_ $prj >]
//           );
//         }
//     };
// }

//
// Solar benchmarks
//
macro_rules! solar_define_payload_test {
    ($prj: ident, $prj_name: expr) => {
        paste! {
          #[library_benchmark(setup = tests::setup::setup)]
          #[bench::first($prj_name)]
          pub fn [< solar_ $prj >](project: &SolidityProject) {
              black_box(tests::solar_parser::run(project));
          }

          library_benchmark_group!(
            name = [< solar_ $prj _group>];
            benchmarks = [< solar_ $prj >]
          );
        }
    };
}

include!("../../src/solar_benches_list.rs");

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

    // Note: the trailing comma is required: without it, it won't test the last one
    library_benchmark_groups = slang_protocol_uniswap,slang_largest_file_trivia_oslf,solar_protocol_uniswap_group,solar_largest_file_trivia_oslf_group,
);

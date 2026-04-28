#![allow(clippy::exit)]

use std::hint::black_box;

use iai_callgrind::{library_benchmark, library_benchmark_group, main};
use paste::paste;
use slang_solidity_v2_cst::structured_cst::nodes::SourceUnit;
use slang_solidity_v2_ir::ir;
use slang_solidity_v2_semantic::context::SemanticContext;
use solidity_testing_perf_cargo::config::default_benchmark_config;
use solidity_testing_perf_cargo::dataset::SolidityProject;
use solidity_testing_perf_cargo::tests;

mod __dependencies_used_in_lib__ {
    use anyhow as _;
    use infra_utils as _;
    use semver as _;
    use serde as _;
    use serde_json as _;
    use slang_solidity as _;
    use slang_solidity_v2_common as _;
    use slang_solidity_v2_ir as _;
    use slang_solidity_v2_parser as _;
    use slang_solidity_v2_semantic as _;
    use solar as _;
    use solidity_testing_utils as _;
    use streaming_iterator as _;
    use tree_sitter as _;
    use tree_sitter_solidity as _;
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
            #[library_benchmark(setup = tests::slang_v2::parser::setup)]
            #[bench::test(stringify!($prj))]
            pub fn [< $prj _parser >](project: &SolidityProject) -> Vec<(String, SourceUnit)> {
                black_box(tests::slang_v2::parser::run(black_box(project)))
            }

            #[library_benchmark(setup = tests::slang_v2::ir_builder::setup)]
            #[bench::test(stringify!($prj))]
            // Note: the input CST source units are consumed (dropped) during IR building.
            // This is the intended use case: the CST is replaced by the IR representation.
            pub fn [< $prj _ir_builder >]((project, source_units): (&'static SolidityProject, Vec<(String, SourceUnit)>)) -> Vec<ir::SourceUnit> {
                black_box(tests::slang_v2::ir_builder::run(black_box(project), black_box(source_units)))
            }

            #[library_benchmark(setup = tests::slang_v2::semantic::setup)]
            #[bench::test(stringify!($prj))]
            pub fn [< $prj _semantic >](
                (project, input_files): (&'static SolidityProject, Vec<tests::slang_v2::semantic::File>),
            ) -> SemanticContext {
                black_box(tests::slang_v2::semantic::run(black_box(project), black_box(input_files)))
            }

            library_benchmark_group!(
                name = [< $prj _full_v2 >];

                // __SLANG_V2_INFRA_BENCHMARKS_LIST__ (keep in sync)
                benchmarks =
                [< $prj _parser >],
                [< $prj _ir_builder >],
                [< $prj _semantic >],
            );
        }
    };
}

// Only 0.8.x-compatible projects can be used here.
// __SLANG_INFRA_PROJECT_LIST__ (keep in sync)
slang_v2_define_tests!(uniswap);
slang_v2_define_tests!(multicall3);
slang_v2_define_tests!(create_x);
slang_v2_define_tests!(ui_pool_data_provider_v3);
slang_v2_define_tests!(cooldogs);
slang_v2_define_tests!(one_step_leverage_f);
slang_v2_define_tests!(pointer_libraries);
slang_v2_define_tests!(merkle_proof);

main!(
    config = default_benchmark_config();

    // NOTE: the trailing comma is required: without it, it won't test the last one
    // __SLANG_INFRA_PROJECT_LIST__ (keep in sync)
    library_benchmark_groups =
        uniswap_full_v2,
        multicall3_full_v2,
        create_x_full_v2,
        ui_pool_data_provider_v3_full_v2,
        cooldogs_full_v2,
        one_step_leverage_f_full_v2,
        pointer_libraries_full_v2,
        merkle_proof_full_v2,
);

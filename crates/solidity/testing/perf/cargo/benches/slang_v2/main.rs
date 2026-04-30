#![allow(clippy::exit)]

use std::hint::black_box;

use gungraun::{library_benchmark, library_benchmark_group, main};
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

/*
 * WARNING:
 * The reported `gungraun` benchmark ID is constructed from
 * `{file_name}::{group_name}::{function_name} <project>:"<project>"`.
 * Changing any of the above would change the resulting benchmark ID, and
 * disconnect it from previous results.
 *
 * __SLANG_V2_INFRA_BENCHMARKS_LIST__ (keep in sync)
 */

// Single source for the project list used by every stage benchmark below.
// Only 0.8.x-compatible projects belong here.
// Edit this macro (and only this macro) to add or remove a project.
// __SLANG_INFRA_PROJECT_LIST__ (keep in sync)
macro_rules! projects {
    (
        #[$lb:meta]
        $($rest:tt)*
    ) => {
        #[$lb]
        #[bench::uniswap("uniswap")]
        #[bench::multicall3("multicall3")]
        #[bench::create_x("create_x")]
        #[bench::ui_pool_data_provider_v3("ui_pool_data_provider_v3")]
        #[bench::cooldogs("cooldogs")]
        #[bench::one_step_leverage_f("one_step_leverage_f")]
        #[bench::pointer_libraries("pointer_libraries")]
        #[bench::merkle_proof("merkle_proof")]
        $($rest)*
    };
}

projects! {
    #[library_benchmark(setup = tests::slang_v2::parser::setup)]
    fn parser(project: &SolidityProject) -> Vec<(String, SourceUnit)> {
        black_box(tests::slang_v2::parser::run(black_box(project)))
    }
}

// Note: the input CST source units are consumed (dropped) during IR building.
// This is the intended use case: the CST is replaced by the IR representation.
projects! {
    #[library_benchmark(setup = tests::slang_v2::ir_builder::setup)]
    fn ir_builder(
        (project, source_units): (&'static SolidityProject, Vec<(String, SourceUnit)>),
    ) -> Vec<ir::SourceUnit> {
        black_box(tests::slang_v2::ir_builder::run(
            black_box(project),
            black_box(source_units),
        ))
    }
}

projects! {
    #[library_benchmark(setup = tests::slang_v2::semantic::setup)]
    fn semantic(
        (project, input_files): (
            &'static SolidityProject,
            Vec<tests::slang_v2::semantic::File>,
        ),
    ) -> SemanticContext {
        black_box(tests::slang_v2::semantic::run(
            black_box(project),
            black_box(input_files),
        ))
    }
}

library_benchmark_group!(
    name = pipeline;
    // __SLANG_V2_INFRA_BENCHMARKS_LIST__ (keep in sync)
    benchmarks = parser, ir_builder, semantic,
);

main!(
    config = default_benchmark_config();
    library_benchmark_groups = pipeline,
);

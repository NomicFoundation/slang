#![allow(clippy::exit)]

use std::hint::black_box;

use gungraun::{library_benchmark, library_benchmark_group, main};
use solidity_testing_perf_cargo::config::benchmark_config_with_num_callers;
use solidity_testing_perf_cargo::tests;
// Local aliases for the setup functions, so the generated benchmark ID reads
// `parser_setup("uniswap")` instead of `tests :: slang_v2 :: parser :: setup("uniswap")`.
use tests::slang_v2::compute_contracts_abi::setup as compute_contracts_abi_setup;
use tests::slang_v2::ir_builder::setup as ir_builder_setup;
use tests::slang_v2::parser::setup as parser_setup;
use tests::slang_v2::semantic::setup as semantic_setup;

mod __dependencies_used_in_lib__ {
    use anyhow as _;
    use inflector as _;
    use infra_utils as _;
    use paste as _;
    use semver as _;
    use serde as _;
    use serde_json as _;
    use slang_solidity as _;
    use slang_solidity_v2_ast as _;
    use slang_solidity_v2_common as _;
    use slang_solidity_v2_cst as _;
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
 * `{file_name}::{group_name}::{function_name} <project>:(<arguments>)`.
 * Changing any of the above would change the resulting benchmark ID, and
 * disconnect it from previous results.
 *
 * __SLANG_V2_INFRA_BENCHMARKS_LIST__ (keep in sync)
 */

// Single source for the project list used by every stage benchmark below.
// Only 0.8.x-compatible projects belong here.
// Edit this macro (and only this macro) to add or remove a project.
// __SLANG_INFRA_PROJECT_LIST__ (keep in sync)
macro_rules! bench_projects {
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
        #[bench::ens_registrar_controller("ens_registrar_controller")]
        $($rest)*
    };
}

bench_projects! {
    #[library_benchmark(setup = parser_setup)]
    fn parser(input: tests::slang_v2::parser::Input) -> tests::slang_v2::parser::Output {
        black_box(tests::slang_v2::parser::run(black_box(input)))
    }
}

// Note: the input CST source units are consumed (dropped) during IR building.
// This is the intended use case: the CST is replaced by the IR representation.
bench_projects! {
    #[library_benchmark(setup = ir_builder_setup)]
    fn ir_builder(
        input: tests::slang_v2::ir_builder::Input,
    ) -> tests::slang_v2::ir_builder::Output {
        black_box(tests::slang_v2::ir_builder::run(
            black_box(input),
        ))
    }
}

bench_projects! {
    #[library_benchmark(setup = semantic_setup)]
    fn semantic(
        input: tests::slang_v2::semantic::Input,
    ) -> tests::slang_v2::semantic::Output {
        black_box(tests::slang_v2::semantic::run(
            black_box(input),
        ))
    }
}

bench_projects! {
    #[library_benchmark(setup = compute_contracts_abi_setup)]
    fn compute_contracts_abi(
        input: tests::slang_v2::compute_contracts_abi::Input,
    ) -> tests::slang_v2::compute_contracts_abi::Output {
        black_box(tests::slang_v2::compute_contracts_abi::run(
            black_box(input),
        ))
    }
}

library_benchmark_group!(
    name = pipeline;
    // __SLANG_V2_INFRA_BENCHMARKS_LIST__ (keep in sync)
    benchmarks = parser, ir_builder, semantic, compute_contracts_abi,
);

main!(
    // We use the maximum possible value of `num-callers` to ensure DHAT values
    // are sensible
    config = benchmark_config_with_num_callers(500);
    library_benchmark_groups = pipeline,
);

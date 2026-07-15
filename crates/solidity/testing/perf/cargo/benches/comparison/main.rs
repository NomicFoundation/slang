#![allow(clippy::exit)]

use std::hint::black_box;
use std::rc::Rc;

// Renamed to avoid shadowing by gungraun's `pub mod tree_sitter` that wraps
// the `tree_sitter` benchmark fn below.
use ::tree_sitter::Tree as TreeSitterTree;
use gungraun::{library_benchmark, library_benchmark_group, main};
use slang_solidity::compilation::CompilationUnit;
use solidity_testing_perf_cargo::config::benchmark_config_with_num_callers;
use solidity_testing_perf_cargo::dataset::SolidityProject;
use solidity_testing_perf_cargo::tests;
// Import the shared setup function directly, so the generated benchmark ID reads
// `setup("uniswap")` instead of `tests :: setup :: setup("uniswap")`.
use tests::setup::setup;

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
 * Each parser fn lists only the projects it supports via `#[bench::PROJECT]`.
 * The `slang` fn enumerates the full project list (the master copy of
 * __SLANG_INFRA_PROJECT_LIST__); other parsers omit projects they cannot parse.
 */

// __SLANG_INFRA_PROJECT_LIST__ (keep in sync)
#[library_benchmark(setup = setup)]
#[bench::mooniswap("mooniswap")]
#[bench::weighted_pool("weighted_pool")]
#[bench::uniswap("uniswap")]
#[bench::multicall3("multicall3")]
#[bench::create_x("create_x")]
#[bench::ui_pool_data_provider_v3("ui_pool_data_provider_v3")]
#[bench::cooldogs("cooldogs")]
#[bench::one_step_leverage_f("one_step_leverage_f")]
#[bench::pointer_libraries("pointer_libraries")]
#[bench::merkle_proof("merkle_proof")]
#[bench::ens_registrar_controller("ens_registrar_controller")]
fn slang(project: &SolidityProject) -> Rc<CompilationUnit> {
    black_box(tests::slang::parser::run(black_box(project)))
}

// slang_v2 cannot parse mooniswap or weighted_pool (Solidity 0.7.1).
#[library_benchmark(setup = setup)]
#[bench::uniswap("uniswap")]
#[bench::multicall3("multicall3")]
#[bench::create_x("create_x")]
#[bench::ui_pool_data_provider_v3("ui_pool_data_provider_v3")]
#[bench::cooldogs("cooldogs")]
#[bench::one_step_leverage_f("one_step_leverage_f")]
#[bench::pointer_libraries("pointer_libraries")]
#[bench::merkle_proof("merkle_proof")]
#[bench::ens_registrar_controller("ens_registrar_controller")]
fn slang_v2(input: tests::slang_v2::parser::Input) -> tests::slang_v2::parser::Output {
    black_box(tests::slang_v2::parser::run(black_box(input)))
}

// solar cannot parse mooniswap.
#[library_benchmark(setup = setup)]
#[bench::weighted_pool("weighted_pool")]
#[bench::uniswap("uniswap")]
#[bench::multicall3("multicall3")]
#[bench::create_x("create_x")]
#[bench::ui_pool_data_provider_v3("ui_pool_data_provider_v3")]
#[bench::cooldogs("cooldogs")]
#[bench::one_step_leverage_f("one_step_leverage_f")]
#[bench::pointer_libraries("pointer_libraries")]
#[bench::merkle_proof("merkle_proof")]
#[bench::ens_registrar_controller("ens_registrar_controller")]
fn solar(project: &SolidityProject) -> usize {
    black_box(tests::solar::parser::run(black_box(project)))
}

// tree_sitter cannot parse uniswap, create_x, or pointer_libraries.
#[library_benchmark(setup = setup)]
#[bench::mooniswap("mooniswap")]
#[bench::weighted_pool("weighted_pool")]
#[bench::multicall3("multicall3")]
#[bench::ui_pool_data_provider_v3("ui_pool_data_provider_v3")]
#[bench::cooldogs("cooldogs")]
#[bench::one_step_leverage_f("one_step_leverage_f")]
#[bench::merkle_proof("merkle_proof")]
#[bench::ens_registrar_controller("ens_registrar_controller")]
fn tree_sitter(project: &SolidityProject) -> Vec<TreeSitterTree> {
    black_box(tests::tree_sitter::parser::run(black_box(project)))
}

library_benchmark_group!(
    name = parsers;
    benchmarks = slang, slang_v2, solar, tree_sitter,
);

main!(
    // We use the maximum possible value of `num-callers` to ensure DHAT values
    // are sensible
    config = benchmark_config_with_num_callers(500);
    library_benchmark_groups = parsers,
);

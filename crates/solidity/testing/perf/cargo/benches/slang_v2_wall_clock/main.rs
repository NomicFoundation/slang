//! Wall-clock benchmarks for the Slang v2 pipeline, driven by [`divan`].
//!
//! These complement the `slang_v2` suite, which measures the same pipeline with
//! `gungraun`/Valgrind.

use divan::counter::{BytesCount, ItemsCount};
use divan::{Bencher, black_box};
use solidity_testing_perf_cargo::dataset::SolidityProject;
use solidity_testing_perf_cargo::tests;

mod __dependencies_used_in_lib__ {
    use anyhow as _;
    use gungraun as _;
    use inflector as _;
    use infra_utils as _;
    use paste as _;
    use semver as _;
    use serde as _;
    use serde_json as _;
    use slang_solidity as _;
    use slang_solidity_v2 as _;
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

fn main() {
    divan::main();
}

// Single source for the project list used by every benchmark below.
// Only 0.8.x-compatible projects belong here.
// Edit this constant (and only this constant) to add or remove a project.
// __SLANG_INFRA_PROJECT_LIST__ (keep in sync)
const PROJECTS: [&str; 9] = [
    "uniswap",
    "multicall3",
    "create_x",
    "ui_pool_data_provider_v3",
    "cooldogs",
    "one_step_leverage_f",
    "pointer_libraries",
    "merkle_proof",
    "ens_registrar_controller",
];

/// Time budget per (benchmark, project) pair, in seconds.
const MAX_TIME_SECS: u64 = 10;

/// Builds a whole [`slang_solidity_v2::compilation::CompilationUnit`]: parsing,
/// IR building, and semantic analysis.
#[divan::bench(args = PROJECTS, max_time = MAX_TIME_SECS)]
fn full_compilation(bencher: Bencher<'_, '_>, project_name: &str) {
    let project = tests::slang_v2::full_compilation::setup(project_name);

    with_throughput_counters(bencher, project)
        .bench(|| black_box(tests::slang_v2::full_compilation::run(black_box(project))));
}

/// Parses every source of the project into a CST, without any of the later
/// stages.
#[divan::bench(args = PROJECTS, max_time = MAX_TIME_SECS)]
fn parser(bencher: Bencher<'_, '_>, project_name: &str) {
    let project = tests::slang_v2::parser::setup(project_name);

    with_throughput_counters(bencher, project)
        .bench(|| black_box(tests::slang_v2::parser::run(black_box(project))));
}

/// Reports bytes and files processed per second, so that results remain
/// comparable across projects of very different sizes.
fn with_throughput_counters<'a, 'b>(
    bencher: Bencher<'a, 'b>,
    project: &SolidityProject,
) -> Bencher<'a, 'b> {
    let total_bytes: usize = project.sources.values().map(String::len).sum();

    bencher
        .counter(BytesCount::new(total_bytes))
        .counter(ItemsCount::new(project.sources.len()))
}

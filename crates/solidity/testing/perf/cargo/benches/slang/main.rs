#![allow(clippy::exit)]

use std::hint::black_box;
use std::rc::Rc;

use gungraun::{library_benchmark, library_benchmark_group, main};
use slang_solidity::compilation::CompilationUnit;
use solidity_testing_perf_cargo::config::default_benchmark_config;
use solidity_testing_perf_cargo::dataset::SolidityProject;
use solidity_testing_perf_cargo::tests;
use solidity_testing_perf_cargo::tests::slang::binder_v2_run::BuiltSemanticAnalysis;
use solidity_testing_perf_cargo::tests::slang::bindings_build::BuiltBindingGraph;

mod __dependencies_used_in_lib__ {
    use anyhow as _;
    use infra_utils as _;
    use semver as _;
    use serde as _;
    use serde_json as _;
    use slang_solidity as _;
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
 * `{file_name}::{group_name}::{function_name} <project>:"<project>"`.
 * Changing any of the above would change the resulting benchmark ID, and
 * disconnect it from previous results.
 *
 * __SLANG_INFRA_BENCHMARKS_LIST__ (keep in sync)
 */

// Single source for the project list used by every stage benchmark below.
// Edit this macro (and only this macro) to add or remove a project.
// __SLANG_INFRA_PROJECT_LIST__ (keep in sync)
macro_rules! projects {
    (
        #[$lb:meta]
        $($rest:tt)*
    ) => {
        #[$lb]
        #[bench::weighted_pool("weighted_pool")]
        #[bench::merkle_proof("merkle_proof")]
        $($rest)*
    };
}

projects! {
    #[library_benchmark(setup = tests::slang::parser::setup)]
    fn parser(project: &SolidityProject) -> Rc<CompilationUnit> {
        black_box(tests::slang::parser::run(black_box(project)))
    }
}

projects! {
    #[library_benchmark(setup = tests::slang::cursor::setup)]
    fn cursor(unit: Rc<CompilationUnit>) -> Rc<CompilationUnit> {
        black_box(tests::slang::cursor::run(black_box(unit)))
    }
}

projects! {
    #[library_benchmark(setup = tests::slang::query::setup)]
    fn query(unit: Rc<CompilationUnit>) -> Rc<CompilationUnit> {
        black_box(tests::slang::query::run(black_box(unit)))
    }
}

projects! {
    #[library_benchmark(setup = tests::slang::bindings_build::setup)]
    fn bindings_build(unit: Rc<CompilationUnit>) -> BuiltBindingGraph {
        black_box(tests::slang::bindings_build::run(black_box(unit)))
    }
}

projects! {
    #[library_benchmark(setup = tests::slang::bindings_resolve::setup)]
    fn bindings_resolve(unit: BuiltBindingGraph) -> BuiltBindingGraph {
        black_box(tests::slang::bindings_resolve::run(black_box(unit)))
    }
}

projects! {
    #[library_benchmark(setup = tests::slang::bindings_resolve::setup)]
    fn cleanup(unit: BuiltBindingGraph) {
        black_box(unit);
    }
}

projects! {
    #[library_benchmark(setup = tests::slang::binder_v2_run::setup)]
    fn binder_v2_run(unit: Rc<CompilationUnit>) -> BuiltSemanticAnalysis {
        black_box(tests::slang::binder_v2_run::run(black_box(unit)))
    }
}

projects! {
    #[library_benchmark(setup = tests::slang::binder_v2_cleanup::setup)]
    fn binder_v2_cleanup(unit: BuiltSemanticAnalysis) {
        black_box(unit);
    }
}

library_benchmark_group!(
    name = pipeline;
    // __SLANG_INFRA_BENCHMARKS_LIST__ (keep in sync)
    benchmarks =
        parser,
        cursor,
        query,
        bindings_build,
        bindings_resolve,
        cleanup,
        binder_v2_run,
        binder_v2_cleanup,
);

main!(
    config = default_benchmark_config();
    library_benchmark_groups = pipeline,
);

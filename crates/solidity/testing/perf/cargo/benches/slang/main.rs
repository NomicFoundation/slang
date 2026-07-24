#![allow(clippy::exit)]

use std::hint::black_box;
use std::rc::Rc;

use gungraun::{library_benchmark, library_benchmark_group, main};
use slang_solidity::compilation::CompilationUnit;
use solidity_testing_perf_cargo::config::benchmark_config_with_num_callers;
use solidity_testing_perf_cargo::dataset::SolidityProject;
use solidity_testing_perf_cargo::tests;
use solidity_testing_perf_cargo::tests::slang::binder_v2_run::BuiltSemanticAnalysis;
use solidity_testing_perf_cargo::tests::slang::bindings_build::BuiltBindingGraph;
// Local aliases for the setup functions, so the generated benchmark ID reads
// `parser_setup("weighted_pool")` instead of `tests :: slang :: parser :: setup("weighted_pool")`.
use tests::slang::binder_v2_cleanup::setup as binder_v2_cleanup_setup;
use tests::slang::binder_v2_run::setup as binder_v2_run_setup;
use tests::slang::bindings_build::setup as bindings_build_setup;
use tests::slang::bindings_resolve::setup as bindings_resolve_setup;
use tests::slang::cursor::setup as cursor_setup;
use tests::slang::parser::setup as parser_setup;
use tests::slang::query::setup as query_setup;

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
 * __SLANG_INFRA_BENCHMARKS_LIST__ (keep in sync)
 */

// Single source for the project list used by every stage benchmark below.
// Edit this macro (and only this macro) to add or remove a project.
// __SLANG_INFRA_PROJECT_LIST__ (keep in sync)
macro_rules! bench_projects {
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

mod cst_group {
    use super::{
        CompilationUnit, Rc, SolidityProject, black_box, cursor_setup, library_benchmark,
        library_benchmark_group, parser_setup, query_setup, tests,
    };

    bench_projects! {
        #[library_benchmark(setup = parser_setup)]
        pub fn parser(project: &SolidityProject) -> Rc<CompilationUnit> {
            black_box(tests::slang::parser::run(black_box(project)))
        }
    }

    bench_projects! {
        #[library_benchmark(setup = cursor_setup)]
        pub fn cursor(unit: Rc<CompilationUnit>) -> Rc<CompilationUnit> {
            black_box(tests::slang::cursor::run(black_box(unit)))
        }
    }

    bench_projects! {
        #[library_benchmark(setup = query_setup)]
        pub fn query(unit: Rc<CompilationUnit>) -> Rc<CompilationUnit> {
            black_box(tests::slang::query::run(black_box(unit)))
        }
    }

    library_benchmark_group!(
        name = cst;
        // __SLANG_INFRA_BENCHMARKS_LIST__ (keep in sync)
        benchmarks = parser, cursor, query,
    );
}
use cst_group::cst;

mod bindings_group {
    use super::{
        BuiltBindingGraph, CompilationUnit, Rc, bindings_build_setup, bindings_resolve_setup,
        black_box, library_benchmark, library_benchmark_group, tests,
    };

    bench_projects! {
        #[library_benchmark(setup = bindings_build_setup)]
        pub fn build(unit: Rc<CompilationUnit>) -> BuiltBindingGraph {
            black_box(tests::slang::bindings_build::run(black_box(unit)))
        }
    }

    bench_projects! {
        #[library_benchmark(setup = bindings_resolve_setup)]
        pub fn resolve(unit: BuiltBindingGraph) -> BuiltBindingGraph {
            black_box(tests::slang::bindings_resolve::run(black_box(unit)))
        }
    }

    bench_projects! {
        #[library_benchmark(setup = bindings_resolve_setup)]
        pub fn cleanup(unit: BuiltBindingGraph) {
            black_box(unit);
        }
    }

    library_benchmark_group!(
        name = bindings;
        // __SLANG_INFRA_BENCHMARKS_LIST__ (keep in sync)
        benchmarks = build, resolve, cleanup,
    );
}
use bindings_group::bindings;

mod v2_binder_group {
    use super::{
        BuiltSemanticAnalysis, CompilationUnit, Rc, binder_v2_cleanup_setup, binder_v2_run_setup,
        black_box, library_benchmark, library_benchmark_group, tests,
    };

    bench_projects! {
        #[library_benchmark(setup = binder_v2_run_setup)]
        pub fn run(unit: Rc<CompilationUnit>) -> BuiltSemanticAnalysis {
            black_box(tests::slang::binder_v2_run::run(black_box(unit)))
        }
    }

    bench_projects! {
        #[library_benchmark(setup = binder_v2_cleanup_setup)]
        pub fn cleanup(unit: BuiltSemanticAnalysis) {
            black_box(unit);
        }
    }

    library_benchmark_group!(
        name = v2_binder;
        // __SLANG_INFRA_BENCHMARKS_LIST__ (keep in sync)
        benchmarks = run, cleanup,
    );
}
use v2_binder_group::v2_binder;

main!(
    // Slang v1 is quite slow, so we use a smaller `num-callers` value
    // and live with not so accurate DHAT measurements.
    config = benchmark_config_with_num_callers(12);
    library_benchmark_groups = cst, bindings, v2_binder,
);

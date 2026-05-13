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
        black_box, library_benchmark, library_benchmark_group, tests, CompilationUnit, Rc,
        SolidityProject,
    };

    bench_projects! {
        #[library_benchmark(setup = tests::slang::parser::setup)]
        pub fn parser(project: &SolidityProject) -> Rc<CompilationUnit> {
            black_box(tests::slang::parser::run(black_box(project)))
        }
    }

    bench_projects! {
        #[library_benchmark(setup = tests::slang::cursor::setup)]
        pub fn cursor(unit: Rc<CompilationUnit>) -> Rc<CompilationUnit> {
            black_box(tests::slang::cursor::run(black_box(unit)))
        }
    }

    bench_projects! {
        #[library_benchmark(setup = tests::slang::query::setup)]
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
        black_box, library_benchmark, library_benchmark_group, tests, BuiltBindingGraph,
        CompilationUnit, Rc,
    };

    bench_projects! {
        #[library_benchmark(setup = tests::slang::bindings_build::setup)]
        pub fn build(unit: Rc<CompilationUnit>) -> BuiltBindingGraph {
            black_box(tests::slang::bindings_build::run(black_box(unit)))
        }
    }

    bench_projects! {
        #[library_benchmark(setup = tests::slang::bindings_resolve::setup)]
        pub fn resolve(unit: BuiltBindingGraph) -> BuiltBindingGraph {
            black_box(tests::slang::bindings_resolve::run(black_box(unit)))
        }
    }

    bench_projects! {
        #[library_benchmark(setup = tests::slang::bindings_resolve::setup)]
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
        black_box, library_benchmark, library_benchmark_group, tests, BuiltSemanticAnalysis,
        CompilationUnit, Rc,
    };

    bench_projects! {
        #[library_benchmark(setup = tests::slang::binder_v2_run::setup)]
        pub fn run(unit: Rc<CompilationUnit>) -> BuiltSemanticAnalysis {
            black_box(tests::slang::binder_v2_run::run(black_box(unit)))
        }
    }

    bench_projects! {
        #[library_benchmark(setup = tests::slang::binder_v2_cleanup::setup)]
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
    config = default_benchmark_config();
    library_benchmark_groups = cst, bindings, v2_binder,
);

#![allow(clippy::exit)]

use std::hint::black_box;
use std::rc::Rc;

use gungraun::{library_benchmark, library_benchmark_group, main};
use paste::paste;
use slang_solidity::compilation::CompilationUnit;
use slang_solidity_v2_cst::structured_cst::nodes::SourceUnit;
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

macro_rules! slang_test {
    ($prj:ident) => {
        paste! {
            #[library_benchmark(setup = tests::setup::setup)]
            #[bench::test(stringify!($prj))]
            pub fn [<slang_ $prj>](project: &SolidityProject) -> Rc<CompilationUnit> {
                black_box(tests::slang::parser::run(black_box(project)))
            }
        }
    };
}

macro_rules! solar_test {
    ($prj:ident) => {
        paste! {
            #[library_benchmark(setup = tests::setup::setup)]
            #[bench::test(stringify!($prj))]
            pub fn [<solar_ $prj>](project: &SolidityProject) {
                black_box(tests::solar::parser::run(black_box(project)));
            }
        }
    };
}

macro_rules! tree_sitter_test {
    ($prj:ident) => {
        paste! {
            #[library_benchmark(setup = tests::setup::setup)]
            #[bench::test(stringify!($prj))]
            pub fn [<tree_sitter_ $prj>](project: &SolidityProject) -> Vec<tree_sitter::Tree> {
                black_box(tests::tree_sitter::parser::run(black_box(project)))
            }
        }
    };
}

macro_rules! slang_v2_test {
    ($prj:ident) => {
        paste! {
            #[library_benchmark(setup = tests::setup::setup)]
            #[bench::test(stringify!($prj))]
            pub fn [<slang_v2_ $prj>](project: &SolidityProject) -> Vec<(String, SourceUnit)> {
                black_box(tests::slang_v2::parser::run(black_box(project)))
            }
        }
    };
}

// Some projects can't be parsed by tree-sitter, so we test them only in slang and solar.
// This macro abstracts that logic.
macro_rules! slang_and_solar_tests {
    ($prj:ident) => {
        slang_test!($prj);
        slang_v2_test!($prj);
        solar_test!($prj);
        paste! {
            library_benchmark_group!(
              name = [< $prj _group >];
              benchmarks = [< slang_ $prj >],[< slang_v2_ $prj >],[< solar_ $prj >],
          );
        }
    };
}

/*
 * WARNING:
 * The reported `gungraun` benchmark ID is constructed from: `{file_name}::{group_name}::{function_name}`
 * Changing any of the above would change the resulting benchmark ID, and disconnect it from previous results.
 */
macro_rules! comparison_tests {
    (mooniswap) => {
        // Incompatible with solar or slang v2
        slang_test!(mooniswap);
        tree_sitter_test!(mooniswap);
        library_benchmark_group!(
            name = mooniswap_group;
            benchmarks = slang_mooniswap,tree_sitter_mooniswap,
        );
    };
    (weighted_pool) => {
        // Incompatible with slang v2 (Solidity 0.7.1)
        slang_test!(weighted_pool);
        solar_test!(weighted_pool);
        tree_sitter_test!(weighted_pool);
        paste! {
          library_benchmark_group!(
              name = weighted_pool_group;
              benchmarks = slang_weighted_pool,solar_weighted_pool,tree_sitter_weighted_pool,
          );
        }
    };
    (uniswap) => {
        slang_and_solar_tests!(uniswap);
    };
    (create_x) => {
        slang_and_solar_tests!(create_x);
    };
    (pointer_libraries) => {
        slang_and_solar_tests!(pointer_libraries);
    };
    ($prj:ident) => {
        slang_test!($prj);
        slang_v2_test!($prj);
        solar_test!($prj);
        tree_sitter_test!($prj);
        paste! {
          library_benchmark_group!(
              name = [< $prj _group >];
              benchmarks = [< slang_ $prj >],[< slang_v2_ $prj >],[< solar_ $prj >],[< tree_sitter_ $prj >],
          );
        }
    };
}

// __SLANG_INFRA_PROJECT_LIST__ (keep in sync)
comparison_tests!(mooniswap);
comparison_tests!(weighted_pool);
comparison_tests!(uniswap);
comparison_tests!(multicall3);
comparison_tests!(create_x);
comparison_tests!(ui_pool_data_provider_v3);
comparison_tests!(cooldogs);
comparison_tests!(one_step_leverage_f);
comparison_tests!(pointer_libraries);
comparison_tests!(merkle_proof);

main!(
    config = default_benchmark_config();

    // NOTE: the trailing comma is required: without it, it won't test the last one
    // __SLANG_INFRA_PROJECT_LIST__ (keep in sync)
    library_benchmark_groups =
        mooniswap_group,
        weighted_pool_group,
        uniswap_group,
        multicall3_group,
        create_x_group,
        ui_pool_data_provider_v3_group,
        cooldogs_group,
        one_step_leverage_f_group,
        pointer_libraries_group,
        merkle_proof_group,
);

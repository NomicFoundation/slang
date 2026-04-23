#![allow(clippy::needless_pass_by_value)]

pub mod dataset;
pub mod tests;

mod __dependencies_used_in_benches__ {
    use iai_callgrind as _;
    use infra_utils as _;
    use paste as _;
}

#[cfg(test)]
mod unit_tests {
    // Whitness project to test that the crates are working properly.
    // It can be any that can be parsed by all of the libraries being tested.
    const PROJECT_TO_TEST: &str = "ui_pool_data_provider_v3";
    // Sum of the contracts, interfaces, and libraries in all of the files of the project.
    const CONTRACT_COUNT: usize = 25;
    // Sum of the identifiers resolved by the binder.
    const IDENTIFIER_COUNT: usize = 2829;

    mod slang {
        macro_rules! define_payload_test {
            ($test_phase:ident) => {
                #[test]
                fn $test_phase() {
                    let payload = crate::tests::slang::$test_phase::setup(super::PROJECT_TO_TEST);
                    crate::tests::slang::$test_phase::test(payload);
                }
            };
        }

        macro_rules! define_payload_test_and_assert_count_eq {
            ($test_phase:ident, $value:expr) => {
                #[test]
                fn $test_phase() {
                    let payload = crate::tests::slang::$test_phase::setup(super::PROJECT_TO_TEST);
                    let value = crate::tests::slang::$test_phase::test(payload);
                    assert_eq!(value, $value);
                }
            };
        }
        /*
         * __SLANG_INFRA_BENCHMARKS_LIST__ (keep in sync)
         */
        define_payload_test!(parser);
        define_payload_test_and_assert_count_eq!(cursor, super::CONTRACT_COUNT);
        define_payload_test_and_assert_count_eq!(query, super::CONTRACT_COUNT);
        define_payload_test!(bindings_build);
        define_payload_test_and_assert_count_eq!(bindings_resolve, super::IDENTIFIER_COUNT);
        define_payload_test_and_assert_count_eq!(binder_v2_run, super::IDENTIFIER_COUNT);
    }

    mod slang_v2 {
        // __SLANG_V2_INFRA_BENCHMARKS_LIST__ (keep in sync)

        #[test]
        fn parser() {
            let project = crate::tests::slang_v2::parser::setup(super::PROJECT_TO_TEST);
            let source_units = crate::tests::slang_v2::parser::test(project);
            let contract_count = crate::tests::slang_v2::parser::count_contracts(&source_units);
            assert_eq!(contract_count, super::CONTRACT_COUNT);
        }

        #[test]
        fn ir_builder() {
            let (project, source_units) =
                crate::tests::slang_v2::ir_builder::setup(super::PROJECT_TO_TEST);
            let (_, ir_source_units) =
                crate::tests::slang_v2::ir_builder::test(project, source_units);
            let ir_contract_count =
                crate::tests::slang_v2::ir_builder::count_contracts(&ir_source_units);
            assert_eq!(ir_contract_count, super::CONTRACT_COUNT);
        }

        #[test]
        fn semantic() {
            let (project, interner, input_files) =
                crate::tests::slang_v2::semantic::setup(super::PROJECT_TO_TEST);
            let semantic_context =
                crate::tests::slang_v2::semantic::test(project, interner, input_files);
            let semantic_contract_count =
                crate::tests::slang_v2::semantic::count_contracts(&semantic_context);
            assert_eq!(semantic_contract_count, super::CONTRACT_COUNT);
        }
    }

    mod solar {
        #[test]
        fn parser() {
            let payload = crate::tests::setup::setup(super::PROJECT_TO_TEST);
            let contract_count = crate::tests::solar::parser::test(payload);
            assert_eq!(contract_count, super::CONTRACT_COUNT);
        }
    }

    mod tree_sitter {
        #[test]
        fn parser() {
            let payload = crate::tests::setup::setup(super::PROJECT_TO_TEST);
            let contract_count = crate::tests::tree_sitter::parser::test(payload);
            assert_eq!(contract_count, super::CONTRACT_COUNT);
        }
    }
}

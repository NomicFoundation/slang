#![allow(clippy::needless_pass_by_value)]

pub mod dataset;
pub mod tests;
pub mod tests_v2;

mod __dependencies_used_in_benches__ {
    use {iai_callgrind as _, infra_utils as _, paste as _};
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
                    let payload = crate::tests::$test_phase::setup(super::PROJECT_TO_TEST);
                    crate::tests::$test_phase::test(payload);
                }
            };
        }

        macro_rules! define_payload_test_and_assert_count_eq {
            ($test_phase:ident, $value:expr) => {
                #[test]
                fn $test_phase() {
                    let payload = crate::tests::$test_phase::setup(super::PROJECT_TO_TEST);
                    let value = crate::tests::$test_phase::test(payload);
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
        #[test]
        fn parser() {
            // __SLANG_V2_INFRA_BENCHMARKS_LIST__ (keep in sync)

            let payload = crate::tests::setup::setup(super::PROJECT_TO_TEST);
            let source_units = crate::tests_v2::parser::test(payload);
            let contract_count = crate::tests_v2::parser::count_contracts(&source_units);
            assert_eq!(contract_count, super::CONTRACT_COUNT);

            let ir_source_units = crate::tests_v2::ir_builder::test(source_units);
            let ir_contract_count = crate::tests_v2::ir_builder::count_contracts(&ir_source_units);
            assert_eq!(ir_contract_count, super::CONTRACT_COUNT);
        }
    }

    mod solar {
        #[test]
        fn parser() {
            let payload = crate::tests::setup::setup(super::PROJECT_TO_TEST);
            let contract_count = crate::tests::solar_parser::test(payload);
            assert_eq!(contract_count, super::CONTRACT_COUNT);
        }
    }

    mod tree_sitter {
        #[test]
        fn parser() {
            let payload = crate::tests::setup::setup(super::PROJECT_TO_TEST);
            let contract_count = crate::tests::tree_sitter_parser::test(payload);
            assert_eq!(contract_count, super::CONTRACT_COUNT);
        }
    }
}

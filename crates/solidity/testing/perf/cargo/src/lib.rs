#![allow(clippy::needless_pass_by_value)]

mod compilation_builder;
pub mod dataset;
pub mod tests;

mod __dependencies_used_in_benches__ {
    use {iai_callgrind as _, infra_utils as _, paste as _};
}

#[cfg(test)]
mod unit_tests {
    const PROJECT_TO_TEST: &str = "protocol_ui_pool_data_provider_v3";
    const CONTRACT_COUNT: usize = 25;
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
        define_payload_test_and_assert_count_eq!(bindings_resolve, 2829);
    }

    mod solar {
        #[test]
        fn parser() {
            let payload = crate::tests::setup::setup(super::PROJECT_TO_TEST);
            let contract_count = crate::tests::solar_parser::test(payload);
            assert_eq!(contract_count, super::IDENTIFIER_COUNT);
        }
    }

    mod tree_sitter {
        #[test]
        fn parser() {
            let payload = crate::tests::setup::setup(super::PROJECT_TO_TEST);
            let contract_count = crate::tests::tree_sitter_parser::test(payload);
            assert_eq!(contract_count, 25);
        }
    }
}

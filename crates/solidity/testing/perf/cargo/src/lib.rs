#![allow(clippy::needless_pass_by_value)]

pub mod dataset;
pub mod tests;

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

    mod v2_parser {
        // V2 parser only supports 0.8.30; uniswap (0.8.26) is the closest available.
        // Sources are preprocessed to strip pragma and assembly (not yet supported).
        const V2_PROJECT: &str = "uniswap_v2";

        #[test]
        fn parser() {
            let payload = crate::tests::setup::setup(V2_PROJECT);
            crate::tests::v2_parser::test(payload);
        }

        #[test]
        fn weighted_pool_parser() {
            let payload = crate::tests::setup::setup("weighted_pool_v2");
            crate::tests::v2_parser::test(payload);
        }

        #[test]
        fn create_x_parser() {
            let payload = crate::tests::setup::setup("create_x_v2");
            crate::tests::v2_parser::test(payload);
        }
    }
}

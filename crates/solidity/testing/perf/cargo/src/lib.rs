#![allow(clippy::exit)]
#![allow(clippy::needless_pass_by_value)]

mod compilation_builder;
pub mod dataset;
pub mod tests;

mod __dependencies_used_in_benches__ {
    use {iai_callgrind as _, infra_utils as _, paste as _};
}

#[cfg(test)]
mod unit_tests {
    mod slang {
        macro_rules! slang_define_payload_test {
            ($test_phase:ident, $prj:ident) => {
                #[test]
                fn $test_phase() {
                    let payload = crate::tests::$test_phase::setup(stringify!($prj));
                    crate::tests::$test_phase::test(payload);
                }
            };
        }

        macro_rules! slang_define_payload_test_and_assert {
            ($test_phase:ident, $prj:ident, $value:expr) => {
                #[test]
                fn $test_phase() {
                    let payload = crate::tests::$test_phase::setup(stringify!($prj));
                    let value = crate::tests::$test_phase::test(payload);
                    assert_eq!(value, $value);
                }
            };
        }
        /*
         * __SLANG_INFRA_BENCHMARKS_LIST__ (keep in sync)
         */
        macro_rules! slang_define_tests {
            ($prj:ident) => {
                mod $prj {
                    slang_define_payload_test!(parser, $prj);
                    slang_define_payload_test_and_assert!(cursor, $prj, 25);
                    slang_define_payload_test_and_assert!(query, $prj, 25);
                    slang_define_payload_test!(bindings_build, $prj);
                    slang_define_payload_test_and_assert!(bindings_resolve, $prj, 2829);
                }
            };
        }

        slang_define_tests!(protocol_ui_pool_data_provider_v3);
    }

    mod solar {
        macro_rules! solar_define_tests {
            ($prj:ident) => {
                #[test]
                fn $prj() {
                    let payload = crate::tests::setup::setup(stringify!($prj));
                    let contract_count = crate::tests::solar_parser::test(payload, true);
                    assert_eq!(contract_count, 25);
                }
            };
        }

        solar_define_tests!(protocol_ui_pool_data_provider_v3);
    }

    mod tree_sitter {
        macro_rules! tree_sitter_define_tests {
            ($prj:ident) => {
                #[test]
                fn $prj() {
                    let payload = crate::tests::setup::setup(stringify!($prj));
                    let contract_count = crate::tests::tree_sitter_parser::test(payload, true);
                    assert_eq!(contract_count, 25);
                }
            };
        }

        tree_sitter_define_tests!(protocol_ui_pool_data_provider_v3);
    }
}

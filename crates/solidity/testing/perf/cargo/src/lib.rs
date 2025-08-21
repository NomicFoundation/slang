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
            ($name:ident, $prj:expr) => {
                #[test]
                fn $name() {
                    let payload = crate::tests::$name::setup($prj);
                    crate::tests::$name::run(payload);
                }
            };
        }
        /*
         * __SLANG_INFRA_BENCHMARKS_LIST__ (keep in sync)
         */
        macro_rules! slang_define_payload_tests {
            ($prj:ident, $name:tt) => {
                mod $prj {
                    slang_define_payload_test!(parser, $name);
                    slang_define_payload_test!(cursor, $name);
                    slang_define_payload_test!(query, $name);
                    slang_define_payload_test!(bindings_build, $name);
                    slang_define_payload_test!(bindings_resolve, $name);
                }
            };
        }

        include!("slang_benches_list.rs");
    }
    mod solar {
        macro_rules! solar_define_payload_test {
            ($name:ident, $prj:expr) => {
                #[test]
                fn $name() {
                    let payload = crate::tests::setup::setup($prj);
                    crate::tests::solar_parser::run(payload);
                }
            };
        }

        include!("solar_benches_list.rs");
    }

    mod tree_sitter {
        macro_rules! tree_sitter_define_payload_test {
            ($name:ident, $prj:expr) => {
                #[test]
                fn $name() {
                    let payload = crate::tests::setup::setup($prj);
                    crate::tests::tree_sitter_parser::run(payload);
                }
            };
        }

        include!("tree_sitter_benches_list.rs");
    }
}

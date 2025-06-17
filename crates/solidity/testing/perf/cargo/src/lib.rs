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

    mod solang {
        macro_rules! solang_define_payload_test {
            ($name:ident, $prj:expr) => {
                #[test]
                fn $name() {
                    let payload = crate::tests::solang::setup($prj);
                    crate::tests::solang::run(payload);
                }
            };
        }

        include!("solang_benches_list.rs");
    }

    include!("benches_list.rs");
}

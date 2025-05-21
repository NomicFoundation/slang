#![allow(clippy::exit)]
#![allow(clippy::needless_pass_by_value)]

mod compilation_builder;
pub mod dataset;
mod import_resolver;
pub mod tests;

mod __dependencies_used_in_benches__ {
    use {iai_callgrind as _, paste as _};
}

#[cfg(test)]
mod unit_tests {
    macro_rules! define_payload_test {
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
    macro_rules! define_payload_tests {
        ($prj:ident, $name:tt) => {
            mod $prj {
                define_payload_test!(parser, $name);
                define_payload_test!(cursor, $name);
                define_payload_test!(query, $name);
                define_payload_test!(bindings_build, $name);
                define_payload_test!(bindings_resolve, $name);
            }
        };
    }

    include!("benches_list.rs");
}

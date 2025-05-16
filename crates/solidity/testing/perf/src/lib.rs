#![allow(clippy::exit)]
#![allow(clippy::needless_pass_by_value)]

mod compilation_builder;
pub mod dataset;
mod import_resolver;
pub mod tests;

#[cfg(test)]
mod __dependencies_used_in_benches__ {
    use iai_callgrind as _;
}

#[cfg(test)]
mod unit_tests {
    macro_rules! define_payload_test {
        ($name:ident) => {
            #[test]
            fn $name() {
                let payload = crate::tests::$name::setup();
                crate::tests::$name::run(payload);
            }
        };
    }

    /*
     * __SLANG_INFRA_BENCHMARKS_LIST__ (keep in sync)
     */
    define_payload_test!(parser);
    define_payload_test!(cursor);
    define_payload_test!(query);
    define_payload_test!(bindings_build);
    define_payload_test!(bindings_resolve);
}

#![allow(clippy::exit)]
#![allow(clippy::needless_pass_by_value)]

pub mod dataset;
pub mod tests;

#[cfg(test)]
mod __dependencies_used_in_benches__ {
    use iai_callgrind as _;
}

#[cfg(test)]
mod unit_tests {
    macro_rules! define_test {
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
    define_test!(parser);
    define_test!(cursor);
    define_test!(query);
    define_test!(init_bindings);
    define_test!(definitions);
    define_test!(references);
}

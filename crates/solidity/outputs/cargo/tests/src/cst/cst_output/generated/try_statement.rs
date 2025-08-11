// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use anyhow::Result;

use crate::cst::cst_output::runner::run;

const T: &str = "TryStatement";

#[test]
fn method_call() -> Result<()> {
    run(T, "method_call")
}

#[test]
fn method_call_with_body() -> Result<()> {
    run(T, "method_call_with_body")
}

#[test]
fn method_call_with_options() -> Result<()> {
    run(T, "method_call_with_options")
}

#[test]
fn method_call_with_options_and_body() -> Result<()> {
    run(T, "method_call_with_options_and_body")
}

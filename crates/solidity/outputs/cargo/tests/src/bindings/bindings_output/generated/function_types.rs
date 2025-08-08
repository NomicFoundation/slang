// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use anyhow::Result;

use crate::bindings::bindings_output::runner::run;

const T: &str = "function_types";

#[test]
fn args_return_types() -> Result<()> {
    run(T, "args_return_types")
}

#[test]
fn call() -> Result<()> {
    run(T, "call")
}

#[test]
fn externals() -> Result<()> {
    run(T, "externals")
}

#[test]
fn reference() -> Result<()> {
    run(T, "reference")
}

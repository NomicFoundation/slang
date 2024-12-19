// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use anyhow::Result;

use crate::bindings_output::runner::run;

#[test]
fn args_return_types() -> Result<()> {
    run("function_types", "args_return_types")
}

#[test]
fn call() -> Result<()> {
    run("function_types", "call")
}

#[test]
fn externals() -> Result<()> {
    run("function_types", "externals")
}

#[test]
fn reference() -> Result<()> {
    run("function_types", "reference")
}

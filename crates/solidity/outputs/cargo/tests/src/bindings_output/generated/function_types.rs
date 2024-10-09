// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use anyhow::Result;

use crate::bindings_output::runner::run;

#[test]
fn call() -> Result<()> {
    run("function_types", "call")
}

#[test]
fn reference() -> Result<()> {
    run("function_types", "reference")
}

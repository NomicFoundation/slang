// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use anyhow::Result;

use crate::bindings_output::runner::run;

#[test]
fn functions() -> Result<()> {
    run("yul", "functions")
}

#[test]
fn variables() -> Result<()> {
    run("yul", "variables")
}

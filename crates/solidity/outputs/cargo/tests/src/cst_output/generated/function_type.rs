// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use anyhow::Result;

use crate::cst_output::runner::run;

#[test]
fn basic() -> Result<()> {
    run("FunctionType", "basic")
}

#[test]
fn pure_state_mutability() -> Result<()> {
    run("FunctionType", "pure_state_mutability")
}

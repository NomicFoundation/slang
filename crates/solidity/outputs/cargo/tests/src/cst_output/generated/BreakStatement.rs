// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use anyhow::Result;

use crate::cst_output::runner::run;

#[test]
fn error_recovery() -> Result<()> {
    run("BreakStatement", "error_recovery")
}

#[test]
fn valid() -> Result<()> {
    run("BreakStatement", "valid")
}

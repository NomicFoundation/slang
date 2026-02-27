// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use anyhow::Result;

use crate::cst::cst_output::runner::run;

const T: &str = "BreakStatement";

#[test]
fn error_recovery() -> Result<()> {
    run(T, "error_recovery")
}

#[test]
fn valid() -> Result<()> {
    run(T, "valid")
}

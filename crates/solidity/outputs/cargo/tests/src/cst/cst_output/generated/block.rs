// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use anyhow::Result;

use crate::cst::cst_output::runner::run;

const T: &str = "Block";

#[test]
fn postfix_recovery_regression() -> Result<()> {
    run(T, "postfix_recovery_regression")
}

#[test]
fn unchecked() -> Result<()> {
    run(T, "unchecked")
}

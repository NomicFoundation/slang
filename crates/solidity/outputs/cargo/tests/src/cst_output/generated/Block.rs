// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use anyhow::Result;

use crate::cst_output::runner::run;

#[test]
fn postfix_recovery_regression() -> Result<()> {
    run("Block", "postfix_recovery_regression")
}

#[test]
fn unchecked() -> Result<()> {
    run("Block", "unchecked")
}

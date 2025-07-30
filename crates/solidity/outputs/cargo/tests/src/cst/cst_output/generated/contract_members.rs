// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use anyhow::Result;

use crate::cst::cst_output::runner::run;

const T: &str = "ContractMembers";

#[test]
fn constructor() -> Result<()> {
    run(T, "constructor")
}

#[test]
fn local_expression() -> Result<()> {
    run(T, "local_expression")
}

#[test]
fn mismatched_delimiter() -> Result<()> {
    run(T, "mismatched_delimiter")
}

#[test]
fn separated_recovery() -> Result<()> {
    run(T, "separated_recovery")
}

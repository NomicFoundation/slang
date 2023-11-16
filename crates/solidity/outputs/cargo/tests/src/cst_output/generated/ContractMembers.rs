// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use crate::cst_output::runner::run;
use anyhow::Result;

#[test]
fn constructor() -> Result<()> {
    return run("ContractMembers", "constructor");
}

#[test]
fn local_expression() -> Result<()> {
    return run("ContractMembers", "local_expression");
}

#[test]
fn mismatched_delimiter() -> Result<()> {
    return run("ContractMembers", "mismatched_delimiter");
}

#[test]
fn separated_recovery() -> Result<()> {
    return run("ContractMembers", "separated_recovery");
}

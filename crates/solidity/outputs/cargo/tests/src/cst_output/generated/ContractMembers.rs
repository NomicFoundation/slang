// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use anyhow::Result;

use crate::cst_output::runner::run;

#[test]
fn constructor() -> Result<()> {
    run("ContractMembers", "constructor")
}

#[test]
fn local_expression() -> Result<()> {
    run("ContractMembers", "local_expression")
}

#[test]
fn mismatched_delimiter() -> Result<()> {
    run("ContractMembers", "mismatched_delimiter")
}

#[test]
fn separated_recovery() -> Result<()> {
    run("ContractMembers", "separated_recovery")
}

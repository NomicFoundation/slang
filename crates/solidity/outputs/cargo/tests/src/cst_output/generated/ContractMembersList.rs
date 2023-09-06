// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use crate::cst_output::runner::run;
use anyhow::Result;

#[test]
fn constructor() -> Result<()> {
    return run("ContractMembersList", "constructor");
}

#[test]
fn local_expression() -> Result<()> {
    return run("ContractMembersList", "local_expression");
}

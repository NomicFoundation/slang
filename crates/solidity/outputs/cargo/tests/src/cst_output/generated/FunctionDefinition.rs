// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use crate::cst_output::runner::run;
use anyhow::Result;

#[test]
fn constant_state_mutability() -> Result<()> {
    return run("FunctionDefinition", "constant_state_mutability");
}

#[test]
fn ierc20_approve() -> Result<()> {
    return run("FunctionDefinition", "ierc20_approve");
}

#[test]
fn ierc20_transferFrom() -> Result<()> {
    return run("FunctionDefinition", "ierc20_transferFrom");
}

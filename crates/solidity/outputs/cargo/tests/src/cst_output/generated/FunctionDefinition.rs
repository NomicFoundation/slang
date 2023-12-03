// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use crate::cst_output::runner::run;
use anyhow::Result;

#[test]
fn constant_state_mutability() -> Result<()> {
    run("FunctionDefinition", "constant_state_mutability")
}

#[test]
fn from_contextual_keyword() -> Result<()> {
    run("FunctionDefinition", "from_contextual_keyword")
}

#[test]
fn overridden() -> Result<()> {
    run("FunctionDefinition", "overridden")
}

// This file is @generated automatically by infrastructure scripts. Please don't edit by hand.
#![allow(clippy::all)]

use crate::cst_output::runner::run;
use anyhow::Result;

#[test]
fn constant_state_mutability() -> Result<()> {
    return run("FunctionDefinition", "constant_state_mutability");
}

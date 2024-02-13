// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use anyhow::Result;

use crate::cst_output::runner::run;

#[test]
fn internal_attribute() -> Result<()> {
    run("UnnamedFunctionDefinition", "internal_attribute")
}

#[test]
fn public_attribute() -> Result<()> {
    run("UnnamedFunctionDefinition", "public_attribute")
}

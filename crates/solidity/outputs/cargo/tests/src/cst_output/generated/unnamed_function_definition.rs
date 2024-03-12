// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use anyhow::Result;

use crate::cst_output::runner::run;

#[test]
fn constant_attribute() -> Result<()> {
    run("UnnamedFunctionDefinition", "constant_attribute")
}

#[test]
fn internal_attribute() -> Result<()> {
    run("UnnamedFunctionDefinition", "internal_attribute")
}

#[test]
fn private_attribute() -> Result<()> {
    run("UnnamedFunctionDefinition", "private_attribute")
}

#[test]
fn public_attribute() -> Result<()> {
    run("UnnamedFunctionDefinition", "public_attribute")
}

// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use anyhow::Result;

use crate::cst::cst_output::runner::run;

const T: &str = "UnnamedFunctionDefinition";

#[test]
fn constant_attribute() -> Result<()> {
    run(T, "constant_attribute")
}

#[test]
fn internal_attribute() -> Result<()> {
    run(T, "internal_attribute")
}

#[test]
fn private_attribute() -> Result<()> {
    run(T, "private_attribute")
}

#[test]
fn public_attribute() -> Result<()> {
    run(T, "public_attribute")
}

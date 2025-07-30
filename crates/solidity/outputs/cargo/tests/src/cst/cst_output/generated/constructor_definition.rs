// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use anyhow::Result;

use crate::cst::cst_output::runner::run;

const T: &str = "ConstructorDefinition";

#[test]
fn override_attribute() -> Result<()> {
    run(T, "override_attribute")
}

#[test]
fn simple() -> Result<()> {
    run(T, "simple")
}

#[test]
fn virtual_attribute() -> Result<()> {
    run(T, "virtual_attribute")
}

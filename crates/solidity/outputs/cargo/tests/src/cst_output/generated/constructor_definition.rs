// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use anyhow::Result;

use crate::cst_output::runner::run;

#[test]
fn override_attribute() -> Result<()> {
    run("ConstructorDefinition", "override_attribute")
}

#[test]
fn simple() -> Result<()> {
    run("ConstructorDefinition", "simple")
}

#[test]
fn virtual_attribute() -> Result<()> {
    run("ConstructorDefinition", "virtual_attribute")
}

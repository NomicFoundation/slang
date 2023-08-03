// This file is @generated automatically by infrastructure scripts. Please don't edit by hand.

use crate::cst_output::runner::run;
use anyhow::Result;

#[test]
fn multiple_members() -> Result<()> {
    return run("EnumDefinition", "multiple_members");
}

#[test]
fn no_members() -> Result<()> {
    return run("EnumDefinition", "no_members");
}

// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use anyhow::Result;

use crate::cst_output::runner::run;

#[test]
fn no_parens() -> Result<()> {
    run("EventDefinition", "no_parens")
}

#[test]
fn transfer() -> Result<()> {
    run("EventDefinition", "transfer")
}

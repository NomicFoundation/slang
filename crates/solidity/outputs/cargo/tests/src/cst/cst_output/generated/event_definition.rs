// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use anyhow::Result;

use crate::cst::cst_output::runner::run;

const T: &str = "EventDefinition";

#[test]
fn no_parens() -> Result<()> {
    run(T, "no_parens")
}

#[test]
fn transfer() -> Result<()> {
    run(T, "transfer")
}

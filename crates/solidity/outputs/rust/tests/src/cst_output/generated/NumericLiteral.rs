// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use crate::cst_output::runner::run;
use anyhow::Result;

#[test]
fn days_unit() -> Result<()> {
    return run("NumericLiteral", "days_unit");
}

#[test]
fn ether_unit() -> Result<()> {
    return run("NumericLiteral", "ether_unit");
}

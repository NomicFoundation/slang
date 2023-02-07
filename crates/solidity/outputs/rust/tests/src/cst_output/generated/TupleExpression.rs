// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use crate::cst_output::runner::run;
use anyhow::Result;

#[test]
fn empty() -> Result<()> {
    return run("TupleExpression", "empty");
}

#[test]
fn full() -> Result<()> {
    return run("TupleExpression", "full");
}

#[test]
fn missing_elements() -> Result<()> {
    return run("TupleExpression", "missing_elements");
}

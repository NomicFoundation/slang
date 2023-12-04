// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use crate::cst_output::runner::run;
use anyhow::Result;

#[test]
fn empty() -> Result<()> {
    run("TupleExpression", "empty")
}

#[test]
fn full() -> Result<()> {
    run("TupleExpression", "full")
}

#[test]
fn missing_elements() -> Result<()> {
    run("TupleExpression", "missing_elements")
}

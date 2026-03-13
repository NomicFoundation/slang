// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use anyhow::Result;

use crate::cst::cst_output::runner::run;

const T: &str = "TupleExpression";

#[test]
fn empty() -> Result<()> {
    run(T, "empty")
}

#[test]
fn full() -> Result<()> {
    run(T, "full")
}

#[test]
fn missing_elements() -> Result<()> {
    run(T, "missing_elements")
}

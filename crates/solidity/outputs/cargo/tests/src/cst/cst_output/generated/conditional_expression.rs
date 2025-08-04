// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use anyhow::Result;

use crate::cst::cst_output::runner::run;

const T: &str = "ConditionalExpression";

#[test]
fn identifier_base() -> Result<()> {
    run(T, "identifier_base")
}

#[test]
fn nested_base() -> Result<()> {
    run(T, "nested_base")
}

#[test]
fn nested_conditions() -> Result<()> {
    run(T, "nested_conditions")
}

#[test]
fn recursive() -> Result<()> {
    run(T, "recursive")
}

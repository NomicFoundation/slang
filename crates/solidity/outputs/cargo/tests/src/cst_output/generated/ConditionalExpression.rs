// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use crate::cst_output::runner::run;
use anyhow::Result;

#[test]
fn identifier_base() -> Result<()> {
    run("ConditionalExpression", "identifier_base")
}

#[test]
fn nested_base() -> Result<()> {
    run("ConditionalExpression", "nested_base")
}

#[test]
fn nested_conditions() -> Result<()> {
    run("ConditionalExpression", "nested_conditions")
}

#[test]
fn recursive() -> Result<()> {
    run("ConditionalExpression", "recursive")
}

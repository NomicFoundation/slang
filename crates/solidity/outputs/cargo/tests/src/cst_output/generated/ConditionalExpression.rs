// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use anyhow::Result;

use crate::cst_output::runner::run;

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

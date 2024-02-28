// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use anyhow::Result;

use crate::cst_output::runner::run;

#[test]
fn array_1d() -> Result<()> {
    run("NewExpression", "array_1d")
}

#[test]
fn array_1d_expression() -> Result<()> {
    run("NewExpression", "array_1d_expression")
}

#[test]
fn array_2d() -> Result<()> {
    run("NewExpression", "array_2d")
}

#[test]
fn identifier_path() -> Result<()> {
    run("NewExpression", "identifier_path")
}

// This file is @generated automatically by infrastructure scripts. Please don't edit by hand.

use crate::cst_output::runner::run;
use anyhow::Result;

#[test]
fn array_1d() -> Result<()> {
    return run("NewExpression", "array_1d");
}

#[test]
fn array_1d_expression() -> Result<()> {
    return run("NewExpression", "array_1d_expression");
}

#[test]
fn array_2d() -> Result<()> {
    return run("NewExpression", "array_2d");
}

#[test]
fn identifier_path() -> Result<()> {
    return run("NewExpression", "identifier_path");
}

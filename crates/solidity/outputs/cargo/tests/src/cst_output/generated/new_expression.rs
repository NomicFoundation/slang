// This file is generated automatically by infrastructure scripts (crates/codegen/testing/src/cst_output.rs:12:18). Please don't edit by hand.

use anyhow::Result;

use crate::cst_output::runner::run;

const T: &str = "NewExpression";

#[test]
fn array_1d() -> Result<()> {
    run(T, "array_1d")
}

#[test]
fn array_1d_expression() -> Result<()> {
    run(T, "array_1d_expression")
}

#[test]
fn array_2d() -> Result<()> {
    run(T, "array_2d")
}

#[test]
fn identifier_path() -> Result<()> {
    run(T, "identifier_path")
}

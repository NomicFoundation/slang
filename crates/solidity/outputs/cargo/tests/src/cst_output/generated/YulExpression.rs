// This file is @generated automatically by infrastructure scripts. Please don't edit by hand.

use crate::cst_output::runner::run;
use anyhow::Result;

#[test]
fn decimal_literal() -> Result<()> {
    return run("YulExpression", "decimal_literal");
}

#[test]
fn function_call() -> Result<()> {
    return run("YulExpression", "function_call");
}

#[test]
fn hex_literal() -> Result<()> {
    return run("YulExpression", "hex_literal");
}

#[test]
fn identifier_path() -> Result<()> {
    return run("YulExpression", "identifier_path");
}

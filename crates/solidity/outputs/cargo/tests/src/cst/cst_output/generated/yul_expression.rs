// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use anyhow::Result;

use crate::cst::cst_output::runner::run;

const T: &str = "YulExpression";

#[test]
fn decimal_literal() -> Result<()> {
    run(T, "decimal_literal")
}

#[test]
fn decimal_trailing_ident_start() -> Result<()> {
    run(T, "decimal_trailing_ident_start")
}

#[test]
fn false_keyword() -> Result<()> {
    run(T, "false_keyword")
}

#[test]
fn function_call() -> Result<()> {
    run(T, "function_call")
}

#[test]
fn hex_literal() -> Result<()> {
    run(T, "hex_literal")
}

#[test]
fn hex_trailing_ident_start() -> Result<()> {
    run(T, "hex_trailing_ident_start")
}

#[test]
fn identifier_path() -> Result<()> {
    run(T, "identifier_path")
}

#[test]
fn identifier_with_dot() -> Result<()> {
    run(T, "identifier_with_dot")
}

#[test]
fn true_keyword() -> Result<()> {
    run(T, "true_keyword")
}

// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use anyhow::Result;

use crate::cst_output::runner::run;

#[test]
fn hex_consecutive_underscores() -> Result<()> {
    run("HexNumberExpression", "hex_consecutive_underscores")
}

#[test]
fn hex_invalid_alpha_digit() -> Result<()> {
    run("HexNumberExpression", "hex_invalid_alpha_digit")
}

#[test]
fn hex_leading_underscore() -> Result<()> {
    run("HexNumberExpression", "hex_leading_underscore")
}

#[test]
fn hex_multiple_digits() -> Result<()> {
    run("HexNumberExpression", "hex_multiple_digits")
}

#[test]
fn hex_no_digits() -> Result<()> {
    run("HexNumberExpression", "hex_no_digits")
}

#[test]
fn hex_no_unit() -> Result<()> {
    run("HexNumberExpression", "hex_no_unit")
}

#[test]
fn hex_trailing_ident_start() -> Result<()> {
    run("HexNumberExpression", "hex_trailing_ident_start")
}

#[test]
fn hex_trailing_underscore() -> Result<()> {
    run("HexNumberExpression", "hex_trailing_underscore")
}

#[test]
fn hex_unit() -> Result<()> {
    run("HexNumberExpression", "hex_unit")
}

#[test]
fn hex_uppercase_prefix() -> Result<()> {
    run("HexNumberExpression", "hex_uppercase_prefix")
}

#[test]
fn hex_with_underscores() -> Result<()> {
    run("HexNumberExpression", "hex_with_underscores")
}

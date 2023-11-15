// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use crate::cst_output::runner::run;
use anyhow::Result;

#[test]
fn hex_consecutive_underscores() -> Result<()> {
    return run("HexNumberExpression", "hex_consecutive_underscores");
}

#[test]
fn hex_leading_underscore() -> Result<()> {
    return run("HexNumberExpression", "hex_leading_underscore");
}

#[test]
fn hex_multiple_digits() -> Result<()> {
    return run("HexNumberExpression", "hex_multiple_digits");
}

#[test]
fn hex_no_digits() -> Result<()> {
    return run("HexNumberExpression", "hex_no_digits");
}

#[test]
fn hex_no_unit() -> Result<()> {
    return run("HexNumberExpression", "hex_no_unit");
}

#[test]
fn hex_trailing_ident_start() -> Result<()> {
    return run("HexNumberExpression", "hex_trailing_ident_start");
}

#[test]
fn hex_trailing_underscore() -> Result<()> {
    return run("HexNumberExpression", "hex_trailing_underscore");
}

#[test]
fn hex_unit() -> Result<()> {
    return run("HexNumberExpression", "hex_unit");
}

#[test]
fn hex_uppercase_prefix() -> Result<()> {
    return run("HexNumberExpression", "hex_uppercase_prefix");
}

#[test]
fn hex_with_underscores() -> Result<()> {
    return run("HexNumberExpression", "hex_with_underscores");
}
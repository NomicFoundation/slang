// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use anyhow::Result;

use crate::cst::cst_output::runner::run;

const T: &str = "HexNumberExpression";

#[test]
fn hex_consecutive_underscores() -> Result<()> {
    run(T, "hex_consecutive_underscores")
}

#[test]
fn hex_invalid_alpha_digit() -> Result<()> {
    run(T, "hex_invalid_alpha_digit")
}

#[test]
fn hex_leading_underscore() -> Result<()> {
    run(T, "hex_leading_underscore")
}

#[test]
fn hex_multiple_digits() -> Result<()> {
    run(T, "hex_multiple_digits")
}

#[test]
fn hex_no_digits() -> Result<()> {
    run(T, "hex_no_digits")
}

#[test]
fn hex_no_unit() -> Result<()> {
    run(T, "hex_no_unit")
}

#[test]
fn hex_trailing_ident_start() -> Result<()> {
    run(T, "hex_trailing_ident_start")
}

#[test]
fn hex_trailing_underscore() -> Result<()> {
    run(T, "hex_trailing_underscore")
}

#[test]
fn hex_unit() -> Result<()> {
    run(T, "hex_unit")
}

#[test]
fn hex_uppercase_prefix() -> Result<()> {
    run(T, "hex_uppercase_prefix")
}

#[test]
fn hex_with_underscores() -> Result<()> {
    run(T, "hex_with_underscores")
}

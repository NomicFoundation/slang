// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use crate::cst_output::runner::run;
use anyhow::Result;

#[test]
fn days_unit() -> Result<()> {
    return run("NumericExpression", "days_unit");
}

#[test]
fn decimal_no_unit() -> Result<()> {
    return run("NumericExpression", "decimal_no_unit");
}

#[test]
fn decimal_trailing_ident_start() -> Result<()> {
    return run("NumericExpression", "decimal_trailing_ident_start");
}

#[test]
fn ether_unit() -> Result<()> {
    return run("NumericExpression", "ether_unit");
}

#[test]
fn float() -> Result<()> {
    return run("NumericExpression", "float");
}

#[test]
fn float_no_fraction() -> Result<()> {
    return run("NumericExpression", "float_no_fraction");
}

#[test]
fn float_no_mantissa() -> Result<()> {
    return run("NumericExpression", "float_no_mantissa");
}

#[test]
fn hex_consecutive_underscores() -> Result<()> {
    return run("NumericExpression", "hex_consecutive_underscores");
}

#[test]
fn hex_leading_underscore() -> Result<()> {
    return run("NumericExpression", "hex_leading_underscore");
}

#[test]
fn hex_multiple_digits() -> Result<()> {
    return run("NumericExpression", "hex_multiple_digits");
}

#[test]
fn hex_no_digits() -> Result<()> {
    return run("NumericExpression", "hex_no_digits");
}

#[test]
fn hex_no_unit() -> Result<()> {
    return run("NumericExpression", "hex_no_unit");
}

#[test]
fn hex_trailing_ident_start() -> Result<()> {
    return run("NumericExpression", "hex_trailing_ident_start");
}

#[test]
fn hex_trailing_underscore() -> Result<()> {
    return run("NumericExpression", "hex_trailing_underscore");
}

#[test]
fn hex_unit() -> Result<()> {
    return run("NumericExpression", "hex_unit");
}

#[test]
fn hex_uppercase_prefix() -> Result<()> {
    return run("NumericExpression", "hex_uppercase_prefix");
}

#[test]
fn hex_with_underscores() -> Result<()> {
    return run("NumericExpression", "hex_with_underscores");
}

#[test]
fn integer() -> Result<()> {
    return run("NumericExpression", "integer");
}

#[test]
fn integer_with_exponent() -> Result<()> {
    return run("NumericExpression", "integer_with_exponent");
}

#[test]
fn integer_with_separators() -> Result<()> {
    return run("NumericExpression", "integer_with_separators");
}

#[test]
fn years_unit() -> Result<()> {
    return run("NumericExpression", "years_unit");
}

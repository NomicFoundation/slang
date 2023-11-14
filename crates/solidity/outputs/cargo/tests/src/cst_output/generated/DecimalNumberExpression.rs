// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use crate::cst_output::runner::run;
use anyhow::Result;

#[test]
fn days_unit() -> Result<()> {
    return run("DecimalNumberExpression", "days_unit");
}

#[test]
fn decimal_no_unit() -> Result<()> {
    return run("DecimalNumberExpression", "decimal_no_unit");
}

#[test]
fn decimal_trailing_ident_start() -> Result<()> {
    return run("DecimalNumberExpression", "decimal_trailing_ident_start");
}

#[test]
fn ether_unit() -> Result<()> {
    return run("DecimalNumberExpression", "ether_unit");
}

#[test]
fn float() -> Result<()> {
    return run("DecimalNumberExpression", "float");
}

#[test]
fn float_no_fraction() -> Result<()> {
    return run("DecimalNumberExpression", "float_no_fraction");
}

#[test]
fn float_no_mantissa() -> Result<()> {
    return run("DecimalNumberExpression", "float_no_mantissa");
}

#[test]
fn integer() -> Result<()> {
    return run("DecimalNumberExpression", "integer");
}

#[test]
fn integer_with_exponent() -> Result<()> {
    return run("DecimalNumberExpression", "integer_with_exponent");
}

#[test]
fn integer_with_separators() -> Result<()> {
    return run("DecimalNumberExpression", "integer_with_separators");
}

#[test]
fn years_unit() -> Result<()> {
    return run("DecimalNumberExpression", "years_unit");
}

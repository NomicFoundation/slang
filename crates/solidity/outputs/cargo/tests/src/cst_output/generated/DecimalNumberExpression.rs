// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use crate::cst_output::runner::run;
use anyhow::Result;

#[test]
fn days_unit() -> Result<()> {
    run("DecimalNumberExpression", "days_unit")
}

#[test]
fn decimal_no_unit() -> Result<()> {
    run("DecimalNumberExpression", "decimal_no_unit")
}

#[test]
fn decimal_trailing_ident_start() -> Result<()> {
    run("DecimalNumberExpression", "decimal_trailing_ident_start")
}

#[test]
fn ether_unit() -> Result<()> {
    run("DecimalNumberExpression", "ether_unit")
}

#[test]
fn float() -> Result<()> {
    run("DecimalNumberExpression", "float")
}

#[test]
fn float_no_fraction() -> Result<()> {
    run("DecimalNumberExpression", "float_no_fraction")
}

#[test]
fn float_no_mantissa() -> Result<()> {
    run("DecimalNumberExpression", "float_no_mantissa")
}

#[test]
fn integer() -> Result<()> {
    run("DecimalNumberExpression", "integer")
}

#[test]
fn integer_with_exponent() -> Result<()> {
    run("DecimalNumberExpression", "integer_with_exponent")
}

#[test]
fn integer_with_separators() -> Result<()> {
    run("DecimalNumberExpression", "integer_with_separators")
}

#[test]
fn years_unit() -> Result<()> {
    run("DecimalNumberExpression", "years_unit")
}

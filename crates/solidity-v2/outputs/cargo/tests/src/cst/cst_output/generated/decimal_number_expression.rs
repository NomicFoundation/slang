// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use anyhow::Result;

use crate::cst::cst_output::runner::run;

const T: &str = "DecimalNumberExpression";

#[test]
fn days_unit() -> Result<()> {
    run(T, "days_unit")
}

#[test]
fn decimal_no_unit() -> Result<()> {
    run(T, "decimal_no_unit")
}

#[test]
fn decimal_trailing_ident_start() -> Result<()> {
    run(T, "decimal_trailing_ident_start")
}

#[test]
fn ether_unit() -> Result<()> {
    run(T, "ether_unit")
}

#[test]
fn float() -> Result<()> {
    run(T, "float")
}

#[test]
fn float_ident_after_period() -> Result<()> {
    run(T, "float_ident_after_period")
}

#[test]
fn float_no_fraction() -> Result<()> {
    run(T, "float_no_fraction")
}

#[test]
fn float_no_mantissa() -> Result<()> {
    run(T, "float_no_mantissa")
}

#[test]
fn integer() -> Result<()> {
    run(T, "integer")
}

#[test]
fn integer_ident_after_period() -> Result<()> {
    run(T, "integer_ident_after_period")
}

#[test]
fn integer_with_exponent() -> Result<()> {
    run(T, "integer_with_exponent")
}

#[test]
fn integer_with_separators() -> Result<()> {
    run(T, "integer_with_separators")
}

#[test]
fn leading_period_ident_after_decimal() -> Result<()> {
    run(T, "leading_period_ident_after_decimal")
}

#[test]
fn leading_period_ident_after_period() -> Result<()> {
    run(T, "leading_period_ident_after_period")
}

#[test]
fn years_unit() -> Result<()> {
    run(T, "years_unit")
}

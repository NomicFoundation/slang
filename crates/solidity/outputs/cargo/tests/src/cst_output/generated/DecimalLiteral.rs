// This file is @generated automatically by infrastructure scripts. Please don't edit by hand.

use crate::cst_output::runner::run;
use anyhow::Result;

#[test]
fn float() -> Result<()> {
    return run("DecimalLiteral", "float");
}

#[test]
fn float_no_fraction() -> Result<()> {
    return run("DecimalLiteral", "float_no_fraction");
}

#[test]
fn float_no_mantissa() -> Result<()> {
    return run("DecimalLiteral", "float_no_mantissa");
}

#[test]
fn integer() -> Result<()> {
    return run("DecimalLiteral", "integer");
}

#[test]
fn integer_with_exponent() -> Result<()> {
    return run("DecimalLiteral", "integer_with_exponent");
}

#[test]
fn integer_with_separators() -> Result<()> {
    return run("DecimalLiteral", "integer_with_separators");
}

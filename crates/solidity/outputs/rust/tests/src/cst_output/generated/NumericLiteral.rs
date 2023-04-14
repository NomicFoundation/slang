// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use crate::cst_output::runner::run;
use anyhow::Result;

#[test]
fn days_unit() -> Result<()> {
    return run("NumericLiteral", "days_unit");
}

#[test]
fn decimal_no_unit() -> Result<()> {
    return run("NumericLiteral", "decimal_no_unit");
}

#[test]
fn ether_unit() -> Result<()> {
    return run("NumericLiteral", "ether_unit");
}

#[test]
fn hex_no_unit() -> Result<()> {
    return run("NumericLiteral", "hex_no_unit");
}

#[test]
fn hex_unit() -> Result<()> {
    return run("NumericLiteral", "hex_unit");
}

#[test]
fn hex_uppercase_prefix() -> Result<()> {
    return run("NumericLiteral", "hex_uppercase_prefix");
}

#[test]
fn years_unit() -> Result<()> {
    return run("NumericLiteral", "years_unit");
}

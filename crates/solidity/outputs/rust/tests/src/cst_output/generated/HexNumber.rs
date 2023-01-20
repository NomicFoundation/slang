// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use crate::cst_output::runner::run;
use anyhow::Result;

#[test]
fn consecutive_underscores() -> Result<()> {
    return run("HexNumber", "consecutive_underscores");
}

#[test]
fn leading_underscore() -> Result<()> {
    return run("HexNumber", "leading_underscore");
}

#[test]
fn multiple_digits() -> Result<()> {
    return run("HexNumber", "multiple_digits");
}

#[test]
fn no_digits() -> Result<()> {
    return run("HexNumber", "no_digits");
}

#[test]
fn trailing_underscore() -> Result<()> {
    return run("HexNumber", "trailing_underscore");
}

#[test]
fn with_underscores() -> Result<()> {
    return run("HexNumber", "with_underscores");
}

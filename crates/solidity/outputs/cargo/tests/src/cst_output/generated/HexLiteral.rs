// This file is @generated automatically by infrastructure scripts. Please don't edit by hand.
#![allow(clippy::all)]

use crate::cst_output::runner::run;
use anyhow::Result;

#[test]
fn consecutive_underscores() -> Result<()> {
    return run("HexLiteral", "consecutive_underscores");
}

#[test]
fn leading_underscore() -> Result<()> {
    return run("HexLiteral", "leading_underscore");
}

#[test]
fn multiple_digits() -> Result<()> {
    return run("HexLiteral", "multiple_digits");
}

#[test]
fn no_digits() -> Result<()> {
    return run("HexLiteral", "no_digits");
}

#[test]
fn trailing_underscore() -> Result<()> {
    return run("HexLiteral", "trailing_underscore");
}

#[test]
fn with_underscores() -> Result<()> {
    return run("HexLiteral", "with_underscores");
}

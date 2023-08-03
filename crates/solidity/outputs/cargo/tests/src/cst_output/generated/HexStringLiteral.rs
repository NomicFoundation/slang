// This file is @generated automatically by infrastructure scripts. Please don't edit by hand.

use crate::cst_output::runner::run;
use anyhow::Result;

#[test]
fn all_separated_pairs() -> Result<()> {
    return run("HexStringLiteral", "all_separated_pairs");
}

#[test]
fn invalid_consecutive_separators() -> Result<()> {
    return run("HexStringLiteral", "invalid_consecutive_separators");
}

#[test]
fn invalid_leading_separator() -> Result<()> {
    return run("HexStringLiteral", "invalid_leading_separator");
}

#[test]
fn invalid_separator_after_single_char() -> Result<()> {
    return run("HexStringLiteral", "invalid_separator_after_single_char");
}

#[test]
fn invalid_trailing_separator() -> Result<()> {
    return run("HexStringLiteral", "invalid_trailing_separator");
}

#[test]
fn no_separators() -> Result<()> {
    return run("HexStringLiteral", "no_separators");
}

#[test]
fn some_separated_pairs() -> Result<()> {
    return run("HexStringLiteral", "some_separated_pairs");
}

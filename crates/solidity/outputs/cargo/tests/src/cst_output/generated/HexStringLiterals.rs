// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use crate::cst_output::runner::run;
use anyhow::Result;

#[test]
fn all_separated_pairs() -> Result<()> {
    run("HexStringLiterals", "all_separated_pairs")
}

#[test]
fn invalid_consecutive_separators() -> Result<()> {
    run("HexStringLiterals", "invalid_consecutive_separators")
}

#[test]
fn invalid_leading_separator() -> Result<()> {
    run("HexStringLiterals", "invalid_leading_separator")
}

#[test]
fn invalid_separator_after_single_char() -> Result<()> {
    run("HexStringLiterals", "invalid_separator_after_single_char")
}

#[test]
fn invalid_trailing_separator() -> Result<()> {
    run("HexStringLiterals", "invalid_trailing_separator")
}

#[test]
fn multiple() -> Result<()> {
    run("HexStringLiterals", "multiple")
}

#[test]
fn no_separators() -> Result<()> {
    run("HexStringLiterals", "no_separators")
}

#[test]
fn single() -> Result<()> {
    run("HexStringLiterals", "single")
}

#[test]
fn single_trailing_ident() -> Result<()> {
    run("HexStringLiterals", "single_trailing_ident")
}

#[test]
fn some_separated_pairs() -> Result<()> {
    run("HexStringLiterals", "some_separated_pairs")
}

// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use anyhow::Result;

use crate::cst::cst_output::runner::run;

const T: &str = "HexStringLiterals";

#[test]
fn all_separated_pairs() -> Result<()> {
    run(T, "all_separated_pairs")
}

#[test]
fn invalid_consecutive_separators() -> Result<()> {
    run(T, "invalid_consecutive_separators")
}

#[test]
fn invalid_leading_separator() -> Result<()> {
    run(T, "invalid_leading_separator")
}

#[test]
fn invalid_separator_after_single_char() -> Result<()> {
    run(T, "invalid_separator_after_single_char")
}

#[test]
fn invalid_trailing_separator() -> Result<()> {
    run(T, "invalid_trailing_separator")
}

#[test]
fn multiple() -> Result<()> {
    run(T, "multiple")
}

#[test]
fn no_separators() -> Result<()> {
    run(T, "no_separators")
}

#[test]
fn single() -> Result<()> {
    run(T, "single")
}

#[test]
fn single_trailing_ident() -> Result<()> {
    run(T, "single_trailing_ident")
}

#[test]
fn some_separated_pairs() -> Result<()> {
    run(T, "some_separated_pairs")
}

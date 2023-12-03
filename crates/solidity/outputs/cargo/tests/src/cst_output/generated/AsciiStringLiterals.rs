// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use crate::cst_output::runner::run;
use anyhow::Result;

#[test]
fn multiple() -> Result<()> {
    run("AsciiStringLiterals", "multiple")
}

#[test]
fn single() -> Result<()> {
    run("AsciiStringLiterals", "single")
}

#[test]
fn single_trailing_ident() -> Result<()> {
    run("AsciiStringLiterals", "single_trailing_ident")
}

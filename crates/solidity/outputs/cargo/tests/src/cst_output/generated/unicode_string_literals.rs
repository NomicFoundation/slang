// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use anyhow::Result;

use crate::cst_output::runner::run;

#[test]
fn multiple() -> Result<()> {
    run("UnicodeStringLiterals", "multiple")
}

#[test]
fn single() -> Result<()> {
    run("UnicodeStringLiterals", "single")
}

#[test]
fn single_trailing_ident() -> Result<()> {
    run("UnicodeStringLiterals", "single_trailing_ident")
}

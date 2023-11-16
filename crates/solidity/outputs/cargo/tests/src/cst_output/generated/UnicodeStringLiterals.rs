// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use crate::cst_output::runner::run;
use anyhow::Result;

#[test]
fn multiple() -> Result<()> {
    return run("UnicodeStringLiterals", "multiple");
}

#[test]
fn single() -> Result<()> {
    return run("UnicodeStringLiterals", "single");
}

#[test]
fn single_trailing_ident() -> Result<()> {
    return run("UnicodeStringLiterals", "single_trailing_ident");
}

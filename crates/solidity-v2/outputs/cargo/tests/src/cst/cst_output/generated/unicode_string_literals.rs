// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use anyhow::Result;

use crate::cst::cst_output::runner::run;

const T: &str = "UnicodeStringLiterals";

#[test]
fn multiple() -> Result<()> {
    run(T, "multiple")
}

#[test]
fn single() -> Result<()> {
    run(T, "single")
}

#[test]
fn single_trailing_ident() -> Result<()> {
    run(T, "single_trailing_ident")
}

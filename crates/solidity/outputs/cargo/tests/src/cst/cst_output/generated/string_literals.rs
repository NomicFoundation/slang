// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use anyhow::Result;

use crate::cst::cst_output::runner::run;

const T: &str = "StringLiterals";

#[test]
fn both_quotes() -> Result<()> {
    run(T, "both_quotes")
}

#[test]
fn double_quote() -> Result<()> {
    run(T, "double_quote")
}

#[test]
fn double_quote_unicode() -> Result<()> {
    run(T, "double_quote_unicode")
}

#[test]
fn single_quote() -> Result<()> {
    run(T, "single_quote")
}

#[test]
fn single_quote_unicode() -> Result<()> {
    run(T, "single_quote_unicode")
}

#[test]
fn single_trailing_ident() -> Result<()> {
    run(T, "single_trailing_ident")
}

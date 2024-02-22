// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use anyhow::Result;

use crate::cst_output::runner::run;

#[test]
fn both_quotes() -> Result<()> {
    run("StringLiterals", "both_quotes")
}

#[test]
fn double_quote() -> Result<()> {
    run("StringLiterals", "double_quote")
}

#[test]
fn double_quote_unicode() -> Result<()> {
    run("StringLiterals", "double_quote_unicode")
}

#[test]
fn single_quote() -> Result<()> {
    run("StringLiterals", "single_quote")
}

#[test]
fn single_quote_unicode() -> Result<()> {
    run("StringLiterals", "single_quote_unicode")
}

#[test]
fn single_trailing_ident() -> Result<()> {
    run("StringLiterals", "single_trailing_ident")
}

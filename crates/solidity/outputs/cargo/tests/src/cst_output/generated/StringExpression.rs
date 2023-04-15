// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use crate::cst_output::runner::run;
use anyhow::Result;

#[test]
fn ascii_multiple() -> Result<()> {
    return run("StringExpression", "ascii_multiple");
}

#[test]
fn ascii_single() -> Result<()> {
    return run("StringExpression", "ascii_single");
}

#[test]
fn hex_multiple() -> Result<()> {
    return run("StringExpression", "hex_multiple");
}

#[test]
fn hex_single() -> Result<()> {
    return run("StringExpression", "hex_single");
}

#[test]
fn unicode_multiple() -> Result<()> {
    return run("StringExpression", "unicode_multiple");
}

#[test]
fn unicode_single() -> Result<()> {
    return run("StringExpression", "unicode_single");
}

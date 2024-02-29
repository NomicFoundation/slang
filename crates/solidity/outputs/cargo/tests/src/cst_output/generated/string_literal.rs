// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use anyhow::Result;

use crate::cst_output::runner::run;

#[test]
fn escape_newline_double_quote() -> Result<()> {
    run("StringLiteral", "escape_newline_double_quote")
}

#[test]
fn escape_newline_single_quote() -> Result<()> {
    run("StringLiteral", "escape_newline_single_quote")
}

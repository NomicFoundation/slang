// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use crate::cst_output::runner::run;
use anyhow::Result;

#[test]
fn compound_tokens() -> Result<()> {
    return run("Statements", "compound_tokens");
}

#[test]
fn invalid_termination() -> Result<()> {
    return run("Statements", "invalid_termination");
}

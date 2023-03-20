// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use crate::cst_output::runner::run;
use anyhow::Result;

#[test]
fn true_keyword() -> Result<()> {
    return run("Keyword", "true_keyword");
}

#[test]
fn while_keyword() -> Result<()> {
    return run("Keyword", "while_keyword");
}

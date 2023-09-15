// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use crate::cst_output::runner::run;
use anyhow::Result;

#[test]
fn function_def() -> Result<()> {
    return run("YulBlock", "function_def");
}

#[test]
fn ignore_unknown_delim() -> Result<()> {
    return run("YulBlock", "ignore_unknown_delim");
}

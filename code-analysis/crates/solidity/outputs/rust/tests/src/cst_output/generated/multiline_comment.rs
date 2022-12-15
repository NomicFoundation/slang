// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use crate::cst_output::run;
use anyhow::Result;

#[test]
fn multiple_lines() -> Result<()> {
    return run("multiline_comment", "multiple_lines");
}

#[test]
fn same_line() -> Result<()> {
    return run("multiline_comment", "same_line");
}

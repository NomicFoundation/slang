// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use crate::cst_output::runner::run;
use anyhow::Result;

#[test]
fn double_asterisks() -> Result<()> {
    return run("MultilineComment", "double_asterisks");
}

#[test]
fn long() -> Result<()> {
    return run("MultilineComment", "long");
}

#[test]
fn multiple_lines() -> Result<()> {
    return run("MultilineComment", "multiple_lines");
}

#[test]
fn same_line() -> Result<()> {
    return run("MultilineComment", "same_line");
}

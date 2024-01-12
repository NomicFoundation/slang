// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use anyhow::Result;

use crate::cst_output::runner::run;

#[test]
fn multi_line_spanning_multiple_lines() -> Result<()> {
    run("TrailingTrivia", "multi_line_spanning_multiple_lines")
}

#[test]
fn multi_line_without_newline() -> Result<()> {
    run("TrailingTrivia", "multi_line_without_newline")
}

#[test]
fn only_until_newline() -> Result<()> {
    run("TrailingTrivia", "only_until_newline")
}

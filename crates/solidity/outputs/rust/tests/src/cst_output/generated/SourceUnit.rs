// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use crate::cst_output::runner::run;
use anyhow::Result;

#[test]
fn empty_file() -> Result<()> {
    return run("SourceUnit", "empty_file");
}

#[test]
fn end_of_file_trivia() -> Result<()> {
    return run("SourceUnit", "end_of_file_trivia");
}

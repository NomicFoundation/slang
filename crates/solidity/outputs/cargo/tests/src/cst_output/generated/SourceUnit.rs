// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use crate::cst_output::runner::run;
use anyhow::Result;

#[test]
fn SafeMath() -> Result<()> {
    run("SourceUnit", "SafeMath")
}

#[test]
fn empty_file() -> Result<()> {
    run("SourceUnit", "empty_file")
}

#[test]
fn end_of_file_trivia() -> Result<()> {
    run("SourceUnit", "end_of_file_trivia")
}

#[test]
fn everything() -> Result<()> {
    run("SourceUnit", "everything")
}

#[test]
fn partial_definition() -> Result<()> {
    run("SourceUnit", "partial_definition")
}

#[test]
fn top_level_event() -> Result<()> {
    run("SourceUnit", "top_level_event")
}

#[test]
fn top_level_function() -> Result<()> {
    run("SourceUnit", "top_level_function")
}

#[test]
fn trailing_trivia() -> Result<()> {
    run("SourceUnit", "trailing_trivia")
}

#[test]
fn using_directive() -> Result<()> {
    run("SourceUnit", "using_directive")
}

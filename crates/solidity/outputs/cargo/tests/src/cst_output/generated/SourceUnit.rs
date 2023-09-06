// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use crate::cst_output::runner::run;
use anyhow::Result;

#[test]
fn SafeMath() -> Result<()> {
    return run("SourceUnit", "SafeMath");
}

#[test]
fn empty_file() -> Result<()> {
    return run("SourceUnit", "empty_file");
}

#[test]
fn end_of_file_trivia() -> Result<()> {
    return run("SourceUnit", "end_of_file_trivia");
}

#[test]
fn everything() -> Result<()> {
    return run("SourceUnit", "everything");
}

#[test]
fn partial_definition() -> Result<()> {
    return run("SourceUnit", "partial_definition");
}

#[test]
fn top_level_function() -> Result<()> {
    return run("SourceUnit", "top_level_function");
}

#[test]
fn trailing_trivia() -> Result<()> {
    return run("SourceUnit", "trailing_trivia");
}

#[test]
fn using_directive() -> Result<()> {
    return run("SourceUnit", "using_directive");
}

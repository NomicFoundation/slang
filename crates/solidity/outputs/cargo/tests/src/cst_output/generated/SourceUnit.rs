// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use anyhow::Result;

use crate::cst_output::runner::run;

#[test]
fn SafeMath() -> Result<()> {
    run("SourceUnit", "SafeMath")
}

#[test]
fn empty_file() -> Result<()> {
    run("SourceUnit", "empty_file")
}

#[test]
fn empty_multiline_comment_1_asterisk() -> Result<()> {
    run("SourceUnit", "empty_multiline_comment_1_asterisk")
}

#[test]
fn empty_multiline_comment_2_asterisks() -> Result<()> {
    run("SourceUnit", "empty_multiline_comment_2_asterisks")
}

#[test]
fn empty_multiline_comment_3_asterisks() -> Result<()> {
    run("SourceUnit", "empty_multiline_comment_3_asterisks")
}

#[test]
fn empty_multiline_comment_4_asterisks() -> Result<()> {
    run("SourceUnit", "empty_multiline_comment_4_asterisks")
}

#[test]
fn empty_multiline_over_natspec() -> Result<()> {
    run("SourceUnit", "empty_multiline_over_natspec")
}

#[test]
fn end_of_file_trivia() -> Result<()> {
    run("SourceUnit", "end_of_file_trivia")
}

#[test]
fn end_of_file_trivia_incomplete() -> Result<()> {
    run("SourceUnit", "end_of_file_trivia_incomplete")
}

#[test]
fn end_of_file_trivia_unexpected_after() -> Result<()> {
    run("SourceUnit", "end_of_file_trivia_unexpected_after")
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

// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use anyhow::Result;

use crate::cst_output::runner::run;

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
fn leading_trivia_multi_line() -> Result<()> {
    run("SourceUnit", "leading_trivia_multi_line")
}

#[test]
fn leading_trivia_multi_line_natspec_comment() -> Result<()> {
    run("SourceUnit", "leading_trivia_multi_line_natspec_comment")
}

#[test]
fn leading_trivia_multi_line_trailing_double_star() -> Result<()> {
    run(
        "SourceUnit",
        "leading_trivia_multi_line_trailing_double_star",
    )
}

#[test]
fn leading_trivia_new_line() -> Result<()> {
    run("SourceUnit", "leading_trivia_new_line")
}

#[test]
fn leading_trivia_single_line_comment() -> Result<()> {
    run("SourceUnit", "leading_trivia_single_line_comment")
}

#[test]
fn leading_trivia_single_line_natspec_comment() -> Result<()> {
    run("SourceUnit", "leading_trivia_single_line_natspec_comment")
}

#[test]
fn leading_trivia_whitespace() -> Result<()> {
    run("SourceUnit", "leading_trivia_whitespace")
}

#[test]
fn partial_definition() -> Result<()> {
    run("SourceUnit", "partial_definition")
}

#[test]
fn pratt_precedence_recovery() -> Result<()> {
    run("SourceUnit", "pratt_precedence_recovery")
}

#[test]
fn safe_math() -> Result<()> {
    run("SourceUnit", "safe_math")
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
fn trailing_trivia_multi_line_spanning_multiple_lines() -> Result<()> {
    run(
        "SourceUnit",
        "trailing_trivia_multi_line_spanning_multiple_lines",
    )
}

#[test]
fn trailing_trivia_multi_line_without_newline() -> Result<()> {
    run("SourceUnit", "trailing_trivia_multi_line_without_newline")
}

#[test]
fn trailing_trivia_only_until_newline() -> Result<()> {
    run("SourceUnit", "trailing_trivia_only_until_newline")
}

#[test]
fn using_directive() -> Result<()> {
    run("SourceUnit", "using_directive")
}

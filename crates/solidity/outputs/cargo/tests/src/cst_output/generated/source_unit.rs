// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use anyhow::Result;

use crate::cst_output::runner::run;

const T: &str = "SourceUnit";

#[test]
fn empty_file() -> Result<()> {
    run(T, "empty_file")
}

#[test]
fn empty_multiline_comment_1_asterisk() -> Result<()> {
    run(T, "empty_multiline_comment_1_asterisk")
}

#[test]
fn empty_multiline_comment_2_asterisks() -> Result<()> {
    run(T, "empty_multiline_comment_2_asterisks")
}

#[test]
fn empty_multiline_comment_3_asterisks() -> Result<()> {
    run(T, "empty_multiline_comment_3_asterisks")
}

#[test]
fn empty_multiline_comment_4_asterisks() -> Result<()> {
    run(T, "empty_multiline_comment_4_asterisks")
}

#[test]
fn empty_multiline_over_natspec() -> Result<()> {
    run(T, "empty_multiline_over_natspec")
}

#[test]
fn end_of_file_trivia() -> Result<()> {
    run(T, "end_of_file_trivia")
}

#[test]
fn end_of_file_trivia_incomplete() -> Result<()> {
    run(T, "end_of_file_trivia_incomplete")
}

#[test]
fn end_of_file_trivia_unexpected_after() -> Result<()> {
    run(T, "end_of_file_trivia_unexpected_after")
}

#[test]
fn everything() -> Result<()> {
    run(T, "everything")
}

#[test]
fn leading_trivia_multi_line() -> Result<()> {
    run(T, "leading_trivia_multi_line")
}

#[test]
fn leading_trivia_multi_line_natspec_comment() -> Result<()> {
    run(T, "leading_trivia_multi_line_natspec_comment")
}

#[test]
fn leading_trivia_multi_line_trailing_double_star() -> Result<()> {
    run(T, "leading_trivia_multi_line_trailing_double_star")
}

#[test]
fn leading_trivia_new_line() -> Result<()> {
    run(T, "leading_trivia_new_line")
}

#[test]
fn leading_trivia_single_line_comment() -> Result<()> {
    run(T, "leading_trivia_single_line_comment")
}

#[test]
fn leading_trivia_single_line_natspec_comment() -> Result<()> {
    run(T, "leading_trivia_single_line_natspec_comment")
}

#[test]
fn leading_trivia_whitespace() -> Result<()> {
    run(T, "leading_trivia_whitespace")
}

#[test]
fn partial_definition() -> Result<()> {
    run(T, "partial_definition")
}

#[test]
fn pratt_precedence_recovery() -> Result<()> {
    run(T, "pratt_precedence_recovery")
}

#[test]
fn safe_math() -> Result<()> {
    run(T, "safe_math")
}

#[test]
fn top_level_event() -> Result<()> {
    run(T, "top_level_event")
}

#[test]
fn top_level_function() -> Result<()> {
    run(T, "top_level_function")
}

#[test]
fn trailing_trivia() -> Result<()> {
    run(T, "trailing_trivia")
}

#[test]
fn trailing_trivia_multi_line_spanning_multiple_lines() -> Result<()> {
    run(T, "trailing_trivia_multi_line_spanning_multiple_lines")
}

#[test]
fn trailing_trivia_multi_line_without_newline() -> Result<()> {
    run(T, "trailing_trivia_multi_line_without_newline")
}

#[test]
fn trailing_trivia_only_until_newline() -> Result<()> {
    run(T, "trailing_trivia_only_until_newline")
}

#[test]
fn using_directive() -> Result<()> {
    run(T, "using_directive")
}

// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use anyhow::Result;

use crate::cst_output::runner::run;

#[test]
fn multi_line() -> Result<()> {
    run("LeadingTrivia", "multi_line")
}

#[test]
fn multi_line_natspec_comment() -> Result<()> {
    run("LeadingTrivia", "multi_line_natspec_comment")
}

#[test]
fn multi_line_trailing_double_star() -> Result<()> {
    run("LeadingTrivia", "multi_line_trailing_double_star")
}

#[test]
fn new_line() -> Result<()> {
    run("LeadingTrivia", "new_line")
}

#[test]
fn single_line_comment() -> Result<()> {
    run("LeadingTrivia", "single_line_comment")
}

#[test]
fn single_line_natspec_comment() -> Result<()> {
    run("LeadingTrivia", "single_line_natspec_comment")
}

#[test]
fn whitespace() -> Result<()> {
    run("LeadingTrivia", "whitespace")
}

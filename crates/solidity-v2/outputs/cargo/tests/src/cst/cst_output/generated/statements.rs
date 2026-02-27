// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use anyhow::Result;

use crate::cst::cst_output::runner::run;

const T: &str = "Statements";

#[test]
fn compound_tokens() -> Result<()> {
    run(T, "compound_tokens")
}

#[test]
fn contextual_keywords() -> Result<()> {
    run(T, "contextual_keywords")
}

#[test]
fn delete_identifier() -> Result<()> {
    run(T, "delete_identifier")
}

#[test]
fn delete_index() -> Result<()> {
    run(T, "delete_index")
}

#[test]
fn invalid_termination() -> Result<()> {
    run(T, "invalid_termination")
}

#[test]
fn recovery_ignore_multiple_empty_matches() -> Result<()> {
    run(T, "recovery_ignore_multiple_empty_matches")
}

// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use anyhow::Result;

use crate::cst::cst_output::runner::run;

const T: &str = "VersionPragma";

#[test]
fn alternatives() -> Result<()> {
    run(T, "alternatives")
}

#[test]
fn double_quotes_string() -> Result<()> {
    run(T, "double_quotes_string")
}

#[test]
fn equal_operator() -> Result<()> {
    run(T, "equal_operator")
}

#[test]
fn exact_version() -> Result<()> {
    run(T, "exact_version")
}

#[test]
fn less_than_operator() -> Result<()> {
    run(T, "less_than_operator")
}

#[test]
fn multiple_exact_versions() -> Result<()> {
    run(T, "multiple_exact_versions")
}

#[test]
fn multiple_strings() -> Result<()> {
    run(T, "multiple_strings")
}

#[test]
fn nested_expressions() -> Result<()> {
    run(T, "nested_expressions")
}

#[test]
fn ranges() -> Result<()> {
    run(T, "ranges")
}

#[test]
fn single_quote_string() -> Result<()> {
    run(T, "single_quote_string")
}

#[test]
fn with_trivia() -> Result<()> {
    run(T, "with_trivia")
}

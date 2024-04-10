// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use anyhow::Result;

use crate::cst_output::runner::run;

#[test]
fn alternatives() -> Result<()> {
    run("VersionPragma", "alternatives")
}

#[test]
fn double_quotes_string() -> Result<()> {
    run("VersionPragma", "double_quotes_string")
}

#[test]
fn equal_operator() -> Result<()> {
    run("VersionPragma", "equal_operator")
}

#[test]
fn exact_version() -> Result<()> {
    run("VersionPragma", "exact_version")
}

#[test]
fn less_than_operator() -> Result<()> {
    run("VersionPragma", "less_than_operator")
}

#[test]
fn multiple_exact_versions() -> Result<()> {
    run("VersionPragma", "multiple_exact_versions")
}

#[test]
fn multiple_strings() -> Result<()> {
    run("VersionPragma", "multiple_strings")
}

#[test]
fn nested_expressions() -> Result<()> {
    run("VersionPragma", "nested_expressions")
}

#[test]
fn ranges() -> Result<()> {
    run("VersionPragma", "ranges")
}

#[test]
fn single_quote_string() -> Result<()> {
    run("VersionPragma", "single_quote_string")
}

#[test]
fn with_trivia() -> Result<()> {
    run("VersionPragma", "with_trivia")
}

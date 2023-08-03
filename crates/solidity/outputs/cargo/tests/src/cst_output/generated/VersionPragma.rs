// This file is @generated automatically by infrastructure scripts. Please don't edit by hand.

use crate::cst_output::runner::run;
use anyhow::Result;

#[test]
fn alternatives() -> Result<()> {
    return run("VersionPragma", "alternatives");
}

#[test]
fn equal_operator() -> Result<()> {
    return run("VersionPragma", "equal_operator");
}

#[test]
fn exact_version() -> Result<()> {
    return run("VersionPragma", "exact_version");
}

#[test]
fn less_than_operator() -> Result<()> {
    return run("VersionPragma", "less_than_operator");
}

#[test]
fn multiple_exact_versions() -> Result<()> {
    return run("VersionPragma", "multiple_exact_versions");
}

#[test]
fn nested_expressions() -> Result<()> {
    return run("VersionPragma", "nested_expressions");
}

#[test]
fn ranges() -> Result<()> {
    return run("VersionPragma", "ranges");
}

#[test]
fn with_trivia() -> Result<()> {
    return run("VersionPragma", "with_trivia");
}

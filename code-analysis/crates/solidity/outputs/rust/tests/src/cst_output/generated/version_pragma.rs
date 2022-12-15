// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use crate::cst_output::run;
use anyhow::Result;

#[test]
fn equal_operator() -> Result<()> {
    return run("version_pragma", "equal_operator");
}

#[test]
fn exact_version() -> Result<()> {
    return run("version_pragma", "exact_version");
}

#[test]
fn less_than_operator() -> Result<()> {
    return run("version_pragma", "less_than_operator");
}

#[test]
fn multiple_exact_versions() -> Result<()> {
    return run("version_pragma", "multiple_exact_versions");
}

#[test]
fn with_trivia() -> Result<()> {
    return run("version_pragma", "with_trivia");
}

// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use anyhow::Result;

use crate::semantic::semantic_output::runner::run;

const T: &str = "scoping";

#[test]
fn c99_scopes() -> Result<()> {
    run(T, "c99_scopes")
}

#[test]
fn functions() -> Result<()> {
    run(T, "functions")
}

#[test]
fn private_variables() -> Result<()> {
    run(T, "private_variables")
}

#[test]
fn shadowing() -> Result<()> {
    run(T, "shadowing")
}

#[test]
fn statement_scope() -> Result<()> {
    run(T, "statement_scope")
}

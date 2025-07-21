// This file is generated automatically by infrastructure scripts (crates/codegen/testing/src/bindings_output.rs). Please don't edit by hand.

use anyhow::Result;

use crate::bindings_output::runner::run;

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
fn hoisting_scopes() -> Result<()> {
    run(T, "hoisting_scopes")
}

#[test]
fn shadowing() -> Result<()> {
    run(T, "shadowing")
}

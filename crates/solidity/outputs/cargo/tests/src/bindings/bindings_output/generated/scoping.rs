// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use anyhow::Result;

use crate::bindings::bindings_output::runner::run;

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
fn private_variables() -> Result<()> {
    run(T, "private_variables")
}

#[test]
fn shadowing() -> Result<()> {
    run(T, "shadowing")
}

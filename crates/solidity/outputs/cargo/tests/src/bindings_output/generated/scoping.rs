// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use anyhow::Result;

use crate::bindings_output::runner::run;

#[test]
fn c99_scopes() -> Result<()> {
    run("scoping", "c99_scopes")
}

#[test]
fn functions() -> Result<()> {
    run("scoping", "functions")
}

#[test]
fn hoisting_scopes() -> Result<()> {
    run("scoping", "hoisting_scopes")
}

#[test]
fn shadowing() -> Result<()> {
    run("scoping", "shadowing")
}

// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use anyhow::Result;

use crate::bindings_assertions::runner::run;

#[test]
fn c99_scopes() -> Result<()> {
    run("scoping", "c99_scopes")
}

#[test]
fn hoisting_scopes() -> Result<()> {
    run("scoping", "hoisting_scopes")
}

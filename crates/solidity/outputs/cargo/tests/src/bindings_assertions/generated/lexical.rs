// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use anyhow::Result;

use crate::bindings_assertions::runner::run;

#[test]
fn c99_scopes() -> Result<()> {
    run("lexical", "c99_scopes")
}

#[test]
fn hoisting_scopes() -> Result<()> {
    run("lexical", "hoisting_scopes")
}

#[test]
fn local_vars() -> Result<()> {
    run("lexical", "local_vars")
}

#[test]
fn params() -> Result<()> {
    run("lexical", "params")
}

#[test]
fn state_vars() -> Result<()> {
    run("lexical", "state_vars")
}

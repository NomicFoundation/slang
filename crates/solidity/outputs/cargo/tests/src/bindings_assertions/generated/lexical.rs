// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use anyhow::Result;

use crate::bindings_assertions::runner::run;

#[test]
fn c_99_scopes() -> Result<()> {
    run("lexical", "c99_scopes.sol")
}

#[test]
fn hoisting_scopes() -> Result<()> {
    run("lexical", "hoisting_scopes.sol")
}

#[test]
fn local_vars() -> Result<()> {
    run("lexical", "local_vars.sol")
}

#[test]
fn params() -> Result<()> {
    run("lexical", "params.sol")
}

#[test]
fn state_vars() -> Result<()> {
    run("lexical", "state_vars.sol")
}

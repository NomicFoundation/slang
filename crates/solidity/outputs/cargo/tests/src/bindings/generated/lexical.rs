// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use anyhow::Result;

use crate::bindings::runner::run;

#[test]
fn local_vars() -> Result<()> {
    run("lexical", "local_vars.sol")
}

#[test]
fn params() -> Result<()> {
    run("lexical", "params.sol")
}

#[test]
fn shadowing() -> Result<()> {
    run("lexical", "shadowing.sol")
}

#[test]
fn state_vars() -> Result<()> {
    run("lexical", "state_vars.sol")
}

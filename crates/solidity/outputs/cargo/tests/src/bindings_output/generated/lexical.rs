// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use anyhow::Result;

use crate::bindings_output::runner::run;

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

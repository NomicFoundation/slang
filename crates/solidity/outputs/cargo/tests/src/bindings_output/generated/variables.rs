// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use anyhow::Result;

use crate::bindings_output::runner::run;

#[test]
fn destructuring() -> Result<()> {
    run("variables", "destructuring")
}

#[test]
fn local_vars() -> Result<()> {
    run("variables", "local_vars")
}

#[test]
fn params() -> Result<()> {
    run("variables", "params")
}

#[test]
fn state_vars() -> Result<()> {
    run("variables", "state_vars")
}

#[test]
fn var_declaration() -> Result<()> {
    run("variables", "var_declaration")
}

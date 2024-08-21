// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use anyhow::Result;

use crate::bindings_assertions::runner::run;

#[test]
fn blocks() -> Result<()> {
    run("yul", "blocks")
}

#[test]
fn fun_scopes() -> Result<()> {
    run("yul", "fun_scopes")
}

#[test]
fn functions() -> Result<()> {
    run("yul", "functions")
}

#[test]
fn variables() -> Result<()> {
    run("yul", "variables")
}

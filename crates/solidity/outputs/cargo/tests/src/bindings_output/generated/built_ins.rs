// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use anyhow::Result;

use crate::bindings_output::runner::run;

#[test]
fn address() -> Result<()> {
    run("built_ins", "address")
}

#[test]
fn arrays() -> Result<()> {
    run("built_ins", "arrays")
}

#[test]
fn functions() -> Result<()> {
    run("built_ins", "functions")
}

#[test]
fn global_properties() -> Result<()> {
    run("built_ins", "global_properties")
}

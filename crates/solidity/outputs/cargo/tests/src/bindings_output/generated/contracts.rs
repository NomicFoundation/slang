// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use anyhow::Result;

use crate::bindings_output::runner::run;

#[test]
fn fallback_receive() -> Result<()> {
    run("contracts", "fallback_receive")
}

#[test]
fn inheritance() -> Result<()> {
    run("contracts", "inheritance")
}

#[test]
fn super_linearisation() -> Result<()> {
    run("contracts", "super_linearisation")
}

#[test]
fn super_scope() -> Result<()> {
    run("contracts", "super_scope")
}

#[test]
fn virtual_lookup() -> Result<()> {
    run("contracts", "virtual_lookup")
}

#[test]
fn virtual_methods() -> Result<()> {
    run("contracts", "virtual_methods")
}

#[test]
fn visibility() -> Result<()> {
    run("contracts", "visibility")
}

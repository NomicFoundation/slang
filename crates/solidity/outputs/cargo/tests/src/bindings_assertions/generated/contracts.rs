// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use anyhow::Result;

use crate::bindings_assertions::runner::run;

#[test]
fn diamond() -> Result<()> {
    run("contracts", "diamond")
}

#[test]
fn inheritance() -> Result<()> {
    run("contracts", "inheritance")
}

#[test]
fn virtual_methods() -> Result<()> {
    run("contracts", "virtual_methods")
}

#[test]
fn virtual_single() -> Result<()> {
    run("contracts", "virtual_single")
}

#[test]
fn visibility() -> Result<()> {
    run("contracts", "visibility")
}

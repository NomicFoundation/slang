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
fn linearisation_order() -> Result<()> {
    run("contracts", "linearisation_order")
}

#[test]
fn virtual_lookup() -> Result<()> {
    run("contracts", "virtual_lookup")
}

#[test]
fn virtual_single() -> Result<()> {
    run("contracts", "virtual_single")
}

#[test]
fn visibility() -> Result<()> {
    run("contracts", "visibility")
}

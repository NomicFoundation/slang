// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use anyhow::Result;

use crate::bindings_assertions::runner::run;

#[test]
fn inheritance() -> Result<()> {
    run("contracts", "inheritance")
}

#[test]
fn visibility() -> Result<()> {
    run("contracts", "visibility")
}

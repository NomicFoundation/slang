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
fn visibility() -> Result<()> {
    run("contracts", "visibility")
}

// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use anyhow::Result;

use crate::bindings_output::runner::run;

#[test]
fn inheritance() -> Result<()> {
    run("interfaces", "inheritance")
}

#[test]
fn own_types_access() -> Result<()> {
    run("interfaces", "own_types_access")
}

#[test]
fn simple() -> Result<()> {
    run("interfaces", "simple")
}

#[test]
fn visibility() -> Result<()> {
    run("interfaces", "visibility")
}

// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use anyhow::Result;

use crate::bindings_output::runner::run;

#[test]
fn casting_dynamic_scope() -> Result<()> {
    run("interfaces", "casting_dynamic_scope")
}

#[test]
fn inherit_dynamic_scope() -> Result<()> {
    run("interfaces", "inherit_dynamic_scope")
}

#[test]
fn inheritance() -> Result<()> {
    run("interfaces", "inheritance")
}

#[test]
fn simple() -> Result<()> {
    run("interfaces", "simple")
}

#[test]
fn visibility() -> Result<()> {
    run("interfaces", "visibility")
}

// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use anyhow::Result;

use crate::binder::runner::run;

const T: &str = "interfaces";

#[test]
fn inheritance() -> Result<()> {
    run(T, "inheritance")
}

#[test]
fn modifiers() -> Result<()> {
    run(T, "modifiers")
}

#[test]
fn own_types_access() -> Result<()> {
    run(T, "own_types_access")
}

#[test]
fn simple() -> Result<()> {
    run(T, "simple")
}

#[test]
fn visibility() -> Result<()> {
    run(T, "visibility")
}

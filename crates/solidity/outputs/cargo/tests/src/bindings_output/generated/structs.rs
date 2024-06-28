// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use anyhow::Result;

use crate::bindings_output::runner::run;

#[test]
fn declaration() -> Result<()> {
    run("structs", "declaration")
}

#[test]
fn sample() -> Result<()> {
    run("structs", "sample")
}

#[test]
fn simple() -> Result<()> {
    run("structs", "simple")
}

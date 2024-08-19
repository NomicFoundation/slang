// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use anyhow::Result;

use crate::bindings_assertions::runner::run;

#[test]
fn nested() -> Result<()> {
    run("mappings", "nested")
}

#[test]
fn nested_custom() -> Result<()> {
    run("mappings", "nested_custom")
}

#[test]
fn simple() -> Result<()> {
    run("mappings", "simple")
}

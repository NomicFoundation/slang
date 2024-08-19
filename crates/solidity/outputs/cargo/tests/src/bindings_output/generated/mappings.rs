// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use anyhow::Result;

use crate::bindings_output::runner::run;

#[test]
fn custom_types() -> Result<()> {
    run("mappings", "custom_types")
}

#[test]
fn indexing() -> Result<()> {
    run("mappings", "indexing")
}

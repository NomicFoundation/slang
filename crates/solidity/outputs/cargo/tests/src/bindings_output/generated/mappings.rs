// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use anyhow::Result;

use crate::bindings_output::runner::run;

#[test]
fn custom_types() -> Result<()> {
    run("mappings", "custom_types")
}

#[test]
fn elementary() -> Result<()> {
    run("mappings", "elementary")
}

#[test]
fn indexing() -> Result<()> {
    run("mappings", "indexing")
}

#[test]
fn named_parameters() -> Result<()> {
    run("mappings", "named_parameters")
}

#[test]
fn nested() -> Result<()> {
    run("mappings", "nested")
}

#[test]
fn nested_custom() -> Result<()> {
    run("mappings", "nested_custom")
}

// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use anyhow::Result;

use crate::binder::runner::run;

const T: &str = "mappings";

#[test]
fn custom_types() -> Result<()> {
    run(T, "custom_types")
}

#[test]
fn elementary() -> Result<()> {
    run(T, "elementary")
}

#[test]
fn indexing() -> Result<()> {
    run(T, "indexing")
}

#[test]
fn named_parameters() -> Result<()> {
    run(T, "named_parameters")
}

#[test]
fn nested() -> Result<()> {
    run(T, "nested")
}

#[test]
fn nested_custom() -> Result<()> {
    run(T, "nested_custom")
}

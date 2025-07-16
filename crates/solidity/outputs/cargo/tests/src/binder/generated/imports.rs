// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use anyhow::Result;

use crate::binder::runner::run;

const T: &str = "imports";

#[test]
fn alias_import() -> Result<()> {
    run(T, "alias_import")
}

#[test]
fn deconstruction() -> Result<()> {
    run(T, "deconstruction")
}

#[test]
fn default() -> Result<()> {
    run(T, "default")
}

#[test]
fn default_deep() -> Result<()> {
    run(T, "default_deep")
}

#[test]
fn named() -> Result<()> {
    run(T, "named")
}

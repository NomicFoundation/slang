// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use anyhow::Result;

use crate::bindings_output::runner::run;

#[test]
fn custom_types() -> Result<()> {
    run("errors", "custom_types")
}

#[test]
fn named_args() -> Result<()> {
    run("errors", "named_args")
}

#[test]
fn revert_stmt() -> Result<()> {
    run("errors", "revert_stmt")
}

#[test]
fn selector() -> Result<()> {
    run("errors", "selector")
}

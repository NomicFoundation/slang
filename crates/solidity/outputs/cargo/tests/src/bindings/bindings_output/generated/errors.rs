// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use anyhow::Result;

use crate::bindings::bindings_output::runner::run;

const T: &str = "errors";

#[test]
fn custom_types() -> Result<()> {
    run(T, "custom_types")
}

#[test]
fn definitions() -> Result<()> {
    run(T, "definitions")
}

#[test]
fn named_args() -> Result<()> {
    run(T, "named_args")
}

#[test]
fn revert_as_function() -> Result<()> {
    run(T, "revert_as_function")
}

#[test]
fn revert_stmt() -> Result<()> {
    run(T, "revert_stmt")
}

#[test]
fn selector() -> Result<()> {
    run(T, "selector")
}

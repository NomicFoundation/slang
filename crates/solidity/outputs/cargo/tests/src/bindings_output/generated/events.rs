// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use anyhow::Result;

use crate::bindings_output::runner::run;

#[test]
fn custom_types() -> Result<()> {
    run("events", "custom_types")
}

#[test]
fn emit_stmt() -> Result<()> {
    run("events", "emit_stmt")
}

#[test]
fn named_args() -> Result<()> {
    run("events", "named_args")
}

// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use anyhow::Result;

use crate::bindings_assertions::runner::run;

#[test]
fn emit_stmt() -> Result<()> {
    run("events", "emit_stmt")
}

#[test]
fn named_args() -> Result<()> {
    run("events", "named_args")
}

#[test]
fn typed_args() -> Result<()> {
    run("events", "typed_args")
}

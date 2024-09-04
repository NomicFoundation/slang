// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use anyhow::Result;

use crate::bindings_output::runner::run;

#[test]
fn simple() -> Result<()> {
    run("modifiers", "simple")
}

#[test]
fn with_args() -> Result<()> {
    run("modifiers", "with_args")
}

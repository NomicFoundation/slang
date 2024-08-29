// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use anyhow::Result;

use crate::bindings_assertions::runner::run;

#[test]
fn named_args() -> Result<()> {
    run("errors", "named_args")
}

#[test]
fn revert_stmt() -> Result<()> {
    run("errors", "revert_stmt")
}

#[test]
fn typed_args() -> Result<()> {
    run("errors", "typed_args")
}

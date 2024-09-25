// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use anyhow::Result;

use crate::bindings_output::runner::run;

#[test]
fn deconstruction() -> Result<()> {
    run("using", "deconstruction")
}

#[test]
fn in_contract() -> Result<()> {
    run("using", "in_contract")
}

#[test]
fn in_library() -> Result<()> {
    run("using", "in_library")
}

#[test]
fn top_level() -> Result<()> {
    run("using", "top_level")
}

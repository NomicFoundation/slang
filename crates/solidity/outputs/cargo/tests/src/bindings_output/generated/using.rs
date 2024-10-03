// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use anyhow::Result;

use crate::bindings_output::runner::run;

#[test]
fn deconstruction() -> Result<()> {
    run("using", "deconstruction")
}

#[test]
fn elementary() -> Result<()> {
    run("using", "elementary")
}

#[test]
fn elementary_arrays() -> Result<()> {
    run("using", "elementary_arrays")
}

#[test]
fn function_types() -> Result<()> {
    run("using", "function_types")
}

#[test]
fn global() -> Result<()> {
    run("using", "global")
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

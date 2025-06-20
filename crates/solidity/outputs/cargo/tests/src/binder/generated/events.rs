// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use anyhow::Result;

use crate::binder::runner::run;

const T: &str = "events";

#[test]
fn custom_types() -> Result<()> {
    run(T, "custom_types")
}

#[test]
fn definitions() -> Result<()> {
    run(T, "definitions")
}

#[test]
fn emit_stmt() -> Result<()> {
    run(T, "emit_stmt")
}

#[test]
fn named_args() -> Result<()> {
    run(T, "named_args")
}

#[test]
fn selector() -> Result<()> {
    run(T, "selector")
}

// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use anyhow::Result;

use crate::binder::runner::run;

const T: &str = "events";

#[test]
fn called_as_functions() -> Result<()> {
    run(T, "called_as_functions")
}

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
fn overload_selection() -> Result<()> {
    run(T, "overload_selection")
}

#[test]
fn selector() -> Result<()> {
    run(T, "selector")
}

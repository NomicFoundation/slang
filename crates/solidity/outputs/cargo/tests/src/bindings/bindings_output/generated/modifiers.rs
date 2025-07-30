// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use anyhow::Result;

use crate::bindings::bindings_output::runner::run;

const T: &str = "modifiers";

#[test]
fn diamond() -> Result<()> {
    run(T, "diamond")
}

#[test]
fn function_named_underscore() -> Result<()> {
    run(T, "function_named_underscore")
}

#[test]
fn imported() -> Result<()> {
    run(T, "imported")
}

#[test]
fn inherited() -> Result<()> {
    run(T, "inherited")
}

#[test]
fn simple() -> Result<()> {
    run(T, "simple")
}

#[test]
fn virtual_modifier() -> Result<()> {
    run(T, "virtual_modifier")
}

#[test]
fn with_args() -> Result<()> {
    run(T, "with_args")
}

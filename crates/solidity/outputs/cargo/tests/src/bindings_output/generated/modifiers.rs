// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use anyhow::Result;

use crate::bindings_output::runner::run;

#[test]
fn diamond() -> Result<()> {
    run("modifiers", "diamond")
}

#[test]
fn function_named_underscore() -> Result<()> {
    run("modifiers", "function_named_underscore")
}

#[test]
fn inherited() -> Result<()> {
    run("modifiers", "inherited")
}

#[test]
fn simple() -> Result<()> {
    run("modifiers", "simple")
}

#[test]
fn virtual_modifier() -> Result<()> {
    run("modifiers", "virtual_modifier")
}

#[test]
fn with_args() -> Result<()> {
    run("modifiers", "with_args")
}

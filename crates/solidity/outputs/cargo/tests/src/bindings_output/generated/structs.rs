// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use anyhow::Result;

use crate::bindings_output::runner::run;

#[test]
fn declaration() -> Result<()> {
    run("structs", "declaration")
}

#[test]
fn function_call() -> Result<()> {
    run("structs", "function_call")
}

#[test]
fn named_params_construction() -> Result<()> {
    run("structs", "named_params_construction")
}

#[test]
fn nested() -> Result<()> {
    run("structs", "nested")
}

#[test]
fn recursive() -> Result<()> {
    run("structs", "recursive")
}

#[test]
fn sample() -> Result<()> {
    run("structs", "sample")
}

#[test]
fn simple() -> Result<()> {
    run("structs", "simple")
}

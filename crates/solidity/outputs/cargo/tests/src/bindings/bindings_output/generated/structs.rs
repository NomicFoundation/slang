// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use anyhow::Result;

use crate::bindings::bindings_output::runner::run;

const T: &str = "structs";

#[test]
fn declaration() -> Result<()> {
    run(T, "declaration")
}

#[test]
fn function_call() -> Result<()> {
    run(T, "function_call")
}

#[test]
fn named_params_construction() -> Result<()> {
    run(T, "named_params_construction")
}

#[test]
fn nested() -> Result<()> {
    run(T, "nested")
}

#[test]
fn recursive() -> Result<()> {
    run(T, "recursive")
}

#[test]
fn sample() -> Result<()> {
    run(T, "sample")
}

#[test]
fn simple() -> Result<()> {
    run(T, "simple")
}

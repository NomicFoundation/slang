// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use anyhow::Result;

use crate::bindings_assertions::runner::run;

#[test]
fn basic() -> Result<()> {
    run("expressions", "basic")
}

#[test]
fn function_call() -> Result<()> {
    run("expressions", "function_call")
}

#[test]
fn type_expression() -> Result<()> {
    run("expressions", "type_expression")
}

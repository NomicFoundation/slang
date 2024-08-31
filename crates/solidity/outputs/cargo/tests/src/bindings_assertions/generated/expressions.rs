// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use anyhow::Result;

use crate::bindings_assertions::runner::run;

#[test]
fn basic() -> Result<()> {
    run("expressions", "basic")
}

#[test]
fn call_options() -> Result<()> {
    run("expressions", "call_options")
}

#[test]
fn call_output() -> Result<()> {
    run("expressions", "call_output")
}

#[test]
fn function_call() -> Result<()> {
    run("expressions", "function_call")
}

#[test]
fn new_output() -> Result<()> {
    run("expressions", "new_output")
}

#[test]
fn type_expression() -> Result<()> {
    run("expressions", "type_expression")
}

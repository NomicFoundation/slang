// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use anyhow::Result;

use crate::cst_output::runner::run;

#[test]
fn method_call() -> Result<()> {
    run("TryStatement", "method_call")
}

#[test]
fn method_call_with_body() -> Result<()> {
    run("TryStatement", "method_call_with_body")
}

#[test]
fn method_call_with_options() -> Result<()> {
    run("TryStatement", "method_call_with_options")
}

#[test]
fn method_call_with_options_and_body() -> Result<()> {
    run("TryStatement", "method_call_with_options_and_body")
}

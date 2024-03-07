// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use anyhow::Result;

use crate::cst_output::runner::run;

#[test]
fn try_catch() -> Result<()> {
    run("TryStatement", "try_catch")
}

#[test]
fn try_catch_empty_body() -> Result<()> {
    run("TryStatement", "try_catch_empty_body")
}

#[test]
fn try_expr_call_options() -> Result<()> {
    run("TryStatement", "try_expr_call_options")
}

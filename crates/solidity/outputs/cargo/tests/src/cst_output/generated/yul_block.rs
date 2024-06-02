// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use anyhow::Result;

use crate::cst_output::runner::run;

#[test]
fn function_def() -> Result<()> {
    run("YulBlock", "function_def")
}

#[test]
fn ignore_unknown_delim() -> Result<()> {
    run("YulBlock", "ignore_unknown_delim")
}

#[test]
fn multiple_stack_assignments() -> Result<()> {
    run("YulBlock", "multiple_stack_assignments")
}

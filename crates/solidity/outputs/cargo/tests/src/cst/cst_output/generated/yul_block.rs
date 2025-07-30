// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use anyhow::Result;

use crate::cst::cst_output::runner::run;

const T: &str = "YulBlock";

#[test]
fn function_def() -> Result<()> {
    run(T, "function_def")
}

#[test]
fn ignore_unknown_delim() -> Result<()> {
    run(T, "ignore_unknown_delim")
}

#[test]
fn multiple_stack_assignments() -> Result<()> {
    run(T, "multiple_stack_assignments")
}

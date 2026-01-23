// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use anyhow::Result;

use crate::cst::cst_output::runner::run;

const T: &str = "FunctionCallExpression";

#[test]
fn empty_named_arguments() -> Result<()> {
    run(T, "empty_named_arguments")
}

#[test]
fn empty_positional_arguments() -> Result<()> {
    run(T, "empty_positional_arguments")
}

#[test]
fn payable_conversion() -> Result<()> {
    run(T, "payable_conversion")
}

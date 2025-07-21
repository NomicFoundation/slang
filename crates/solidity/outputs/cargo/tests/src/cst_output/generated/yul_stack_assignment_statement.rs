// This file is generated automatically by infrastructure scripts (crates/codegen/testing/src/cst_output.rs:12:18). Please don't edit by hand.

use anyhow::Result;

use crate::cst_output::runner::run;

const T: &str = "YulStackAssignmentStatement";

#[test]
fn equal_colon_separated() -> Result<()> {
    run(T, "equal_colon_separated")
}

#[test]
fn single_variable() -> Result<()> {
    run(T, "single_variable")
}

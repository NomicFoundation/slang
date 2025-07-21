// This file is generated automatically by infrastructure scripts (crates/codegen/testing/src/cst_output.rs:12:18). Please don't edit by hand.

use anyhow::Result;

use crate::cst_output::runner::run;

const T: &str = "YulStatement";

#[test]
fn label() -> Result<()> {
    run(T, "label")
}

#[test]
fn var_assign_colon_and_equals() -> Result<()> {
    run(T, "var_assign_colon_and_equals")
}

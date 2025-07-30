// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use anyhow::Result;

use crate::cst::cst_output::runner::run;

const T: &str = "YulStatement";

#[test]
fn label() -> Result<()> {
    run(T, "label")
}

#[test]
fn var_assign_colon_and_equals() -> Result<()> {
    run(T, "var_assign_colon_and_equals")
}

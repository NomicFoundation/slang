// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use anyhow::Result;

use crate::cst_output::runner::run;

#[test]
fn label() -> Result<()> {
    run("YulStatement", "label")
}

#[test]
fn var_assign_colon_and_equals() -> Result<()> {
    run("YulStatement", "var_assign_colon_and_equals")
}

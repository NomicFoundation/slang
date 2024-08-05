// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use anyhow::Result;

use crate::cst_output::runner::run;

#[test]
fn equal_colon_separated() -> Result<()> {
    run("YulStackAssignmentStatement", "equal_colon_separated")
}

#[test]
fn single_variable() -> Result<()> {
    run("YulStackAssignmentStatement", "single_variable")
}

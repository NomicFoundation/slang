// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use anyhow::Result;

use crate::cst::cst_output::runner::run;

const T: &str = "YulVariableAssignmentStatement";

#[test]
fn colon_equal_separated() -> Result<()> {
    run(T, "colon_equal_separated")
}

#[test]
fn identifier_and() -> Result<()> {
    run(T, "identifier_and")
}

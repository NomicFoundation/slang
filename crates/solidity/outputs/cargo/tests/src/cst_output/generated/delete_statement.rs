// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use anyhow::Result;

use crate::cst_output::runner::run;

#[test]
fn delete_identifier() -> Result<()> {
    run("DeleteStatement", "delete_identifier")
}

#[test]
fn delete_index() -> Result<()> {
    run("DeleteStatement", "delete_index")
}

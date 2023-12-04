// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use crate::cst_output::runner::run;
use anyhow::Result;

#[test]
fn delete_identifier() -> Result<()> {
    run("DeleteStatement", "delete_identifier")
}

#[test]
fn delete_index() -> Result<()> {
    run("DeleteStatement", "delete_index")
}

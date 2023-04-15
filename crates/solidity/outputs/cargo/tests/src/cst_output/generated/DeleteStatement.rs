// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use crate::cst_output::runner::run;
use anyhow::Result;

#[test]
fn delete_identifier() -> Result<()> {
    return run("DeleteStatement", "delete_identifier");
}

#[test]
fn delete_index() -> Result<()> {
    return run("DeleteStatement", "delete_index");
}

// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use anyhow::Result;

use crate::bindings_assertions::runner::run;

#[test]
fn if_else() -> Result<()> {
    run("control", "if_else")
}

#[test]
fn return_stmt() -> Result<()> {
    run("control", "return_stmt")
}

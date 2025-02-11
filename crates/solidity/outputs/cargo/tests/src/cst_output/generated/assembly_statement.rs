// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use anyhow::Result;

use crate::cst_output::runner::run;

#[test]
fn simple() -> Result<()> {
    run("AssemblyStatement", "simple")
}

#[test]
fn with_flags() -> Result<()> {
    run("AssemblyStatement", "with_flags")
}

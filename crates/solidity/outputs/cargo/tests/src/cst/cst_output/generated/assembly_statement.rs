// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use anyhow::Result;

use crate::cst::cst_output::runner::run;

const T: &str = "AssemblyStatement";

#[test]
fn simple() -> Result<()> {
    run(T, "simple")
}

#[test]
fn with_flags() -> Result<()> {
    run(T, "with_flags")
}

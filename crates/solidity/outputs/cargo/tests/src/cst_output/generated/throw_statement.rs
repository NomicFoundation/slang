// This file is generated automatically by infrastructure scripts (crates/codegen/testing/src/cst_output.rs:12:18). Please don't edit by hand.

use anyhow::Result;

use crate::cst_output::runner::run;

const T: &str = "ThrowStatement";

#[test]
fn throw() -> Result<()> {
    run(T, "throw")
}

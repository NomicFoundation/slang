// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use anyhow::Result;

use crate::cst::cst_output::runner::run;

const T: &str = "FallbackFunctionDefinition";

#[test]
fn simple() -> Result<()> {
    run(T, "simple")
}

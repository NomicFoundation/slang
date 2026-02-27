// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use anyhow::Result;

use crate::cst::cst_output::runner::run;

const T: &str = "ReceiveFunctionDefinition";

#[test]
fn simple() -> Result<()> {
    run(T, "simple")
}

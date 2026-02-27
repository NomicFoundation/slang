// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use anyhow::Result;

use crate::cst::cst_output::runner::run;

const T: &str = "ConstantDefinition";

#[test]
fn int() -> Result<()> {
    run(T, "int")
}

// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use anyhow::Result;

use crate::cst::cst_output::runner::run;

const T: &str = "InterfaceDefinition";

#[test]
fn sample_counter() -> Result<()> {
    run(T, "sample_counter")
}

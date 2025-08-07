// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use anyhow::Result;

use crate::cst::cst_output::runner::run;

const T: &str = "YulStatements";

#[test]
fn function_pointer() -> Result<()> {
    run(T, "function_pointer")
}

// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use anyhow::Result;

use crate::cst_output::runner::run;

const T: &str = "YulFunctionCallExpression";

#[test]
fn built_in_and() -> Result<()> {
    run(T, "built_in_and")
}

// This file is generated automatically by infrastructure scripts (crates/codegen/testing/src/cst_output.rs). Please don't edit by hand.

use anyhow::Result;

use crate::cst_output::runner::run;

const T: &str = "UserDefinedValueTypeDefinition";

#[test]
fn bool() -> Result<()> {
    run(T, "bool")
}

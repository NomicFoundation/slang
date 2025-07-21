// This file is generated automatically by infrastructure scripts (crates/codegen/testing/src/cst_output.rs). Please don't edit by hand.

use anyhow::Result;

use crate::cst_output::runner::run;

const T: &str = "EnumDefinition";

#[test]
fn multiple_members() -> Result<()> {
    run(T, "multiple_members")
}

#[test]
fn no_members() -> Result<()> {
    run(T, "no_members")
}

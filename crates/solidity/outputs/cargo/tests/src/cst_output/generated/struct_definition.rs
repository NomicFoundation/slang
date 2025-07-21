// This file is generated automatically by infrastructure scripts (crates/codegen/testing/src/cst_output.rs:12:18). Please don't edit by hand.

use anyhow::Result;

use crate::cst_output::runner::run;

const T: &str = "StructDefinition";

#[test]
fn member_function_pointer() -> Result<()> {
    run(T, "member_function_pointer")
}

#[test]
fn no_members() -> Result<()> {
    run(T, "no_members")
}

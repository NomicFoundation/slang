// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use anyhow::Result;

use crate::cst::cst_output::runner::run;

const T: &str = "StructDefinition";

#[test]
fn member_function_pointer() -> Result<()> {
    run(T, "member_function_pointer")
}

#[test]
fn no_members() -> Result<()> {
    run(T, "no_members")
}

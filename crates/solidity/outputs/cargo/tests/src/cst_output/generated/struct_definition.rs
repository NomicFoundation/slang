// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use anyhow::Result;

use crate::cst_output::runner::run;

#[test]
fn member_function_pointer() -> Result<()> {
    run("StructDefinition", "member_function_pointer")
}

#[test]
fn no_members() -> Result<()> {
    run("StructDefinition", "no_members")
}

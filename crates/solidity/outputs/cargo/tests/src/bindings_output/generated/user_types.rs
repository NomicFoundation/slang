// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use anyhow::Result;

use crate::bindings_output::runner::run;

#[test]
fn in_contract() -> Result<()> {
    run("user_types", "in_contract")
}

#[test]
fn in_library() -> Result<()> {
    run("user_types", "in_library")
}

#[test]
fn top_level() -> Result<()> {
    run("user_types", "top_level")
}

#[test]
fn wrap_unwrap() -> Result<()> {
    run("user_types", "wrap_unwrap")
}

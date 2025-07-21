// This file is generated automatically by infrastructure scripts (crates/codegen/testing/src/bindings_output.rs:12:18). Please don't edit by hand.

use anyhow::Result;

use crate::bindings_output::runner::run;

const T: &str = "user_types";

#[test]
fn in_contract() -> Result<()> {
    run(T, "in_contract")
}

#[test]
fn in_library() -> Result<()> {
    run(T, "in_library")
}

#[test]
fn top_level() -> Result<()> {
    run(T, "top_level")
}

#[test]
fn wrap_unwrap() -> Result<()> {
    run(T, "wrap_unwrap")
}

// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use anyhow::Result;

use crate::binder::runner::run;

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

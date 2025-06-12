// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use anyhow::Result;

use crate::binder::runner::run;

const T: &str = "enums";

#[test]
fn decls() -> Result<()> {
    run(T, "decls")
}

#[test]
fn in_params() -> Result<()> {
    run(T, "in_params")
}

#[test]
fn in_state_vars() -> Result<()> {
    run(T, "in_state_vars")
}

#[test]
fn sample() -> Result<()> {
    run(T, "sample")
}

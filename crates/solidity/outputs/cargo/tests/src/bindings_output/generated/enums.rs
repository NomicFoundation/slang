// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use anyhow::Result;

use crate::bindings_output::runner::run;

#[test]
fn decls() -> Result<()> {
    run("enums", "decls")
}

#[test]
fn in_params() -> Result<()> {
    run("enums", "in_params")
}

#[test]
fn in_state_vars() -> Result<()> {
    run("enums", "in_state_vars")
}

#[test]
fn sample() -> Result<()> {
    run("enums", "sample")
}

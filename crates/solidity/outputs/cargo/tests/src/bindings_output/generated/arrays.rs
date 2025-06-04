// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use anyhow::Result;

use crate::bindings_output::runner::run;

const T: &str = "arrays";

#[test]
fn byte_length() -> Result<()> {
    run(T, "byte_length")
}

#[test]
fn indexing() -> Result<()> {
    run(T, "indexing")
}

#[test]
fn length() -> Result<()> {
    run(T, "length")
}

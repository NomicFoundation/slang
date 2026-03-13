// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use anyhow::Result;

use crate::cst::cst_output::runner::run;

const T: &str = "MappingType";

#[test]
fn named_both() -> Result<()> {
    run(T, "named_both")
}

#[test]
fn named_key() -> Result<()> {
    run(T, "named_key")
}

#[test]
fn named_value() -> Result<()> {
    run(T, "named_value")
}

#[test]
fn stray_delimiter() -> Result<()> {
    run(T, "stray_delimiter")
}

#[test]
fn unnamed() -> Result<()> {
    run(T, "unnamed")
}

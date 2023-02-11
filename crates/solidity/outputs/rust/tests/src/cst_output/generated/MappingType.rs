// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use crate::cst_output::runner::run;
use anyhow::Result;

#[test]
fn named_both() -> Result<()> {
    return run("MappingType", "named_both");
}

#[test]
fn named_key() -> Result<()> {
    return run("MappingType", "named_key");
}

#[test]
fn named_value() -> Result<()> {
    return run("MappingType", "named_value");
}

#[test]
fn unnamed() -> Result<()> {
    return run("MappingType", "unnamed");
}

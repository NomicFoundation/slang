// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use crate::cst_output::runner::run;
use anyhow::Result;

#[test]
fn named_both() -> Result<()> {
    run("MappingType", "named_both")
}

#[test]
fn named_key() -> Result<()> {
    run("MappingType", "named_key")
}

#[test]
fn named_value() -> Result<()> {
    run("MappingType", "named_value")
}

#[test]
fn stray_delimiter() -> Result<()> {
    run("MappingType", "stray_delimiter")
}

#[test]
fn unnamed() -> Result<()> {
    run("MappingType", "unnamed")
}

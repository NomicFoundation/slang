// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use crate::cst_output::runner::run;
use anyhow::Result;

#[test]
fn _0() -> Result<()> {
    return run("Identifier", "_0");
}

#[test]
fn _a() -> Result<()> {
    return run("Identifier", "_a");
}

#[test]
fn areturn() -> Result<()> {
    return run("Identifier", "areturn");
}

#[test]
fn returna() -> Result<()> {
    return run("Identifier", "returna");
}

#[test]
fn returns() -> Result<()> {
    return run("Identifier", "returns");
}

#[test]
fn underscore_is_identifier() -> Result<()> {
    return run("Identifier", "underscore_is_identifier");
}

// This file is @generated automatically by infrastructure scripts. Please don't edit by hand.
#![allow(clippy::all)]

use crate::cst_output::runner::run;
use anyhow::Result;

#[test]
fn multiple() -> Result<()> {
    return run("AsciiStringLiteralsList", "multiple");
}

#[test]
fn single() -> Result<()> {
    return run("AsciiStringLiteralsList", "single");
}

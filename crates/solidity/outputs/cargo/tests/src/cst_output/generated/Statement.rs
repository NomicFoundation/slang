// This file is @generated automatically by infrastructure scripts. Please don't edit by hand.
#![allow(clippy::all)]

use crate::cst_output::runner::run;
use anyhow::Result;

#[test]
fn throw() -> Result<()> {
    return run("Statement", "throw");
}

#[test]
fn try_catch() -> Result<()> {
    return run("Statement", "try_catch");
}

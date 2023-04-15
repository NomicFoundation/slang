// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use crate::cst_output::runner::run;
use anyhow::Result;

#[test]
fn correct_length() -> Result<()> {
    return run("FixedBytesType", "correct_length");
}

#[test]
fn incorrect_length() -> Result<()> {
    return run("FixedBytesType", "incorrect_length");
}

#[test]
fn no_length() -> Result<()> {
    return run("FixedBytesType", "no_length");
}

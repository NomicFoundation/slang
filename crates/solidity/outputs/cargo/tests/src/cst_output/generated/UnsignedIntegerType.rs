// This file is @generated automatically by infrastructure scripts. Please don't edit by hand.
#![allow(clippy::all)]

use crate::cst_output::runner::run;
use anyhow::Result;

#[test]
fn correct_length() -> Result<()> {
    return run("UnsignedIntegerType", "correct_length");
}

#[test]
fn incorrect_length() -> Result<()> {
    return run("UnsignedIntegerType", "incorrect_length");
}

#[test]
fn no_length() -> Result<()> {
    return run("UnsignedIntegerType", "no_length");
}

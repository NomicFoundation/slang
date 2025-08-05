// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use anyhow::Result;

use crate::cst::cst_output::runner::run;

const T: &str = "TupleDeconstructionStatement";

#[test]
fn abi_decode_array_type() -> Result<()> {
    run(T, "abi_decode_array_type")
}

#[test]
fn abi_decode_singleton_type() -> Result<()> {
    run(T, "abi_decode_singleton_type")
}

#[test]
fn empty() -> Result<()> {
    run(T, "empty")
}

#[test]
fn ignored_members() -> Result<()> {
    run(T, "ignored_members")
}

#[test]
fn invalid_termination() -> Result<()> {
    run(T, "invalid_termination")
}

#[test]
fn with_location() -> Result<()> {
    run(T, "with_location")
}

#[test]
fn with_type() -> Result<()> {
    run(T, "with_type")
}

#[test]
fn with_type_and_location() -> Result<()> {
    run(T, "with_type_and_location")
}

#[test]
fn with_var() -> Result<()> {
    run(T, "with_var")
}

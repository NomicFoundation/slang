// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use anyhow::Result;

use crate::cst::cst_output::runner::run;

const T: &str = "TypeName";

#[test]
fn address_payable() -> Result<()> {
    run(T, "address_payable")
}

#[test]
fn byte() -> Result<()> {
    run(T, "byte")
}

#[test]
fn bytes_invalid_size_as_identifier() -> Result<()> {
    run(T, "bytes_invalid_size_as_identifier")
}

#[test]
fn bytes_no_size_reserved() -> Result<()> {
    run(T, "bytes_no_size_reserved")
}

#[test]
fn bytes_valid_size() -> Result<()> {
    run(T, "bytes_valid_size")
}

#[test]
fn int_invalid_size_as_identifier() -> Result<()> {
    run(T, "int_invalid_size_as_identifier")
}

#[test]
fn int_no_size() -> Result<()> {
    run(T, "int_no_size")
}

#[test]
fn int_valid_size() -> Result<()> {
    run(T, "int_valid_size")
}

#[test]
fn uint_invalid_size_as_identifier() -> Result<()> {
    run(T, "uint_invalid_size_as_identifier")
}

#[test]
fn uint_no_size() -> Result<()> {
    run(T, "uint_no_size")
}

#[test]
fn uint_valid_size() -> Result<()> {
    run(T, "uint_valid_size")
}

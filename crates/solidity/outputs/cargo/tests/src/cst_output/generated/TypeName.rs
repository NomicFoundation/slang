// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use crate::cst_output::runner::run;
use anyhow::Result;

#[test]
fn byte() -> Result<()> {
    run("TypeName", "byte")
}

#[test]
fn bytes_invalid_size_as_identifier() -> Result<()> {
    run("TypeName", "bytes_invalid_size_as_identifier")
}

#[test]
fn bytes_no_size_as_identifier() -> Result<()> {
    run("TypeName", "bytes_no_size_as_identifier")
}

#[test]
fn bytes_valid_size() -> Result<()> {
    run("TypeName", "bytes_valid_size")
}

#[test]
fn int_invalid_size_as_identifier() -> Result<()> {
    run("TypeName", "int_invalid_size_as_identifier")
}

#[test]
fn int_no_size() -> Result<()> {
    run("TypeName", "int_no_size")
}

#[test]
fn int_valid_size() -> Result<()> {
    run("TypeName", "int_valid_size")
}

#[test]
fn uint_invalid_size_as_identifier() -> Result<()> {
    run("TypeName", "uint_invalid_size_as_identifier")
}

#[test]
fn uint_no_size() -> Result<()> {
    run("TypeName", "uint_no_size")
}

#[test]
fn uint_valid_size() -> Result<()> {
    run("TypeName", "uint_valid_size")
}

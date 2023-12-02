// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use crate::cst_output::runner::run;
use anyhow::Result;

#[test]
fn abi_decode_array_type() -> Result<()> {
    run("TupleDeconstructionStatement", "abi_decode_array_type")
}

#[test]
fn abi_decode_singleton_type() -> Result<()> {
    run("TupleDeconstructionStatement", "abi_decode_singleton_type")
}

#[test]
fn empty() -> Result<()> {
    run("TupleDeconstructionStatement", "empty")
}

#[test]
fn ignored_members() -> Result<()> {
    run("TupleDeconstructionStatement", "ignored_members")
}

#[test]
fn invalid_termination() -> Result<()> {
    run("TupleDeconstructionStatement", "invalid_termination")
}

#[test]
fn with_location() -> Result<()> {
    run("TupleDeconstructionStatement", "with_location")
}

#[test]
fn with_type() -> Result<()> {
    run("TupleDeconstructionStatement", "with_type")
}

#[test]
fn with_type_and_location() -> Result<()> {
    run("TupleDeconstructionStatement", "with_type_and_location")
}

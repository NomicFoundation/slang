// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use anyhow::Result;

use crate::binder::runner::run;

const T: &str = "arrays";

#[test]
fn byte_array_indexing() -> Result<()> {
    run(T, "byte_array_indexing")
}

#[test]
fn byte_length() -> Result<()> {
    run(T, "byte_length")
}

#[test]
fn bytes_as_arrays() -> Result<()> {
    run(T, "bytes_as_arrays")
}

#[test]
fn bytes_index_access() -> Result<()> {
    run(T, "bytes_index_access")
}

#[test]
fn fixed_arrays() -> Result<()> {
    run(T, "fixed_arrays")
}

#[test]
fn indexing() -> Result<()> {
    run(T, "indexing")
}

#[test]
fn length() -> Result<()> {
    run(T, "length")
}

#[test]
fn static_length() -> Result<()> {
    run(T, "static_length")
}

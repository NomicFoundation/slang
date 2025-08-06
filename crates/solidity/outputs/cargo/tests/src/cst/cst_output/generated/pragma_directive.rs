// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use anyhow::Result;

use crate::cst::cst_output::runner::run;

const T: &str = "PragmaDirective";

#[test]
fn abi_coder() -> Result<()> {
    run(T, "abi_coder")
}

#[test]
fn experimental() -> Result<()> {
    run(T, "experimental")
}

#[test]
fn experimental_string() -> Result<()> {
    run(T, "experimental_string")
}

#[test]
fn version() -> Result<()> {
    run(T, "version")
}

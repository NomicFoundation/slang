// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use anyhow::Result;

use crate::cst_output::runner::run;

#[test]
fn abi_coder() -> Result<()> {
    run("PragmaDirective", "abi_coder")
}

#[test]
fn experimental() -> Result<()> {
    run("PragmaDirective", "experimental")
}

#[test]
fn experimental_string() -> Result<()> {
    run("PragmaDirective", "experimental_string")
}

#[test]
fn version() -> Result<()> {
    run("PragmaDirective", "version")
}

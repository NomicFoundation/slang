// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use crate::cst_output::runner::run;
use anyhow::Result;

#[test]
fn abi_coder() -> Result<()> {
    return run("PragmaDirective", "abi_coder");
}

#[test]
fn experimental() -> Result<()> {
    return run("PragmaDirective", "experimental");
}

#[test]
fn experimental_string() -> Result<()> {
    return run("PragmaDirective", "experimental_string");
}

#[test]
fn version() -> Result<()> {
    return run("PragmaDirective", "version");
}

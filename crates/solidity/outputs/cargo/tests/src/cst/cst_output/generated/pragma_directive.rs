// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use anyhow::Result;

use crate::cst::cst_output::runner::run;

const T: &str = "PragmaDirective";

#[test]
fn abi_coder_unkown_identifier() -> Result<()> {
    run(T, "abi_coder_unkown_identifier")
}

#[test]
fn abi_coder_v1() -> Result<()> {
    run(T, "abi_coder_v1")
}

#[test]
fn abi_coder_v2() -> Result<()> {
    run(T, "abi_coder_v2")
}

#[test]
fn experimental_abiencoderv2() -> Result<()> {
    run(T, "experimental_abiencoderv2")
}

#[test]
fn experimental_smtchecker() -> Result<()> {
    run(T, "experimental_smtchecker")
}

#[test]
fn experimental_string() -> Result<()> {
    run(T, "experimental_string")
}

#[test]
fn experimental_unknown_identifier() -> Result<()> {
    run(T, "experimental_unknown_identifier")
}

#[test]
fn version() -> Result<()> {
    run(T, "version")
}

// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use anyhow::Result;

use crate::cst::cst_output::runner::run;

const T: &str = "VariableDeclarationStatement";

#[test]
fn keyword_abicoder() -> Result<()> {
    run(T, "keyword_abicoder")
}

#[test]
fn keyword_bytes() -> Result<()> {
    run(T, "keyword_bytes")
}

#[test]
fn keyword_bytes1() -> Result<()> {
    run(T, "keyword_bytes1")
}

#[test]
fn keyword_bytes11() -> Result<()> {
    run(T, "keyword_bytes11")
}

#[test]
fn keyword_experimental() -> Result<()> {
    run(T, "keyword_experimental")
}

#[test]
fn keyword_solidity() -> Result<()> {
    run(T, "keyword_solidity")
}

#[test]
fn keyword_ufixed() -> Result<()> {
    run(T, "keyword_ufixed")
}

#[test]
fn keyword_ufixed184x80() -> Result<()> {
    run(T, "keyword_ufixed184x80")
}

#[test]
fn keyword_ufixed8x0() -> Result<()> {
    run(T, "keyword_ufixed8x0")
}

#[test]
fn keyword_ufixed8x8() -> Result<()> {
    run(T, "keyword_ufixed8x8")
}

#[test]
fn var() -> Result<()> {
    run(T, "var")
}

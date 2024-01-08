// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use anyhow::Result;

use crate::cst_output::runner::run;

#[test]
fn keyword_bytes() -> Result<()> {
    run("VariableDeclarationStatement", "keyword_bytes")
}

#[test]
fn keyword_bytes1() -> Result<()> {
    run("VariableDeclarationStatement", "keyword_bytes1")
}

#[test]
fn keyword_bytes11() -> Result<()> {
    run("VariableDeclarationStatement", "keyword_bytes11")
}

#[test]
fn keyword_ufixed() -> Result<()> {
    run("VariableDeclarationStatement", "keyword_ufixed")
}

#[test]
fn keyword_ufixed184x80() -> Result<()> {
    run("VariableDeclarationStatement", "keyword_ufixed184x80")
}

#[test]
fn keyword_ufixed8x0() -> Result<()> {
    run("VariableDeclarationStatement", "keyword_ufixed8x0")
}

#[test]
fn keyword_ufixed8x8() -> Result<()> {
    run("VariableDeclarationStatement", "keyword_ufixed8x8")
}

#[test]
fn var() -> Result<()> {
    run("VariableDeclarationStatement", "var")
}

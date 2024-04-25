// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use anyhow::Result;

use crate::cst_output::runner::run;

#[test]
fn colon_equal_separated() -> Result<()> {
    run("YulVariableDeclarationStatement", "colon_equal_separated")
}

#[test]
fn identifier_with_dots() -> Result<()> {
    run("YulVariableDeclarationStatement", "identifier_with_dots")
}

#[test]
fn keyword_bytes() -> Result<()> {
    run("YulVariableDeclarationStatement", "keyword_bytes")
}

#[test]
fn keyword_bytes1() -> Result<()> {
    run("YulVariableDeclarationStatement", "keyword_bytes1")
}

#[test]
fn keyword_bytes11() -> Result<()> {
    run("YulVariableDeclarationStatement", "keyword_bytes11")
}

#[test]
fn keyword_ufixed184x80() -> Result<()> {
    run("YulVariableDeclarationStatement", "keyword_ufixed184x80")
}

#[test]
fn keyword_ufixed8x0() -> Result<()> {
    run("YulVariableDeclarationStatement", "keyword_ufixed8x0")
}

#[test]
fn keyword_ufixed8x8() -> Result<()> {
    run("YulVariableDeclarationStatement", "keyword_ufixed8x8")
}

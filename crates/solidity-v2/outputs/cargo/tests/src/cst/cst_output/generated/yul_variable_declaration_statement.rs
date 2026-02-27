// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use anyhow::Result;

use crate::cst::cst_output::runner::run;

const T: &str = "YulVariableDeclarationStatement";

#[test]
fn colon_equal_separated() -> Result<()> {
    run(T, "colon_equal_separated")
}

#[test]
fn identifier_with_dots() -> Result<()> {
    run(T, "identifier_with_dots")
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
fn multiple_variables() -> Result<()> {
    run(T, "multiple_variables")
}

#[test]
fn multiple_variables_with_value() -> Result<()> {
    run(T, "multiple_variables_with_value")
}

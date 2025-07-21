// This file is generated automatically by infrastructure scripts (crates/codegen/testing/src/cst_output.rs:12:18). Please don't edit by hand.

use anyhow::Result;

use crate::cst_output::runner::run;

const T: &str = "UsingDirective";

#[test]
fn destructure_empty() -> Result<()> {
    run(T, "destructure_empty")
}

#[test]
fn destructure_multiple() -> Result<()> {
    run(T, "destructure_multiple")
}

#[test]
fn destructure_single() -> Result<()> {
    run(T, "destructure_single")
}

#[test]
fn path_named() -> Result<()> {
    run(T, "path_named")
}

#[test]
fn path_unnamed() -> Result<()> {
    run(T, "path_unnamed")
}

#[test]
fn user_defined_operator() -> Result<()> {
    run(T, "user_defined_operator")
}

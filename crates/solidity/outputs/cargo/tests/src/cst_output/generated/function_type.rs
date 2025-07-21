// This file is generated automatically by infrastructure scripts (crates/codegen/testing/src/cst_output.rs). Please don't edit by hand.

use anyhow::Result;

use crate::cst_output::runner::run;

const T: &str = "FunctionType";

#[test]
fn basic() -> Result<()> {
    run(T, "basic")
}

#[test]
fn constant_state_mutability() -> Result<()> {
    run(T, "constant_state_mutability")
}

#[test]
fn pure_state_mutability() -> Result<()> {
    run(T, "pure_state_mutability")
}

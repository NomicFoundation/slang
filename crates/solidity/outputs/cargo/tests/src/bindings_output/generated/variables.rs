// This file is generated automatically by infrastructure scripts (crates/codegen/testing/src/bindings_output.rs:12:18). Please don't edit by hand.

use anyhow::Result;

use crate::bindings_output::runner::run;

const T: &str = "variables";

#[test]
fn destructuring() -> Result<()> {
    run(T, "destructuring")
}

#[test]
fn local_vars() -> Result<()> {
    run(T, "local_vars")
}

#[test]
fn params() -> Result<()> {
    run(T, "params")
}

#[test]
fn state_vars() -> Result<()> {
    run(T, "state_vars")
}

#[test]
fn tuple_assignment() -> Result<()> {
    run(T, "tuple_assignment")
}

#[test]
fn var_declaration() -> Result<()> {
    run(T, "var_declaration")
}

#[test]
fn var_tuple_declaration() -> Result<()> {
    run(T, "var_tuple_declaration")
}

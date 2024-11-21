// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use anyhow::Result;

use crate::bindings_output::runner::run;

#[test]
fn address() -> Result<()> {
    run("built_ins", "address")
}

#[test]
fn array_push() -> Result<()> {
    run("built_ins", "array_push")
}

#[test]
fn arrays() -> Result<()> {
    run("built_ins", "arrays")
}

#[test]
fn bytes() -> Result<()> {
    run("built_ins", "bytes")
}

#[test]
fn function_type() -> Result<()> {
    run("built_ins", "function_type")
}

#[test]
fn functions() -> Result<()> {
    run("built_ins", "functions")
}

#[test]
fn global_properties() -> Result<()> {
    run("built_ins", "global_properties")
}

#[test]
fn shadowing() -> Result<()> {
    run("built_ins", "shadowing")
}

#[test]
fn this() -> Result<()> {
    run("built_ins", "this")
}

#[test]
fn this_as_address() -> Result<()> {
    run("built_ins", "this_as_address")
}

#[test]
fn type_expr() -> Result<()> {
    run("built_ins", "type_expr")
}

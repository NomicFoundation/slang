// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use anyhow::Result;

use crate::bindings::bindings_output::runner::run;

const T: &str = "built_ins";

#[test]
fn abi_decode() -> Result<()> {
    run(T, "abi_decode")
}

#[test]
fn address() -> Result<()> {
    run(T, "address")
}

#[test]
fn address_payable() -> Result<()> {
    run(T, "address_payable")
}

#[test]
fn array_push() -> Result<()> {
    run(T, "array_push")
}

#[test]
fn arrays() -> Result<()> {
    run(T, "arrays")
}

#[test]
fn bytes() -> Result<()> {
    run(T, "bytes")
}

#[test]
fn function_type() -> Result<()> {
    run(T, "function_type")
}

#[test]
fn functions() -> Result<()> {
    run(T, "functions")
}

#[test]
fn global_properties() -> Result<()> {
    run(T, "global_properties")
}

#[test]
fn instance_as_address() -> Result<()> {
    run(T, "instance_as_address")
}

#[test]
fn msg_sender() -> Result<()> {
    run(T, "msg_sender")
}

#[test]
fn now() -> Result<()> {
    run(T, "now")
}

#[test]
fn shadowing() -> Result<()> {
    run(T, "shadowing")
}

#[test]
fn this() -> Result<()> {
    run(T, "this")
}

#[test]
fn this_as_address() -> Result<()> {
    run(T, "this_as_address")
}

#[test]
fn type_expr() -> Result<()> {
    run(T, "type_expr")
}

#[test]
fn yul_built_in_doesn_t_bind() -> Result<()> {
    run(T, "yul_built_in_doesn_t_bind")
}

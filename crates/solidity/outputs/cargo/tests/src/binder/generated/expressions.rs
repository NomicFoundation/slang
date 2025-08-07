// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use anyhow::Result;

use crate::binder::runner::run;

const T: &str = "expressions";

#[test]
fn basic() -> Result<()> {
    run(T, "basic")
}

#[test]
fn binary_operators() -> Result<()> {
    run(T, "binary_operators")
}

#[test]
fn call_options() -> Result<()> {
    run(T, "call_options")
}

#[test]
fn elementary_casting() -> Result<()> {
    run(T, "elementary_casting")
}

#[test]
fn emit_named_args() -> Result<()> {
    run(T, "emit_named_args")
}

#[test]
fn funcalls() -> Result<()> {
    run(T, "funcalls")
}

#[test]
fn funcalls_named_args() -> Result<()> {
    run(T, "funcalls_named_args")
}

#[test]
fn funcalls_output() -> Result<()> {
    run(T, "funcalls_output")
}

#[test]
fn legacy_call_options() -> Result<()> {
    run(T, "legacy_call_options")
}

#[test]
fn literal_address() -> Result<()> {
    run(T, "literal_address")
}

#[test]
fn literal_booleans() -> Result<()> {
    run(T, "literal_booleans")
}

#[test]
fn literal_integers() -> Result<()> {
    run(T, "literal_integers")
}

#[test]
fn new_output() -> Result<()> {
    run(T, "new_output")
}

#[test]
fn new_with_legacy_call_options() -> Result<()> {
    run(T, "new_with_legacy_call_options")
}

#[test]
fn revert_named_args() -> Result<()> {
    run(T, "revert_named_args")
}

#[test]
fn type_expr() -> Result<()> {
    run(T, "type_expr")
}

#[test]
fn type_expr_integers() -> Result<()> {
    run(T, "type_expr_integers")
}

#[test]
fn type_expr_minmax() -> Result<()> {
    run(T, "type_expr_minmax")
}

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
fn conditional() -> Result<()> {
    run(T, "conditional")
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
fn funcalls_ambiguous_overload() -> Result<()> {
    run(T, "funcalls_ambiguous_overload")
}

#[test]
fn funcalls_named_args() -> Result<()> {
    run(T, "funcalls_named_args")
}

#[test]
fn funcalls_named_overloads() -> Result<()> {
    run(T, "funcalls_named_overloads")
}

#[test]
fn funcalls_nested_overload() -> Result<()> {
    run(T, "funcalls_nested_overload")
}

#[test]
fn funcalls_output() -> Result<()> {
    run(T, "funcalls_output")
}

#[test]
fn funcalls_overload() -> Result<()> {
    run(T, "funcalls_overload")
}

#[test]
fn funcalls_overload_indexed() -> Result<()> {
    run(T, "funcalls_overload_indexed")
}

#[test]
fn funcalls_overload_member_access() -> Result<()> {
    run(T, "funcalls_overload_member_access")
}

#[test]
fn funcalls_overload_options() -> Result<()> {
    run(T, "funcalls_overload_options")
}

#[test]
fn funcalls_overload_this() -> Result<()> {
    run(T, "funcalls_overload_this")
}

#[test]
fn funcalls_overload_using_for() -> Result<()> {
    run(T, "funcalls_overload_using_for")
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
fn literal_arithmetic() -> Result<()> {
    run(T, "literal_arithmetic")
}

#[test]
fn literal_booleans() -> Result<()> {
    run(T, "literal_booleans")
}

#[test]
fn literal_implicit_conversion() -> Result<()> {
    run(T, "literal_implicit_conversion")
}

#[test]
fn literal_integers() -> Result<()> {
    run(T, "literal_integers")
}

#[test]
fn member_inherits_data_location() -> Result<()> {
    run(T, "member_inherits_data_location")
}

#[test]
fn new_array() -> Result<()> {
    run(T, "new_array")
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
fn revert_shadowing() -> Result<()> {
    run(T, "revert_shadowing")
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

#[test]
fn uint160_address_casting() -> Result<()> {
    run(T, "uint160_address_casting")
}

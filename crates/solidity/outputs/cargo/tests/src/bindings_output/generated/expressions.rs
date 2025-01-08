// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use anyhow::Result;

use crate::bindings_output::runner::run;

#[test]
fn binary_operators() -> Result<()> {
    run("expressions", "binary_operators")
}

#[test]
fn call_options() -> Result<()> {
    run("expressions", "call_options")
}

#[test]
fn elementary_casting() -> Result<()> {
    run("expressions", "elementary_casting")
}

#[test]
fn emit_named_args() -> Result<()> {
    run("expressions", "emit_named_args")
}

#[test]
fn funcalls() -> Result<()> {
    run("expressions", "funcalls")
}

#[test]
fn funcalls_named_args() -> Result<()> {
    run("expressions", "funcalls_named_args")
}

#[test]
fn funcalls_output() -> Result<()> {
    run("expressions", "funcalls_output")
}

#[test]
fn legacy_call_options() -> Result<()> {
    run("expressions", "legacy_call_options")
}

#[test]
fn literal_address() -> Result<()> {
    run("expressions", "literal_address")
}

#[test]
fn literal_integers() -> Result<()> {
    run("expressions", "literal_integers")
}

#[test]
fn new_output() -> Result<()> {
    run("expressions", "new_output")
}

#[test]
fn revert_named_args() -> Result<()> {
    run("expressions", "revert_named_args")
}

#[test]
fn type_expr() -> Result<()> {
    run("expressions", "type_expr")
}

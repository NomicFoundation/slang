// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use anyhow::Result;

use crate::bindings_output::runner::run;

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

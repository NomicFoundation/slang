// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use anyhow::Result;

use crate::bindings_output::runner::run;

#[test]
fn funcalls() -> Result<()> {
    run("expressions", "funcalls")
}

#[test]
fn funcalls_named_args() -> Result<()> {
    run("expressions", "funcalls_named_args")
}

#[test]
fn named_args() -> Result<()> {
    run("expressions", "named_args")
}

#[test]
fn type_expr() -> Result<()> {
    run("expressions", "type_expr")
}

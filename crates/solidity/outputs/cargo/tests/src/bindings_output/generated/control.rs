// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use anyhow::Result;

use crate::bindings_output::runner::run;

#[test]
fn do_while() -> Result<()> {
    run("control", "do_while")
}

#[test]
fn emit_event() -> Result<()> {
    run("control", "emit_event")
}

#[test]
fn for_empty_clauses() -> Result<()> {
    run("control", "for_empty_clauses")
}

#[test]
fn for_empty_cond() -> Result<()> {
    run("control", "for_empty_cond")
}

#[test]
fn for_empty_init() -> Result<()> {
    run("control", "for_empty_init")
}

#[test]
fn for_expr_init() -> Result<()> {
    run("control", "for_expr_init")
}

#[test]
fn for_stmt() -> Result<()> {
    run("control", "for_stmt")
}

#[test]
fn for_tuple_init() -> Result<()> {
    run("control", "for_tuple_init")
}

#[test]
fn if_else() -> Result<()> {
    run("control", "if_else")
}

#[test]
fn return_stmt() -> Result<()> {
    run("control", "return_stmt")
}

#[test]
fn try_catch() -> Result<()> {
    run("control", "try_catch")
}

#[test]
fn try_stmt() -> Result<()> {
    run("control", "try_stmt")
}

#[test]
fn unchecked() -> Result<()> {
    run("control", "unchecked")
}

#[test]
fn while_stmt() -> Result<()> {
    run("control", "while_stmt")
}

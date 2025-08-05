// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use anyhow::Result;

use crate::binder::runner::run;

const T: &str = "control";

#[test]
fn do_while() -> Result<()> {
    run(T, "do_while")
}

#[test]
fn emit_event() -> Result<()> {
    run(T, "emit_event")
}

#[test]
fn for_empty_clauses() -> Result<()> {
    run(T, "for_empty_clauses")
}

#[test]
fn for_empty_cond() -> Result<()> {
    run(T, "for_empty_cond")
}

#[test]
fn for_empty_init() -> Result<()> {
    run(T, "for_empty_init")
}

#[test]
fn for_expr_init() -> Result<()> {
    run(T, "for_expr_init")
}

#[test]
fn for_stmt() -> Result<()> {
    run(T, "for_stmt")
}

#[test]
fn for_tuple_init() -> Result<()> {
    run(T, "for_tuple_init")
}

#[test]
fn if_else() -> Result<()> {
    run(T, "if_else")
}

#[test]
fn return_stmt() -> Result<()> {
    run(T, "return_stmt")
}

#[test]
fn try_catch() -> Result<()> {
    run(T, "try_catch")
}

#[test]
fn try_stmt() -> Result<()> {
    run(T, "try_stmt")
}

#[test]
fn unchecked() -> Result<()> {
    run(T, "unchecked")
}

#[test]
fn while_stmt() -> Result<()> {
    run(T, "while_stmt")
}

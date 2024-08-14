// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use anyhow::Result;

use crate::bindings_output::runner::run;

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
fn while_stmt() -> Result<()> {
    run("control", "while_stmt")
}

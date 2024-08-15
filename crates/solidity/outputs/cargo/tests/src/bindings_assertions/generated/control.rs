// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use anyhow::Result;

use crate::bindings_assertions::runner::run;

#[test]
fn do_while() -> Result<()> {
    run("control", "do_while")
}

#[test]
fn emit_event() -> Result<()> {
    run("control", "emit_event")
}

#[test]
fn for_empty_init_or_cond() -> Result<()> {
    run("control", "for_empty_init_or_cond")
}

#[test]
fn for_stmt() -> Result<()> {
    run("control", "for_stmt")
}

#[test]
fn for_tuple_decon() -> Result<()> {
    run("control", "for_tuple_decon")
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
fn try_stmt() -> Result<()> {
    run("control", "try_stmt")
}

#[test]
fn while_stmt() -> Result<()> {
    run("control", "while_stmt")
}

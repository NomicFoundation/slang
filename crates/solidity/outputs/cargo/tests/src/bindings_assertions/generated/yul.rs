// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use anyhow::Result;

use crate::bindings_assertions::runner::run;

#[test]
fn blocks() -> Result<()> {
    run("yul", "blocks")
}

#[test]
fn fun_scopes() -> Result<()> {
    run("yul", "fun_scopes")
}

#[test]
fn functions() -> Result<()> {
    run("yul", "functions")
}

#[test]
fn if_stmt() -> Result<()> {
    run("yul", "if_stmt")
}

#[test]
fn labels() -> Result<()> {
    run("yul", "labels")
}

#[test]
fn stack_assign() -> Result<()> {
    run("yul", "stack_assign")
}

#[test]
fn variables() -> Result<()> {
    run("yul", "variables")
}

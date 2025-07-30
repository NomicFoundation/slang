// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use anyhow::Result;

use crate::bindings::bindings_output::runner::run;

const T: &str = "libraries";

#[test]
fn constants() -> Result<()> {
    run(T, "constants")
}

#[test]
fn modifiers() -> Result<()> {
    run(T, "modifiers")
}

#[test]
fn modifiers_scope() -> Result<()> {
    run(T, "modifiers_scope")
}

#[test]
fn propagate_dynamic_scope() -> Result<()> {
    run(T, "propagate_dynamic_scope")
}

#[test]
fn visibility() -> Result<()> {
    run(T, "visibility")
}

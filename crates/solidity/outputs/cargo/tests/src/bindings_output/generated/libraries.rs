// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use anyhow::Result;

use crate::bindings_output::runner::run;

#[test]
fn constants() -> Result<()> {
    run("libraries", "constants")
}

#[test]
fn modifiers() -> Result<()> {
    run("libraries", "modifiers")
}

#[test]
fn modifiers_scope() -> Result<()> {
    run("libraries", "modifiers_scope")
}

#[test]
fn propagate_dynamic_scope() -> Result<()> {
    run("libraries", "propagate_dynamic_scope")
}

#[test]
fn visibility() -> Result<()> {
    run("libraries", "visibility")
}

// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use anyhow::Result;

use crate::bindings_assertions::runner::run;

#[test]
fn fixed() -> Result<()> {
    run("arrays", "fixed")
}

#[test]
fn nested_custom() -> Result<()> {
    run("arrays", "nested_custom")
}

#[test]
fn simple() -> Result<()> {
    run("arrays", "simple")
}

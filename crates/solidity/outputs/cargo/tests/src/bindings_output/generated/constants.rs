// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use anyhow::Result;

use crate::bindings_output::runner::run;

#[test]
fn in_contract() -> Result<()> {
    run("constants", "in_contract")
}

#[test]
fn top_level() -> Result<()> {
    run("constants", "top_level")
}

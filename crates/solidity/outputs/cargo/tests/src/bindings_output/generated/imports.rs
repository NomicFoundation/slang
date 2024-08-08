// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use anyhow::Result;

use crate::bindings_output::runner::run;

#[test]
fn alias_import() -> Result<()> {
    run("imports", "alias_import")
}

#[test]
fn default() -> Result<()> {
    run("imports", "default")
}

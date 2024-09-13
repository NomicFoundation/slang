// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use anyhow::Result;

use crate::bindings_assertions::runner::run;

#[test]
fn deconstruction_imports() -> Result<()> {
    run("imports", "deconstruction_imports")
}

#[test]
fn named_imports() -> Result<()> {
    run("imports", "named_imports")
}

#[test]
fn path_imports() -> Result<()> {
    run("imports", "path_imports")
}

#[test]
fn unresolved_aliases() -> Result<()> {
    run("imports", "unresolved_aliases")
}

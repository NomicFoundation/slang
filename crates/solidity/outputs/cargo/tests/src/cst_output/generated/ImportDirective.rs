// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use crate::cst_output::runner::run;
use anyhow::Result;

#[test]
fn destructure_import_empty() -> Result<()> {
    run("ImportDirective", "destructure_import_empty")
}

#[test]
fn destructure_import_multiple() -> Result<()> {
    run("ImportDirective", "destructure_import_multiple")
}

#[test]
fn destructure_import_single() -> Result<()> {
    run("ImportDirective", "destructure_import_single")
}

#[test]
fn named_import() -> Result<()> {
    run("ImportDirective", "named_import")
}

#[test]
fn path_import() -> Result<()> {
    run("ImportDirective", "path_import")
}

#[test]
fn path_import_with_alias() -> Result<()> {
    run("ImportDirective", "path_import_with_alias")
}

// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use crate::cst_output::runner::run;
use anyhow::Result;

#[test]
fn destructure_import_empty() -> Result<()> {
    return run("ImportDirective", "destructure_import_empty");
}

#[test]
fn destructure_import_multiple() -> Result<()> {
    return run("ImportDirective", "destructure_import_multiple");
}

#[test]
fn destructure_import_single() -> Result<()> {
    return run("ImportDirective", "destructure_import_single");
}

#[test]
fn named_import() -> Result<()> {
    return run("ImportDirective", "named_import");
}

#[test]
fn path_import() -> Result<()> {
    return run("ImportDirective", "path_import");
}

#[test]
fn path_import_with_alias() -> Result<()> {
    return run("ImportDirective", "path_import_with_alias");
}

// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use anyhow::Result;

use crate::cst::cst_output::runner::run;

const T: &str = "ImportDirective";

#[test]
fn destructure_import_empty() -> Result<()> {
    run(T, "destructure_import_empty")
}

#[test]
fn destructure_import_multiple() -> Result<()> {
    run(T, "destructure_import_multiple")
}

#[test]
fn destructure_import_single() -> Result<()> {
    run(T, "destructure_import_single")
}

#[test]
fn named_import() -> Result<()> {
    run(T, "named_import")
}

#[test]
fn path_import() -> Result<()> {
    run(T, "path_import")
}

#[test]
fn path_import_with_alias() -> Result<()> {
    run(T, "path_import_with_alias")
}

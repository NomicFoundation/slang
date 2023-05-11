// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use crate::cst_output::runner::run;
use anyhow::Result;

#[test]
fn asterisk_import() -> Result<()> {
    return run("ImportDirective", "asterisk_import");
}

#[test]
fn selective_import_multiple() -> Result<()> {
    return run("ImportDirective", "selective_import_multiple");
}

#[test]
fn selective_import_single() -> Result<()> {
    return run("ImportDirective", "selective_import_single");
}

#[test]
fn simple_import() -> Result<()> {
    return run("ImportDirective", "simple_import");
}

#[test]
fn simple_import_with_alias() -> Result<()> {
    return run("ImportDirective", "simple_import_with_alias");
}

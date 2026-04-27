// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use anyhow::Result;

use crate::diagnostics_output::runner::run;

const T: &str = "compilation";

#[test]
fn missing_file() -> Result<()> {
    run(T, "missing_file")
}

#[test]
fn unresolved_import() -> Result<()> {
    run(T, "unresolved_import")
}

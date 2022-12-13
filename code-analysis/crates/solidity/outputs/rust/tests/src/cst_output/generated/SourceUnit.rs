// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use crate::cst_output::run;
use anyhow::Result;

#[test]
fn empty_file() -> Result<()> {
    return run("SourceUnit", "empty_file");
}

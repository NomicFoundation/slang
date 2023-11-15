// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use crate::cst_output::runner::run;
use anyhow::Result;

#[test]
fn throw() -> Result<()> {
    return run("ThrowStatement", "throw");
}

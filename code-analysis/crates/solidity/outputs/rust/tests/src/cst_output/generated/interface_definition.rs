// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use crate::cst_output::run;
use anyhow::Result;

#[test]
fn sample_counter() -> Result<()> {
    return run("interface_definition", "sample_counter");
}

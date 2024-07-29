// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use anyhow::Result;

use crate::bindings_assertions::runner::run;

#[test]
fn visibility() -> Result<()> {
    run("functions", "visibility")
}

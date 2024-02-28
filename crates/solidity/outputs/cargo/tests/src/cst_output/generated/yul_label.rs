// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use anyhow::Result;

use crate::cst_output::runner::run;

#[test]
fn single_label() -> Result<()> {
    run("YulLabel", "single_label")
}

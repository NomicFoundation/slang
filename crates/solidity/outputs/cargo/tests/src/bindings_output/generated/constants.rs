// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use anyhow::Result;

use crate::bindings_output::runner::run;

const T: &str = "constants";

#[test]
fn in_contract() -> Result<()> {
    run(T, "in_contract")
}

#[test]
fn top_level() -> Result<()> {
    run(T, "top_level")
}

// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use anyhow::Result;

use crate::cst::cst_output::runner::run;

const T: &str = "ModifierDefinition";

#[test]
fn override_attr() -> Result<()> {
    run(T, "override_attr")
}

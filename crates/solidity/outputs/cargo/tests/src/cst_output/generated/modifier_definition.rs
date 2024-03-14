// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use anyhow::Result;

use crate::cst_output::runner::run;

#[test]
fn override_attr() -> Result<()> {
    run("ModifierDefinition", "override_attr")
}

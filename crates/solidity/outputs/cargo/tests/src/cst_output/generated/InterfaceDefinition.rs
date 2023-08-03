// This file is @generated automatically by infrastructure scripts. Please don't edit by hand.
#![allow(clippy::all)]

use crate::cst_output::runner::run;
use anyhow::Result;

#[test]
fn sample_counter() -> Result<()> {
    return run("InterfaceDefinition", "sample_counter");
}

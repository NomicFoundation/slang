// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use anyhow::Result;

use crate::bindings::bindings_output::runner::run;

const T: &str = "functions";

#[test]
fn definition_of_parameters() -> Result<()> {
    run(T, "definition_of_parameters")
}

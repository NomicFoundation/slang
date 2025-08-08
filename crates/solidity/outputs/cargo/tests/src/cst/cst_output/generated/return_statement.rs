// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use anyhow::Result;

use crate::cst::cst_output::runner::run;

const T: &str = "ReturnStatement";

#[test]
fn invalid_terminator() -> Result<()> {
    run(T, "invalid_terminator")
}

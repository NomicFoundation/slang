// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use anyhow::Result;

use crate::cst::cst_output::runner::run;

const T: &str = "YulLabel";

#[test]
fn single_label() -> Result<()> {
    run(T, "single_label")
}

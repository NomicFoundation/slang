// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use crate::cst_output::run;
use anyhow::Result;

#[test]
fn operator_associativity() -> Result<()> {
    return run("exponentiation_expression", "operator_associativity");
}

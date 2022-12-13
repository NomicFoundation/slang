// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use crate::cst_output::run;
use anyhow::Result;

#[test]
fn empty_contract() -> Result<()> {
    return run("ContractDefinition", "empty_contract");
}

#[test]
fn missing_field_type() -> Result<()> {
    return run("ContractDefinition", "missing_field_type");
}

#[test]
fn unterminated_body() -> Result<()> {
    return run("ContractDefinition", "unterminated_body");
}

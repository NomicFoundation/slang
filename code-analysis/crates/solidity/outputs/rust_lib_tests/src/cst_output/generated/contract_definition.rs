use crate::cst_output::run;
use anyhow::Result;

#[test]
fn empty_contract() -> Result<()> {
    return run("contract_definition", "empty_contract");
}

#[test]
fn missing_field_type() -> Result<()> {
    return run("contract_definition", "missing_field_type");
}

#[test]
fn unterminated_body() -> Result<()> {
    return run("contract_definition", "unterminated_body");
}

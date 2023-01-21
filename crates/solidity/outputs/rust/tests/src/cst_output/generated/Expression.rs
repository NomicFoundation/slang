// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use crate::cst_output::runner::run;
use anyhow::Result;

#[test]
fn add_mul() -> Result<()> {
    return run("Expression", "add_mul");
}

#[test]
fn address_call() -> Result<()> {
    return run("Expression", "address_call");
}

#[test]
fn address_payable_call() -> Result<()> {
    return run("Expression", "address_payable_call");
}

#[test]
fn exponentiation_operator_associativity() -> Result<()> {
    return run("Expression", "exponentiation_operator_associativity");
}

#[test]
fn function_call() -> Result<()> {
    return run("Expression", "function_call");
}

#[test]
fn function_call_chain() -> Result<()> {
    return run("Expression", "function_call_chain");
}

#[test]
fn function_call_member_access() -> Result<()> {
    return run("Expression", "function_call_member_access");
}

#[test]
fn identifier_call() -> Result<()> {
    return run("Expression", "identifier_call");
}

#[test]
fn index_access() -> Result<()> {
    return run("Expression", "index_access");
}

#[test]
fn index_access_chain() -> Result<()> {
    return run("Expression", "index_access_chain");
}

#[test]
fn member_access() -> Result<()> {
    return run("Expression", "member_access");
}

#[test]
fn member_access_chain() -> Result<()> {
    return run("Expression", "member_access_chain");
}

#[test]
fn member_access_function_call() -> Result<()> {
    return run("Expression", "member_access_function_call");
}

#[test]
fn member_access_index_access() -> Result<()> {
    return run("Expression", "member_access_index_access");
}

#[test]
fn payable_call() -> Result<()> {
    return run("Expression", "payable_call");
}

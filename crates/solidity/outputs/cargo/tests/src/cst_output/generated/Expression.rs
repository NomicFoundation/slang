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
fn conditional_expression_identifier_base() -> Result<()> {
    return run("Expression", "conditional_expression_identifier_base");
}

#[test]
fn conditional_expression_nested_base() -> Result<()> {
    return run("Expression", "conditional_expression_nested_base");
}

#[test]
fn conditional_expression_nested_conditions() -> Result<()> {
    return run("Expression", "conditional_expression_nested_conditions");
}

#[test]
fn conditional_expression_recursive() -> Result<()> {
    return run("Expression", "conditional_expression_recursive");
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
fn function_call_argument_has_type_name_as_prefix() -> Result<()> {
    return run(
        "Expression",
        "function_call_argument_has_type_name_as_prefix",
    );
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
fn function_call_options() -> Result<()> {
    return run("Expression", "function_call_options");
}

#[test]
fn function_call_options_split() -> Result<()> {
    return run("Expression", "function_call_options_split");
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
fn index_missing() -> Result<()> {
    return run("Expression", "index_missing");
}

#[test]
fn index_slice_end() -> Result<()> {
    return run("Expression", "index_slice_end");
}

#[test]
fn index_slice_start() -> Result<()> {
    return run("Expression", "index_slice_start");
}

#[test]
fn index_slice_start_end() -> Result<()> {
    return run("Expression", "index_slice_start_end");
}

#[test]
fn index_slice_unbounded() -> Result<()> {
    return run("Expression", "index_slice_unbounded");
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
fn overlapping_operators() -> Result<()> {
    return run("Expression", "overlapping_operators");
}

#[test]
fn payable_call() -> Result<()> {
    return run("Expression", "payable_call");
}

#[test]
fn postfix_decrement() -> Result<()> {
    return run("Expression", "postfix_decrement");
}

#[test]
fn prefix_decrement() -> Result<()> {
    return run("Expression", "prefix_decrement");
}

#[test]
fn prefix_minus() -> Result<()> {
    return run("Expression", "prefix_minus");
}

#[test]
fn prefix_plus() -> Result<()> {
    return run("Expression", "prefix_plus");
}

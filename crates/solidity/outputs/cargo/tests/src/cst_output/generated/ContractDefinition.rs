// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use crate::cst_output::runner::run;
use anyhow::Result;

#[test]
fn abstract_contract() -> Result<()> {
    return run("ContractDefinition", "abstract_contract");
}

#[test]
fn empty_contract() -> Result<()> {
    return run("ContractDefinition", "empty_contract");
}

#[test]
fn header_comment() -> Result<()> {
    return run("ContractDefinition", "header_comment");
}

#[test]
fn inheritence_specifier() -> Result<()> {
    return run("ContractDefinition", "inheritence_specifier");
}

#[test]
fn member_constructor_definition() -> Result<()> {
    return run("ContractDefinition", "member_constructor_definition");
}

#[test]
fn member_enum_definition() -> Result<()> {
    return run("ContractDefinition", "member_enum_definition");
}

#[test]
fn member_error_definition() -> Result<()> {
    return run("ContractDefinition", "member_error_definition");
}

#[test]
fn member_event_definition() -> Result<()> {
    return run("ContractDefinition", "member_event_definition");
}

#[test]
fn member_fallback_function_definition() -> Result<()> {
    return run("ContractDefinition", "member_fallback_function_definition");
}

#[test]
fn member_function_definition() -> Result<()> {
    return run("ContractDefinition", "member_function_definition");
}

#[test]
fn member_modifier_definition() -> Result<()> {
    return run("ContractDefinition", "member_modifier_definition");
}

#[test]
fn member_receive_function_definition() -> Result<()> {
    return run("ContractDefinition", "member_receive_function_definition");
}

#[test]
fn member_state_variable_declaration() -> Result<()> {
    return run("ContractDefinition", "member_state_variable_declaration");
}

#[test]
fn member_struct_definition() -> Result<()> {
    return run("ContractDefinition", "member_struct_definition");
}

#[test]
fn member_unnamed_function_definition() -> Result<()> {
    return run("ContractDefinition", "member_unnamed_function_definition");
}

#[test]
fn member_user_defined_value_type_definition() -> Result<()> {
    return run(
        "ContractDefinition",
        "member_user_defined_value_type_definition",
    );
}

#[test]
fn member_using_directive() -> Result<()> {
    return run("ContractDefinition", "member_using_directive");
}

#[test]
fn missing_field_type() -> Result<()> {
    return run("ContractDefinition", "missing_field_type");
}

#[test]
fn unterminated_body() -> Result<()> {
    return run("ContractDefinition", "unterminated_body");
}

#[test]
fn zero_length_input() -> Result<()> {
    return run("ContractDefinition", "zero_length_input");
}

// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use anyhow::Result;

use crate::cst_output::runner::run;

#[test]
fn abstract_contract() -> Result<()> {
    run("ContractDefinition", "abstract_contract")
}

#[test]
fn empty_contract() -> Result<()> {
    run("ContractDefinition", "empty_contract")
}

#[test]
fn function_multiple_delimiters() -> Result<()> {
    run("ContractDefinition", "function_multiple_delimiters")
}

#[test]
fn header_comment() -> Result<()> {
    run("ContractDefinition", "header_comment")
}

#[test]
fn inheritence_specifier() -> Result<()> {
    run("ContractDefinition", "inheritence_specifier")
}

#[test]
fn member_constructor_definition() -> Result<()> {
    run("ContractDefinition", "member_constructor_definition")
}

#[test]
fn member_enum_definition() -> Result<()> {
    run("ContractDefinition", "member_enum_definition")
}

#[test]
fn member_error_definition() -> Result<()> {
    run("ContractDefinition", "member_error_definition")
}

#[test]
fn member_event_definition() -> Result<()> {
    run("ContractDefinition", "member_event_definition")
}

#[test]
fn member_fallback_function_definition() -> Result<()> {
    run("ContractDefinition", "member_fallback_function_definition")
}

#[test]
fn member_function_definition() -> Result<()> {
    run("ContractDefinition", "member_function_definition")
}

#[test]
fn member_modifier_definition() -> Result<()> {
    run("ContractDefinition", "member_modifier_definition")
}

#[test]
fn member_receive_function_definition() -> Result<()> {
    run("ContractDefinition", "member_receive_function_definition")
}

#[test]
fn member_state_variable_declaration() -> Result<()> {
    run("ContractDefinition", "member_state_variable_declaration")
}

#[test]
fn member_struct_definition() -> Result<()> {
    run("ContractDefinition", "member_struct_definition")
}

#[test]
fn member_unnamed_function_definition() -> Result<()> {
    run("ContractDefinition", "member_unnamed_function_definition")
}

#[test]
fn member_unnamed_function_with_attrs_definition() -> Result<()> {
    run(
        "ContractDefinition",
        "member_unnamed_function_with_attrs_definition",
    )
}

#[test]
fn member_user_defined_value_type_definition() -> Result<()> {
    run(
        "ContractDefinition",
        "member_user_defined_value_type_definition",
    )
}

#[test]
fn member_using_directive() -> Result<()> {
    run("ContractDefinition", "member_using_directive")
}

#[test]
fn missing_field_type() -> Result<()> {
    run("ContractDefinition", "missing_field_type")
}

#[test]
fn recovery_testbed() -> Result<()> {
    run("ContractDefinition", "recovery_testbed")
}

#[test]
fn unterminated_body() -> Result<()> {
    run("ContractDefinition", "unterminated_body")
}

#[test]
fn zero_length_input() -> Result<()> {
    run("ContractDefinition", "zero_length_input")
}

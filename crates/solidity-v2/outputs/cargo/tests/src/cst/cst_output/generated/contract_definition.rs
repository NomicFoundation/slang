// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use anyhow::Result;

use crate::cst::cst_output::runner::run;

const T: &str = "ContractDefinition";

#[test]
fn abstract_contract() -> Result<()> {
    run(T, "abstract_contract")
}

#[test]
fn constructor_contextual() -> Result<()> {
    run(T, "constructor_contextual")
}

#[test]
fn emit_contextual() -> Result<()> {
    run(T, "emit_contextual")
}

#[test]
fn empty_contract() -> Result<()> {
    run(T, "empty_contract")
}

#[test]
fn function_multiple_delimiters() -> Result<()> {
    run(T, "function_multiple_delimiters")
}

#[test]
fn header_comment() -> Result<()> {
    run(T, "header_comment")
}

#[test]
fn inheritance_specifier() -> Result<()> {
    run(T, "inheritance_specifier")
}

#[test]
fn member_constructor_definition() -> Result<()> {
    run(T, "member_constructor_definition")
}

#[test]
fn member_enum_definition() -> Result<()> {
    run(T, "member_enum_definition")
}

#[test]
fn member_error_definition() -> Result<()> {
    run(T, "member_error_definition")
}

#[test]
fn member_event_definition() -> Result<()> {
    run(T, "member_event_definition")
}

#[test]
fn member_fallback_function_definition() -> Result<()> {
    run(T, "member_fallback_function_definition")
}

#[test]
fn member_function_definition() -> Result<()> {
    run(T, "member_function_definition")
}

#[test]
fn member_modifier_definition() -> Result<()> {
    run(T, "member_modifier_definition")
}

#[test]
fn member_receive_function_definition() -> Result<()> {
    run(T, "member_receive_function_definition")
}

#[test]
fn member_state_variable_declaration() -> Result<()> {
    run(T, "member_state_variable_declaration")
}

#[test]
fn member_struct_definition() -> Result<()> {
    run(T, "member_struct_definition")
}

#[test]
fn member_unnamed_function_definition() -> Result<()> {
    run(T, "member_unnamed_function_definition")
}

#[test]
fn member_unnamed_function_with_attrs_definition() -> Result<()> {
    run(T, "member_unnamed_function_with_attrs_definition")
}

#[test]
fn member_user_defined_value_type_definition() -> Result<()> {
    run(T, "member_user_defined_value_type_definition")
}

#[test]
fn member_using_directive() -> Result<()> {
    run(T, "member_using_directive")
}

#[test]
fn missing_field_type() -> Result<()> {
    run(T, "missing_field_type")
}

#[test]
fn recovery_testbed() -> Result<()> {
    run(T, "recovery_testbed")
}

#[test]
fn storage_specifier_after_inheritance() -> Result<()> {
    run(T, "storage_specifier_after_inheritance")
}

#[test]
fn storage_specifier_before_inheritance() -> Result<()> {
    run(T, "storage_specifier_before_inheritance")
}

#[test]
fn storage_specifier_only() -> Result<()> {
    run(T, "storage_specifier_only")
}

#[test]
fn unicode_in_doc_comments() -> Result<()> {
    run(T, "unicode_in_doc_comments")
}

#[test]
fn unterminated_body() -> Result<()> {
    run(T, "unterminated_body")
}

#[test]
fn zero_length_input() -> Result<()> {
    run(T, "zero_length_input")
}

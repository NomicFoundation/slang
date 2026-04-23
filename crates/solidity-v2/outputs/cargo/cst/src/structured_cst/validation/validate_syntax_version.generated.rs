// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

#![allow(clippy::needless_return)]
#![allow(clippy::wildcard_imports)]

use std::ops::Range;

use slang_solidity_v2_common::versions::{LanguageVersion, LanguageVersionSpecifier};

use crate::structured_cst::nodes::*;
use crate::structured_cst::text_range::TextRange;

/// An error produced when a CST node is not valid for the given language version.
#[derive(Clone, Debug, PartialEq)]
pub struct SyntaxVersionError {
    /// The text range of the invalid node.
    pub range: Range<usize>,
    /// The version specifier that this syntax is enabled in.
    pub enabled: LanguageVersionSpecifier,
}

/// Validate that all nodes in the given `SourceUnit` are valid for the given language version.
pub fn validate_syntax_version(
    root: &SourceUnit,
    version: LanguageVersion,
) -> Vec<SyntaxVersionError> {
    let mut errors = Vec::new();
    check_source_unit(root, version, &mut errors);
    errors
}

fn expect_range(range: &impl TextRange) -> Range<usize> {
    range
        .text_range()
        .expect("Structured CST node should have a range")
}

//
// Sequence validators
//

fn check_additive_expression(
    node: &AdditiveExpression,
    version: LanguageVersion,
    errors: &mut Vec<SyntaxVersionError>,
) {
    let node = node.as_ref();

    check_expression(&node.left_operand, version, errors);

    check_expression(&node.right_operand, version, errors);
}

fn check_and_expression(
    node: &AndExpression,
    version: LanguageVersion,
    errors: &mut Vec<SyntaxVersionError>,
) {
    let node = node.as_ref();

    check_expression(&node.left_operand, version, errors);

    check_expression(&node.right_operand, version, errors);
}

fn check_array_expression(
    node: &ArrayExpression,
    version: LanguageVersion,
    errors: &mut Vec<SyntaxVersionError>,
) {
    let node = node.as_ref();

    check_array_values(&node.items, version, errors);
}

fn check_array_type_name(
    node: &ArrayTypeName,
    version: LanguageVersion,
    errors: &mut Vec<SyntaxVersionError>,
) {
    let node = node.as_ref();

    check_type_name(&node.operand, version, errors);

    if let Some(ref child) = node.index {
        check_expression(child, version, errors);
    }
}

fn check_assembly_statement(
    node: &AssemblyStatement,
    version: LanguageVersion,
    errors: &mut Vec<SyntaxVersionError>,
) {
    let node = node.as_ref();

    if let Some(ref child) = node.flags {
        if version < LanguageVersion::V0_8_13 {
            errors.push(SyntaxVersionError {
                range: expect_range(child),
                enabled: LanguageVersionSpecifier::From {
                    from: LanguageVersion::V0_8_13,
                },
            });
        } else {
            check_yul_flags_declaration(child, version, errors);
        }
    }
}

fn check_assignment_expression(
    node: &AssignmentExpression,
    version: LanguageVersion,
    errors: &mut Vec<SyntaxVersionError>,
) {
    let node = node.as_ref();

    check_expression(&node.left_operand, version, errors);

    check_expression(&node.right_operand, version, errors);
}

fn check_bitwise_and_expression(
    node: &BitwiseAndExpression,
    version: LanguageVersion,
    errors: &mut Vec<SyntaxVersionError>,
) {
    let node = node.as_ref();

    check_expression(&node.left_operand, version, errors);

    check_expression(&node.right_operand, version, errors);
}

fn check_bitwise_or_expression(
    node: &BitwiseOrExpression,
    version: LanguageVersion,
    errors: &mut Vec<SyntaxVersionError>,
) {
    let node = node.as_ref();

    check_expression(&node.left_operand, version, errors);

    check_expression(&node.right_operand, version, errors);
}

fn check_bitwise_xor_expression(
    node: &BitwiseXorExpression,
    version: LanguageVersion,
    errors: &mut Vec<SyntaxVersionError>,
) {
    let node = node.as_ref();

    check_expression(&node.left_operand, version, errors);

    check_expression(&node.right_operand, version, errors);
}

fn check_block(node: &Block, version: LanguageVersion, errors: &mut Vec<SyntaxVersionError>) {
    let node = node.as_ref();

    check_statements(&node.statements, version, errors);
}

fn check_call_options_expression(
    node: &CallOptionsExpression,
    version: LanguageVersion,
    errors: &mut Vec<SyntaxVersionError>,
) {
    let node = node.as_ref();

    check_expression(&node.operand, version, errors);

    check_call_options(&node.options, version, errors);
}

fn check_catch_clause(
    node: &CatchClause,
    version: LanguageVersion,
    errors: &mut Vec<SyntaxVersionError>,
) {
    let node = node.as_ref();

    if let Some(ref child) = node.error {
        check_catch_clause_error(child, version, errors);
    }

    check_block(&node.body, version, errors);
}

fn check_catch_clause_error(
    node: &CatchClauseError,
    version: LanguageVersion,
    errors: &mut Vec<SyntaxVersionError>,
) {
    let node = node.as_ref();

    check_parameters_declaration(&node.parameters, version, errors);
}

fn check_conditional_expression(
    node: &ConditionalExpression,
    version: LanguageVersion,
    errors: &mut Vec<SyntaxVersionError>,
) {
    let node = node.as_ref();

    check_expression(&node.operand, version, errors);

    check_expression(&node.true_expression, version, errors);

    check_expression(&node.false_expression, version, errors);
}

fn check_constant_definition(
    node: &ConstantDefinition,
    version: LanguageVersion,
    errors: &mut Vec<SyntaxVersionError>,
) {
    let node = node.as_ref();

    check_type_name(&node.type_name, version, errors);

    check_expression(&node.value, version, errors);
}

fn check_constructor_definition(
    node: &ConstructorDefinition,
    version: LanguageVersion,
    errors: &mut Vec<SyntaxVersionError>,
) {
    let node = node.as_ref();

    check_parameters_declaration(&node.parameters, version, errors);

    check_constructor_attributes(&node.attributes, version, errors);

    check_block(&node.body, version, errors);
}

fn check_contract_definition(
    node: &ContractDefinition,
    version: LanguageVersion,
    errors: &mut Vec<SyntaxVersionError>,
) {
    let node = node.as_ref();

    check_contract_specifiers(&node.specifiers, version, errors);

    check_contract_members(&node.members, version, errors);
}

fn check_do_while_statement(
    node: &DoWhileStatement,
    version: LanguageVersion,
    errors: &mut Vec<SyntaxVersionError>,
) {
    let node = node.as_ref();

    check_statement(&node.body, version, errors);

    check_expression(&node.condition, version, errors);
}

fn check_else_branch(
    node: &ElseBranch,
    version: LanguageVersion,
    errors: &mut Vec<SyntaxVersionError>,
) {
    let node = node.as_ref();

    check_statement(&node.body, version, errors);
}

fn check_emit_statement(
    node: &EmitStatement,
    version: LanguageVersion,
    errors: &mut Vec<SyntaxVersionError>,
) {
    let node = node.as_ref();

    check_arguments_declaration(&node.arguments, version, errors);
}

fn check_equality_expression(
    node: &EqualityExpression,
    version: LanguageVersion,
    errors: &mut Vec<SyntaxVersionError>,
) {
    let node = node.as_ref();

    check_expression(&node.left_operand, version, errors);

    check_expression(&node.right_operand, version, errors);
}

fn check_error_definition(
    node: &ErrorDefinition,
    version: LanguageVersion,
    errors: &mut Vec<SyntaxVersionError>,
) {
    let node = node.as_ref();

    if version < LanguageVersion::V0_8_4 {
        errors.push(SyntaxVersionError {
            range: expect_range(node),
            enabled: LanguageVersionSpecifier::From {
                from: LanguageVersion::V0_8_4,
            },
        });
        return;
    }

    check_error_parameters_declaration(&node.members, version, errors);
}

fn check_error_parameter(
    node: &ErrorParameter,
    version: LanguageVersion,
    errors: &mut Vec<SyntaxVersionError>,
) {
    let node = node.as_ref();

    if version < LanguageVersion::V0_8_4 {
        errors.push(SyntaxVersionError {
            range: expect_range(node),
            enabled: LanguageVersionSpecifier::From {
                from: LanguageVersion::V0_8_4,
            },
        });
        return;
    }

    check_type_name(&node.type_name, version, errors);
}

fn check_error_parameters_declaration(
    node: &ErrorParametersDeclaration,
    version: LanguageVersion,
    errors: &mut Vec<SyntaxVersionError>,
) {
    let node = node.as_ref();

    if version < LanguageVersion::V0_8_4 {
        errors.push(SyntaxVersionError {
            range: expect_range(node),
            enabled: LanguageVersionSpecifier::From {
                from: LanguageVersion::V0_8_4,
            },
        });
        return;
    }

    check_error_parameters(&node.parameters, version, errors);
}

fn check_event_definition(
    node: &EventDefinition,
    version: LanguageVersion,
    errors: &mut Vec<SyntaxVersionError>,
) {
    let node = node.as_ref();

    check_event_parameters_declaration(&node.parameters, version, errors);
}

fn check_event_parameter(
    node: &EventParameter,
    version: LanguageVersion,
    errors: &mut Vec<SyntaxVersionError>,
) {
    let node = node.as_ref();

    check_type_name(&node.type_name, version, errors);
}

fn check_event_parameters_declaration(
    node: &EventParametersDeclaration,
    version: LanguageVersion,
    errors: &mut Vec<SyntaxVersionError>,
) {
    let node = node.as_ref();

    check_event_parameters(&node.parameters, version, errors);
}

fn check_exponentiation_expression(
    node: &ExponentiationExpression,
    version: LanguageVersion,
    errors: &mut Vec<SyntaxVersionError>,
) {
    let node = node.as_ref();

    check_expression(&node.left_operand, version, errors);

    check_expression(&node.right_operand, version, errors);
}

fn check_expression_statement(
    node: &ExpressionStatement,
    version: LanguageVersion,
    errors: &mut Vec<SyntaxVersionError>,
) {
    let node = node.as_ref();

    check_expression(&node.expression, version, errors);
}

fn check_fallback_function_definition(
    node: &FallbackFunctionDefinition,
    version: LanguageVersion,
    errors: &mut Vec<SyntaxVersionError>,
) {
    let node = node.as_ref();

    check_parameters_declaration(&node.parameters, version, errors);

    check_fallback_function_attributes(&node.attributes, version, errors);
    if let Some(ref child) = node.returns {
        check_returns_declaration(child, version, errors);
    }

    check_function_body(&node.body, version, errors);
}

fn check_for_statement(
    node: &ForStatement,
    version: LanguageVersion,
    errors: &mut Vec<SyntaxVersionError>,
) {
    let node = node.as_ref();

    check_for_statement_initialization(&node.initialization, version, errors);

    check_for_statement_condition(&node.condition, version, errors);
    if let Some(ref child) = node.iterator {
        check_expression(child, version, errors);
    }

    check_statement(&node.body, version, errors);
}

fn check_function_call_expression(
    node: &FunctionCallExpression,
    version: LanguageVersion,
    errors: &mut Vec<SyntaxVersionError>,
) {
    let node = node.as_ref();

    check_expression(&node.operand, version, errors);

    check_arguments_declaration(&node.arguments, version, errors);
}

fn check_function_definition(
    node: &FunctionDefinition,
    version: LanguageVersion,
    errors: &mut Vec<SyntaxVersionError>,
) {
    let node = node.as_ref();

    check_parameters_declaration(&node.parameters, version, errors);

    check_function_attributes(&node.attributes, version, errors);
    if let Some(ref child) = node.returns {
        check_returns_declaration(child, version, errors);
    }

    check_function_body(&node.body, version, errors);
}

fn check_function_type(
    node: &FunctionType,
    version: LanguageVersion,
    errors: &mut Vec<SyntaxVersionError>,
) {
    let node = node.as_ref();

    check_parameters_declaration(&node.parameters, version, errors);

    if let Some(ref child) = node.returns {
        check_returns_declaration(child, version, errors);
    }
}

fn check_if_statement(
    node: &IfStatement,
    version: LanguageVersion,
    errors: &mut Vec<SyntaxVersionError>,
) {
    let node = node.as_ref();

    check_expression(&node.condition, version, errors);

    check_statement(&node.body, version, errors);
    if let Some(ref child) = node.else_branch {
        check_else_branch(child, version, errors);
    }
}

fn check_index_access_end(
    node: &IndexAccessEnd,
    version: LanguageVersion,
    errors: &mut Vec<SyntaxVersionError>,
) {
    let node = node.as_ref();

    if let Some(ref child) = node.end {
        check_expression(child, version, errors);
    }
}

fn check_index_access_expression(
    node: &IndexAccessExpression,
    version: LanguageVersion,
    errors: &mut Vec<SyntaxVersionError>,
) {
    let node = node.as_ref();

    check_expression(&node.operand, version, errors);

    if let Some(ref child) = node.start {
        check_expression(child, version, errors);
    }
    if let Some(ref child) = node.end {
        check_index_access_end(child, version, errors);
    }
}

fn check_inequality_expression(
    node: &InequalityExpression,
    version: LanguageVersion,
    errors: &mut Vec<SyntaxVersionError>,
) {
    let node = node.as_ref();

    check_expression(&node.left_operand, version, errors);

    check_expression(&node.right_operand, version, errors);
}

fn check_inheritance_specifier(
    node: &InheritanceSpecifier,
    version: LanguageVersion,
    errors: &mut Vec<SyntaxVersionError>,
) {
    let node = node.as_ref();

    check_inheritance_types(&node.types, version, errors);
}

fn check_inheritance_type(
    node: &InheritanceType,
    version: LanguageVersion,
    errors: &mut Vec<SyntaxVersionError>,
) {
    let node = node.as_ref();

    if let Some(ref child) = node.arguments {
        check_arguments_declaration(child, version, errors);
    }
}

fn check_interface_definition(
    node: &InterfaceDefinition,
    version: LanguageVersion,
    errors: &mut Vec<SyntaxVersionError>,
) {
    let node = node.as_ref();

    if let Some(ref child) = node.inheritance {
        check_inheritance_specifier(child, version, errors);
    }

    check_interface_members(&node.members, version, errors);
}

fn check_library_definition(
    node: &LibraryDefinition,
    version: LanguageVersion,
    errors: &mut Vec<SyntaxVersionError>,
) {
    let node = node.as_ref();

    check_library_members(&node.members, version, errors);
}

fn check_mapping_key(
    node: &MappingKey,
    version: LanguageVersion,
    errors: &mut Vec<SyntaxVersionError>,
) {
    let node = node.as_ref();

    if let Some(ref child) = node.name {
        if version < LanguageVersion::V0_8_18 {
            errors.push(SyntaxVersionError {
                range: expect_range(child),
                enabled: LanguageVersionSpecifier::From {
                    from: LanguageVersion::V0_8_18,
                },
            });
        }
    }
}

fn check_mapping_type(
    node: &MappingType,
    version: LanguageVersion,
    errors: &mut Vec<SyntaxVersionError>,
) {
    let node = node.as_ref();

    check_mapping_key(&node.key_type, version, errors);

    check_mapping_value(&node.value_type, version, errors);
}

fn check_mapping_value(
    node: &MappingValue,
    version: LanguageVersion,
    errors: &mut Vec<SyntaxVersionError>,
) {
    let node = node.as_ref();

    check_type_name(&node.type_name, version, errors);
    if let Some(ref child) = node.name {
        if version < LanguageVersion::V0_8_18 {
            errors.push(SyntaxVersionError {
                range: expect_range(child),
                enabled: LanguageVersionSpecifier::From {
                    from: LanguageVersion::V0_8_18,
                },
            });
        }
    }
}

fn check_member_access_expression(
    node: &MemberAccessExpression,
    version: LanguageVersion,
    errors: &mut Vec<SyntaxVersionError>,
) {
    let node = node.as_ref();

    check_expression(&node.operand, version, errors);
}

fn check_modifier_definition(
    node: &ModifierDefinition,
    version: LanguageVersion,
    errors: &mut Vec<SyntaxVersionError>,
) {
    let node = node.as_ref();

    if let Some(ref child) = node.parameters {
        check_parameters_declaration(child, version, errors);
    }

    check_function_body(&node.body, version, errors);
}

fn check_modifier_invocation(
    node: &ModifierInvocation,
    version: LanguageVersion,
    errors: &mut Vec<SyntaxVersionError>,
) {
    let node = node.as_ref();

    if let Some(ref child) = node.arguments {
        check_arguments_declaration(child, version, errors);
    }
}

fn check_multi_typed_declaration(
    node: &MultiTypedDeclaration,
    version: LanguageVersion,
    errors: &mut Vec<SyntaxVersionError>,
) {
    let node = node.as_ref();

    check_multi_typed_declaration_elements(&node.elements, version, errors);

    check_variable_declaration_value(&node.value, version, errors);
}

fn check_multi_typed_declaration_element(
    node: &MultiTypedDeclarationElement,
    version: LanguageVersion,
    errors: &mut Vec<SyntaxVersionError>,
) {
    let node = node.as_ref();

    if let Some(ref child) = node.member {
        check_variable_declaration(child, version, errors);
    }
}

fn check_multiplicative_expression(
    node: &MultiplicativeExpression,
    version: LanguageVersion,
    errors: &mut Vec<SyntaxVersionError>,
) {
    let node = node.as_ref();

    check_expression(&node.left_operand, version, errors);

    check_expression(&node.right_operand, version, errors);
}

fn check_named_argument(
    node: &NamedArgument,
    version: LanguageVersion,
    errors: &mut Vec<SyntaxVersionError>,
) {
    let node = node.as_ref();

    check_expression(&node.value, version, errors);
}

fn check_named_argument_group(
    node: &NamedArgumentGroup,
    version: LanguageVersion,
    errors: &mut Vec<SyntaxVersionError>,
) {
    let node = node.as_ref();

    check_named_arguments(&node.arguments, version, errors);
}

fn check_named_arguments_declaration(
    node: &NamedArgumentsDeclaration,
    version: LanguageVersion,
    errors: &mut Vec<SyntaxVersionError>,
) {
    let node = node.as_ref();

    check_named_argument_group(&node.arguments, version, errors);
}

fn check_new_expression(
    node: &NewExpression,
    version: LanguageVersion,
    errors: &mut Vec<SyntaxVersionError>,
) {
    let node = node.as_ref();

    check_type_name(&node.type_name, version, errors);
}

fn check_or_expression(
    node: &OrExpression,
    version: LanguageVersion,
    errors: &mut Vec<SyntaxVersionError>,
) {
    let node = node.as_ref();

    check_expression(&node.left_operand, version, errors);

    check_expression(&node.right_operand, version, errors);
}

fn check_parameter(
    node: &Parameter,
    version: LanguageVersion,
    errors: &mut Vec<SyntaxVersionError>,
) {
    let node = node.as_ref();

    check_type_name(&node.type_name, version, errors);
}

fn check_parameters_declaration(
    node: &ParametersDeclaration,
    version: LanguageVersion,
    errors: &mut Vec<SyntaxVersionError>,
) {
    let node = node.as_ref();

    check_parameters(&node.parameters, version, errors);
}

fn check_positional_arguments_declaration(
    node: &PositionalArgumentsDeclaration,
    version: LanguageVersion,
    errors: &mut Vec<SyntaxVersionError>,
) {
    let node = node.as_ref();

    check_positional_arguments(&node.arguments, version, errors);
}

fn check_postfix_expression(
    node: &PostfixExpression,
    version: LanguageVersion,
    errors: &mut Vec<SyntaxVersionError>,
) {
    let node = node.as_ref();

    check_expression(&node.operand, version, errors);
}

fn check_prefix_expression(
    node: &PrefixExpression,
    version: LanguageVersion,
    errors: &mut Vec<SyntaxVersionError>,
) {
    let node = node.as_ref();

    check_expression(&node.operand, version, errors);
}

fn check_receive_function_definition(
    node: &ReceiveFunctionDefinition,
    version: LanguageVersion,
    errors: &mut Vec<SyntaxVersionError>,
) {
    let node = node.as_ref();

    check_parameters_declaration(&node.parameters, version, errors);

    check_receive_function_attributes(&node.attributes, version, errors);

    check_function_body(&node.body, version, errors);
}

fn check_return_statement(
    node: &ReturnStatement,
    version: LanguageVersion,
    errors: &mut Vec<SyntaxVersionError>,
) {
    let node = node.as_ref();

    if let Some(ref child) = node.expression {
        check_expression(child, version, errors);
    }
}

fn check_returns_declaration(
    node: &ReturnsDeclaration,
    version: LanguageVersion,
    errors: &mut Vec<SyntaxVersionError>,
) {
    let node = node.as_ref();

    check_parameters_declaration(&node.variables, version, errors);
}

fn check_revert_statement(
    node: &RevertStatement,
    version: LanguageVersion,
    errors: &mut Vec<SyntaxVersionError>,
) {
    let node = node.as_ref();

    if version < LanguageVersion::V0_8_4 {
        errors.push(SyntaxVersionError {
            range: expect_range(node),
            enabled: LanguageVersionSpecifier::From {
                from: LanguageVersion::V0_8_4,
            },
        });
        return;
    }

    check_arguments_declaration(&node.arguments, version, errors);
}

fn check_shift_expression(
    node: &ShiftExpression,
    version: LanguageVersion,
    errors: &mut Vec<SyntaxVersionError>,
) {
    let node = node.as_ref();

    check_expression(&node.left_operand, version, errors);

    check_expression(&node.right_operand, version, errors);
}

fn check_single_typed_declaration(
    node: &SingleTypedDeclaration,
    version: LanguageVersion,
    errors: &mut Vec<SyntaxVersionError>,
) {
    let node = node.as_ref();

    check_variable_declaration(&node.declaration, version, errors);
    if let Some(ref child) = node.value {
        check_variable_declaration_value(child, version, errors);
    }
}

fn check_source_unit(
    node: &SourceUnit,
    version: LanguageVersion,
    errors: &mut Vec<SyntaxVersionError>,
) {
    let node = node.as_ref();

    check_source_unit_members(&node.members, version, errors);
}

fn check_state_variable_definition(
    node: &StateVariableDefinition,
    version: LanguageVersion,
    errors: &mut Vec<SyntaxVersionError>,
) {
    let node = node.as_ref();

    check_type_name(&node.type_name, version, errors);

    check_state_variable_attributes(&node.attributes, version, errors);

    if let Some(ref child) = node.value {
        check_state_variable_definition_value(child, version, errors);
    }
}

fn check_state_variable_definition_value(
    node: &StateVariableDefinitionValue,
    version: LanguageVersion,
    errors: &mut Vec<SyntaxVersionError>,
) {
    let node = node.as_ref();

    check_expression(&node.value, version, errors);
}

fn check_storage_layout_specifier(
    node: &StorageLayoutSpecifier,
    version: LanguageVersion,
    errors: &mut Vec<SyntaxVersionError>,
) {
    let node = node.as_ref();

    if version < LanguageVersion::V0_8_29 {
        errors.push(SyntaxVersionError {
            range: expect_range(node),
            enabled: LanguageVersionSpecifier::From {
                from: LanguageVersion::V0_8_29,
            },
        });
        return;
    }

    check_expression(&node.expression, version, errors);
}

fn check_struct_definition(
    node: &StructDefinition,
    version: LanguageVersion,
    errors: &mut Vec<SyntaxVersionError>,
) {
    let node = node.as_ref();

    check_struct_members(&node.members, version, errors);
}

fn check_struct_member(
    node: &StructMember,
    version: LanguageVersion,
    errors: &mut Vec<SyntaxVersionError>,
) {
    let node = node.as_ref();

    check_type_name(&node.type_name, version, errors);
}

fn check_try_statement(
    node: &TryStatement,
    version: LanguageVersion,
    errors: &mut Vec<SyntaxVersionError>,
) {
    let node = node.as_ref();

    check_expression(&node.expression, version, errors);
    if let Some(ref child) = node.returns {
        check_returns_declaration(child, version, errors);
    }

    check_block(&node.body, version, errors);

    check_catch_clauses(&node.catch_clauses, version, errors);
}

fn check_tuple_expression(
    node: &TupleExpression,
    version: LanguageVersion,
    errors: &mut Vec<SyntaxVersionError>,
) {
    let node = node.as_ref();

    check_tuple_values(&node.items, version, errors);
}

fn check_tuple_value(
    node: &TupleValue,
    version: LanguageVersion,
    errors: &mut Vec<SyntaxVersionError>,
) {
    let node = node.as_ref();

    if let Some(ref child) = node.expression {
        check_expression(child, version, errors);
    }
}

fn check_type_expression(
    node: &TypeExpression,
    version: LanguageVersion,
    errors: &mut Vec<SyntaxVersionError>,
) {
    let node = node.as_ref();

    check_type_name(&node.type_name, version, errors);
}

fn check_unchecked_block(
    node: &UncheckedBlock,
    version: LanguageVersion,
    errors: &mut Vec<SyntaxVersionError>,
) {
    let node = node.as_ref();

    check_block(&node.block, version, errors);
}

fn check_user_defined_value_type_definition(
    node: &UserDefinedValueTypeDefinition,
    version: LanguageVersion,
    errors: &mut Vec<SyntaxVersionError>,
) {
    let node = node.as_ref();

    if version < LanguageVersion::V0_8_8 {
        errors.push(SyntaxVersionError {
            range: expect_range(node),
            enabled: LanguageVersionSpecifier::From {
                from: LanguageVersion::V0_8_8,
            },
        });
        return;
    }
}

fn check_using_alias(
    node: &UsingAlias,
    version: LanguageVersion,
    errors: &mut Vec<SyntaxVersionError>,
) {
    let node = node.as_ref();

    if version < LanguageVersion::V0_8_19 {
        errors.push(SyntaxVersionError {
            range: expect_range(node),
            enabled: LanguageVersionSpecifier::From {
                from: LanguageVersion::V0_8_19,
            },
        });
        return;
    }

    check_using_operator(&node.operator, version, errors);
}

fn check_using_deconstruction(
    node: &UsingDeconstruction,
    version: LanguageVersion,
    errors: &mut Vec<SyntaxVersionError>,
) {
    let node = node.as_ref();

    if version < LanguageVersion::V0_8_13 {
        errors.push(SyntaxVersionError {
            range: expect_range(node),
            enabled: LanguageVersionSpecifier::From {
                from: LanguageVersion::V0_8_13,
            },
        });
        return;
    }

    check_using_deconstruction_symbols(&node.symbols, version, errors);
}

fn check_using_deconstruction_symbol(
    node: &UsingDeconstructionSymbol,
    version: LanguageVersion,
    errors: &mut Vec<SyntaxVersionError>,
) {
    let node = node.as_ref();

    if version < LanguageVersion::V0_8_13 {
        errors.push(SyntaxVersionError {
            range: expect_range(node),
            enabled: LanguageVersionSpecifier::From {
                from: LanguageVersion::V0_8_13,
            },
        });
        return;
    }

    if let Some(ref child) = node.alias {
        if version < LanguageVersion::V0_8_19 {
            errors.push(SyntaxVersionError {
                range: expect_range(child),
                enabled: LanguageVersionSpecifier::From {
                    from: LanguageVersion::V0_8_19,
                },
            });
        } else {
            check_using_alias(child, version, errors);
        }
    }
}

fn check_using_directive(
    node: &UsingDirective,
    version: LanguageVersion,
    errors: &mut Vec<SyntaxVersionError>,
) {
    let node = node.as_ref();

    check_using_clause(&node.clause, version, errors);

    check_using_target(&node.target, version, errors);
    if let Some(ref child) = node.global_keyword {
        if version < LanguageVersion::V0_8_13 {
            errors.push(SyntaxVersionError {
                range: expect_range(child),
                enabled: LanguageVersionSpecifier::From {
                    from: LanguageVersion::V0_8_13,
                },
            });
        }
    }
}

fn check_variable_declaration(
    node: &VariableDeclaration,
    version: LanguageVersion,
    errors: &mut Vec<SyntaxVersionError>,
) {
    let node = node.as_ref();

    check_type_name(&node.type_name, version, errors);
}

fn check_variable_declaration_statement(
    node: &VariableDeclarationStatement,
    version: LanguageVersion,
    errors: &mut Vec<SyntaxVersionError>,
) {
    let node = node.as_ref();

    check_variable_declaration_target(&node.target, version, errors);
}

fn check_variable_declaration_value(
    node: &VariableDeclarationValue,
    version: LanguageVersion,
    errors: &mut Vec<SyntaxVersionError>,
) {
    let node = node.as_ref();

    check_expression(&node.expression, version, errors);
}

fn check_while_statement(
    node: &WhileStatement,
    version: LanguageVersion,
    errors: &mut Vec<SyntaxVersionError>,
) {
    let node = node.as_ref();

    check_expression(&node.condition, version, errors);

    check_statement(&node.body, version, errors);
}

fn check_yul_flags_declaration(
    node: &YulFlagsDeclaration,
    version: LanguageVersion,
    errors: &mut Vec<SyntaxVersionError>,
) {
    let node = node.as_ref();

    if version < LanguageVersion::V0_8_13 {
        errors.push(SyntaxVersionError {
            range: expect_range(node),
            enabled: LanguageVersionSpecifier::From {
                from: LanguageVersion::V0_8_13,
            },
        });
        return;
    }

    check_yul_flags(&node.flags, version, errors);
}

//
// Choice validators
//

fn check_arguments_declaration(
    node: &ArgumentsDeclaration,
    version: LanguageVersion,
    errors: &mut Vec<SyntaxVersionError>,
) {
    match node {
        ArgumentsDeclaration::PositionalArgumentsDeclaration(child) => {
            check_positional_arguments_declaration(child, version, errors);
        }
        ArgumentsDeclaration::NamedArgumentsDeclaration(child) => {
            check_named_arguments_declaration(child, version, errors);
        }
    }
}

fn check_constructor_attribute(
    node: &ConstructorAttribute,
    version: LanguageVersion,
    errors: &mut Vec<SyntaxVersionError>,
) {
    match node {
        ConstructorAttribute::ModifierInvocation(child) => {
            check_modifier_invocation(child, version, errors);
        }
        ConstructorAttribute::InternalKeyword(_) => {}
        ConstructorAttribute::PayableKeyword(_) => {}
        ConstructorAttribute::PublicKeyword(_) => {}
    }
}

fn check_contract_member(
    node: &ContractMember,
    version: LanguageVersion,
    errors: &mut Vec<SyntaxVersionError>,
) {
    match node {
        ContractMember::UsingDirective(child) => {
            check_using_directive(child, version, errors);
        }
        ContractMember::FunctionDefinition(child) => {
            check_function_definition(child, version, errors);
        }
        ContractMember::ConstructorDefinition(child) => {
            check_constructor_definition(child, version, errors);
        }
        ContractMember::ReceiveFunctionDefinition(child) => {
            check_receive_function_definition(child, version, errors);
        }
        ContractMember::FallbackFunctionDefinition(child) => {
            check_fallback_function_definition(child, version, errors);
        }
        ContractMember::ModifierDefinition(child) => {
            check_modifier_definition(child, version, errors);
        }
        ContractMember::StructDefinition(child) => {
            check_struct_definition(child, version, errors);
        }
        ContractMember::EnumDefinition(_) => {}
        ContractMember::EventDefinition(child) => {
            check_event_definition(child, version, errors);
        }
        ContractMember::ErrorDefinition(child) => {
            if version < LanguageVersion::V0_8_4 {
                errors.push(SyntaxVersionError {
                    range: expect_range(child),
                    enabled: LanguageVersionSpecifier::From {
                        from: LanguageVersion::V0_8_4,
                    },
                });
                return;
            }
            check_error_definition(child, version, errors);
        }
        ContractMember::UserDefinedValueTypeDefinition(child) => {
            if version < LanguageVersion::V0_8_8 {
                errors.push(SyntaxVersionError {
                    range: expect_range(child),
                    enabled: LanguageVersionSpecifier::From {
                        from: LanguageVersion::V0_8_8,
                    },
                });
                return;
            }
            check_user_defined_value_type_definition(child, version, errors);
        }
        ContractMember::StateVariableDefinition(child) => {
            check_state_variable_definition(child, version, errors);
        }
    }
}

fn check_contract_specifier(
    node: &ContractSpecifier,
    version: LanguageVersion,
    errors: &mut Vec<SyntaxVersionError>,
) {
    match node {
        ContractSpecifier::InheritanceSpecifier(child) => {
            check_inheritance_specifier(child, version, errors);
        }
        ContractSpecifier::StorageLayoutSpecifier(child) => {
            if version < LanguageVersion::V0_8_29 {
                errors.push(SyntaxVersionError {
                    range: expect_range(child),
                    enabled: LanguageVersionSpecifier::From {
                        from: LanguageVersion::V0_8_29,
                    },
                });
                return;
            }
            check_storage_layout_specifier(child, version, errors);
        }
    }
}

fn check_expression(
    node: &Expression,
    version: LanguageVersion,
    errors: &mut Vec<SyntaxVersionError>,
) {
    match node {
        Expression::AssignmentExpression(child) => {
            check_assignment_expression(child, version, errors);
        }
        Expression::ConditionalExpression(child) => {
            check_conditional_expression(child, version, errors);
        }
        Expression::OrExpression(child) => {
            check_or_expression(child, version, errors);
        }
        Expression::AndExpression(child) => {
            check_and_expression(child, version, errors);
        }
        Expression::EqualityExpression(child) => {
            check_equality_expression(child, version, errors);
        }
        Expression::InequalityExpression(child) => {
            check_inequality_expression(child, version, errors);
        }
        Expression::BitwiseOrExpression(child) => {
            check_bitwise_or_expression(child, version, errors);
        }
        Expression::BitwiseXorExpression(child) => {
            check_bitwise_xor_expression(child, version, errors);
        }
        Expression::BitwiseAndExpression(child) => {
            check_bitwise_and_expression(child, version, errors);
        }
        Expression::ShiftExpression(child) => {
            check_shift_expression(child, version, errors);
        }
        Expression::AdditiveExpression(child) => {
            check_additive_expression(child, version, errors);
        }
        Expression::MultiplicativeExpression(child) => {
            check_multiplicative_expression(child, version, errors);
        }
        Expression::ExponentiationExpression(child) => {
            check_exponentiation_expression(child, version, errors);
        }
        Expression::PostfixExpression(child) => {
            check_postfix_expression(child, version, errors);
        }
        Expression::PrefixExpression(child) => {
            check_prefix_expression(child, version, errors);
        }
        Expression::FunctionCallExpression(child) => {
            check_function_call_expression(child, version, errors);
        }
        Expression::CallOptionsExpression(child) => {
            check_call_options_expression(child, version, errors);
        }
        Expression::MemberAccessExpression(child) => {
            check_member_access_expression(child, version, errors);
        }
        Expression::IndexAccessExpression(child) => {
            check_index_access_expression(child, version, errors);
        }
        Expression::NewExpression(child) => {
            check_new_expression(child, version, errors);
        }
        Expression::TupleExpression(child) => {
            check_tuple_expression(child, version, errors);
        }
        Expression::TypeExpression(child) => {
            check_type_expression(child, version, errors);
        }
        Expression::ArrayExpression(child) => {
            check_array_expression(child, version, errors);
        }
        Expression::HexNumberExpression(_) => {}
        Expression::DecimalNumberExpression(_) => {}
        Expression::StringExpression(_) => {}
        Expression::ElementaryType(_) => {}
        Expression::PayableKeyword(_) => {}
        Expression::ThisKeyword(_) => {}
        Expression::SuperKeyword(_) => {}
        Expression::TrueKeyword(_) => {}
        Expression::FalseKeyword(_) => {}
        Expression::Identifier(_) => {}
    }
}

fn check_fallback_function_attribute(
    node: &FallbackFunctionAttribute,
    version: LanguageVersion,
    errors: &mut Vec<SyntaxVersionError>,
) {
    match node {
        FallbackFunctionAttribute::ModifierInvocation(child) => {
            check_modifier_invocation(child, version, errors);
        }
        FallbackFunctionAttribute::OverrideSpecifier(_) => {}
        FallbackFunctionAttribute::ExternalKeyword(_) => {}
        FallbackFunctionAttribute::PayableKeyword(_) => {}
        FallbackFunctionAttribute::PureKeyword(_) => {}
        FallbackFunctionAttribute::ViewKeyword(_) => {}
        FallbackFunctionAttribute::VirtualKeyword(_) => {}
    }
}

fn check_for_statement_condition(
    node: &ForStatementCondition,
    version: LanguageVersion,
    errors: &mut Vec<SyntaxVersionError>,
) {
    match node {
        ForStatementCondition::ExpressionStatement(child) => {
            check_expression_statement(child, version, errors);
        }
        ForStatementCondition::Semicolon(_) => {}
    }
}

fn check_for_statement_initialization(
    node: &ForStatementInitialization,
    version: LanguageVersion,
    errors: &mut Vec<SyntaxVersionError>,
) {
    match node {
        ForStatementInitialization::VariableDeclarationStatement(child) => {
            check_variable_declaration_statement(child, version, errors);
        }
        ForStatementInitialization::ExpressionStatement(child) => {
            check_expression_statement(child, version, errors);
        }
        ForStatementInitialization::Semicolon(_) => {}
    }
}

fn check_function_attribute(
    node: &FunctionAttribute,
    version: LanguageVersion,
    errors: &mut Vec<SyntaxVersionError>,
) {
    match node {
        FunctionAttribute::ModifierInvocation(child) => {
            check_modifier_invocation(child, version, errors);
        }
        FunctionAttribute::OverrideSpecifier(_) => {}
        FunctionAttribute::ExternalKeyword(_) => {}
        FunctionAttribute::InternalKeyword(_) => {}
        FunctionAttribute::PayableKeyword(_) => {}
        FunctionAttribute::PrivateKeyword(_) => {}
        FunctionAttribute::PublicKeyword(_) => {}
        FunctionAttribute::PureKeyword(_) => {}
        FunctionAttribute::ViewKeyword(_) => {}
        FunctionAttribute::VirtualKeyword(_) => {}
    }
}

fn check_function_body(
    node: &FunctionBody,
    version: LanguageVersion,
    errors: &mut Vec<SyntaxVersionError>,
) {
    match node {
        FunctionBody::Block(child) => {
            check_block(child, version, errors);
        }
        FunctionBody::Semicolon(_) => {}
    }
}

fn check_receive_function_attribute(
    node: &ReceiveFunctionAttribute,
    version: LanguageVersion,
    errors: &mut Vec<SyntaxVersionError>,
) {
    match node {
        ReceiveFunctionAttribute::ModifierInvocation(child) => {
            check_modifier_invocation(child, version, errors);
        }
        ReceiveFunctionAttribute::OverrideSpecifier(_) => {}
        ReceiveFunctionAttribute::ExternalKeyword(_) => {}
        ReceiveFunctionAttribute::PayableKeyword(_) => {}
        ReceiveFunctionAttribute::VirtualKeyword(_) => {}
    }
}

fn check_source_unit_member(
    node: &SourceUnitMember,
    version: LanguageVersion,
    errors: &mut Vec<SyntaxVersionError>,
) {
    match node {
        SourceUnitMember::PragmaDirective(_) => {}
        SourceUnitMember::ImportDirective(_) => {}
        SourceUnitMember::ContractDefinition(child) => {
            check_contract_definition(child, version, errors);
        }
        SourceUnitMember::InterfaceDefinition(child) => {
            check_interface_definition(child, version, errors);
        }
        SourceUnitMember::LibraryDefinition(child) => {
            check_library_definition(child, version, errors);
        }
        SourceUnitMember::StructDefinition(child) => {
            check_struct_definition(child, version, errors);
        }
        SourceUnitMember::EnumDefinition(_) => {}
        SourceUnitMember::FunctionDefinition(child) => {
            check_function_definition(child, version, errors);
        }
        SourceUnitMember::ErrorDefinition(child) => {
            if version < LanguageVersion::V0_8_4 {
                errors.push(SyntaxVersionError {
                    range: expect_range(child),
                    enabled: LanguageVersionSpecifier::From {
                        from: LanguageVersion::V0_8_4,
                    },
                });
                return;
            }
            check_error_definition(child, version, errors);
        }
        SourceUnitMember::UserDefinedValueTypeDefinition(child) => {
            if version < LanguageVersion::V0_8_8 {
                errors.push(SyntaxVersionError {
                    range: expect_range(child),
                    enabled: LanguageVersionSpecifier::From {
                        from: LanguageVersion::V0_8_8,
                    },
                });
                return;
            }
            check_user_defined_value_type_definition(child, version, errors);
        }
        SourceUnitMember::UsingDirective(child) => {
            if version < LanguageVersion::V0_8_13 {
                errors.push(SyntaxVersionError {
                    range: expect_range(child),
                    enabled: LanguageVersionSpecifier::From {
                        from: LanguageVersion::V0_8_13,
                    },
                });
                return;
            }
            check_using_directive(child, version, errors);
        }
        SourceUnitMember::EventDefinition(child) => {
            if version < LanguageVersion::V0_8_22 {
                errors.push(SyntaxVersionError {
                    range: expect_range(child),
                    enabled: LanguageVersionSpecifier::From {
                        from: LanguageVersion::V0_8_22,
                    },
                });
                return;
            }
            check_event_definition(child, version, errors);
        }
        SourceUnitMember::ConstantDefinition(child) => {
            check_constant_definition(child, version, errors);
        }
    }
}

fn check_state_variable_attribute(
    node: &StateVariableAttribute,
    version: LanguageVersion,
    errors: &mut Vec<SyntaxVersionError>,
) {
    match node {
        StateVariableAttribute::OverrideSpecifier(_) => {}
        StateVariableAttribute::ConstantKeyword(_) => {}
        StateVariableAttribute::InternalKeyword(_) => {}
        StateVariableAttribute::PrivateKeyword(_) => {}
        StateVariableAttribute::PublicKeyword(_) => {}
        StateVariableAttribute::ImmutableKeyword(_) => {}
        StateVariableAttribute::TransientKeyword(child) => {
            if version < LanguageVersion::V0_8_27 {
                errors.push(SyntaxVersionError {
                    range: expect_range(child),
                    enabled: LanguageVersionSpecifier::From {
                        from: LanguageVersion::V0_8_27,
                    },
                });
                return;
            }
        }
    }
}

fn check_statement(
    node: &Statement,
    version: LanguageVersion,
    errors: &mut Vec<SyntaxVersionError>,
) {
    match node {
        Statement::IfStatement(child) => {
            check_if_statement(child, version, errors);
        }
        Statement::ForStatement(child) => {
            check_for_statement(child, version, errors);
        }
        Statement::WhileStatement(child) => {
            check_while_statement(child, version, errors);
        }
        Statement::DoWhileStatement(child) => {
            check_do_while_statement(child, version, errors);
        }
        Statement::ContinueStatement(_) => {}
        Statement::BreakStatement(_) => {}
        Statement::ReturnStatement(child) => {
            check_return_statement(child, version, errors);
        }
        Statement::EmitStatement(child) => {
            check_emit_statement(child, version, errors);
        }
        Statement::TryStatement(child) => {
            check_try_statement(child, version, errors);
        }
        Statement::RevertStatement(child) => {
            if version < LanguageVersion::V0_8_4 {
                errors.push(SyntaxVersionError {
                    range: expect_range(child),
                    enabled: LanguageVersionSpecifier::From {
                        from: LanguageVersion::V0_8_4,
                    },
                });
                return;
            }
            check_revert_statement(child, version, errors);
        }
        Statement::AssemblyStatement(child) => {
            check_assembly_statement(child, version, errors);
        }
        Statement::Block(child) => {
            check_block(child, version, errors);
        }
        Statement::UncheckedBlock(child) => {
            check_unchecked_block(child, version, errors);
        }
        Statement::VariableDeclarationStatement(child) => {
            check_variable_declaration_statement(child, version, errors);
        }
        Statement::ExpressionStatement(child) => {
            check_expression_statement(child, version, errors);
        }
    }
}

fn check_type_name(
    node: &TypeName,
    version: LanguageVersion,
    errors: &mut Vec<SyntaxVersionError>,
) {
    match node {
        TypeName::ArrayTypeName(child) => {
            check_array_type_name(child, version, errors);
        }
        TypeName::FunctionType(child) => {
            check_function_type(child, version, errors);
        }
        TypeName::MappingType(child) => {
            check_mapping_type(child, version, errors);
        }
        TypeName::ElementaryType(_) => {}
        TypeName::IdentifierPath(_) => {}
    }
}

fn check_using_clause(
    node: &UsingClause,
    version: LanguageVersion,
    errors: &mut Vec<SyntaxVersionError>,
) {
    match node {
        UsingClause::IdentifierPath(_) => {}
        UsingClause::UsingDeconstruction(child) => {
            if version < LanguageVersion::V0_8_13 {
                errors.push(SyntaxVersionError {
                    range: expect_range(child),
                    enabled: LanguageVersionSpecifier::From {
                        from: LanguageVersion::V0_8_13,
                    },
                });
                return;
            }
            check_using_deconstruction(child, version, errors);
        }
    }
}

fn check_using_operator(
    node: &UsingOperator,
    version: LanguageVersion,
    errors: &mut Vec<SyntaxVersionError>,
) {
    if version < LanguageVersion::V0_8_19 {
        errors.push(SyntaxVersionError {
            range: expect_range(node),
            enabled: LanguageVersionSpecifier::From {
                from: LanguageVersion::V0_8_19,
            },
        });
        return;
    }

    match node {
        UsingOperator::Ampersand(_) => {}
        UsingOperator::Asterisk(_) => {}
        UsingOperator::BangEqual(_) => {}
        UsingOperator::Bar(_) => {}
        UsingOperator::Caret(_) => {}
        UsingOperator::EqualEqual(_) => {}
        UsingOperator::GreaterThan(_) => {}
        UsingOperator::GreaterThanEqual(_) => {}
        UsingOperator::LessThan(_) => {}
        UsingOperator::LessThanEqual(_) => {}
        UsingOperator::Minus(_) => {}
        UsingOperator::Percent(_) => {}
        UsingOperator::Plus(_) => {}
        UsingOperator::Slash(_) => {}
        UsingOperator::Tilde(_) => {}
    }
}

fn check_using_target(
    node: &UsingTarget,
    version: LanguageVersion,
    errors: &mut Vec<SyntaxVersionError>,
) {
    match node {
        UsingTarget::TypeName(child) => {
            check_type_name(child, version, errors);
        }
        UsingTarget::Asterisk(_) => {}
    }
}

fn check_variable_declaration_target(
    node: &VariableDeclarationTarget,
    version: LanguageVersion,
    errors: &mut Vec<SyntaxVersionError>,
) {
    match node {
        VariableDeclarationTarget::SingleTypedDeclaration(child) => {
            check_single_typed_declaration(child, version, errors);
        }
        VariableDeclarationTarget::MultiTypedDeclaration(child) => {
            check_multi_typed_declaration(child, version, errors);
        }
    }
}

//
// Collection validators
//

fn check_array_values(
    node: &ArrayValues,
    version: LanguageVersion,
    errors: &mut Vec<SyntaxVersionError>,
) {
    for child in node.elements() {
        check_expression(child, version, errors);
    }
}

fn check_call_options(
    node: &CallOptions,
    version: LanguageVersion,
    errors: &mut Vec<SyntaxVersionError>,
) {
    for child in node.elements() {
        check_named_argument(child, version, errors);
    }
}

fn check_catch_clauses(
    node: &CatchClauses,
    version: LanguageVersion,
    errors: &mut Vec<SyntaxVersionError>,
) {
    for child in node.elements() {
        check_catch_clause(child, version, errors);
    }
}

fn check_constructor_attributes(
    node: &ConstructorAttributes,
    version: LanguageVersion,
    errors: &mut Vec<SyntaxVersionError>,
) {
    for child in node.elements() {
        check_constructor_attribute(child, version, errors);
    }
}

fn check_contract_members(
    node: &ContractMembers,
    version: LanguageVersion,
    errors: &mut Vec<SyntaxVersionError>,
) {
    for child in node.elements() {
        check_contract_member(child, version, errors);
    }
}

fn check_contract_specifiers(
    node: &ContractSpecifiers,
    version: LanguageVersion,
    errors: &mut Vec<SyntaxVersionError>,
) {
    for child in node.elements() {
        check_contract_specifier(child, version, errors);
    }
}

fn check_error_parameters(
    node: &ErrorParameters,
    version: LanguageVersion,
    errors: &mut Vec<SyntaxVersionError>,
) {
    if version < LanguageVersion::V0_8_4 {
        errors.push(SyntaxVersionError {
            range: expect_range(node),
            enabled: LanguageVersionSpecifier::From {
                from: LanguageVersion::V0_8_4,
            },
        });
        return;
    }
    for child in node.elements() {
        check_error_parameter(child, version, errors);
    }
}

fn check_event_parameters(
    node: &EventParameters,
    version: LanguageVersion,
    errors: &mut Vec<SyntaxVersionError>,
) {
    for child in node.elements() {
        check_event_parameter(child, version, errors);
    }
}

fn check_fallback_function_attributes(
    node: &FallbackFunctionAttributes,
    version: LanguageVersion,
    errors: &mut Vec<SyntaxVersionError>,
) {
    for child in node.elements() {
        check_fallback_function_attribute(child, version, errors);
    }
}

fn check_function_attributes(
    node: &FunctionAttributes,
    version: LanguageVersion,
    errors: &mut Vec<SyntaxVersionError>,
) {
    for child in node.elements() {
        check_function_attribute(child, version, errors);
    }
}

fn check_inheritance_types(
    node: &InheritanceTypes,
    version: LanguageVersion,
    errors: &mut Vec<SyntaxVersionError>,
) {
    for child in node.elements() {
        check_inheritance_type(child, version, errors);
    }
}

fn check_interface_members(
    node: &InterfaceMembers,
    version: LanguageVersion,
    errors: &mut Vec<SyntaxVersionError>,
) {
    for child in node.elements() {
        check_contract_member(child, version, errors);
    }
}

fn check_library_members(
    node: &LibraryMembers,
    version: LanguageVersion,
    errors: &mut Vec<SyntaxVersionError>,
) {
    for child in node.elements() {
        check_contract_member(child, version, errors);
    }
}

fn check_multi_typed_declaration_elements(
    node: &MultiTypedDeclarationElements,
    version: LanguageVersion,
    errors: &mut Vec<SyntaxVersionError>,
) {
    for child in node.elements() {
        check_multi_typed_declaration_element(child, version, errors);
    }
}

fn check_named_arguments(
    node: &NamedArguments,
    version: LanguageVersion,
    errors: &mut Vec<SyntaxVersionError>,
) {
    for child in node.elements() {
        check_named_argument(child, version, errors);
    }
}

fn check_parameters(
    node: &Parameters,
    version: LanguageVersion,
    errors: &mut Vec<SyntaxVersionError>,
) {
    for child in node.elements() {
        check_parameter(child, version, errors);
    }
}

fn check_positional_arguments(
    node: &PositionalArguments,
    version: LanguageVersion,
    errors: &mut Vec<SyntaxVersionError>,
) {
    for child in node.elements() {
        check_expression(child, version, errors);
    }
}

fn check_receive_function_attributes(
    node: &ReceiveFunctionAttributes,
    version: LanguageVersion,
    errors: &mut Vec<SyntaxVersionError>,
) {
    for child in node.elements() {
        check_receive_function_attribute(child, version, errors);
    }
}

fn check_source_unit_members(
    node: &SourceUnitMembers,
    version: LanguageVersion,
    errors: &mut Vec<SyntaxVersionError>,
) {
    for child in node.elements() {
        check_source_unit_member(child, version, errors);
    }
}

fn check_state_variable_attributes(
    node: &StateVariableAttributes,
    version: LanguageVersion,
    errors: &mut Vec<SyntaxVersionError>,
) {
    for child in node.elements() {
        check_state_variable_attribute(child, version, errors);
    }
}

fn check_statements(
    node: &Statements,
    version: LanguageVersion,
    errors: &mut Vec<SyntaxVersionError>,
) {
    for child in node.elements() {
        check_statement(child, version, errors);
    }
}

fn check_struct_members(
    node: &StructMembers,
    version: LanguageVersion,
    errors: &mut Vec<SyntaxVersionError>,
) {
    for child in node.elements() {
        check_struct_member(child, version, errors);
    }
}

fn check_tuple_values(
    node: &TupleValues,
    version: LanguageVersion,
    errors: &mut Vec<SyntaxVersionError>,
) {
    for child in node.elements() {
        check_tuple_value(child, version, errors);
    }
}

fn check_using_deconstruction_symbols(
    node: &UsingDeconstructionSymbols,
    version: LanguageVersion,
    errors: &mut Vec<SyntaxVersionError>,
) {
    if version < LanguageVersion::V0_8_13 {
        errors.push(SyntaxVersionError {
            range: expect_range(node),
            enabled: LanguageVersionSpecifier::From {
                from: LanguageVersion::V0_8_13,
            },
        });
        return;
    }
    for child in node.elements() {
        check_using_deconstruction_symbol(child, version, errors);
    }
}

fn check_yul_flags(
    node: &YulFlags,
    version: LanguageVersion,
    errors: &mut Vec<SyntaxVersionError>,
) {
    if version < LanguageVersion::V0_8_13 {
        errors.push(SyntaxVersionError {
            range: expect_range(node),
            enabled: LanguageVersionSpecifier::From {
                from: LanguageVersion::V0_8_13,
            },
        });
        return;
    }
}

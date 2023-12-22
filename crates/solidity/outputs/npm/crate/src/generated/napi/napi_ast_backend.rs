// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

#![allow(clippy::too_many_lines)]

use napi::bindgen_prelude::Env;
use napi::JsObject;
use napi_derive::napi;

use crate::napi::napi_ast_helpers::Helper;
use crate::napi::napi_cst::RuleNode;
use crate::napi::{RuleKind, TokenKind};

#[napi(
    namespace = "ast_internal",
    ts_return_type = "Array<cst.Node | null>",
    catch_unwind
)]
pub fn pick_sequence(
    #[napi(ts_arg_type = "cst.RuleNode")] node: &RuleNode,
    #[napi(ts_arg_type = "kinds.RuleKind")] kind: RuleKind,
    env: Env,
) -> Vec<Option<JsObject>> {
    let mut helper = Helper::new(node, kind, env);

    match kind {
        RuleKind::SourceUnit => pick_source_unit(&mut helper),
        RuleKind::PragmaDirective => pick_pragma_directive(&mut helper),
        RuleKind::ABICoderPragma => pick_abi_coder_pragma(&mut helper),
        RuleKind::ExperimentalPragma => pick_experimental_pragma(&mut helper),
        RuleKind::VersionPragma => pick_version_pragma(&mut helper),
        RuleKind::VersionPragmaOrExpression => pick_version_pragma_or_expression(&mut helper),
        RuleKind::VersionPragmaRangeExpression => pick_version_pragma_range_expression(&mut helper),
        RuleKind::VersionPragmaPrefixExpression => {
            pick_version_pragma_prefix_expression(&mut helper)
        }
        RuleKind::ImportDirective => pick_import_directive(&mut helper),
        RuleKind::PathImport => pick_path_import(&mut helper),
        RuleKind::NamedImport => pick_named_import(&mut helper),
        RuleKind::ImportDeconstruction => pick_import_deconstruction(&mut helper),
        RuleKind::ImportDeconstructionSymbol => pick_import_deconstruction_symbol(&mut helper),
        RuleKind::ImportAlias => pick_import_alias(&mut helper),
        RuleKind::UsingDirective => pick_using_directive(&mut helper),
        RuleKind::UsingDeconstruction => pick_using_deconstruction(&mut helper),
        RuleKind::UsingDeconstructionSymbol => pick_using_deconstruction_symbol(&mut helper),
        RuleKind::UsingAlias => pick_using_alias(&mut helper),
        RuleKind::ContractDefinition => pick_contract_definition(&mut helper),
        RuleKind::InheritanceSpecifier => pick_inheritance_specifier(&mut helper),
        RuleKind::InheritanceType => pick_inheritance_type(&mut helper),
        RuleKind::InterfaceDefinition => pick_interface_definition(&mut helper),
        RuleKind::LibraryDefinition => pick_library_definition(&mut helper),
        RuleKind::StructDefinition => pick_struct_definition(&mut helper),
        RuleKind::StructMember => pick_struct_member(&mut helper),
        RuleKind::EnumDefinition => pick_enum_definition(&mut helper),
        RuleKind::ConstantDefinition => pick_constant_definition(&mut helper),
        RuleKind::StateVariableDefinition => pick_state_variable_definition(&mut helper),
        RuleKind::StateVariableDefinitionValue => pick_state_variable_definition_value(&mut helper),
        RuleKind::FunctionDefinition => pick_function_definition(&mut helper),
        RuleKind::ParametersDeclaration => pick_parameters_declaration(&mut helper),
        RuleKind::Parameter => pick_parameter(&mut helper),
        RuleKind::OverrideSpecifier => pick_override_specifier(&mut helper),
        RuleKind::OverridePathsDeclaration => pick_override_paths_declaration(&mut helper),
        RuleKind::ReturnsDeclaration => pick_returns_declaration(&mut helper),
        RuleKind::ConstructorDefinition => pick_constructor_definition(&mut helper),
        RuleKind::UnnamedFunctionDefinition => pick_unnamed_function_definition(&mut helper),
        RuleKind::FallbackFunctionDefinition => pick_fallback_function_definition(&mut helper),
        RuleKind::ReceiveFunctionDefinition => pick_receive_function_definition(&mut helper),
        RuleKind::ModifierDefinition => pick_modifier_definition(&mut helper),
        RuleKind::ModifierInvocation => pick_modifier_invocation(&mut helper),
        RuleKind::EventDefinition => pick_event_definition(&mut helper),
        RuleKind::EventParametersDeclaration => pick_event_parameters_declaration(&mut helper),
        RuleKind::EventParameter => pick_event_parameter(&mut helper),
        RuleKind::UserDefinedValueTypeDefinition => {
            pick_user_defined_value_type_definition(&mut helper)
        }
        RuleKind::ErrorDefinition => pick_error_definition(&mut helper),
        RuleKind::ErrorParametersDeclaration => pick_error_parameters_declaration(&mut helper),
        RuleKind::ErrorParameter => pick_error_parameter(&mut helper),
        RuleKind::ArrayTypeName => pick_array_type_name(&mut helper),
        RuleKind::FunctionType => pick_function_type(&mut helper),
        RuleKind::MappingType => pick_mapping_type(&mut helper),
        RuleKind::MappingKey => pick_mapping_key(&mut helper),
        RuleKind::MappingValue => pick_mapping_value(&mut helper),
        RuleKind::AddressType => pick_address_type(&mut helper),
        RuleKind::Block => pick_block(&mut helper),
        RuleKind::UncheckedBlock => pick_unchecked_block(&mut helper),
        RuleKind::ExpressionStatement => pick_expression_statement(&mut helper),
        RuleKind::AssemblyStatement => pick_assembly_statement(&mut helper),
        RuleKind::AssemblyFlagsDeclaration => pick_assembly_flags_declaration(&mut helper),
        RuleKind::TupleDeconstructionStatement => pick_tuple_deconstruction_statement(&mut helper),
        RuleKind::TupleDeconstructionElement => pick_tuple_deconstruction_element(&mut helper),
        RuleKind::TypedTupleMember => pick_typed_tuple_member(&mut helper),
        RuleKind::UntypedTupleMember => pick_untyped_tuple_member(&mut helper),
        RuleKind::VariableDeclarationStatement => pick_variable_declaration_statement(&mut helper),
        RuleKind::VariableDeclarationValue => pick_variable_declaration_value(&mut helper),
        RuleKind::IfStatement => pick_if_statement(&mut helper),
        RuleKind::ElseBranch => pick_else_branch(&mut helper),
        RuleKind::ForStatement => pick_for_statement(&mut helper),
        RuleKind::WhileStatement => pick_while_statement(&mut helper),
        RuleKind::DoWhileStatement => pick_do_while_statement(&mut helper),
        RuleKind::ContinueStatement => pick_continue_statement(&mut helper),
        RuleKind::BreakStatement => pick_break_statement(&mut helper),
        RuleKind::ReturnStatement => pick_return_statement(&mut helper),
        RuleKind::EmitStatement => pick_emit_statement(&mut helper),
        RuleKind::DeleteStatement => pick_delete_statement(&mut helper),
        RuleKind::TryStatement => pick_try_statement(&mut helper),
        RuleKind::CatchClause => pick_catch_clause(&mut helper),
        RuleKind::CatchClauseError => pick_catch_clause_error(&mut helper),
        RuleKind::RevertStatement => pick_revert_statement(&mut helper),
        RuleKind::ThrowStatement => pick_throw_statement(&mut helper),
        RuleKind::AssignmentExpression => pick_assignment_expression(&mut helper),
        RuleKind::ConditionalExpression => pick_conditional_expression(&mut helper),
        RuleKind::OrExpression => pick_or_expression(&mut helper),
        RuleKind::AndExpression => pick_and_expression(&mut helper),
        RuleKind::EqualityExpression => pick_equality_expression(&mut helper),
        RuleKind::ComparisonExpression => pick_comparison_expression(&mut helper),
        RuleKind::BitwiseOrExpression => pick_bitwise_or_expression(&mut helper),
        RuleKind::BitwiseXorExpression => pick_bitwise_xor_expression(&mut helper),
        RuleKind::BitwiseAndExpression => pick_bitwise_and_expression(&mut helper),
        RuleKind::ShiftExpression => pick_shift_expression(&mut helper),
        RuleKind::AdditiveExpression => pick_additive_expression(&mut helper),
        RuleKind::MultiplicativeExpression => pick_multiplicative_expression(&mut helper),
        RuleKind::ExponentiationExpression => pick_exponentiation_expression(&mut helper),
        RuleKind::PostfixExpression => pick_postfix_expression(&mut helper),
        RuleKind::PrefixExpression => pick_prefix_expression(&mut helper),
        RuleKind::FunctionCallExpression => pick_function_call_expression(&mut helper),
        RuleKind::MemberAccessExpression => pick_member_access_expression(&mut helper),
        RuleKind::IndexAccessExpression => pick_index_access_expression(&mut helper),
        RuleKind::IndexAccessEnd => pick_index_access_end(&mut helper),
        RuleKind::PositionalArgumentsDeclaration => {
            pick_positional_arguments_declaration(&mut helper)
        }
        RuleKind::NamedArgumentsDeclaration => pick_named_arguments_declaration(&mut helper),
        RuleKind::NamedArgumentGroup => pick_named_argument_group(&mut helper),
        RuleKind::NamedArgument => pick_named_argument(&mut helper),
        RuleKind::TypeExpression => pick_type_expression(&mut helper),
        RuleKind::NewExpression => pick_new_expression(&mut helper),
        RuleKind::TupleExpression => pick_tuple_expression(&mut helper),
        RuleKind::TupleValue => pick_tuple_value(&mut helper),
        RuleKind::ArrayExpression => pick_array_expression(&mut helper),
        RuleKind::HexNumberExpression => pick_hex_number_expression(&mut helper),
        RuleKind::DecimalNumberExpression => pick_decimal_number_expression(&mut helper),
        RuleKind::YulBlock => pick_yul_block(&mut helper),
        RuleKind::YulFunctionDefinition => pick_yul_function_definition(&mut helper),
        RuleKind::YulParametersDeclaration => pick_yul_parameters_declaration(&mut helper),
        RuleKind::YulReturnsDeclaration => pick_yul_returns_declaration(&mut helper),
        RuleKind::YulVariableDeclarationStatement => {
            pick_yul_variable_declaration_statement(&mut helper)
        }
        RuleKind::YulVariableDeclarationValue => pick_yul_variable_declaration_value(&mut helper),
        RuleKind::YulAssignmentStatement => pick_yul_assignment_statement(&mut helper),
        RuleKind::YulIfStatement => pick_yul_if_statement(&mut helper),
        RuleKind::YulLeaveStatement => pick_yul_leave_statement(&mut helper),
        RuleKind::YulBreakStatement => pick_yul_break_statement(&mut helper),
        RuleKind::YulContinueStatement => pick_yul_continue_statement(&mut helper),
        RuleKind::YulForStatement => pick_yul_for_statement(&mut helper),
        RuleKind::YulSwitchStatement => pick_yul_switch_statement(&mut helper),
        RuleKind::YulDefaultCase => pick_yul_default_case(&mut helper),
        RuleKind::YulValueCase => pick_yul_value_case(&mut helper),
        RuleKind::YulFunctionCallExpression => pick_yul_function_call_expression(&mut helper),
        _ => {
            unreachable!("Unknown sequence: '{kind:?}'.");
        }
    }
}

fn pick_source_unit(helper: &mut Helper) -> Vec<Option<JsObject>> {
    vec![helper.try_pick(|node| node.is_rule_with_kind(RuleKind::SourceUnitMembers))]
}

fn pick_pragma_directive(helper: &mut Helper) -> Vec<Option<JsObject>> {
    vec![
        Some(helper.pick(|node| node.is_token_with_kind(TokenKind::PragmaKeyword))),
        Some(helper.pick(|node| node.is_rule_with_kind(RuleKind::Pragma))),
        Some(helper.pick(|node| node.is_token_with_kind(TokenKind::Semicolon))),
    ]
}

fn pick_abi_coder_pragma(helper: &mut Helper) -> Vec<Option<JsObject>> {
    vec![
        Some(helper.pick(|node| node.is_token_with_kind(TokenKind::AbicoderKeyword))),
        Some(helper.pick(|node| node.is_token_with_kind(TokenKind::Identifier))),
    ]
}

fn pick_experimental_pragma(helper: &mut Helper) -> Vec<Option<JsObject>> {
    vec![
        Some(helper.pick(|node| node.is_token_with_kind(TokenKind::ExperimentalKeyword))),
        Some(helper.pick(|node| node.is_rule_with_kind(RuleKind::ExperimentalFeature))),
    ]
}

fn pick_version_pragma(helper: &mut Helper) -> Vec<Option<JsObject>> {
    vec![
        Some(helper.pick(|node| node.is_token_with_kind(TokenKind::SolidityKeyword))),
        Some(helper.pick(|node| node.is_rule_with_kind(RuleKind::VersionPragmaExpressions))),
    ]
}

fn pick_version_pragma_or_expression(helper: &mut Helper) -> Vec<Option<JsObject>> {
    vec![
        Some(helper.pick(|node| node.is_rule_with_kind(RuleKind::VersionPragmaExpression))),
        Some(helper.pick(|node| node.is_token_with_kind(TokenKind::BarBar))),
        Some(helper.pick(|node| node.is_rule_with_kind(RuleKind::VersionPragmaExpression))),
    ]
}

fn pick_version_pragma_range_expression(helper: &mut Helper) -> Vec<Option<JsObject>> {
    vec![
        Some(helper.pick(|node| node.is_rule_with_kind(RuleKind::VersionPragmaExpression))),
        Some(helper.pick(|node| node.is_token_with_kind(TokenKind::Minus))),
        Some(helper.pick(|node| node.is_rule_with_kind(RuleKind::VersionPragmaExpression))),
    ]
}

fn pick_version_pragma_prefix_expression(helper: &mut Helper) -> Vec<Option<JsObject>> {
    vec![
        Some(helper.pick(|node| node.is_rule_with_kind(RuleKind::VersionPragmaExpression))),
        Some(helper.pick(|node| node.is_token_with_kind(TokenKind::Caret))),
    ]
}

fn pick_import_directive(helper: &mut Helper) -> Vec<Option<JsObject>> {
    vec![
        Some(helper.pick(|node| node.is_token_with_kind(TokenKind::ImportKeyword))),
        Some(helper.pick(|node| node.is_rule_with_kind(RuleKind::ImportClause))),
        Some(helper.pick(|node| node.is_token_with_kind(TokenKind::Semicolon))),
    ]
}

fn pick_path_import(helper: &mut Helper) -> Vec<Option<JsObject>> {
    vec![
        Some(helper.pick(|node| node.is_token_with_kind(TokenKind::AsciiStringLiteral))),
        helper.try_pick(|node| node.is_rule_with_kind(RuleKind::ImportAlias)),
    ]
}

fn pick_named_import(helper: &mut Helper) -> Vec<Option<JsObject>> {
    vec![
        Some(helper.pick(|node| node.is_token_with_kind(TokenKind::Asterisk))),
        Some(helper.pick(|node| node.is_rule_with_kind(RuleKind::ImportAlias))),
        Some(helper.pick(|node| node.is_token_with_kind(TokenKind::FromKeyword))),
        Some(helper.pick(|node| node.is_token_with_kind(TokenKind::AsciiStringLiteral))),
    ]
}

fn pick_import_deconstruction(helper: &mut Helper) -> Vec<Option<JsObject>> {
    vec![
        Some(helper.pick(|node| node.is_token_with_kind(TokenKind::OpenBrace))),
        Some(helper.pick(|node| node.is_rule_with_kind(RuleKind::ImportDeconstructionSymbols))),
        Some(helper.pick(|node| node.is_token_with_kind(TokenKind::CloseBrace))),
        Some(helper.pick(|node| node.is_token_with_kind(TokenKind::FromKeyword))),
        Some(helper.pick(|node| node.is_token_with_kind(TokenKind::AsciiStringLiteral))),
    ]
}

fn pick_import_deconstruction_symbol(helper: &mut Helper) -> Vec<Option<JsObject>> {
    vec![
        Some(helper.pick(|node| node.is_token_with_kind(TokenKind::Identifier))),
        helper.try_pick(|node| node.is_rule_with_kind(RuleKind::ImportAlias)),
    ]
}

fn pick_import_alias(helper: &mut Helper) -> Vec<Option<JsObject>> {
    vec![
        Some(helper.pick(|node| node.is_token_with_kind(TokenKind::AsKeyword))),
        Some(helper.pick(|node| node.is_token_with_kind(TokenKind::Identifier))),
    ]
}

fn pick_using_directive(helper: &mut Helper) -> Vec<Option<JsObject>> {
    vec![
        Some(helper.pick(|node| node.is_token_with_kind(TokenKind::UsingKeyword))),
        Some(helper.pick(|node| node.is_rule_with_kind(RuleKind::UsingClause))),
        Some(helper.pick(|node| node.is_token_with_kind(TokenKind::ForKeyword))),
        Some(helper.pick(|node| node.is_rule_with_kind(RuleKind::UsingTarget))),
        helper.try_pick(|node| node.is_token_with_kind(TokenKind::GlobalKeyword)),
        Some(helper.pick(|node| node.is_token_with_kind(TokenKind::Semicolon))),
    ]
}

fn pick_using_deconstruction(helper: &mut Helper) -> Vec<Option<JsObject>> {
    vec![
        Some(helper.pick(|node| node.is_token_with_kind(TokenKind::OpenBrace))),
        Some(helper.pick(|node| node.is_rule_with_kind(RuleKind::UsingDeconstructionSymbols))),
        Some(helper.pick(|node| node.is_token_with_kind(TokenKind::CloseBrace))),
    ]
}

fn pick_using_deconstruction_symbol(helper: &mut Helper) -> Vec<Option<JsObject>> {
    vec![
        Some(helper.pick(|node| node.is_rule_with_kind(RuleKind::IdentifierPath))),
        helper.try_pick(|node| node.is_rule_with_kind(RuleKind::UsingAlias)),
    ]
}

fn pick_using_alias(helper: &mut Helper) -> Vec<Option<JsObject>> {
    vec![
        Some(helper.pick(|node| node.is_token_with_kind(TokenKind::AsKeyword))),
        Some(helper.pick(|node| node.is_rule_with_kind(RuleKind::UsingOperator))),
    ]
}

fn pick_contract_definition(helper: &mut Helper) -> Vec<Option<JsObject>> {
    vec![
        helper.try_pick(|node| node.is_token_with_kind(TokenKind::AbstractKeyword)),
        Some(helper.pick(|node| node.is_token_with_kind(TokenKind::ContractKeyword))),
        Some(helper.pick(|node| node.is_token_with_kind(TokenKind::Identifier))),
        helper.try_pick(|node| node.is_rule_with_kind(RuleKind::InheritanceSpecifier)),
        Some(helper.pick(|node| node.is_token_with_kind(TokenKind::OpenBrace))),
        helper.try_pick(|node| node.is_rule_with_kind(RuleKind::ContractMembers)),
        Some(helper.pick(|node| node.is_token_with_kind(TokenKind::CloseBrace))),
    ]
}

fn pick_inheritance_specifier(helper: &mut Helper) -> Vec<Option<JsObject>> {
    vec![
        Some(helper.pick(|node| node.is_token_with_kind(TokenKind::IsKeyword))),
        Some(helper.pick(|node| node.is_rule_with_kind(RuleKind::InheritanceTypes))),
    ]
}

fn pick_inheritance_type(helper: &mut Helper) -> Vec<Option<JsObject>> {
    vec![
        Some(helper.pick(|node| node.is_rule_with_kind(RuleKind::IdentifierPath))),
        helper.try_pick(|node| node.is_rule_with_kind(RuleKind::ArgumentsDeclaration)),
    ]
}

fn pick_interface_definition(helper: &mut Helper) -> Vec<Option<JsObject>> {
    vec![
        Some(helper.pick(|node| node.is_token_with_kind(TokenKind::InterfaceKeyword))),
        Some(helper.pick(|node| node.is_token_with_kind(TokenKind::Identifier))),
        helper.try_pick(|node| node.is_rule_with_kind(RuleKind::InheritanceSpecifier)),
        Some(helper.pick(|node| node.is_token_with_kind(TokenKind::OpenBrace))),
        helper.try_pick(|node| node.is_rule_with_kind(RuleKind::InterfaceMembers)),
        Some(helper.pick(|node| node.is_token_with_kind(TokenKind::CloseBrace))),
    ]
}

fn pick_library_definition(helper: &mut Helper) -> Vec<Option<JsObject>> {
    vec![
        Some(helper.pick(|node| node.is_token_with_kind(TokenKind::LibraryKeyword))),
        Some(helper.pick(|node| node.is_token_with_kind(TokenKind::Identifier))),
        Some(helper.pick(|node| node.is_token_with_kind(TokenKind::OpenBrace))),
        helper.try_pick(|node| node.is_rule_with_kind(RuleKind::LibraryMembers)),
        Some(helper.pick(|node| node.is_token_with_kind(TokenKind::CloseBrace))),
    ]
}

fn pick_struct_definition(helper: &mut Helper) -> Vec<Option<JsObject>> {
    vec![
        Some(helper.pick(|node| node.is_token_with_kind(TokenKind::StructKeyword))),
        Some(helper.pick(|node| node.is_token_with_kind(TokenKind::Identifier))),
        Some(helper.pick(|node| node.is_token_with_kind(TokenKind::OpenBrace))),
        helper.try_pick(|node| node.is_rule_with_kind(RuleKind::StructMembers)),
        Some(helper.pick(|node| node.is_token_with_kind(TokenKind::CloseBrace))),
    ]
}

fn pick_struct_member(helper: &mut Helper) -> Vec<Option<JsObject>> {
    vec![
        Some(helper.pick(|node| node.is_rule_with_kind(RuleKind::TypeName))),
        Some(helper.pick(|node| node.is_token_with_kind(TokenKind::Identifier))),
        Some(helper.pick(|node| node.is_token_with_kind(TokenKind::Semicolon))),
    ]
}

fn pick_enum_definition(helper: &mut Helper) -> Vec<Option<JsObject>> {
    vec![
        Some(helper.pick(|node| node.is_token_with_kind(TokenKind::EnumKeyword))),
        Some(helper.pick(|node| node.is_token_with_kind(TokenKind::Identifier))),
        Some(helper.pick(|node| node.is_token_with_kind(TokenKind::OpenBrace))),
        helper.try_pick(|node| node.is_rule_with_kind(RuleKind::EnumMembers)),
        Some(helper.pick(|node| node.is_token_with_kind(TokenKind::CloseBrace))),
    ]
}

fn pick_constant_definition(helper: &mut Helper) -> Vec<Option<JsObject>> {
    vec![
        Some(helper.pick(|node| node.is_rule_with_kind(RuleKind::TypeName))),
        Some(helper.pick(|node| node.is_token_with_kind(TokenKind::ConstantKeyword))),
        Some(helper.pick(|node| node.is_token_with_kind(TokenKind::Identifier))),
        Some(helper.pick(|node| node.is_token_with_kind(TokenKind::Equal))),
        Some(helper.pick(|node| node.is_rule_with_kind(RuleKind::Expression))),
        Some(helper.pick(|node| node.is_token_with_kind(TokenKind::Semicolon))),
    ]
}

fn pick_state_variable_definition(helper: &mut Helper) -> Vec<Option<JsObject>> {
    vec![
        Some(helper.pick(|node| node.is_rule_with_kind(RuleKind::TypeName))),
        helper.try_pick(|node| node.is_rule_with_kind(RuleKind::StateVariableAttributes)),
        Some(helper.pick(|node| node.is_token_with_kind(TokenKind::Identifier))),
        helper.try_pick(|node| node.is_rule_with_kind(RuleKind::StateVariableDefinitionValue)),
        Some(helper.pick(|node| node.is_token_with_kind(TokenKind::Semicolon))),
    ]
}

fn pick_state_variable_definition_value(helper: &mut Helper) -> Vec<Option<JsObject>> {
    vec![
        Some(helper.pick(|node| node.is_token_with_kind(TokenKind::Equal))),
        Some(helper.pick(|node| node.is_rule_with_kind(RuleKind::Expression))),
    ]
}

fn pick_function_definition(helper: &mut Helper) -> Vec<Option<JsObject>> {
    vec![
        Some(helper.pick(|node| node.is_token_with_kind(TokenKind::FunctionKeyword))),
        Some(helper.pick(|node| node.is_rule_with_kind(RuleKind::FunctionName))),
        Some(helper.pick(|node| node.is_rule_with_kind(RuleKind::ParametersDeclaration))),
        helper.try_pick(|node| node.is_rule_with_kind(RuleKind::FunctionAttributes)),
        helper.try_pick(|node| node.is_rule_with_kind(RuleKind::ReturnsDeclaration)),
        Some(helper.pick(|node| node.is_rule_with_kind(RuleKind::FunctionBody))),
    ]
}

fn pick_parameters_declaration(helper: &mut Helper) -> Vec<Option<JsObject>> {
    vec![
        Some(helper.pick(|node| node.is_token_with_kind(TokenKind::OpenParen))),
        helper.try_pick(|node| node.is_rule_with_kind(RuleKind::Parameters)),
        Some(helper.pick(|node| node.is_token_with_kind(TokenKind::CloseParen))),
    ]
}

fn pick_parameter(helper: &mut Helper) -> Vec<Option<JsObject>> {
    vec![
        Some(helper.pick(|node| node.is_rule_with_kind(RuleKind::TypeName))),
        helper.try_pick(|node| node.is_rule_with_kind(RuleKind::StorageLocation)),
        helper.try_pick(|node| node.is_token_with_kind(TokenKind::Identifier)),
    ]
}

fn pick_override_specifier(helper: &mut Helper) -> Vec<Option<JsObject>> {
    vec![
        Some(helper.pick(|node| node.is_token_with_kind(TokenKind::OverrideKeyword))),
        helper.try_pick(|node| node.is_rule_with_kind(RuleKind::OverridePathsDeclaration)),
    ]
}

fn pick_override_paths_declaration(helper: &mut Helper) -> Vec<Option<JsObject>> {
    vec![
        Some(helper.pick(|node| node.is_token_with_kind(TokenKind::OpenParen))),
        Some(helper.pick(|node| node.is_rule_with_kind(RuleKind::OverridePaths))),
        Some(helper.pick(|node| node.is_token_with_kind(TokenKind::CloseParen))),
    ]
}

fn pick_returns_declaration(helper: &mut Helper) -> Vec<Option<JsObject>> {
    vec![
        Some(helper.pick(|node| node.is_token_with_kind(TokenKind::ReturnsKeyword))),
        Some(helper.pick(|node| node.is_rule_with_kind(RuleKind::ParametersDeclaration))),
    ]
}

fn pick_constructor_definition(helper: &mut Helper) -> Vec<Option<JsObject>> {
    vec![
        Some(helper.pick(|node| node.is_token_with_kind(TokenKind::ConstructorKeyword))),
        Some(helper.pick(|node| node.is_rule_with_kind(RuleKind::ParametersDeclaration))),
        helper.try_pick(|node| node.is_rule_with_kind(RuleKind::ConstructorAttributes)),
        Some(helper.pick(|node| node.is_rule_with_kind(RuleKind::Block))),
    ]
}

fn pick_unnamed_function_definition(helper: &mut Helper) -> Vec<Option<JsObject>> {
    vec![
        Some(helper.pick(|node| node.is_token_with_kind(TokenKind::FunctionKeyword))),
        Some(helper.pick(|node| node.is_rule_with_kind(RuleKind::ParametersDeclaration))),
        helper.try_pick(|node| node.is_rule_with_kind(RuleKind::UnnamedFunctionAttributes)),
        Some(helper.pick(|node| node.is_rule_with_kind(RuleKind::FunctionBody))),
    ]
}

fn pick_fallback_function_definition(helper: &mut Helper) -> Vec<Option<JsObject>> {
    vec![
        Some(helper.pick(|node| node.is_token_with_kind(TokenKind::FallbackKeyword))),
        Some(helper.pick(|node| node.is_rule_with_kind(RuleKind::ParametersDeclaration))),
        helper.try_pick(|node| node.is_rule_with_kind(RuleKind::FallbackFunctionAttributes)),
        helper.try_pick(|node| node.is_rule_with_kind(RuleKind::ReturnsDeclaration)),
        Some(helper.pick(|node| node.is_rule_with_kind(RuleKind::FunctionBody))),
    ]
}

fn pick_receive_function_definition(helper: &mut Helper) -> Vec<Option<JsObject>> {
    vec![
        Some(helper.pick(|node| node.is_token_with_kind(TokenKind::ReceiveKeyword))),
        Some(helper.pick(|node| node.is_rule_with_kind(RuleKind::ParametersDeclaration))),
        helper.try_pick(|node| node.is_rule_with_kind(RuleKind::ReceiveFunctionAttributes)),
        Some(helper.pick(|node| node.is_rule_with_kind(RuleKind::FunctionBody))),
    ]
}

fn pick_modifier_definition(helper: &mut Helper) -> Vec<Option<JsObject>> {
    vec![
        Some(helper.pick(|node| node.is_token_with_kind(TokenKind::ModifierKeyword))),
        Some(helper.pick(|node| node.is_token_with_kind(TokenKind::Identifier))),
        helper.try_pick(|node| node.is_rule_with_kind(RuleKind::ParametersDeclaration)),
        helper.try_pick(|node| node.is_rule_with_kind(RuleKind::ModifierAttributes)),
        Some(helper.pick(|node| node.is_rule_with_kind(RuleKind::FunctionBody))),
    ]
}

fn pick_modifier_invocation(helper: &mut Helper) -> Vec<Option<JsObject>> {
    vec![
        Some(helper.pick(|node| node.is_rule_with_kind(RuleKind::IdentifierPath))),
        helper.try_pick(|node| node.is_rule_with_kind(RuleKind::ArgumentsDeclaration)),
    ]
}

fn pick_event_definition(helper: &mut Helper) -> Vec<Option<JsObject>> {
    vec![
        Some(helper.pick(|node| node.is_token_with_kind(TokenKind::EventKeyword))),
        Some(helper.pick(|node| node.is_token_with_kind(TokenKind::Identifier))),
        Some(helper.pick(|node| node.is_rule_with_kind(RuleKind::EventParametersDeclaration))),
        helper.try_pick(|node| node.is_token_with_kind(TokenKind::AnonymousKeyword)),
        Some(helper.pick(|node| node.is_token_with_kind(TokenKind::Semicolon))),
    ]
}

fn pick_event_parameters_declaration(helper: &mut Helper) -> Vec<Option<JsObject>> {
    vec![
        Some(helper.pick(|node| node.is_token_with_kind(TokenKind::OpenParen))),
        helper.try_pick(|node| node.is_rule_with_kind(RuleKind::EventParameters)),
        Some(helper.pick(|node| node.is_token_with_kind(TokenKind::CloseParen))),
    ]
}

fn pick_event_parameter(helper: &mut Helper) -> Vec<Option<JsObject>> {
    vec![
        Some(helper.pick(|node| node.is_rule_with_kind(RuleKind::TypeName))),
        helper.try_pick(|node| node.is_token_with_kind(TokenKind::IndexedKeyword)),
        helper.try_pick(|node| node.is_token_with_kind(TokenKind::Identifier)),
    ]
}

fn pick_user_defined_value_type_definition(helper: &mut Helper) -> Vec<Option<JsObject>> {
    vec![
        Some(helper.pick(|node| node.is_token_with_kind(TokenKind::TypeKeyword))),
        Some(helper.pick(|node| node.is_token_with_kind(TokenKind::Identifier))),
        Some(helper.pick(|node| node.is_token_with_kind(TokenKind::IsKeyword))),
        Some(helper.pick(|node| node.is_rule_with_kind(RuleKind::ElementaryType))),
        Some(helper.pick(|node| node.is_token_with_kind(TokenKind::Semicolon))),
    ]
}

fn pick_error_definition(helper: &mut Helper) -> Vec<Option<JsObject>> {
    vec![
        Some(helper.pick(|node| node.is_token_with_kind(TokenKind::ErrorKeyword))),
        Some(helper.pick(|node| node.is_token_with_kind(TokenKind::Identifier))),
        Some(helper.pick(|node| node.is_rule_with_kind(RuleKind::ErrorParametersDeclaration))),
        Some(helper.pick(|node| node.is_token_with_kind(TokenKind::Semicolon))),
    ]
}

fn pick_error_parameters_declaration(helper: &mut Helper) -> Vec<Option<JsObject>> {
    vec![
        Some(helper.pick(|node| node.is_token_with_kind(TokenKind::OpenParen))),
        helper.try_pick(|node| node.is_rule_with_kind(RuleKind::ErrorParameters)),
        Some(helper.pick(|node| node.is_token_with_kind(TokenKind::CloseParen))),
    ]
}

fn pick_error_parameter(helper: &mut Helper) -> Vec<Option<JsObject>> {
    vec![
        Some(helper.pick(|node| node.is_rule_with_kind(RuleKind::TypeName))),
        helper.try_pick(|node| node.is_token_with_kind(TokenKind::Identifier)),
    ]
}

fn pick_array_type_name(helper: &mut Helper) -> Vec<Option<JsObject>> {
    vec![
        Some(helper.pick(|node| node.is_token_with_kind(TokenKind::OpenBracket))),
        helper.try_pick(|node| node.is_rule_with_kind(RuleKind::Expression)),
        Some(helper.pick(|node| node.is_token_with_kind(TokenKind::CloseBracket))),
        Some(helper.pick(|node| node.is_rule_with_kind(RuleKind::TypeName))),
    ]
}

fn pick_function_type(helper: &mut Helper) -> Vec<Option<JsObject>> {
    vec![
        Some(helper.pick(|node| node.is_token_with_kind(TokenKind::FunctionKeyword))),
        Some(helper.pick(|node| node.is_rule_with_kind(RuleKind::ParametersDeclaration))),
        helper.try_pick(|node| node.is_rule_with_kind(RuleKind::FunctionTypeAttributes)),
        helper.try_pick(|node| node.is_rule_with_kind(RuleKind::ReturnsDeclaration)),
    ]
}

fn pick_mapping_type(helper: &mut Helper) -> Vec<Option<JsObject>> {
    vec![
        Some(helper.pick(|node| node.is_token_with_kind(TokenKind::MappingKeyword))),
        Some(helper.pick(|node| node.is_token_with_kind(TokenKind::OpenParen))),
        Some(helper.pick(|node| node.is_rule_with_kind(RuleKind::MappingKey))),
        Some(helper.pick(|node| node.is_token_with_kind(TokenKind::EqualGreaterThan))),
        Some(helper.pick(|node| node.is_rule_with_kind(RuleKind::MappingValue))),
        Some(helper.pick(|node| node.is_token_with_kind(TokenKind::CloseParen))),
    ]
}

fn pick_mapping_key(helper: &mut Helper) -> Vec<Option<JsObject>> {
    vec![
        Some(helper.pick(|node| node.is_rule_with_kind(RuleKind::MappingKeyType))),
        helper.try_pick(|node| node.is_token_with_kind(TokenKind::Identifier)),
    ]
}

fn pick_mapping_value(helper: &mut Helper) -> Vec<Option<JsObject>> {
    vec![
        Some(helper.pick(|node| node.is_rule_with_kind(RuleKind::TypeName))),
        helper.try_pick(|node| node.is_token_with_kind(TokenKind::Identifier)),
    ]
}

fn pick_address_type(helper: &mut Helper) -> Vec<Option<JsObject>> {
    vec![
        Some(helper.pick(|node| node.is_token_with_kind(TokenKind::AddressKeyword))),
        helper.try_pick(|node| node.is_token_with_kind(TokenKind::PayableKeyword)),
    ]
}

fn pick_block(helper: &mut Helper) -> Vec<Option<JsObject>> {
    vec![
        Some(helper.pick(|node| node.is_token_with_kind(TokenKind::OpenBrace))),
        helper.try_pick(|node| node.is_rule_with_kind(RuleKind::Statements)),
        Some(helper.pick(|node| node.is_token_with_kind(TokenKind::CloseBrace))),
    ]
}

fn pick_unchecked_block(helper: &mut Helper) -> Vec<Option<JsObject>> {
    vec![
        Some(helper.pick(|node| node.is_token_with_kind(TokenKind::UncheckedKeyword))),
        Some(helper.pick(|node| node.is_rule_with_kind(RuleKind::Block))),
    ]
}

fn pick_expression_statement(helper: &mut Helper) -> Vec<Option<JsObject>> {
    vec![
        Some(helper.pick(|node| node.is_rule_with_kind(RuleKind::Expression))),
        Some(helper.pick(|node| node.is_token_with_kind(TokenKind::Semicolon))),
    ]
}

fn pick_assembly_statement(helper: &mut Helper) -> Vec<Option<JsObject>> {
    vec![
        Some(helper.pick(|node| node.is_token_with_kind(TokenKind::AssemblyKeyword))),
        helper.try_pick(|node| node.is_token_with_kind(TokenKind::AsciiStringLiteral)),
        helper.try_pick(|node| node.is_rule_with_kind(RuleKind::AssemblyFlagsDeclaration)),
        Some(helper.pick(|node| node.is_rule_with_kind(RuleKind::YulBlock))),
    ]
}

fn pick_assembly_flags_declaration(helper: &mut Helper) -> Vec<Option<JsObject>> {
    vec![
        Some(helper.pick(|node| node.is_token_with_kind(TokenKind::OpenParen))),
        Some(helper.pick(|node| node.is_rule_with_kind(RuleKind::AssemblyFlags))),
        Some(helper.pick(|node| node.is_token_with_kind(TokenKind::CloseParen))),
    ]
}

fn pick_tuple_deconstruction_statement(helper: &mut Helper) -> Vec<Option<JsObject>> {
    vec![
        Some(helper.pick(|node| node.is_token_with_kind(TokenKind::OpenParen))),
        Some(helper.pick(|node| node.is_rule_with_kind(RuleKind::TupleDeconstructionElements))),
        Some(helper.pick(|node| node.is_token_with_kind(TokenKind::CloseParen))),
        Some(helper.pick(|node| node.is_token_with_kind(TokenKind::Equal))),
        Some(helper.pick(|node| node.is_rule_with_kind(RuleKind::Expression))),
        Some(helper.pick(|node| node.is_token_with_kind(TokenKind::Semicolon))),
    ]
}

fn pick_tuple_deconstruction_element(helper: &mut Helper) -> Vec<Option<JsObject>> {
    vec![helper.try_pick(|node| node.is_rule_with_kind(RuleKind::TupleMember))]
}

fn pick_typed_tuple_member(helper: &mut Helper) -> Vec<Option<JsObject>> {
    vec![
        Some(helper.pick(|node| node.is_rule_with_kind(RuleKind::TypeName))),
        helper.try_pick(|node| node.is_rule_with_kind(RuleKind::StorageLocation)),
        Some(helper.pick(|node| node.is_token_with_kind(TokenKind::Identifier))),
    ]
}

fn pick_untyped_tuple_member(helper: &mut Helper) -> Vec<Option<JsObject>> {
    vec![
        helper.try_pick(|node| node.is_rule_with_kind(RuleKind::StorageLocation)),
        Some(helper.pick(|node| node.is_token_with_kind(TokenKind::Identifier))),
    ]
}

fn pick_variable_declaration_statement(helper: &mut Helper) -> Vec<Option<JsObject>> {
    vec![
        Some(helper.pick(|node| node.is_rule_with_kind(RuleKind::VariableDeclarationType))),
        helper.try_pick(|node| node.is_rule_with_kind(RuleKind::StorageLocation)),
        Some(helper.pick(|node| node.is_token_with_kind(TokenKind::Identifier))),
        helper.try_pick(|node| node.is_rule_with_kind(RuleKind::VariableDeclarationValue)),
        Some(helper.pick(|node| node.is_token_with_kind(TokenKind::Semicolon))),
    ]
}

fn pick_variable_declaration_value(helper: &mut Helper) -> Vec<Option<JsObject>> {
    vec![
        Some(helper.pick(|node| node.is_token_with_kind(TokenKind::Equal))),
        Some(helper.pick(|node| node.is_rule_with_kind(RuleKind::Expression))),
    ]
}

fn pick_if_statement(helper: &mut Helper) -> Vec<Option<JsObject>> {
    vec![
        Some(helper.pick(|node| node.is_token_with_kind(TokenKind::IfKeyword))),
        Some(helper.pick(|node| node.is_token_with_kind(TokenKind::OpenParen))),
        Some(helper.pick(|node| node.is_rule_with_kind(RuleKind::Expression))),
        Some(helper.pick(|node| node.is_token_with_kind(TokenKind::CloseParen))),
        Some(helper.pick(|node| node.is_rule_with_kind(RuleKind::Statement))),
        helper.try_pick(|node| node.is_rule_with_kind(RuleKind::ElseBranch)),
    ]
}

fn pick_else_branch(helper: &mut Helper) -> Vec<Option<JsObject>> {
    vec![
        Some(helper.pick(|node| node.is_token_with_kind(TokenKind::ElseKeyword))),
        Some(helper.pick(|node| node.is_rule_with_kind(RuleKind::Statement))),
    ]
}

fn pick_for_statement(helper: &mut Helper) -> Vec<Option<JsObject>> {
    vec![
        Some(helper.pick(|node| node.is_token_with_kind(TokenKind::ForKeyword))),
        Some(helper.pick(|node| node.is_token_with_kind(TokenKind::OpenParen))),
        Some(helper.pick(|node| node.is_rule_with_kind(RuleKind::ForStatementInitialization))),
        Some(helper.pick(|node| node.is_rule_with_kind(RuleKind::ForStatementCondition))),
        helper.try_pick(|node| node.is_rule_with_kind(RuleKind::Expression)),
        Some(helper.pick(|node| node.is_token_with_kind(TokenKind::CloseParen))),
        Some(helper.pick(|node| node.is_rule_with_kind(RuleKind::Statement))),
    ]
}

fn pick_while_statement(helper: &mut Helper) -> Vec<Option<JsObject>> {
    vec![
        Some(helper.pick(|node| node.is_token_with_kind(TokenKind::WhileKeyword))),
        Some(helper.pick(|node| node.is_token_with_kind(TokenKind::OpenParen))),
        Some(helper.pick(|node| node.is_rule_with_kind(RuleKind::Expression))),
        Some(helper.pick(|node| node.is_token_with_kind(TokenKind::CloseParen))),
        Some(helper.pick(|node| node.is_rule_with_kind(RuleKind::Statement))),
    ]
}

fn pick_do_while_statement(helper: &mut Helper) -> Vec<Option<JsObject>> {
    vec![
        Some(helper.pick(|node| node.is_token_with_kind(TokenKind::DoKeyword))),
        Some(helper.pick(|node| node.is_rule_with_kind(RuleKind::Statement))),
        Some(helper.pick(|node| node.is_token_with_kind(TokenKind::WhileKeyword))),
        Some(helper.pick(|node| node.is_token_with_kind(TokenKind::OpenParen))),
        Some(helper.pick(|node| node.is_rule_with_kind(RuleKind::Expression))),
        Some(helper.pick(|node| node.is_token_with_kind(TokenKind::CloseParen))),
        Some(helper.pick(|node| node.is_token_with_kind(TokenKind::Semicolon))),
    ]
}

fn pick_continue_statement(helper: &mut Helper) -> Vec<Option<JsObject>> {
    vec![
        Some(helper.pick(|node| node.is_token_with_kind(TokenKind::ContinueKeyword))),
        Some(helper.pick(|node| node.is_token_with_kind(TokenKind::Semicolon))),
    ]
}

fn pick_break_statement(helper: &mut Helper) -> Vec<Option<JsObject>> {
    vec![
        Some(helper.pick(|node| node.is_token_with_kind(TokenKind::BreakKeyword))),
        Some(helper.pick(|node| node.is_token_with_kind(TokenKind::Semicolon))),
    ]
}

fn pick_return_statement(helper: &mut Helper) -> Vec<Option<JsObject>> {
    vec![
        Some(helper.pick(|node| node.is_token_with_kind(TokenKind::ReturnKeyword))),
        helper.try_pick(|node| node.is_rule_with_kind(RuleKind::Expression)),
        Some(helper.pick(|node| node.is_token_with_kind(TokenKind::Semicolon))),
    ]
}

fn pick_emit_statement(helper: &mut Helper) -> Vec<Option<JsObject>> {
    vec![
        Some(helper.pick(|node| node.is_token_with_kind(TokenKind::EmitKeyword))),
        Some(helper.pick(|node| node.is_rule_with_kind(RuleKind::IdentifierPath))),
        Some(helper.pick(|node| node.is_rule_with_kind(RuleKind::ArgumentsDeclaration))),
        Some(helper.pick(|node| node.is_token_with_kind(TokenKind::Semicolon))),
    ]
}

fn pick_delete_statement(helper: &mut Helper) -> Vec<Option<JsObject>> {
    vec![
        Some(helper.pick(|node| node.is_token_with_kind(TokenKind::DeleteKeyword))),
        Some(helper.pick(|node| node.is_rule_with_kind(RuleKind::Expression))),
        Some(helper.pick(|node| node.is_token_with_kind(TokenKind::Semicolon))),
    ]
}

fn pick_try_statement(helper: &mut Helper) -> Vec<Option<JsObject>> {
    vec![
        Some(helper.pick(|node| node.is_token_with_kind(TokenKind::TryKeyword))),
        Some(helper.pick(|node| node.is_rule_with_kind(RuleKind::Expression))),
        helper.try_pick(|node| node.is_rule_with_kind(RuleKind::ReturnsDeclaration)),
        Some(helper.pick(|node| node.is_rule_with_kind(RuleKind::Block))),
        Some(helper.pick(|node| node.is_rule_with_kind(RuleKind::CatchClauses))),
    ]
}

fn pick_catch_clause(helper: &mut Helper) -> Vec<Option<JsObject>> {
    vec![
        Some(helper.pick(|node| node.is_token_with_kind(TokenKind::CatchKeyword))),
        helper.try_pick(|node| node.is_rule_with_kind(RuleKind::CatchClauseError)),
        Some(helper.pick(|node| node.is_rule_with_kind(RuleKind::Block))),
    ]
}

fn pick_catch_clause_error(helper: &mut Helper) -> Vec<Option<JsObject>> {
    vec![
        helper.try_pick(|node| node.is_token_with_kind(TokenKind::Identifier)),
        Some(helper.pick(|node| node.is_rule_with_kind(RuleKind::ParametersDeclaration))),
    ]
}

fn pick_revert_statement(helper: &mut Helper) -> Vec<Option<JsObject>> {
    vec![
        Some(helper.pick(|node| node.is_token_with_kind(TokenKind::RevertKeyword))),
        helper.try_pick(|node| node.is_rule_with_kind(RuleKind::IdentifierPath)),
        Some(helper.pick(|node| node.is_rule_with_kind(RuleKind::ArgumentsDeclaration))),
        Some(helper.pick(|node| node.is_token_with_kind(TokenKind::Semicolon))),
    ]
}

fn pick_throw_statement(helper: &mut Helper) -> Vec<Option<JsObject>> {
    vec![
        Some(helper.pick(|node| node.is_token_with_kind(TokenKind::ThrowKeyword))),
        Some(helper.pick(|node| node.is_token_with_kind(TokenKind::Semicolon))),
    ]
}

fn pick_assignment_expression(helper: &mut Helper) -> Vec<Option<JsObject>> {
    vec![
        Some(helper.pick(|node| node.is_rule_with_kind(RuleKind::Expression))),
        Some(helper.pick(|node| node.is_token_with_kind(TokenKind::Equal))),
        Some(helper.pick(|node| node.is_rule_with_kind(RuleKind::Expression))),
    ]
}

fn pick_conditional_expression(helper: &mut Helper) -> Vec<Option<JsObject>> {
    vec![
        Some(helper.pick(|node| node.is_token_with_kind(TokenKind::QuestionMark))),
        Some(helper.pick(|node| node.is_rule_with_kind(RuleKind::Expression))),
        Some(helper.pick(|node| node.is_token_with_kind(TokenKind::Colon))),
        Some(helper.pick(|node| node.is_rule_with_kind(RuleKind::Expression))),
        Some(helper.pick(|node| node.is_rule_with_kind(RuleKind::Expression))),
    ]
}

fn pick_or_expression(helper: &mut Helper) -> Vec<Option<JsObject>> {
    vec![
        Some(helper.pick(|node| node.is_rule_with_kind(RuleKind::Expression))),
        Some(helper.pick(|node| node.is_token_with_kind(TokenKind::BarBar))),
        Some(helper.pick(|node| node.is_rule_with_kind(RuleKind::Expression))),
    ]
}

fn pick_and_expression(helper: &mut Helper) -> Vec<Option<JsObject>> {
    vec![
        Some(helper.pick(|node| node.is_rule_with_kind(RuleKind::Expression))),
        Some(helper.pick(|node| node.is_token_with_kind(TokenKind::AmpersandAmpersand))),
        Some(helper.pick(|node| node.is_rule_with_kind(RuleKind::Expression))),
    ]
}

fn pick_equality_expression(helper: &mut Helper) -> Vec<Option<JsObject>> {
    vec![
        Some(helper.pick(|node| node.is_rule_with_kind(RuleKind::Expression))),
        Some(helper.pick(|node| node.is_token_with_kind(TokenKind::EqualEqual))),
        Some(helper.pick(|node| node.is_rule_with_kind(RuleKind::Expression))),
    ]
}

fn pick_comparison_expression(helper: &mut Helper) -> Vec<Option<JsObject>> {
    vec![
        Some(helper.pick(|node| node.is_rule_with_kind(RuleKind::Expression))),
        Some(helper.pick(|node| node.is_token_with_kind(TokenKind::LessThan))),
        Some(helper.pick(|node| node.is_rule_with_kind(RuleKind::Expression))),
    ]
}

fn pick_bitwise_or_expression(helper: &mut Helper) -> Vec<Option<JsObject>> {
    vec![
        Some(helper.pick(|node| node.is_rule_with_kind(RuleKind::Expression))),
        Some(helper.pick(|node| node.is_token_with_kind(TokenKind::Bar))),
        Some(helper.pick(|node| node.is_rule_with_kind(RuleKind::Expression))),
    ]
}

fn pick_bitwise_xor_expression(helper: &mut Helper) -> Vec<Option<JsObject>> {
    vec![
        Some(helper.pick(|node| node.is_rule_with_kind(RuleKind::Expression))),
        Some(helper.pick(|node| node.is_token_with_kind(TokenKind::Caret))),
        Some(helper.pick(|node| node.is_rule_with_kind(RuleKind::Expression))),
    ]
}

fn pick_bitwise_and_expression(helper: &mut Helper) -> Vec<Option<JsObject>> {
    vec![
        Some(helper.pick(|node| node.is_rule_with_kind(RuleKind::Expression))),
        Some(helper.pick(|node| node.is_token_with_kind(TokenKind::Ampersand))),
        Some(helper.pick(|node| node.is_rule_with_kind(RuleKind::Expression))),
    ]
}

fn pick_shift_expression(helper: &mut Helper) -> Vec<Option<JsObject>> {
    vec![
        Some(helper.pick(|node| node.is_rule_with_kind(RuleKind::Expression))),
        Some(helper.pick(|node| node.is_token_with_kind(TokenKind::LessThanLessThan))),
        Some(helper.pick(|node| node.is_rule_with_kind(RuleKind::Expression))),
    ]
}

fn pick_additive_expression(helper: &mut Helper) -> Vec<Option<JsObject>> {
    vec![
        Some(helper.pick(|node| node.is_rule_with_kind(RuleKind::Expression))),
        Some(helper.pick(|node| node.is_token_with_kind(TokenKind::Plus))),
        Some(helper.pick(|node| node.is_rule_with_kind(RuleKind::Expression))),
    ]
}

fn pick_multiplicative_expression(helper: &mut Helper) -> Vec<Option<JsObject>> {
    vec![
        Some(helper.pick(|node| node.is_rule_with_kind(RuleKind::Expression))),
        Some(helper.pick(|node| node.is_token_with_kind(TokenKind::Asterisk))),
        Some(helper.pick(|node| node.is_rule_with_kind(RuleKind::Expression))),
    ]
}

fn pick_exponentiation_expression(helper: &mut Helper) -> Vec<Option<JsObject>> {
    vec![
        Some(helper.pick(|node| node.is_rule_with_kind(RuleKind::Expression))),
        Some(helper.pick(|node| node.is_token_with_kind(TokenKind::AsteriskAsterisk))),
        Some(helper.pick(|node| node.is_rule_with_kind(RuleKind::Expression))),
    ]
}

fn pick_postfix_expression(helper: &mut Helper) -> Vec<Option<JsObject>> {
    vec![
        Some(helper.pick(|node| node.is_token_with_kind(TokenKind::PlusPlus))),
        Some(helper.pick(|node| node.is_rule_with_kind(RuleKind::Expression))),
    ]
}

fn pick_prefix_expression(helper: &mut Helper) -> Vec<Option<JsObject>> {
    vec![
        Some(helper.pick(|node| node.is_rule_with_kind(RuleKind::Expression))),
        Some(helper.pick(|node| node.is_token_with_kind(TokenKind::PlusPlus))),
    ]
}

fn pick_function_call_expression(helper: &mut Helper) -> Vec<Option<JsObject>> {
    vec![
        helper.try_pick(|node| node.is_rule_with_kind(RuleKind::FunctionCallOptions)),
        Some(helper.pick(|node| node.is_rule_with_kind(RuleKind::ArgumentsDeclaration))),
        Some(helper.pick(|node| node.is_rule_with_kind(RuleKind::Expression))),
    ]
}

fn pick_member_access_expression(helper: &mut Helper) -> Vec<Option<JsObject>> {
    vec![
        Some(helper.pick(|node| node.is_token_with_kind(TokenKind::Period))),
        Some(helper.pick(|node| node.is_rule_with_kind(RuleKind::MemberAccess))),
        Some(helper.pick(|node| node.is_rule_with_kind(RuleKind::Expression))),
    ]
}

fn pick_index_access_expression(helper: &mut Helper) -> Vec<Option<JsObject>> {
    vec![
        Some(helper.pick(|node| node.is_token_with_kind(TokenKind::OpenBracket))),
        helper.try_pick(|node| node.is_rule_with_kind(RuleKind::Expression)),
        helper.try_pick(|node| node.is_rule_with_kind(RuleKind::IndexAccessEnd)),
        Some(helper.pick(|node| node.is_token_with_kind(TokenKind::CloseBracket))),
        Some(helper.pick(|node| node.is_rule_with_kind(RuleKind::Expression))),
    ]
}

fn pick_index_access_end(helper: &mut Helper) -> Vec<Option<JsObject>> {
    vec![
        Some(helper.pick(|node| node.is_token_with_kind(TokenKind::Colon))),
        helper.try_pick(|node| node.is_rule_with_kind(RuleKind::Expression)),
    ]
}

fn pick_positional_arguments_declaration(helper: &mut Helper) -> Vec<Option<JsObject>> {
    vec![
        Some(helper.pick(|node| node.is_token_with_kind(TokenKind::OpenParen))),
        helper.try_pick(|node| node.is_rule_with_kind(RuleKind::PositionalArguments)),
        Some(helper.pick(|node| node.is_token_with_kind(TokenKind::CloseParen))),
    ]
}

fn pick_named_arguments_declaration(helper: &mut Helper) -> Vec<Option<JsObject>> {
    vec![
        Some(helper.pick(|node| node.is_token_with_kind(TokenKind::OpenParen))),
        helper.try_pick(|node| node.is_rule_with_kind(RuleKind::NamedArgumentGroup)),
        Some(helper.pick(|node| node.is_token_with_kind(TokenKind::CloseParen))),
    ]
}

fn pick_named_argument_group(helper: &mut Helper) -> Vec<Option<JsObject>> {
    vec![
        Some(helper.pick(|node| node.is_token_with_kind(TokenKind::OpenBrace))),
        helper.try_pick(|node| node.is_rule_with_kind(RuleKind::NamedArguments)),
        Some(helper.pick(|node| node.is_token_with_kind(TokenKind::CloseBrace))),
    ]
}

fn pick_named_argument(helper: &mut Helper) -> Vec<Option<JsObject>> {
    vec![
        Some(helper.pick(|node| node.is_token_with_kind(TokenKind::Identifier))),
        Some(helper.pick(|node| node.is_token_with_kind(TokenKind::Colon))),
        Some(helper.pick(|node| node.is_rule_with_kind(RuleKind::Expression))),
    ]
}

fn pick_type_expression(helper: &mut Helper) -> Vec<Option<JsObject>> {
    vec![
        Some(helper.pick(|node| node.is_token_with_kind(TokenKind::TypeKeyword))),
        Some(helper.pick(|node| node.is_token_with_kind(TokenKind::OpenParen))),
        Some(helper.pick(|node| node.is_rule_with_kind(RuleKind::TypeName))),
        Some(helper.pick(|node| node.is_token_with_kind(TokenKind::CloseParen))),
    ]
}

fn pick_new_expression(helper: &mut Helper) -> Vec<Option<JsObject>> {
    vec![
        Some(helper.pick(|node| node.is_token_with_kind(TokenKind::NewKeyword))),
        Some(helper.pick(|node| node.is_rule_with_kind(RuleKind::TypeName))),
    ]
}

fn pick_tuple_expression(helper: &mut Helper) -> Vec<Option<JsObject>> {
    vec![
        Some(helper.pick(|node| node.is_token_with_kind(TokenKind::OpenParen))),
        Some(helper.pick(|node| node.is_rule_with_kind(RuleKind::TupleValues))),
        Some(helper.pick(|node| node.is_token_with_kind(TokenKind::CloseParen))),
    ]
}

fn pick_tuple_value(helper: &mut Helper) -> Vec<Option<JsObject>> {
    vec![helper.try_pick(|node| node.is_rule_with_kind(RuleKind::Expression))]
}

fn pick_array_expression(helper: &mut Helper) -> Vec<Option<JsObject>> {
    vec![
        Some(helper.pick(|node| node.is_token_with_kind(TokenKind::OpenBracket))),
        Some(helper.pick(|node| node.is_rule_with_kind(RuleKind::ArrayValues))),
        Some(helper.pick(|node| node.is_token_with_kind(TokenKind::CloseBracket))),
    ]
}

fn pick_hex_number_expression(helper: &mut Helper) -> Vec<Option<JsObject>> {
    vec![
        Some(helper.pick(|node| node.is_token_with_kind(TokenKind::HexLiteral))),
        helper.try_pick(|node| node.is_rule_with_kind(RuleKind::NumberUnit)),
    ]
}

fn pick_decimal_number_expression(helper: &mut Helper) -> Vec<Option<JsObject>> {
    vec![
        Some(helper.pick(|node| node.is_token_with_kind(TokenKind::DecimalLiteral))),
        helper.try_pick(|node| node.is_rule_with_kind(RuleKind::NumberUnit)),
    ]
}

fn pick_yul_block(helper: &mut Helper) -> Vec<Option<JsObject>> {
    vec![
        Some(helper.pick(|node| node.is_token_with_kind(TokenKind::OpenBrace))),
        helper.try_pick(|node| node.is_rule_with_kind(RuleKind::YulStatements)),
        Some(helper.pick(|node| node.is_token_with_kind(TokenKind::CloseBrace))),
    ]
}

fn pick_yul_function_definition(helper: &mut Helper) -> Vec<Option<JsObject>> {
    vec![
        Some(helper.pick(|node| node.is_token_with_kind(TokenKind::YulFunctionKeyword))),
        Some(helper.pick(|node| node.is_token_with_kind(TokenKind::YulIdentifier))),
        Some(helper.pick(|node| node.is_rule_with_kind(RuleKind::YulParametersDeclaration))),
        helper.try_pick(|node| node.is_rule_with_kind(RuleKind::YulReturnsDeclaration)),
        Some(helper.pick(|node| node.is_rule_with_kind(RuleKind::YulBlock))),
    ]
}

fn pick_yul_parameters_declaration(helper: &mut Helper) -> Vec<Option<JsObject>> {
    vec![
        Some(helper.pick(|node| node.is_token_with_kind(TokenKind::OpenParen))),
        helper.try_pick(|node| node.is_rule_with_kind(RuleKind::YulParameters)),
        Some(helper.pick(|node| node.is_token_with_kind(TokenKind::CloseParen))),
    ]
}

fn pick_yul_returns_declaration(helper: &mut Helper) -> Vec<Option<JsObject>> {
    vec![
        Some(helper.pick(|node| node.is_token_with_kind(TokenKind::MinusGreaterThan))),
        Some(helper.pick(|node| node.is_rule_with_kind(RuleKind::YulReturnVariables))),
    ]
}

fn pick_yul_variable_declaration_statement(helper: &mut Helper) -> Vec<Option<JsObject>> {
    vec![
        Some(helper.pick(|node| node.is_token_with_kind(TokenKind::YulLetKeyword))),
        Some(helper.pick(|node| node.is_rule_with_kind(RuleKind::YulIdentifierPaths))),
        helper.try_pick(|node| node.is_rule_with_kind(RuleKind::YulVariableDeclarationValue)),
    ]
}

fn pick_yul_variable_declaration_value(helper: &mut Helper) -> Vec<Option<JsObject>> {
    vec![
        Some(helper.pick(|node| node.is_token_with_kind(TokenKind::ColonEqual))),
        Some(helper.pick(|node| node.is_rule_with_kind(RuleKind::YulExpression))),
    ]
}

fn pick_yul_assignment_statement(helper: &mut Helper) -> Vec<Option<JsObject>> {
    vec![
        Some(helper.pick(|node| node.is_rule_with_kind(RuleKind::YulIdentifierPaths))),
        Some(helper.pick(|node| node.is_token_with_kind(TokenKind::ColonEqual))),
        Some(helper.pick(|node| node.is_rule_with_kind(RuleKind::YulExpression))),
    ]
}

fn pick_yul_if_statement(helper: &mut Helper) -> Vec<Option<JsObject>> {
    vec![
        Some(helper.pick(|node| node.is_token_with_kind(TokenKind::YulIfKeyword))),
        Some(helper.pick(|node| node.is_rule_with_kind(RuleKind::YulExpression))),
        Some(helper.pick(|node| node.is_rule_with_kind(RuleKind::YulBlock))),
    ]
}

fn pick_yul_leave_statement(helper: &mut Helper) -> Vec<Option<JsObject>> {
    vec![Some(helper.pick(|node| {
        node.is_token_with_kind(TokenKind::YulLeaveKeyword)
    }))]
}

fn pick_yul_break_statement(helper: &mut Helper) -> Vec<Option<JsObject>> {
    vec![Some(helper.pick(|node| {
        node.is_token_with_kind(TokenKind::YulBreakKeyword)
    }))]
}

fn pick_yul_continue_statement(helper: &mut Helper) -> Vec<Option<JsObject>> {
    vec![Some(helper.pick(|node| {
        node.is_token_with_kind(TokenKind::YulContinueKeyword)
    }))]
}

fn pick_yul_for_statement(helper: &mut Helper) -> Vec<Option<JsObject>> {
    vec![
        Some(helper.pick(|node| node.is_token_with_kind(TokenKind::YulForKeyword))),
        Some(helper.pick(|node| node.is_rule_with_kind(RuleKind::YulBlock))),
        Some(helper.pick(|node| node.is_rule_with_kind(RuleKind::YulExpression))),
        Some(helper.pick(|node| node.is_rule_with_kind(RuleKind::YulBlock))),
        Some(helper.pick(|node| node.is_rule_with_kind(RuleKind::YulBlock))),
    ]
}

fn pick_yul_switch_statement(helper: &mut Helper) -> Vec<Option<JsObject>> {
    vec![
        Some(helper.pick(|node| node.is_token_with_kind(TokenKind::YulSwitchKeyword))),
        Some(helper.pick(|node| node.is_rule_with_kind(RuleKind::YulExpression))),
        Some(helper.pick(|node| node.is_rule_with_kind(RuleKind::YulSwitchCases))),
    ]
}

fn pick_yul_default_case(helper: &mut Helper) -> Vec<Option<JsObject>> {
    vec![
        Some(helper.pick(|node| node.is_token_with_kind(TokenKind::YulDefaultKeyword))),
        Some(helper.pick(|node| node.is_rule_with_kind(RuleKind::YulBlock))),
    ]
}

fn pick_yul_value_case(helper: &mut Helper) -> Vec<Option<JsObject>> {
    vec![
        Some(helper.pick(|node| node.is_token_with_kind(TokenKind::YulCaseKeyword))),
        Some(helper.pick(|node| node.is_rule_with_kind(RuleKind::YulLiteral))),
        Some(helper.pick(|node| node.is_rule_with_kind(RuleKind::YulBlock))),
    ]
}

fn pick_yul_function_call_expression(helper: &mut Helper) -> Vec<Option<JsObject>> {
    vec![
        Some(helper.pick(|node| node.is_token_with_kind(TokenKind::OpenParen))),
        helper.try_pick(|node| node.is_rule_with_kind(RuleKind::YulArguments)),
        Some(helper.pick(|node| node.is_token_with_kind(TokenKind::CloseParen))),
        Some(helper.pick(|node| node.is_rule_with_kind(RuleKind::YulExpression))),
    ]
}

#[napi(namespace = "ast_internal", ts_return_type = "cst.Node", catch_unwind)]
pub fn pick_choice(
    #[napi(ts_arg_type = "cst.RuleNode")] node: &RuleNode,
    #[napi(ts_arg_type = "kinds.RuleKind")] kind: RuleKind,
    env: Env,
) -> JsObject {
    let mut helper = Helper::new(node, kind, env);

    match kind {
        RuleKind::SourceUnitMember => pick_source_unit_member(&mut helper),
        RuleKind::Pragma => pick_pragma(&mut helper),
        RuleKind::ExperimentalFeature => pick_experimental_feature(&mut helper),
        RuleKind::VersionPragmaExpression => pick_version_pragma_expression(&mut helper),
        RuleKind::ImportClause => pick_import_clause(&mut helper),
        RuleKind::UsingClause => pick_using_clause(&mut helper),
        RuleKind::UsingOperator => pick_using_operator(&mut helper),
        RuleKind::UsingTarget => pick_using_target(&mut helper),
        RuleKind::ContractMember => pick_contract_member(&mut helper),
        RuleKind::StateVariableAttribute => pick_state_variable_attribute(&mut helper),
        RuleKind::FunctionName => pick_function_name(&mut helper),
        RuleKind::FunctionAttribute => pick_function_attribute(&mut helper),
        RuleKind::FunctionBody => pick_function_body(&mut helper),
        RuleKind::ConstructorAttribute => pick_constructor_attribute(&mut helper),
        RuleKind::UnnamedFunctionAttribute => pick_unnamed_function_attribute(&mut helper),
        RuleKind::FallbackFunctionAttribute => pick_fallback_function_attribute(&mut helper),
        RuleKind::ReceiveFunctionAttribute => pick_receive_function_attribute(&mut helper),
        RuleKind::ModifierAttribute => pick_modifier_attribute(&mut helper),
        RuleKind::TypeName => pick_type_name(&mut helper),
        RuleKind::FunctionTypeAttribute => pick_function_type_attribute(&mut helper),
        RuleKind::MappingKeyType => pick_mapping_key_type(&mut helper),
        RuleKind::ElementaryType => pick_elementary_type(&mut helper),
        RuleKind::Statement => pick_statement(&mut helper),
        RuleKind::TupleMember => pick_tuple_member(&mut helper),
        RuleKind::VariableDeclarationType => pick_variable_declaration_type(&mut helper),
        RuleKind::StorageLocation => pick_storage_location(&mut helper),
        RuleKind::ForStatementInitialization => pick_for_statement_initialization(&mut helper),
        RuleKind::ForStatementCondition => pick_for_statement_condition(&mut helper),
        RuleKind::Expression => pick_expression(&mut helper),
        RuleKind::MemberAccess => pick_member_access(&mut helper),
        RuleKind::FunctionCallOptions => pick_function_call_options(&mut helper),
        RuleKind::ArgumentsDeclaration => pick_arguments_declaration(&mut helper),
        RuleKind::NumberUnit => pick_number_unit(&mut helper),
        RuleKind::StringExpression => pick_string_expression(&mut helper),
        RuleKind::YulStatement => pick_yul_statement(&mut helper),
        RuleKind::YulSwitchCase => pick_yul_switch_case(&mut helper),
        RuleKind::YulExpression => pick_yul_expression(&mut helper),
        RuleKind::YulLiteral => pick_yul_literal(&mut helper),
        _ => {
            unreachable!("Unknown choice: '{kind:?}'.");
        }
    }
}

fn pick_source_unit_member(helper: &mut Helper) -> JsObject {
    helper.pick(|node| {
        node.is_rule_with_kinds(&[
            RuleKind::PragmaDirective,
            RuleKind::ImportDirective,
            RuleKind::ContractDefinition,
            RuleKind::InterfaceDefinition,
            RuleKind::LibraryDefinition,
            RuleKind::StructDefinition,
            RuleKind::EnumDefinition,
            RuleKind::FunctionDefinition,
            RuleKind::ConstantDefinition,
            RuleKind::ErrorDefinition,
            RuleKind::UserDefinedValueTypeDefinition,
            RuleKind::UsingDirective,
            RuleKind::EventDefinition,
        ])
    })
}

fn pick_pragma(helper: &mut Helper) -> JsObject {
    helper.pick(|node| {
        node.is_rule_with_kinds(&[
            RuleKind::ABICoderPragma,
            RuleKind::ExperimentalPragma,
            RuleKind::VersionPragma,
        ])
    })
}

fn pick_experimental_feature(helper: &mut Helper) -> JsObject {
    helper.pick(|node| {
        node.is_token_with_kinds(&[TokenKind::Identifier, TokenKind::AsciiStringLiteral])
    })
}

fn pick_version_pragma_expression(helper: &mut Helper) -> JsObject {
    helper.pick(|node| {
        node.is_rule_with_kinds(&[
            RuleKind::VersionPragmaOrExpression,
            RuleKind::VersionPragmaRangeExpression,
            RuleKind::VersionPragmaPrefixExpression,
            RuleKind::VersionPragmaSpecifier,
        ])
    })
}

fn pick_import_clause(helper: &mut Helper) -> JsObject {
    helper.pick(|node| {
        node.is_rule_with_kinds(&[
            RuleKind::PathImport,
            RuleKind::NamedImport,
            RuleKind::ImportDeconstruction,
        ])
    })
}

fn pick_using_clause(helper: &mut Helper) -> JsObject {
    helper.pick(|node| {
        node.is_rule_with_kinds(&[RuleKind::IdentifierPath, RuleKind::UsingDeconstruction])
    })
}

fn pick_using_operator(helper: &mut Helper) -> JsObject {
    helper.pick(|node| {
        node.is_token_with_kinds(&[
            TokenKind::Ampersand,
            TokenKind::Asterisk,
            TokenKind::BangEqual,
            TokenKind::Bar,
            TokenKind::Caret,
            TokenKind::EqualEqual,
            TokenKind::GreaterThan,
            TokenKind::GreaterThanEqual,
            TokenKind::LessThan,
            TokenKind::LessThanEqual,
            TokenKind::Minus,
            TokenKind::Percent,
            TokenKind::Plus,
            TokenKind::Slash,
            TokenKind::Tilde,
        ])
    })
}

fn pick_using_target(helper: &mut Helper) -> JsObject {
    helper.pick(|node| {
        node.is_rule_with_kind(RuleKind::TypeName) || node.is_token_with_kind(TokenKind::Asterisk)
    })
}

fn pick_contract_member(helper: &mut Helper) -> JsObject {
    helper.pick(|node| {
        node.is_rule_with_kinds(&[
            RuleKind::UsingDirective,
            RuleKind::FunctionDefinition,
            RuleKind::ConstructorDefinition,
            RuleKind::ReceiveFunctionDefinition,
            RuleKind::FallbackFunctionDefinition,
            RuleKind::UnnamedFunctionDefinition,
            RuleKind::ModifierDefinition,
            RuleKind::StructDefinition,
            RuleKind::EnumDefinition,
            RuleKind::EventDefinition,
            RuleKind::StateVariableDefinition,
            RuleKind::ErrorDefinition,
            RuleKind::UserDefinedValueTypeDefinition,
        ])
    })
}

fn pick_state_variable_attribute(helper: &mut Helper) -> JsObject {
    helper.pick(|node| {
        node.is_rule_with_kind(RuleKind::OverrideSpecifier)
            || node.is_token_with_kinds(&[
                TokenKind::ConstantKeyword,
                TokenKind::InternalKeyword,
                TokenKind::PrivateKeyword,
                TokenKind::PublicKeyword,
                TokenKind::ImmutableKeyword,
            ])
    })
}

fn pick_function_name(helper: &mut Helper) -> JsObject {
    helper.pick(|node| {
        node.is_token_with_kinds(&[
            TokenKind::Identifier,
            TokenKind::FallbackKeyword,
            TokenKind::ReceiveKeyword,
        ])
    })
}

fn pick_function_attribute(helper: &mut Helper) -> JsObject {
    helper.pick(|node| {
        node.is_rule_with_kinds(&[RuleKind::ModifierInvocation, RuleKind::OverrideSpecifier])
            || node.is_token_with_kinds(&[
                TokenKind::ConstantKeyword,
                TokenKind::ExternalKeyword,
                TokenKind::InternalKeyword,
                TokenKind::PayableKeyword,
                TokenKind::PrivateKeyword,
                TokenKind::PublicKeyword,
                TokenKind::PureKeyword,
                TokenKind::ViewKeyword,
                TokenKind::VirtualKeyword,
            ])
    })
}

fn pick_function_body(helper: &mut Helper) -> JsObject {
    helper.pick(|node| {
        node.is_rule_with_kind(RuleKind::Block) || node.is_token_with_kind(TokenKind::Semicolon)
    })
}

fn pick_constructor_attribute(helper: &mut Helper) -> JsObject {
    helper.pick(|node| {
        node.is_rule_with_kind(RuleKind::ModifierInvocation)
            || node.is_token_with_kinds(&[
                TokenKind::InternalKeyword,
                TokenKind::PayableKeyword,
                TokenKind::PublicKeyword,
            ])
    })
}

fn pick_unnamed_function_attribute(helper: &mut Helper) -> JsObject {
    helper.pick(|node| {
        node.is_rule_with_kinds(&[RuleKind::ModifierInvocation, RuleKind::OverrideSpecifier])
            || node.is_token_with_kinds(&[
                TokenKind::ExternalKeyword,
                TokenKind::PayableKeyword,
                TokenKind::PureKeyword,
                TokenKind::ViewKeyword,
            ])
    })
}

fn pick_fallback_function_attribute(helper: &mut Helper) -> JsObject {
    helper.pick(|node| {
        node.is_rule_with_kinds(&[RuleKind::ModifierInvocation, RuleKind::OverrideSpecifier])
            || node.is_token_with_kinds(&[
                TokenKind::ExternalKeyword,
                TokenKind::PayableKeyword,
                TokenKind::PureKeyword,
                TokenKind::ViewKeyword,
                TokenKind::VirtualKeyword,
            ])
    })
}

fn pick_receive_function_attribute(helper: &mut Helper) -> JsObject {
    helper.pick(|node| {
        node.is_rule_with_kinds(&[RuleKind::ModifierInvocation, RuleKind::OverrideSpecifier])
            || node.is_token_with_kinds(&[
                TokenKind::ExternalKeyword,
                TokenKind::PayableKeyword,
                TokenKind::VirtualKeyword,
            ])
    })
}

fn pick_modifier_attribute(helper: &mut Helper) -> JsObject {
    helper.pick(|node| {
        node.is_rule_with_kind(RuleKind::OverrideSpecifier)
            || node.is_token_with_kind(TokenKind::VirtualKeyword)
    })
}

fn pick_type_name(helper: &mut Helper) -> JsObject {
    helper.pick(|node| {
        node.is_rule_with_kinds(&[
            RuleKind::ArrayTypeName,
            RuleKind::FunctionType,
            RuleKind::MappingType,
            RuleKind::ElementaryType,
            RuleKind::IdentifierPath,
        ])
    })
}

fn pick_function_type_attribute(helper: &mut Helper) -> JsObject {
    helper.pick(|node| {
        node.is_token_with_kinds(&[
            TokenKind::InternalKeyword,
            TokenKind::ExternalKeyword,
            TokenKind::PrivateKeyword,
            TokenKind::PublicKeyword,
            TokenKind::PureKeyword,
            TokenKind::ViewKeyword,
            TokenKind::PayableKeyword,
        ])
    })
}

fn pick_mapping_key_type(helper: &mut Helper) -> JsObject {
    helper
        .pick(|node| node.is_rule_with_kinds(&[RuleKind::ElementaryType, RuleKind::IdentifierPath]))
}

fn pick_elementary_type(helper: &mut Helper) -> JsObject {
    helper.pick(|node| {
        node.is_rule_with_kind(RuleKind::AddressType)
            || node.is_token_with_kinds(&[
                TokenKind::BoolKeyword,
                TokenKind::ByteKeyword,
                TokenKind::StringKeyword,
                TokenKind::PayableKeyword,
                TokenKind::BytesKeyword,
                TokenKind::IntKeyword,
                TokenKind::UintKeyword,
                TokenKind::FixedKeyword,
                TokenKind::UfixedKeyword,
            ])
    })
}

fn pick_statement(helper: &mut Helper) -> JsObject {
    helper.pick(|node| {
        node.is_rule_with_kinds(&[
            RuleKind::ExpressionStatement,
            RuleKind::VariableDeclarationStatement,
            RuleKind::TupleDeconstructionStatement,
            RuleKind::IfStatement,
            RuleKind::ForStatement,
            RuleKind::WhileStatement,
            RuleKind::DoWhileStatement,
            RuleKind::ContinueStatement,
            RuleKind::BreakStatement,
            RuleKind::DeleteStatement,
            RuleKind::ReturnStatement,
            RuleKind::ThrowStatement,
            RuleKind::EmitStatement,
            RuleKind::TryStatement,
            RuleKind::RevertStatement,
            RuleKind::AssemblyStatement,
            RuleKind::Block,
            RuleKind::UncheckedBlock,
        ])
    })
}

fn pick_tuple_member(helper: &mut Helper) -> JsObject {
    helper.pick(|node| {
        node.is_rule_with_kinds(&[RuleKind::TypedTupleMember, RuleKind::UntypedTupleMember])
    })
}

fn pick_variable_declaration_type(helper: &mut Helper) -> JsObject {
    helper.pick(|node| {
        node.is_rule_with_kind(RuleKind::TypeName) || node.is_token_with_kind(TokenKind::VarKeyword)
    })
}

fn pick_storage_location(helper: &mut Helper) -> JsObject {
    helper.pick(|node| {
        node.is_token_with_kinds(&[
            TokenKind::MemoryKeyword,
            TokenKind::StorageKeyword,
            TokenKind::CallDataKeyword,
        ])
    })
}

fn pick_for_statement_initialization(helper: &mut Helper) -> JsObject {
    helper.pick(|node| {
        node.is_rule_with_kinds(&[
            RuleKind::ExpressionStatement,
            RuleKind::VariableDeclarationStatement,
            RuleKind::TupleDeconstructionStatement,
        ]) || node.is_token_with_kind(TokenKind::Semicolon)
    })
}

fn pick_for_statement_condition(helper: &mut Helper) -> JsObject {
    helper.pick(|node| {
        node.is_rule_with_kind(RuleKind::ExpressionStatement)
            || node.is_token_with_kind(TokenKind::Semicolon)
    })
}

fn pick_expression(helper: &mut Helper) -> JsObject {
    helper.pick(|node| {
        node.is_rule_with_kinds(&[
            RuleKind::AssignmentExpression,
            RuleKind::ConditionalExpression,
            RuleKind::OrExpression,
            RuleKind::AndExpression,
            RuleKind::EqualityExpression,
            RuleKind::ComparisonExpression,
            RuleKind::BitwiseOrExpression,
            RuleKind::BitwiseXorExpression,
            RuleKind::BitwiseAndExpression,
            RuleKind::ShiftExpression,
            RuleKind::AdditiveExpression,
            RuleKind::MultiplicativeExpression,
            RuleKind::ExponentiationExpression,
            RuleKind::PostfixExpression,
            RuleKind::PrefixExpression,
            RuleKind::FunctionCallExpression,
            RuleKind::MemberAccessExpression,
            RuleKind::IndexAccessExpression,
            RuleKind::NewExpression,
            RuleKind::TupleExpression,
            RuleKind::TypeExpression,
            RuleKind::ArrayExpression,
            RuleKind::HexNumberExpression,
            RuleKind::DecimalNumberExpression,
            RuleKind::StringExpression,
            RuleKind::ElementaryType,
        ]) || node.is_token_with_kinds(&[
            TokenKind::TrueKeyword,
            TokenKind::FalseKeyword,
            TokenKind::Identifier,
        ])
    })
}

fn pick_member_access(helper: &mut Helper) -> JsObject {
    helper
        .pick(|node| node.is_token_with_kinds(&[TokenKind::Identifier, TokenKind::AddressKeyword]))
}

fn pick_function_call_options(helper: &mut Helper) -> JsObject {
    helper.pick(|node| {
        node.is_rule_with_kinds(&[RuleKind::NamedArgumentGroups, RuleKind::NamedArgumentGroup])
    })
}

fn pick_arguments_declaration(helper: &mut Helper) -> JsObject {
    helper.pick(|node| {
        node.is_rule_with_kinds(&[
            RuleKind::PositionalArgumentsDeclaration,
            RuleKind::NamedArgumentsDeclaration,
        ])
    })
}

fn pick_number_unit(helper: &mut Helper) -> JsObject {
    helper.pick(|node| {
        node.is_token_with_kinds(&[
            TokenKind::WeiKeyword,
            TokenKind::GweiKeyword,
            TokenKind::SzaboKeyword,
            TokenKind::FinneyKeyword,
            TokenKind::EtherKeyword,
            TokenKind::SecondsKeyword,
            TokenKind::MinutesKeyword,
            TokenKind::HoursKeyword,
            TokenKind::DaysKeyword,
            TokenKind::WeeksKeyword,
            TokenKind::YearsKeyword,
        ])
    })
}

fn pick_string_expression(helper: &mut Helper) -> JsObject {
    helper.pick(|node| {
        node.is_rule_with_kinds(&[
            RuleKind::HexStringLiterals,
            RuleKind::AsciiStringLiterals,
            RuleKind::UnicodeStringLiterals,
        ])
    })
}

fn pick_yul_statement(helper: &mut Helper) -> JsObject {
    helper.pick(|node| {
        node.is_rule_with_kinds(&[
            RuleKind::YulBlock,
            RuleKind::YulFunctionDefinition,
            RuleKind::YulVariableDeclarationStatement,
            RuleKind::YulAssignmentStatement,
            RuleKind::YulIfStatement,
            RuleKind::YulForStatement,
            RuleKind::YulSwitchStatement,
            RuleKind::YulLeaveStatement,
            RuleKind::YulBreakStatement,
            RuleKind::YulContinueStatement,
            RuleKind::YulExpression,
        ])
    })
}

fn pick_yul_switch_case(helper: &mut Helper) -> JsObject {
    helper.pick(|node| node.is_rule_with_kinds(&[RuleKind::YulDefaultCase, RuleKind::YulValueCase]))
}

fn pick_yul_expression(helper: &mut Helper) -> JsObject {
    helper.pick(|node| {
        node.is_rule_with_kinds(&[
            RuleKind::YulFunctionCallExpression,
            RuleKind::YulLiteral,
            RuleKind::YulIdentifierPath,
        ])
    })
}

fn pick_yul_literal(helper: &mut Helper) -> JsObject {
    helper.pick(|node| {
        node.is_token_with_kinds(&[
            TokenKind::YulTrueKeyword,
            TokenKind::YulFalseKeyword,
            TokenKind::YulDecimalLiteral,
            TokenKind::YulHexLiteral,
            TokenKind::HexStringLiteral,
            TokenKind::AsciiStringLiteral,
        ])
    })
}

#[napi(
    namespace = "ast_internal",
    ts_return_type = "Array<cst.Node>",
    catch_unwind
)]
pub fn pick_repeated(
    #[napi(ts_arg_type = "cst.RuleNode")] node: &RuleNode,
    #[napi(ts_arg_type = "kinds.RuleKind")] kind: RuleKind,
    env: Env,
) -> Vec<JsObject> {
    let mut helper = Helper::new(node, kind, env);

    match kind {
        RuleKind::SourceUnitMembers => pick_source_unit_members(&mut helper),
        RuleKind::VersionPragmaExpressions => pick_version_pragma_expressions(&mut helper),
        RuleKind::ContractMembers => pick_contract_members(&mut helper),
        RuleKind::InterfaceMembers => pick_interface_members(&mut helper),
        RuleKind::LibraryMembers => pick_library_members(&mut helper),
        RuleKind::StructMembers => pick_struct_members(&mut helper),
        RuleKind::StateVariableAttributes => pick_state_variable_attributes(&mut helper),
        RuleKind::FunctionAttributes => pick_function_attributes(&mut helper),
        RuleKind::ConstructorAttributes => pick_constructor_attributes(&mut helper),
        RuleKind::UnnamedFunctionAttributes => pick_unnamed_function_attributes(&mut helper),
        RuleKind::FallbackFunctionAttributes => pick_fallback_function_attributes(&mut helper),
        RuleKind::ReceiveFunctionAttributes => pick_receive_function_attributes(&mut helper),
        RuleKind::ModifierAttributes => pick_modifier_attributes(&mut helper),
        RuleKind::FunctionTypeAttributes => pick_function_type_attributes(&mut helper),
        RuleKind::Statements => pick_statements(&mut helper),
        RuleKind::CatchClauses => pick_catch_clauses(&mut helper),
        RuleKind::NamedArgumentGroups => pick_named_argument_groups(&mut helper),
        RuleKind::HexStringLiterals => pick_hex_string_literals(&mut helper),
        RuleKind::AsciiStringLiterals => pick_ascii_string_literals(&mut helper),
        RuleKind::UnicodeStringLiterals => pick_unicode_string_literals(&mut helper),
        RuleKind::YulStatements => pick_yul_statements(&mut helper),
        RuleKind::YulSwitchCases => pick_yul_switch_cases(&mut helper),
        _ => {
            unreachable!("Unknown repeated: '{kind:?}'.");
        }
    }
}

fn pick_source_unit_members(helper: &mut Helper) -> Vec<JsObject> {
    std::iter::from_fn(|| {
        helper.try_pick(|node| node.is_rule_with_kind(RuleKind::SourceUnitMember))
    })
    .collect()
}

fn pick_version_pragma_expressions(helper: &mut Helper) -> Vec<JsObject> {
    std::iter::from_fn(|| {
        helper.try_pick(|node| node.is_rule_with_kind(RuleKind::VersionPragmaExpression))
    })
    .collect()
}

fn pick_contract_members(helper: &mut Helper) -> Vec<JsObject> {
    std::iter::from_fn(|| helper.try_pick(|node| node.is_rule_with_kind(RuleKind::ContractMember)))
        .collect()
}

fn pick_interface_members(helper: &mut Helper) -> Vec<JsObject> {
    std::iter::from_fn(|| helper.try_pick(|node| node.is_rule_with_kind(RuleKind::ContractMember)))
        .collect()
}

fn pick_library_members(helper: &mut Helper) -> Vec<JsObject> {
    std::iter::from_fn(|| helper.try_pick(|node| node.is_rule_with_kind(RuleKind::ContractMember)))
        .collect()
}

fn pick_struct_members(helper: &mut Helper) -> Vec<JsObject> {
    std::iter::from_fn(|| helper.try_pick(|node| node.is_rule_with_kind(RuleKind::StructMember)))
        .collect()
}

fn pick_state_variable_attributes(helper: &mut Helper) -> Vec<JsObject> {
    std::iter::from_fn(|| {
        helper.try_pick(|node| node.is_rule_with_kind(RuleKind::StateVariableAttribute))
    })
    .collect()
}

fn pick_function_attributes(helper: &mut Helper) -> Vec<JsObject> {
    std::iter::from_fn(|| {
        helper.try_pick(|node| node.is_rule_with_kind(RuleKind::FunctionAttribute))
    })
    .collect()
}

fn pick_constructor_attributes(helper: &mut Helper) -> Vec<JsObject> {
    std::iter::from_fn(|| {
        helper.try_pick(|node| node.is_rule_with_kind(RuleKind::ConstructorAttribute))
    })
    .collect()
}

fn pick_unnamed_function_attributes(helper: &mut Helper) -> Vec<JsObject> {
    std::iter::from_fn(|| {
        helper.try_pick(|node| node.is_rule_with_kind(RuleKind::UnnamedFunctionAttribute))
    })
    .collect()
}

fn pick_fallback_function_attributes(helper: &mut Helper) -> Vec<JsObject> {
    std::iter::from_fn(|| {
        helper.try_pick(|node| node.is_rule_with_kind(RuleKind::FallbackFunctionAttribute))
    })
    .collect()
}

fn pick_receive_function_attributes(helper: &mut Helper) -> Vec<JsObject> {
    std::iter::from_fn(|| {
        helper.try_pick(|node| node.is_rule_with_kind(RuleKind::ReceiveFunctionAttribute))
    })
    .collect()
}

fn pick_modifier_attributes(helper: &mut Helper) -> Vec<JsObject> {
    std::iter::from_fn(|| {
        helper.try_pick(|node| node.is_rule_with_kind(RuleKind::ModifierAttribute))
    })
    .collect()
}

fn pick_function_type_attributes(helper: &mut Helper) -> Vec<JsObject> {
    std::iter::from_fn(|| {
        helper.try_pick(|node| node.is_rule_with_kind(RuleKind::FunctionTypeAttribute))
    })
    .collect()
}

fn pick_statements(helper: &mut Helper) -> Vec<JsObject> {
    std::iter::from_fn(|| helper.try_pick(|node| node.is_rule_with_kind(RuleKind::Statement)))
        .collect()
}

fn pick_catch_clauses(helper: &mut Helper) -> Vec<JsObject> {
    std::iter::from_fn(|| helper.try_pick(|node| node.is_rule_with_kind(RuleKind::CatchClause)))
        .collect()
}

fn pick_named_argument_groups(helper: &mut Helper) -> Vec<JsObject> {
    std::iter::from_fn(|| {
        helper.try_pick(|node| node.is_rule_with_kind(RuleKind::NamedArgumentGroup))
    })
    .collect()
}

fn pick_hex_string_literals(helper: &mut Helper) -> Vec<JsObject> {
    std::iter::from_fn(|| {
        helper.try_pick(|node| node.is_token_with_kind(TokenKind::HexStringLiteral))
    })
    .collect()
}

fn pick_ascii_string_literals(helper: &mut Helper) -> Vec<JsObject> {
    std::iter::from_fn(|| {
        helper.try_pick(|node| node.is_token_with_kind(TokenKind::AsciiStringLiteral))
    })
    .collect()
}

fn pick_unicode_string_literals(helper: &mut Helper) -> Vec<JsObject> {
    std::iter::from_fn(|| {
        helper.try_pick(|node| node.is_token_with_kind(TokenKind::UnicodeStringLiteral))
    })
    .collect()
}

fn pick_yul_statements(helper: &mut Helper) -> Vec<JsObject> {
    std::iter::from_fn(|| helper.try_pick(|node| node.is_rule_with_kind(RuleKind::YulStatement)))
        .collect()
}

fn pick_yul_switch_cases(helper: &mut Helper) -> Vec<JsObject> {
    std::iter::from_fn(|| helper.try_pick(|node| node.is_rule_with_kind(RuleKind::YulSwitchCase)))
        .collect()
}

#[napi(
    namespace = "ast_internal",
    ts_return_type = "[Array<cst.Node>, Array<cst.Node>]",
    catch_unwind
)]
pub fn pick_separated(
    #[napi(ts_arg_type = "cst.RuleNode")] node: &RuleNode,
    #[napi(ts_arg_type = "kinds.RuleKind")] kind: RuleKind,
    env: Env,
) -> Vec<Vec<JsObject>> {
    let mut helper = Helper::new(node, kind, env);

    match kind {
        RuleKind::VersionPragmaSpecifier => pick_version_pragma_specifier(&mut helper),
        RuleKind::ImportDeconstructionSymbols => pick_import_deconstruction_symbols(&mut helper),
        RuleKind::UsingDeconstructionSymbols => pick_using_deconstruction_symbols(&mut helper),
        RuleKind::InheritanceTypes => pick_inheritance_types(&mut helper),
        RuleKind::EnumMembers => pick_enum_members(&mut helper),
        RuleKind::Parameters => pick_parameters(&mut helper),
        RuleKind::OverridePaths => pick_override_paths(&mut helper),
        RuleKind::EventParameters => pick_event_parameters(&mut helper),
        RuleKind::ErrorParameters => pick_error_parameters(&mut helper),
        RuleKind::AssemblyFlags => pick_assembly_flags(&mut helper),
        RuleKind::TupleDeconstructionElements => pick_tuple_deconstruction_elements(&mut helper),
        RuleKind::PositionalArguments => pick_positional_arguments(&mut helper),
        RuleKind::NamedArguments => pick_named_arguments(&mut helper),
        RuleKind::TupleValues => pick_tuple_values(&mut helper),
        RuleKind::ArrayValues => pick_array_values(&mut helper),
        RuleKind::IdentifierPath => pick_identifier_path(&mut helper),
        RuleKind::YulParameters => pick_yul_parameters(&mut helper),
        RuleKind::YulReturnVariables => pick_yul_return_variables(&mut helper),
        RuleKind::YulArguments => pick_yul_arguments(&mut helper),
        RuleKind::YulIdentifierPaths => pick_yul_identifier_paths(&mut helper),
        RuleKind::YulIdentifierPath => pick_yul_identifier_path(&mut helper),
        _ => {
            unreachable!("Unknown separated: '{kind:?}'.");
        }
    }
}

fn pick_version_pragma_specifier(helper: &mut Helper) -> Vec<Vec<JsObject>> {
    let mut separated = vec![];
    let mut separators = vec![];

    separated.push(helper.pick(|node| node.is_token_with_kind(TokenKind::VersionPragmaValue)));

    while let Some(separator) = helper.try_pick(|node| node.is_token_with_kind(TokenKind::Period)) {
        separators.push(separator);

        separated.push(helper.pick(|node| node.is_token_with_kind(TokenKind::VersionPragmaValue)));
    }

    vec![separated, separators]
}

fn pick_import_deconstruction_symbols(helper: &mut Helper) -> Vec<Vec<JsObject>> {
    let mut separated = vec![];
    let mut separators = vec![];

    separated
        .push(helper.pick(|node| node.is_rule_with_kind(RuleKind::ImportDeconstructionSymbol)));

    while let Some(separator) = helper.try_pick(|node| node.is_token_with_kind(TokenKind::Comma)) {
        separators.push(separator);

        separated
            .push(helper.pick(|node| node.is_rule_with_kind(RuleKind::ImportDeconstructionSymbol)));
    }

    vec![separated, separators]
}

fn pick_using_deconstruction_symbols(helper: &mut Helper) -> Vec<Vec<JsObject>> {
    let mut separated = vec![];
    let mut separators = vec![];

    separated.push(helper.pick(|node| node.is_rule_with_kind(RuleKind::UsingDeconstructionSymbol)));

    while let Some(separator) = helper.try_pick(|node| node.is_token_with_kind(TokenKind::Comma)) {
        separators.push(separator);

        separated
            .push(helper.pick(|node| node.is_rule_with_kind(RuleKind::UsingDeconstructionSymbol)));
    }

    vec![separated, separators]
}

fn pick_inheritance_types(helper: &mut Helper) -> Vec<Vec<JsObject>> {
    let mut separated = vec![];
    let mut separators = vec![];

    separated.push(helper.pick(|node| node.is_rule_with_kind(RuleKind::InheritanceType)));

    while let Some(separator) = helper.try_pick(|node| node.is_token_with_kind(TokenKind::Comma)) {
        separators.push(separator);

        separated.push(helper.pick(|node| node.is_rule_with_kind(RuleKind::InheritanceType)));
    }

    vec![separated, separators]
}

fn pick_enum_members(helper: &mut Helper) -> Vec<Vec<JsObject>> {
    let mut separated = vec![];
    let mut separators = vec![];

    separated.push(helper.pick(|node| node.is_token_with_kind(TokenKind::Identifier)));

    while let Some(separator) = helper.try_pick(|node| node.is_token_with_kind(TokenKind::Comma)) {
        separators.push(separator);

        separated.push(helper.pick(|node| node.is_token_with_kind(TokenKind::Identifier)));
    }

    vec![separated, separators]
}

fn pick_parameters(helper: &mut Helper) -> Vec<Vec<JsObject>> {
    let mut separated = vec![];
    let mut separators = vec![];

    separated.push(helper.pick(|node| node.is_rule_with_kind(RuleKind::Parameter)));

    while let Some(separator) = helper.try_pick(|node| node.is_token_with_kind(TokenKind::Comma)) {
        separators.push(separator);

        separated.push(helper.pick(|node| node.is_rule_with_kind(RuleKind::Parameter)));
    }

    vec![separated, separators]
}

fn pick_override_paths(helper: &mut Helper) -> Vec<Vec<JsObject>> {
    let mut separated = vec![];
    let mut separators = vec![];

    separated.push(helper.pick(|node| node.is_rule_with_kind(RuleKind::IdentifierPath)));

    while let Some(separator) = helper.try_pick(|node| node.is_token_with_kind(TokenKind::Comma)) {
        separators.push(separator);

        separated.push(helper.pick(|node| node.is_rule_with_kind(RuleKind::IdentifierPath)));
    }

    vec![separated, separators]
}

fn pick_event_parameters(helper: &mut Helper) -> Vec<Vec<JsObject>> {
    let mut separated = vec![];
    let mut separators = vec![];

    separated.push(helper.pick(|node| node.is_rule_with_kind(RuleKind::EventParameter)));

    while let Some(separator) = helper.try_pick(|node| node.is_token_with_kind(TokenKind::Comma)) {
        separators.push(separator);

        separated.push(helper.pick(|node| node.is_rule_with_kind(RuleKind::EventParameter)));
    }

    vec![separated, separators]
}

fn pick_error_parameters(helper: &mut Helper) -> Vec<Vec<JsObject>> {
    let mut separated = vec![];
    let mut separators = vec![];

    separated.push(helper.pick(|node| node.is_rule_with_kind(RuleKind::ErrorParameter)));

    while let Some(separator) = helper.try_pick(|node| node.is_token_with_kind(TokenKind::Comma)) {
        separators.push(separator);

        separated.push(helper.pick(|node| node.is_rule_with_kind(RuleKind::ErrorParameter)));
    }

    vec![separated, separators]
}

fn pick_assembly_flags(helper: &mut Helper) -> Vec<Vec<JsObject>> {
    let mut separated = vec![];
    let mut separators = vec![];

    separated.push(helper.pick(|node| node.is_token_with_kind(TokenKind::AsciiStringLiteral)));

    while let Some(separator) = helper.try_pick(|node| node.is_token_with_kind(TokenKind::Comma)) {
        separators.push(separator);

        separated.push(helper.pick(|node| node.is_token_with_kind(TokenKind::AsciiStringLiteral)));
    }

    vec![separated, separators]
}

fn pick_tuple_deconstruction_elements(helper: &mut Helper) -> Vec<Vec<JsObject>> {
    let mut separated = vec![];
    let mut separators = vec![];

    separated
        .push(helper.pick(|node| node.is_rule_with_kind(RuleKind::TupleDeconstructionElement)));

    while let Some(separator) = helper.try_pick(|node| node.is_token_with_kind(TokenKind::Comma)) {
        separators.push(separator);

        separated
            .push(helper.pick(|node| node.is_rule_with_kind(RuleKind::TupleDeconstructionElement)));
    }

    vec![separated, separators]
}

fn pick_positional_arguments(helper: &mut Helper) -> Vec<Vec<JsObject>> {
    let mut separated = vec![];
    let mut separators = vec![];

    separated.push(helper.pick(|node| node.is_rule_with_kind(RuleKind::Expression)));

    while let Some(separator) = helper.try_pick(|node| node.is_token_with_kind(TokenKind::Comma)) {
        separators.push(separator);

        separated.push(helper.pick(|node| node.is_rule_with_kind(RuleKind::Expression)));
    }

    vec![separated, separators]
}

fn pick_named_arguments(helper: &mut Helper) -> Vec<Vec<JsObject>> {
    let mut separated = vec![];
    let mut separators = vec![];

    separated.push(helper.pick(|node| node.is_rule_with_kind(RuleKind::NamedArgument)));

    while let Some(separator) = helper.try_pick(|node| node.is_token_with_kind(TokenKind::Comma)) {
        separators.push(separator);

        separated.push(helper.pick(|node| node.is_rule_with_kind(RuleKind::NamedArgument)));
    }

    vec![separated, separators]
}

fn pick_tuple_values(helper: &mut Helper) -> Vec<Vec<JsObject>> {
    let mut separated = vec![];
    let mut separators = vec![];

    separated.push(helper.pick(|node| node.is_rule_with_kind(RuleKind::TupleValue)));

    while let Some(separator) = helper.try_pick(|node| node.is_token_with_kind(TokenKind::Comma)) {
        separators.push(separator);

        separated.push(helper.pick(|node| node.is_rule_with_kind(RuleKind::TupleValue)));
    }

    vec![separated, separators]
}

fn pick_array_values(helper: &mut Helper) -> Vec<Vec<JsObject>> {
    let mut separated = vec![];
    let mut separators = vec![];

    separated.push(helper.pick(|node| node.is_rule_with_kind(RuleKind::Expression)));

    while let Some(separator) = helper.try_pick(|node| node.is_token_with_kind(TokenKind::Comma)) {
        separators.push(separator);

        separated.push(helper.pick(|node| node.is_rule_with_kind(RuleKind::Expression)));
    }

    vec![separated, separators]
}

fn pick_identifier_path(helper: &mut Helper) -> Vec<Vec<JsObject>> {
    let mut separated = vec![];
    let mut separators = vec![];

    separated.push(helper.pick(|node| node.is_token_with_kind(TokenKind::Identifier)));

    while let Some(separator) = helper.try_pick(|node| node.is_token_with_kind(TokenKind::Period)) {
        separators.push(separator);

        separated.push(helper.pick(|node| node.is_token_with_kind(TokenKind::Identifier)));
    }

    vec![separated, separators]
}

fn pick_yul_parameters(helper: &mut Helper) -> Vec<Vec<JsObject>> {
    let mut separated = vec![];
    let mut separators = vec![];

    separated.push(helper.pick(|node| node.is_token_with_kind(TokenKind::YulIdentifier)));

    while let Some(separator) = helper.try_pick(|node| node.is_token_with_kind(TokenKind::Comma)) {
        separators.push(separator);

        separated.push(helper.pick(|node| node.is_token_with_kind(TokenKind::YulIdentifier)));
    }

    vec![separated, separators]
}

fn pick_yul_return_variables(helper: &mut Helper) -> Vec<Vec<JsObject>> {
    let mut separated = vec![];
    let mut separators = vec![];

    separated.push(helper.pick(|node| node.is_token_with_kind(TokenKind::YulIdentifier)));

    while let Some(separator) = helper.try_pick(|node| node.is_token_with_kind(TokenKind::Comma)) {
        separators.push(separator);

        separated.push(helper.pick(|node| node.is_token_with_kind(TokenKind::YulIdentifier)));
    }

    vec![separated, separators]
}

fn pick_yul_arguments(helper: &mut Helper) -> Vec<Vec<JsObject>> {
    let mut separated = vec![];
    let mut separators = vec![];

    separated.push(helper.pick(|node| node.is_rule_with_kind(RuleKind::YulExpression)));

    while let Some(separator) = helper.try_pick(|node| node.is_token_with_kind(TokenKind::Comma)) {
        separators.push(separator);

        separated.push(helper.pick(|node| node.is_rule_with_kind(RuleKind::YulExpression)));
    }

    vec![separated, separators]
}

fn pick_yul_identifier_paths(helper: &mut Helper) -> Vec<Vec<JsObject>> {
    let mut separated = vec![];
    let mut separators = vec![];

    separated.push(helper.pick(|node| node.is_rule_with_kind(RuleKind::YulIdentifierPath)));

    while let Some(separator) = helper.try_pick(|node| node.is_token_with_kind(TokenKind::Comma)) {
        separators.push(separator);

        separated.push(helper.pick(|node| node.is_rule_with_kind(RuleKind::YulIdentifierPath)));
    }

    vec![separated, separators]
}

fn pick_yul_identifier_path(helper: &mut Helper) -> Vec<Vec<JsObject>> {
    let mut separated = vec![];
    let mut separators = vec![];

    separated.push(helper.pick(|node| node.is_token_with_kind(TokenKind::YulIdentifier)));

    while let Some(separator) = helper.try_pick(|node| node.is_token_with_kind(TokenKind::Period)) {
        separators.push(separator);

        separated.push(helper.pick(|node| node.is_token_with_kind(TokenKind::YulIdentifier)));
    }

    vec![separated, separators]
}

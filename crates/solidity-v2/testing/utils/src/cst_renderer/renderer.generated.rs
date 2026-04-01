// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

#![allow(clippy::too_many_lines)]
#![allow(unused_variables)]

#[allow(clippy::wildcard_imports)]
use slang_solidity_v2_cst::structured_cst::nodes::*;

use crate::cst_renderer::RenderContext;

//
// Sequences:
//

pub fn render_abicoder_pragma(ctx: &mut RenderContext<'_>, node: &AbicoderPragmaStruct) {
    ctx.write_nonterminal_start();
    ctx.depth += 1;

    {
        ctx.write_indent();
        ctx.write_key("abicoder_keyword", "AbicoderKeyword");

        ctx.write_terminal_value(&node.abicoder_keyword.range);
    }

    {
        ctx.write_indent();
        ctx.write_key("version", "AbicoderVersion");

        render_abicoder_version(ctx, &node.version);
    }

    ctx.depth -= 1;
}

pub fn render_additive_expression(ctx: &mut RenderContext<'_>, node: &AdditiveExpressionStruct) {
    ctx.write_nonterminal_start();
    ctx.depth += 1;

    {
        ctx.write_indent();
        ctx.write_key("left_operand", "Expression");

        render_expression(ctx, &node.left_operand);
    }

    {
        ctx.write_indent();
        ctx.write_key(
            "expression_additive_expression_operator",
            "Expression_AdditiveExpression_Operator",
        );

        render_expression_additive_expression_operator(
            ctx,
            &node.expression_additive_expression_operator,
        );
    }

    {
        ctx.write_indent();
        ctx.write_key("right_operand", "Expression");

        render_expression(ctx, &node.right_operand);
    }

    ctx.depth -= 1;
}

pub fn render_address_type(ctx: &mut RenderContext<'_>, node: &AddressTypeStruct) {
    ctx.write_nonterminal_start();
    ctx.depth += 1;

    {
        ctx.write_indent();
        ctx.write_key("address_keyword", "AddressKeyword");

        ctx.write_terminal_value(&node.address_keyword.range);
    }

    if let Some(ref payable_keyword) = node.payable_keyword {
        ctx.write_indent();
        ctx.write_key("payable_keyword", "PayableKeyword");

        ctx.write_terminal_value(&payable_keyword.range);
    }

    ctx.depth -= 1;
}

pub fn render_and_expression(ctx: &mut RenderContext<'_>, node: &AndExpressionStruct) {
    ctx.write_nonterminal_start();
    ctx.depth += 1;

    {
        ctx.write_indent();
        ctx.write_key("left_operand", "Expression");

        render_expression(ctx, &node.left_operand);
    }

    {
        ctx.write_indent();
        ctx.write_key("operator", "AmpersandAmpersand");

        ctx.write_terminal_value(&node.operator.range);
    }

    {
        ctx.write_indent();
        ctx.write_key("right_operand", "Expression");

        render_expression(ctx, &node.right_operand);
    }

    ctx.depth -= 1;
}

pub fn render_array_expression(ctx: &mut RenderContext<'_>, node: &ArrayExpressionStruct) {
    ctx.write_nonterminal_start();
    ctx.depth += 1;

    {
        ctx.write_indent();
        ctx.write_key("open_bracket", "OpenBracket");

        ctx.write_terminal_value(&node.open_bracket.range);
    }

    {
        ctx.write_indent();
        ctx.write_key("items", "ArrayValues");

        render_array_values(ctx, &node.items);
    }

    {
        ctx.write_indent();
        ctx.write_key("close_bracket", "CloseBracket");

        ctx.write_terminal_value(&node.close_bracket.range);
    }

    ctx.depth -= 1;
}

pub fn render_array_type_name(ctx: &mut RenderContext<'_>, node: &ArrayTypeNameStruct) {
    ctx.write_nonterminal_start();
    ctx.depth += 1;

    {
        ctx.write_indent();
        ctx.write_key("operand", "TypeName");

        render_type_name(ctx, &node.operand);
    }

    {
        ctx.write_indent();
        ctx.write_key("open_bracket", "OpenBracket");

        ctx.write_terminal_value(&node.open_bracket.range);
    }

    if let Some(ref index) = node.index {
        ctx.write_indent();
        ctx.write_key("index", "Expression");

        render_expression(ctx, index);
    }

    {
        ctx.write_indent();
        ctx.write_key("close_bracket", "CloseBracket");

        ctx.write_terminal_value(&node.close_bracket.range);
    }

    ctx.depth -= 1;
}

pub fn render_assembly_statement(ctx: &mut RenderContext<'_>, node: &AssemblyStatementStruct) {
    ctx.write_nonterminal_start();
    ctx.depth += 1;

    {
        ctx.write_indent();
        ctx.write_key("assembly_keyword", "AssemblyKeyword");

        ctx.write_terminal_value(&node.assembly_keyword.range);
    }

    if let Some(ref label) = node.label {
        ctx.write_indent();
        ctx.write_key("label", "YulStringLiteral");

        ctx.write_terminal_value(&label.range);
    }

    if let Some(ref flags) = node.flags {
        ctx.write_indent();
        ctx.write_key("flags", "YulFlagsDeclaration");

        render_yul_flags_declaration(ctx, flags);
    }

    {
        ctx.write_indent();
        ctx.write_key("body", "YulBlock");

        render_yul_block(ctx, &node.body);
    }

    ctx.depth -= 1;
}

pub fn render_assignment_expression(
    ctx: &mut RenderContext<'_>,
    node: &AssignmentExpressionStruct,
) {
    ctx.write_nonterminal_start();
    ctx.depth += 1;

    {
        ctx.write_indent();
        ctx.write_key("left_operand", "Expression");

        render_expression(ctx, &node.left_operand);
    }

    {
        ctx.write_indent();
        ctx.write_key(
            "expression_assignment_expression_operator",
            "Expression_AssignmentExpression_Operator",
        );

        render_expression_assignment_expression_operator(
            ctx,
            &node.expression_assignment_expression_operator,
        );
    }

    {
        ctx.write_indent();
        ctx.write_key("right_operand", "Expression");

        render_expression(ctx, &node.right_operand);
    }

    ctx.depth -= 1;
}

pub fn render_bitwise_and_expression(
    ctx: &mut RenderContext<'_>,
    node: &BitwiseAndExpressionStruct,
) {
    ctx.write_nonterminal_start();
    ctx.depth += 1;

    {
        ctx.write_indent();
        ctx.write_key("left_operand", "Expression");

        render_expression(ctx, &node.left_operand);
    }

    {
        ctx.write_indent();
        ctx.write_key("operator", "Ampersand");

        ctx.write_terminal_value(&node.operator.range);
    }

    {
        ctx.write_indent();
        ctx.write_key("right_operand", "Expression");

        render_expression(ctx, &node.right_operand);
    }

    ctx.depth -= 1;
}

pub fn render_bitwise_or_expression(ctx: &mut RenderContext<'_>, node: &BitwiseOrExpressionStruct) {
    ctx.write_nonterminal_start();
    ctx.depth += 1;

    {
        ctx.write_indent();
        ctx.write_key("left_operand", "Expression");

        render_expression(ctx, &node.left_operand);
    }

    {
        ctx.write_indent();
        ctx.write_key("operator", "Bar");

        ctx.write_terminal_value(&node.operator.range);
    }

    {
        ctx.write_indent();
        ctx.write_key("right_operand", "Expression");

        render_expression(ctx, &node.right_operand);
    }

    ctx.depth -= 1;
}

pub fn render_bitwise_xor_expression(
    ctx: &mut RenderContext<'_>,
    node: &BitwiseXorExpressionStruct,
) {
    ctx.write_nonterminal_start();
    ctx.depth += 1;

    {
        ctx.write_indent();
        ctx.write_key("left_operand", "Expression");

        render_expression(ctx, &node.left_operand);
    }

    {
        ctx.write_indent();
        ctx.write_key("operator", "Caret");

        ctx.write_terminal_value(&node.operator.range);
    }

    {
        ctx.write_indent();
        ctx.write_key("right_operand", "Expression");

        render_expression(ctx, &node.right_operand);
    }

    ctx.depth -= 1;
}

pub fn render_block(ctx: &mut RenderContext<'_>, node: &BlockStruct) {
    ctx.write_nonterminal_start();
    ctx.depth += 1;

    {
        ctx.write_indent();
        ctx.write_key("open_brace", "OpenBrace");

        ctx.write_terminal_value(&node.open_brace.range);
    }

    {
        ctx.write_indent();
        ctx.write_key("statements", "Statements");

        render_statements(ctx, &node.statements);
    }

    {
        ctx.write_indent();
        ctx.write_key("close_brace", "CloseBrace");

        ctx.write_terminal_value(&node.close_brace.range);
    }

    ctx.depth -= 1;
}

pub fn render_break_statement(ctx: &mut RenderContext<'_>, node: &BreakStatementStruct) {
    ctx.write_nonterminal_start();
    ctx.depth += 1;

    {
        ctx.write_indent();
        ctx.write_key("break_keyword", "BreakKeyword");

        ctx.write_terminal_value(&node.break_keyword.range);
    }

    {
        ctx.write_indent();
        ctx.write_key("semicolon", "Semicolon");

        ctx.write_terminal_value(&node.semicolon.range);
    }

    ctx.depth -= 1;
}

pub fn render_call_options_expression(
    ctx: &mut RenderContext<'_>,
    node: &CallOptionsExpressionStruct,
) {
    ctx.write_nonterminal_start();
    ctx.depth += 1;

    {
        ctx.write_indent();
        ctx.write_key("operand", "Expression");

        render_expression(ctx, &node.operand);
    }

    {
        ctx.write_indent();
        ctx.write_key("open_brace", "OpenBrace");

        ctx.write_terminal_value(&node.open_brace.range);
    }

    {
        ctx.write_indent();
        ctx.write_key("options", "CallOptions");

        render_call_options(ctx, &node.options);
    }

    {
        ctx.write_indent();
        ctx.write_key("close_brace", "CloseBrace");

        ctx.write_terminal_value(&node.close_brace.range);
    }

    ctx.depth -= 1;
}

pub fn render_catch_clause(ctx: &mut RenderContext<'_>, node: &CatchClauseStruct) {
    ctx.write_nonterminal_start();
    ctx.depth += 1;

    {
        ctx.write_indent();
        ctx.write_key("catch_keyword", "CatchKeyword");

        ctx.write_terminal_value(&node.catch_keyword.range);
    }

    if let Some(ref error) = node.error {
        ctx.write_indent();
        ctx.write_key("error", "CatchClauseError");

        render_catch_clause_error(ctx, error);
    }

    {
        ctx.write_indent();
        ctx.write_key("body", "Block");

        render_block(ctx, &node.body);
    }

    ctx.depth -= 1;
}

pub fn render_catch_clause_error(ctx: &mut RenderContext<'_>, node: &CatchClauseErrorStruct) {
    ctx.write_nonterminal_start();
    ctx.depth += 1;

    if let Some(ref name) = node.name {
        ctx.write_indent();
        ctx.write_key("name", "Identifier");

        ctx.write_terminal_value(&name.range);
    }

    {
        ctx.write_indent();
        ctx.write_key("parameters", "ParametersDeclaration");

        render_parameters_declaration(ctx, &node.parameters);
    }

    ctx.depth -= 1;
}

pub fn render_conditional_expression(
    ctx: &mut RenderContext<'_>,
    node: &ConditionalExpressionStruct,
) {
    ctx.write_nonterminal_start();
    ctx.depth += 1;

    {
        ctx.write_indent();
        ctx.write_key("operand", "Expression");

        render_expression(ctx, &node.operand);
    }

    {
        ctx.write_indent();
        ctx.write_key("question_mark", "QuestionMark");

        ctx.write_terminal_value(&node.question_mark.range);
    }

    {
        ctx.write_indent();
        ctx.write_key("true_expression", "Expression");

        render_expression(ctx, &node.true_expression);
    }

    {
        ctx.write_indent();
        ctx.write_key("colon", "Colon");

        ctx.write_terminal_value(&node.colon.range);
    }

    {
        ctx.write_indent();
        ctx.write_key("false_expression", "Expression");

        render_expression(ctx, &node.false_expression);
    }

    ctx.depth -= 1;
}

pub fn render_constant_definition(ctx: &mut RenderContext<'_>, node: &ConstantDefinitionStruct) {
    ctx.write_nonterminal_start();
    ctx.depth += 1;

    {
        ctx.write_indent();
        ctx.write_key("type_name", "TypeName");

        render_type_name(ctx, &node.type_name);
    }

    {
        ctx.write_indent();
        ctx.write_key("constant_keyword", "ConstantKeyword");

        ctx.write_terminal_value(&node.constant_keyword.range);
    }

    {
        ctx.write_indent();
        ctx.write_key("name", "Identifier");

        ctx.write_terminal_value(&node.name.range);
    }

    {
        ctx.write_indent();
        ctx.write_key("equal", "Equal");

        ctx.write_terminal_value(&node.equal.range);
    }

    {
        ctx.write_indent();
        ctx.write_key("value", "Expression");

        render_expression(ctx, &node.value);
    }

    {
        ctx.write_indent();
        ctx.write_key("semicolon", "Semicolon");

        ctx.write_terminal_value(&node.semicolon.range);
    }

    ctx.depth -= 1;
}

pub fn render_constructor_definition(
    ctx: &mut RenderContext<'_>,
    node: &ConstructorDefinitionStruct,
) {
    ctx.write_nonterminal_start();
    ctx.depth += 1;

    {
        ctx.write_indent();
        ctx.write_key("constructor_keyword", "ConstructorKeyword");

        ctx.write_terminal_value(&node.constructor_keyword.range);
    }

    {
        ctx.write_indent();
        ctx.write_key("parameters", "ParametersDeclaration");

        render_parameters_declaration(ctx, &node.parameters);
    }

    {
        ctx.write_indent();
        ctx.write_key("attributes", "ConstructorAttributes");

        render_constructor_attributes(ctx, &node.attributes);
    }

    {
        ctx.write_indent();
        ctx.write_key("body", "Block");

        render_block(ctx, &node.body);
    }

    ctx.depth -= 1;
}

pub fn render_continue_statement(ctx: &mut RenderContext<'_>, node: &ContinueStatementStruct) {
    ctx.write_nonterminal_start();
    ctx.depth += 1;

    {
        ctx.write_indent();
        ctx.write_key("continue_keyword", "ContinueKeyword");

        ctx.write_terminal_value(&node.continue_keyword.range);
    }

    {
        ctx.write_indent();
        ctx.write_key("semicolon", "Semicolon");

        ctx.write_terminal_value(&node.semicolon.range);
    }

    ctx.depth -= 1;
}

pub fn render_contract_definition(ctx: &mut RenderContext<'_>, node: &ContractDefinitionStruct) {
    ctx.write_nonterminal_start();
    ctx.depth += 1;

    if let Some(ref abstract_keyword) = node.abstract_keyword {
        ctx.write_indent();
        ctx.write_key("abstract_keyword", "AbstractKeyword");

        ctx.write_terminal_value(&abstract_keyword.range);
    }

    {
        ctx.write_indent();
        ctx.write_key("contract_keyword", "ContractKeyword");

        ctx.write_terminal_value(&node.contract_keyword.range);
    }

    {
        ctx.write_indent();
        ctx.write_key("name", "Identifier");

        ctx.write_terminal_value(&node.name.range);
    }

    {
        ctx.write_indent();
        ctx.write_key("specifiers", "ContractSpecifiers");

        render_contract_specifiers(ctx, &node.specifiers);
    }

    {
        ctx.write_indent();
        ctx.write_key("open_brace", "OpenBrace");

        ctx.write_terminal_value(&node.open_brace.range);
    }

    {
        ctx.write_indent();
        ctx.write_key("members", "ContractMembers");

        render_contract_members(ctx, &node.members);
    }

    {
        ctx.write_indent();
        ctx.write_key("close_brace", "CloseBrace");

        ctx.write_terminal_value(&node.close_brace.range);
    }

    ctx.depth -= 1;
}

pub fn render_decimal_number_expression(
    ctx: &mut RenderContext<'_>,
    node: &DecimalNumberExpressionStruct,
) {
    ctx.write_nonterminal_start();
    ctx.depth += 1;

    {
        ctx.write_indent();
        ctx.write_key("literal", "DecimalLiteral");

        ctx.write_terminal_value(&node.literal.range);
    }

    if let Some(ref unit) = node.unit {
        ctx.write_indent();
        ctx.write_key("unit", "NumberUnit");

        render_number_unit(ctx, unit);
    }

    ctx.depth -= 1;
}

pub fn render_do_while_statement(ctx: &mut RenderContext<'_>, node: &DoWhileStatementStruct) {
    ctx.write_nonterminal_start();
    ctx.depth += 1;

    {
        ctx.write_indent();
        ctx.write_key("do_keyword", "DoKeyword");

        ctx.write_terminal_value(&node.do_keyword.range);
    }

    {
        ctx.write_indent();
        ctx.write_key("body", "Statement");

        render_statement(ctx, &node.body);
    }

    {
        ctx.write_indent();
        ctx.write_key("while_keyword", "WhileKeyword");

        ctx.write_terminal_value(&node.while_keyword.range);
    }

    {
        ctx.write_indent();
        ctx.write_key("open_paren", "OpenParen");

        ctx.write_terminal_value(&node.open_paren.range);
    }

    {
        ctx.write_indent();
        ctx.write_key("condition", "Expression");

        render_expression(ctx, &node.condition);
    }

    {
        ctx.write_indent();
        ctx.write_key("close_paren", "CloseParen");

        ctx.write_terminal_value(&node.close_paren.range);
    }

    {
        ctx.write_indent();
        ctx.write_key("semicolon", "Semicolon");

        ctx.write_terminal_value(&node.semicolon.range);
    }

    ctx.depth -= 1;
}

pub fn render_else_branch(ctx: &mut RenderContext<'_>, node: &ElseBranchStruct) {
    ctx.write_nonterminal_start();
    ctx.depth += 1;

    {
        ctx.write_indent();
        ctx.write_key("else_keyword", "ElseKeyword");

        ctx.write_terminal_value(&node.else_keyword.range);
    }

    {
        ctx.write_indent();
        ctx.write_key("body", "Statement");

        render_statement(ctx, &node.body);
    }

    ctx.depth -= 1;
}

pub fn render_emit_statement(ctx: &mut RenderContext<'_>, node: &EmitStatementStruct) {
    ctx.write_nonterminal_start();
    ctx.depth += 1;

    {
        ctx.write_indent();
        ctx.write_key("emit_keyword", "EmitKeyword");

        ctx.write_terminal_value(&node.emit_keyword.range);
    }

    {
        ctx.write_indent();
        ctx.write_key("event", "IdentifierPath");

        render_identifier_path(ctx, &node.event);
    }

    {
        ctx.write_indent();
        ctx.write_key("arguments", "ArgumentsDeclaration");

        render_arguments_declaration(ctx, &node.arguments);
    }

    {
        ctx.write_indent();
        ctx.write_key("semicolon", "Semicolon");

        ctx.write_terminal_value(&node.semicolon.range);
    }

    ctx.depth -= 1;
}

pub fn render_enum_definition(ctx: &mut RenderContext<'_>, node: &EnumDefinitionStruct) {
    ctx.write_nonterminal_start();
    ctx.depth += 1;

    {
        ctx.write_indent();
        ctx.write_key("enum_keyword", "EnumKeyword");

        ctx.write_terminal_value(&node.enum_keyword.range);
    }

    {
        ctx.write_indent();
        ctx.write_key("name", "Identifier");

        ctx.write_terminal_value(&node.name.range);
    }

    {
        ctx.write_indent();
        ctx.write_key("open_brace", "OpenBrace");

        ctx.write_terminal_value(&node.open_brace.range);
    }

    {
        ctx.write_indent();
        ctx.write_key("members", "EnumMembers");

        render_enum_members(ctx, &node.members);
    }

    {
        ctx.write_indent();
        ctx.write_key("close_brace", "CloseBrace");

        ctx.write_terminal_value(&node.close_brace.range);
    }

    ctx.depth -= 1;
}

pub fn render_equality_expression(ctx: &mut RenderContext<'_>, node: &EqualityExpressionStruct) {
    ctx.write_nonterminal_start();
    ctx.depth += 1;

    {
        ctx.write_indent();
        ctx.write_key("left_operand", "Expression");

        render_expression(ctx, &node.left_operand);
    }

    {
        ctx.write_indent();
        ctx.write_key(
            "expression_equality_expression_operator",
            "Expression_EqualityExpression_Operator",
        );

        render_expression_equality_expression_operator(
            ctx,
            &node.expression_equality_expression_operator,
        );
    }

    {
        ctx.write_indent();
        ctx.write_key("right_operand", "Expression");

        render_expression(ctx, &node.right_operand);
    }

    ctx.depth -= 1;
}

pub fn render_error_definition(ctx: &mut RenderContext<'_>, node: &ErrorDefinitionStruct) {
    ctx.write_nonterminal_start();
    ctx.depth += 1;

    {
        ctx.write_indent();
        ctx.write_key("error_keyword", "ErrorKeyword");

        ctx.write_terminal_value(&node.error_keyword.range);
    }

    {
        ctx.write_indent();
        ctx.write_key("name", "Identifier");

        ctx.write_terminal_value(&node.name.range);
    }

    {
        ctx.write_indent();
        ctx.write_key("members", "ErrorParametersDeclaration");

        render_error_parameters_declaration(ctx, &node.members);
    }

    {
        ctx.write_indent();
        ctx.write_key("semicolon", "Semicolon");

        ctx.write_terminal_value(&node.semicolon.range);
    }

    ctx.depth -= 1;
}

pub fn render_error_parameter(ctx: &mut RenderContext<'_>, node: &ErrorParameterStruct) {
    ctx.write_nonterminal_start();
    ctx.depth += 1;

    {
        ctx.write_indent();
        ctx.write_key("type_name", "TypeName");

        render_type_name(ctx, &node.type_name);
    }

    if let Some(ref name) = node.name {
        ctx.write_indent();
        ctx.write_key("name", "Identifier");

        ctx.write_terminal_value(&name.range);
    }

    ctx.depth -= 1;
}

pub fn render_error_parameters_declaration(
    ctx: &mut RenderContext<'_>,
    node: &ErrorParametersDeclarationStruct,
) {
    ctx.write_nonterminal_start();
    ctx.depth += 1;

    {
        ctx.write_indent();
        ctx.write_key("open_paren", "OpenParen");

        ctx.write_terminal_value(&node.open_paren.range);
    }

    {
        ctx.write_indent();
        ctx.write_key("parameters", "ErrorParameters");

        render_error_parameters(ctx, &node.parameters);
    }

    {
        ctx.write_indent();
        ctx.write_key("close_paren", "CloseParen");

        ctx.write_terminal_value(&node.close_paren.range);
    }

    ctx.depth -= 1;
}

pub fn render_event_definition(ctx: &mut RenderContext<'_>, node: &EventDefinitionStruct) {
    ctx.write_nonterminal_start();
    ctx.depth += 1;

    {
        ctx.write_indent();
        ctx.write_key("event_keyword", "EventKeyword");

        ctx.write_terminal_value(&node.event_keyword.range);
    }

    {
        ctx.write_indent();
        ctx.write_key("name", "Identifier");

        ctx.write_terminal_value(&node.name.range);
    }

    {
        ctx.write_indent();
        ctx.write_key("parameters", "EventParametersDeclaration");

        render_event_parameters_declaration(ctx, &node.parameters);
    }

    if let Some(ref anonymous_keyword) = node.anonymous_keyword {
        ctx.write_indent();
        ctx.write_key("anonymous_keyword", "AnonymousKeyword");

        ctx.write_terminal_value(&anonymous_keyword.range);
    }

    {
        ctx.write_indent();
        ctx.write_key("semicolon", "Semicolon");

        ctx.write_terminal_value(&node.semicolon.range);
    }

    ctx.depth -= 1;
}

pub fn render_event_parameter(ctx: &mut RenderContext<'_>, node: &EventParameterStruct) {
    ctx.write_nonterminal_start();
    ctx.depth += 1;

    {
        ctx.write_indent();
        ctx.write_key("type_name", "TypeName");

        render_type_name(ctx, &node.type_name);
    }

    if let Some(ref indexed_keyword) = node.indexed_keyword {
        ctx.write_indent();
        ctx.write_key("indexed_keyword", "IndexedKeyword");

        ctx.write_terminal_value(&indexed_keyword.range);
    }

    if let Some(ref name) = node.name {
        ctx.write_indent();
        ctx.write_key("name", "Identifier");

        ctx.write_terminal_value(&name.range);
    }

    ctx.depth -= 1;
}

pub fn render_event_parameters_declaration(
    ctx: &mut RenderContext<'_>,
    node: &EventParametersDeclarationStruct,
) {
    ctx.write_nonterminal_start();
    ctx.depth += 1;

    {
        ctx.write_indent();
        ctx.write_key("open_paren", "OpenParen");

        ctx.write_terminal_value(&node.open_paren.range);
    }

    {
        ctx.write_indent();
        ctx.write_key("parameters", "EventParameters");

        render_event_parameters(ctx, &node.parameters);
    }

    {
        ctx.write_indent();
        ctx.write_key("close_paren", "CloseParen");

        ctx.write_terminal_value(&node.close_paren.range);
    }

    ctx.depth -= 1;
}

pub fn render_experimental_pragma(ctx: &mut RenderContext<'_>, node: &ExperimentalPragmaStruct) {
    ctx.write_nonterminal_start();
    ctx.depth += 1;

    {
        ctx.write_indent();
        ctx.write_key("experimental_keyword", "ExperimentalKeyword");

        ctx.write_terminal_value(&node.experimental_keyword.range);
    }

    {
        ctx.write_indent();
        ctx.write_key("feature", "ExperimentalFeature");

        render_experimental_feature(ctx, &node.feature);
    }

    ctx.depth -= 1;
}

pub fn render_exponentiation_expression(
    ctx: &mut RenderContext<'_>,
    node: &ExponentiationExpressionStruct,
) {
    ctx.write_nonterminal_start();
    ctx.depth += 1;

    {
        ctx.write_indent();
        ctx.write_key("left_operand", "Expression");

        render_expression(ctx, &node.left_operand);
    }

    {
        ctx.write_indent();
        ctx.write_key("operator", "AsteriskAsterisk");

        ctx.write_terminal_value(&node.operator.range);
    }

    {
        ctx.write_indent();
        ctx.write_key("right_operand", "Expression");

        render_expression(ctx, &node.right_operand);
    }

    ctx.depth -= 1;
}

pub fn render_expression_statement(ctx: &mut RenderContext<'_>, node: &ExpressionStatementStruct) {
    ctx.write_nonterminal_start();
    ctx.depth += 1;

    {
        ctx.write_indent();
        ctx.write_key("expression", "Expression");

        render_expression(ctx, &node.expression);
    }

    {
        ctx.write_indent();
        ctx.write_key("semicolon", "Semicolon");

        ctx.write_terminal_value(&node.semicolon.range);
    }

    ctx.depth -= 1;
}

pub fn render_fallback_function_definition(
    ctx: &mut RenderContext<'_>,
    node: &FallbackFunctionDefinitionStruct,
) {
    ctx.write_nonterminal_start();
    ctx.depth += 1;

    {
        ctx.write_indent();
        ctx.write_key("fallback_keyword", "FallbackKeyword");

        ctx.write_terminal_value(&node.fallback_keyword.range);
    }

    {
        ctx.write_indent();
        ctx.write_key("parameters", "ParametersDeclaration");

        render_parameters_declaration(ctx, &node.parameters);
    }

    {
        ctx.write_indent();
        ctx.write_key("attributes", "FallbackFunctionAttributes");

        render_fallback_function_attributes(ctx, &node.attributes);
    }

    if let Some(ref returns) = node.returns {
        ctx.write_indent();
        ctx.write_key("returns", "ReturnsDeclaration");

        render_returns_declaration(ctx, returns);
    }

    {
        ctx.write_indent();
        ctx.write_key("body", "FunctionBody");

        render_function_body(ctx, &node.body);
    }

    ctx.depth -= 1;
}

pub fn render_for_statement(ctx: &mut RenderContext<'_>, node: &ForStatementStruct) {
    ctx.write_nonterminal_start();
    ctx.depth += 1;

    {
        ctx.write_indent();
        ctx.write_key("for_keyword", "ForKeyword");

        ctx.write_terminal_value(&node.for_keyword.range);
    }

    {
        ctx.write_indent();
        ctx.write_key("open_paren", "OpenParen");

        ctx.write_terminal_value(&node.open_paren.range);
    }

    {
        ctx.write_indent();
        ctx.write_key("initialization", "ForStatementInitialization");

        render_for_statement_initialization(ctx, &node.initialization);
    }

    {
        ctx.write_indent();
        ctx.write_key("condition", "ForStatementCondition");

        render_for_statement_condition(ctx, &node.condition);
    }

    if let Some(ref iterator) = node.iterator {
        ctx.write_indent();
        ctx.write_key("iterator", "Expression");

        render_expression(ctx, iterator);
    }

    {
        ctx.write_indent();
        ctx.write_key("close_paren", "CloseParen");

        ctx.write_terminal_value(&node.close_paren.range);
    }

    {
        ctx.write_indent();
        ctx.write_key("body", "Statement");

        render_statement(ctx, &node.body);
    }

    ctx.depth -= 1;
}

pub fn render_function_call_expression(
    ctx: &mut RenderContext<'_>,
    node: &FunctionCallExpressionStruct,
) {
    ctx.write_nonterminal_start();
    ctx.depth += 1;

    {
        ctx.write_indent();
        ctx.write_key("operand", "Expression");

        render_expression(ctx, &node.operand);
    }

    {
        ctx.write_indent();
        ctx.write_key("arguments", "ArgumentsDeclaration");

        render_arguments_declaration(ctx, &node.arguments);
    }

    ctx.depth -= 1;
}

pub fn render_function_definition(ctx: &mut RenderContext<'_>, node: &FunctionDefinitionStruct) {
    ctx.write_nonterminal_start();
    ctx.depth += 1;

    {
        ctx.write_indent();
        ctx.write_key("function_keyword", "FunctionKeyword");

        ctx.write_terminal_value(&node.function_keyword.range);
    }

    {
        ctx.write_indent();
        ctx.write_key("name", "FunctionName");

        render_function_name(ctx, &node.name);
    }

    {
        ctx.write_indent();
        ctx.write_key("parameters", "ParametersDeclaration");

        render_parameters_declaration(ctx, &node.parameters);
    }

    {
        ctx.write_indent();
        ctx.write_key("attributes", "FunctionAttributes");

        render_function_attributes(ctx, &node.attributes);
    }

    if let Some(ref returns) = node.returns {
        ctx.write_indent();
        ctx.write_key("returns", "ReturnsDeclaration");

        render_returns_declaration(ctx, returns);
    }

    {
        ctx.write_indent();
        ctx.write_key("body", "FunctionBody");

        render_function_body(ctx, &node.body);
    }

    ctx.depth -= 1;
}

pub fn render_function_type(ctx: &mut RenderContext<'_>, node: &FunctionTypeStruct) {
    ctx.write_nonterminal_start();
    ctx.depth += 1;

    {
        ctx.write_indent();
        ctx.write_key("function_keyword", "FunctionKeyword");

        ctx.write_terminal_value(&node.function_keyword.range);
    }

    {
        ctx.write_indent();
        ctx.write_key("parameters", "ParametersDeclaration");

        render_parameters_declaration(ctx, &node.parameters);
    }

    {
        ctx.write_indent();
        ctx.write_key("attributes", "FunctionTypeAttributes");

        render_function_type_attributes(ctx, &node.attributes);
    }

    if let Some(ref returns) = node.returns {
        ctx.write_indent();
        ctx.write_key("returns", "ReturnsDeclaration");

        render_returns_declaration(ctx, returns);
    }

    ctx.depth -= 1;
}

pub fn render_hex_number_expression(ctx: &mut RenderContext<'_>, node: &HexNumberExpressionStruct) {
    ctx.write_nonterminal_start();
    ctx.depth += 1;

    {
        ctx.write_indent();
        ctx.write_key("literal", "HexLiteral");

        ctx.write_terminal_value(&node.literal.range);
    }

    ctx.depth -= 1;
}

pub fn render_if_statement(ctx: &mut RenderContext<'_>, node: &IfStatementStruct) {
    ctx.write_nonterminal_start();
    ctx.depth += 1;

    {
        ctx.write_indent();
        ctx.write_key("if_keyword", "IfKeyword");

        ctx.write_terminal_value(&node.if_keyword.range);
    }

    {
        ctx.write_indent();
        ctx.write_key("open_paren", "OpenParen");

        ctx.write_terminal_value(&node.open_paren.range);
    }

    {
        ctx.write_indent();
        ctx.write_key("condition", "Expression");

        render_expression(ctx, &node.condition);
    }

    {
        ctx.write_indent();
        ctx.write_key("close_paren", "CloseParen");

        ctx.write_terminal_value(&node.close_paren.range);
    }

    {
        ctx.write_indent();
        ctx.write_key("body", "Statement");

        render_statement(ctx, &node.body);
    }

    if let Some(ref else_branch) = node.else_branch {
        ctx.write_indent();
        ctx.write_key("else_branch", "ElseBranch");

        render_else_branch(ctx, else_branch);
    }

    ctx.depth -= 1;
}

pub fn render_import_alias(ctx: &mut RenderContext<'_>, node: &ImportAliasStruct) {
    ctx.write_nonterminal_start();
    ctx.depth += 1;

    {
        ctx.write_indent();
        ctx.write_key("as_keyword", "AsKeyword");

        ctx.write_terminal_value(&node.as_keyword.range);
    }

    {
        ctx.write_indent();
        ctx.write_key("identifier", "Identifier");

        ctx.write_terminal_value(&node.identifier.range);
    }

    ctx.depth -= 1;
}

pub fn render_import_deconstruction(
    ctx: &mut RenderContext<'_>,
    node: &ImportDeconstructionStruct,
) {
    ctx.write_nonterminal_start();
    ctx.depth += 1;

    {
        ctx.write_indent();
        ctx.write_key("open_brace", "OpenBrace");

        ctx.write_terminal_value(&node.open_brace.range);
    }

    {
        ctx.write_indent();
        ctx.write_key("symbols", "ImportDeconstructionSymbols");

        render_import_deconstruction_symbols(ctx, &node.symbols);
    }

    {
        ctx.write_indent();
        ctx.write_key("close_brace", "CloseBrace");

        ctx.write_terminal_value(&node.close_brace.range);
    }

    {
        ctx.write_indent();
        ctx.write_key("from_keyword", "FromKeyword");

        ctx.write_terminal_value(&node.from_keyword.range);
    }

    {
        ctx.write_indent();
        ctx.write_key("path", "StringLiteral");

        ctx.write_terminal_value(&node.path.range);
    }

    ctx.depth -= 1;
}

pub fn render_import_deconstruction_symbol(
    ctx: &mut RenderContext<'_>,
    node: &ImportDeconstructionSymbolStruct,
) {
    ctx.write_nonterminal_start();
    ctx.depth += 1;

    {
        ctx.write_indent();
        ctx.write_key("name", "Identifier");

        ctx.write_terminal_value(&node.name.range);
    }

    if let Some(ref alias) = node.alias {
        ctx.write_indent();
        ctx.write_key("alias", "ImportAlias");

        render_import_alias(ctx, alias);
    }

    ctx.depth -= 1;
}

pub fn render_import_directive(ctx: &mut RenderContext<'_>, node: &ImportDirectiveStruct) {
    ctx.write_nonterminal_start();
    ctx.depth += 1;

    {
        ctx.write_indent();
        ctx.write_key("import_keyword", "ImportKeyword");

        ctx.write_terminal_value(&node.import_keyword.range);
    }

    {
        ctx.write_indent();
        ctx.write_key("clause", "ImportClause");

        render_import_clause(ctx, &node.clause);
    }

    {
        ctx.write_indent();
        ctx.write_key("semicolon", "Semicolon");

        ctx.write_terminal_value(&node.semicolon.range);
    }

    ctx.depth -= 1;
}

pub fn render_index_access_end(ctx: &mut RenderContext<'_>, node: &IndexAccessEndStruct) {
    ctx.write_nonterminal_start();
    ctx.depth += 1;

    {
        ctx.write_indent();
        ctx.write_key("colon", "Colon");

        ctx.write_terminal_value(&node.colon.range);
    }

    if let Some(ref end) = node.end {
        ctx.write_indent();
        ctx.write_key("end", "Expression");

        render_expression(ctx, end);
    }

    ctx.depth -= 1;
}

pub fn render_index_access_expression(
    ctx: &mut RenderContext<'_>,
    node: &IndexAccessExpressionStruct,
) {
    ctx.write_nonterminal_start();
    ctx.depth += 1;

    {
        ctx.write_indent();
        ctx.write_key("operand", "Expression");

        render_expression(ctx, &node.operand);
    }

    {
        ctx.write_indent();
        ctx.write_key("open_bracket", "OpenBracket");

        ctx.write_terminal_value(&node.open_bracket.range);
    }

    if let Some(ref start) = node.start {
        ctx.write_indent();
        ctx.write_key("start", "Expression");

        render_expression(ctx, start);
    }

    if let Some(ref end) = node.end {
        ctx.write_indent();
        ctx.write_key("end", "IndexAccessEnd");

        render_index_access_end(ctx, end);
    }

    {
        ctx.write_indent();
        ctx.write_key("close_bracket", "CloseBracket");

        ctx.write_terminal_value(&node.close_bracket.range);
    }

    ctx.depth -= 1;
}

pub fn render_inequality_expression(
    ctx: &mut RenderContext<'_>,
    node: &InequalityExpressionStruct,
) {
    ctx.write_nonterminal_start();
    ctx.depth += 1;

    {
        ctx.write_indent();
        ctx.write_key("left_operand", "Expression");

        render_expression(ctx, &node.left_operand);
    }

    {
        ctx.write_indent();
        ctx.write_key(
            "expression_inequality_expression_operator",
            "Expression_InequalityExpression_Operator",
        );

        render_expression_inequality_expression_operator(
            ctx,
            &node.expression_inequality_expression_operator,
        );
    }

    {
        ctx.write_indent();
        ctx.write_key("right_operand", "Expression");

        render_expression(ctx, &node.right_operand);
    }

    ctx.depth -= 1;
}

pub fn render_inheritance_specifier(
    ctx: &mut RenderContext<'_>,
    node: &InheritanceSpecifierStruct,
) {
    ctx.write_nonterminal_start();
    ctx.depth += 1;

    {
        ctx.write_indent();
        ctx.write_key("is_keyword", "IsKeyword");

        ctx.write_terminal_value(&node.is_keyword.range);
    }

    {
        ctx.write_indent();
        ctx.write_key("types", "InheritanceTypes");

        render_inheritance_types(ctx, &node.types);
    }

    ctx.depth -= 1;
}

pub fn render_inheritance_type(ctx: &mut RenderContext<'_>, node: &InheritanceTypeStruct) {
    ctx.write_nonterminal_start();
    ctx.depth += 1;

    {
        ctx.write_indent();
        ctx.write_key("type_name", "IdentifierPath");

        render_identifier_path(ctx, &node.type_name);
    }

    if let Some(ref arguments) = node.arguments {
        ctx.write_indent();
        ctx.write_key("arguments", "ArgumentsDeclaration");

        render_arguments_declaration(ctx, arguments);
    }

    ctx.depth -= 1;
}

pub fn render_interface_definition(ctx: &mut RenderContext<'_>, node: &InterfaceDefinitionStruct) {
    ctx.write_nonterminal_start();
    ctx.depth += 1;

    {
        ctx.write_indent();
        ctx.write_key("interface_keyword", "InterfaceKeyword");

        ctx.write_terminal_value(&node.interface_keyword.range);
    }

    {
        ctx.write_indent();
        ctx.write_key("name", "Identifier");

        ctx.write_terminal_value(&node.name.range);
    }

    if let Some(ref inheritance) = node.inheritance {
        ctx.write_indent();
        ctx.write_key("inheritance", "InheritanceSpecifier");

        render_inheritance_specifier(ctx, inheritance);
    }

    {
        ctx.write_indent();
        ctx.write_key("open_brace", "OpenBrace");

        ctx.write_terminal_value(&node.open_brace.range);
    }

    {
        ctx.write_indent();
        ctx.write_key("members", "InterfaceMembers");

        render_interface_members(ctx, &node.members);
    }

    {
        ctx.write_indent();
        ctx.write_key("close_brace", "CloseBrace");

        ctx.write_terminal_value(&node.close_brace.range);
    }

    ctx.depth -= 1;
}

pub fn render_library_definition(ctx: &mut RenderContext<'_>, node: &LibraryDefinitionStruct) {
    ctx.write_nonterminal_start();
    ctx.depth += 1;

    {
        ctx.write_indent();
        ctx.write_key("library_keyword", "LibraryKeyword");

        ctx.write_terminal_value(&node.library_keyword.range);
    }

    {
        ctx.write_indent();
        ctx.write_key("name", "Identifier");

        ctx.write_terminal_value(&node.name.range);
    }

    {
        ctx.write_indent();
        ctx.write_key("open_brace", "OpenBrace");

        ctx.write_terminal_value(&node.open_brace.range);
    }

    {
        ctx.write_indent();
        ctx.write_key("members", "LibraryMembers");

        render_library_members(ctx, &node.members);
    }

    {
        ctx.write_indent();
        ctx.write_key("close_brace", "CloseBrace");

        ctx.write_terminal_value(&node.close_brace.range);
    }

    ctx.depth -= 1;
}

pub fn render_mapping_key(ctx: &mut RenderContext<'_>, node: &MappingKeyStruct) {
    ctx.write_nonterminal_start();
    ctx.depth += 1;

    {
        ctx.write_indent();
        ctx.write_key("key_type", "MappingKeyType");

        render_mapping_key_type(ctx, &node.key_type);
    }

    if let Some(ref name) = node.name {
        ctx.write_indent();
        ctx.write_key("name", "Identifier");

        ctx.write_terminal_value(&name.range);
    }

    ctx.depth -= 1;
}

pub fn render_mapping_type(ctx: &mut RenderContext<'_>, node: &MappingTypeStruct) {
    ctx.write_nonterminal_start();
    ctx.depth += 1;

    {
        ctx.write_indent();
        ctx.write_key("mapping_keyword", "MappingKeyword");

        ctx.write_terminal_value(&node.mapping_keyword.range);
    }

    {
        ctx.write_indent();
        ctx.write_key("open_paren", "OpenParen");

        ctx.write_terminal_value(&node.open_paren.range);
    }

    {
        ctx.write_indent();
        ctx.write_key("key_type", "MappingKey");

        render_mapping_key(ctx, &node.key_type);
    }

    {
        ctx.write_indent();
        ctx.write_key("equal_greater_than", "EqualGreaterThan");

        ctx.write_terminal_value(&node.equal_greater_than.range);
    }

    {
        ctx.write_indent();
        ctx.write_key("value_type", "MappingValue");

        render_mapping_value(ctx, &node.value_type);
    }

    {
        ctx.write_indent();
        ctx.write_key("close_paren", "CloseParen");

        ctx.write_terminal_value(&node.close_paren.range);
    }

    ctx.depth -= 1;
}

pub fn render_mapping_value(ctx: &mut RenderContext<'_>, node: &MappingValueStruct) {
    ctx.write_nonterminal_start();
    ctx.depth += 1;

    {
        ctx.write_indent();
        ctx.write_key("type_name", "TypeName");

        render_type_name(ctx, &node.type_name);
    }

    if let Some(ref name) = node.name {
        ctx.write_indent();
        ctx.write_key("name", "Identifier");

        ctx.write_terminal_value(&name.range);
    }

    ctx.depth -= 1;
}

pub fn render_member_access_expression(
    ctx: &mut RenderContext<'_>,
    node: &MemberAccessExpressionStruct,
) {
    ctx.write_nonterminal_start();
    ctx.depth += 1;

    {
        ctx.write_indent();
        ctx.write_key("operand", "Expression");

        render_expression(ctx, &node.operand);
    }

    {
        ctx.write_indent();
        ctx.write_key("period", "Period");

        ctx.write_terminal_value(&node.period.range);
    }

    {
        ctx.write_indent();
        ctx.write_key("member", "IdentifierPathElement");

        render_identifier_path_element(ctx, &node.member);
    }

    ctx.depth -= 1;
}

pub fn render_modifier_definition(ctx: &mut RenderContext<'_>, node: &ModifierDefinitionStruct) {
    ctx.write_nonterminal_start();
    ctx.depth += 1;

    {
        ctx.write_indent();
        ctx.write_key("modifier_keyword", "ModifierKeyword");

        ctx.write_terminal_value(&node.modifier_keyword.range);
    }

    {
        ctx.write_indent();
        ctx.write_key("name", "Identifier");

        ctx.write_terminal_value(&node.name.range);
    }

    if let Some(ref parameters) = node.parameters {
        ctx.write_indent();
        ctx.write_key("parameters", "ParametersDeclaration");

        render_parameters_declaration(ctx, parameters);
    }

    {
        ctx.write_indent();
        ctx.write_key("attributes", "ModifierAttributes");

        render_modifier_attributes(ctx, &node.attributes);
    }

    {
        ctx.write_indent();
        ctx.write_key("body", "FunctionBody");

        render_function_body(ctx, &node.body);
    }

    ctx.depth -= 1;
}

pub fn render_modifier_invocation(ctx: &mut RenderContext<'_>, node: &ModifierInvocationStruct) {
    ctx.write_nonterminal_start();
    ctx.depth += 1;

    {
        ctx.write_indent();
        ctx.write_key("name", "IdentifierPath");

        render_identifier_path(ctx, &node.name);
    }

    if let Some(ref arguments) = node.arguments {
        ctx.write_indent();
        ctx.write_key("arguments", "ArgumentsDeclaration");

        render_arguments_declaration(ctx, arguments);
    }

    ctx.depth -= 1;
}

pub fn render_multi_typed_declaration(
    ctx: &mut RenderContext<'_>,
    node: &MultiTypedDeclarationStruct,
) {
    ctx.write_nonterminal_start();
    ctx.depth += 1;

    {
        ctx.write_indent();
        ctx.write_key("open_paren", "OpenParen");

        ctx.write_terminal_value(&node.open_paren.range);
    }

    {
        ctx.write_indent();
        ctx.write_key("elements", "MultiTypedDeclarationElements");

        render_multi_typed_declaration_elements(ctx, &node.elements);
    }

    {
        ctx.write_indent();
        ctx.write_key("close_paren", "CloseParen");

        ctx.write_terminal_value(&node.close_paren.range);
    }

    {
        ctx.write_indent();
        ctx.write_key("value", "VariableDeclarationValue");

        render_variable_declaration_value(ctx, &node.value);
    }

    ctx.depth -= 1;
}

pub fn render_multi_typed_declaration_element(
    ctx: &mut RenderContext<'_>,
    node: &MultiTypedDeclarationElementStruct,
) {
    ctx.write_nonterminal_start();
    ctx.depth += 1;

    let mut has_children = false;

    if let Some(ref member) = node.member {
        has_children = true;
        ctx.write_indent();
        ctx.write_key("member", "VariableDeclaration");

        render_variable_declaration(ctx, member);
    }

    ctx.depth -= 1;

    ctx.finish_nonterminal(has_children);
}

pub fn render_multiplicative_expression(
    ctx: &mut RenderContext<'_>,
    node: &MultiplicativeExpressionStruct,
) {
    ctx.write_nonterminal_start();
    ctx.depth += 1;

    {
        ctx.write_indent();
        ctx.write_key("left_operand", "Expression");

        render_expression(ctx, &node.left_operand);
    }

    {
        ctx.write_indent();
        ctx.write_key(
            "expression_multiplicative_expression_operator",
            "Expression_MultiplicativeExpression_Operator",
        );

        render_expression_multiplicative_expression_operator(
            ctx,
            &node.expression_multiplicative_expression_operator,
        );
    }

    {
        ctx.write_indent();
        ctx.write_key("right_operand", "Expression");

        render_expression(ctx, &node.right_operand);
    }

    ctx.depth -= 1;
}

pub fn render_named_argument(ctx: &mut RenderContext<'_>, node: &NamedArgumentStruct) {
    ctx.write_nonterminal_start();
    ctx.depth += 1;

    {
        ctx.write_indent();
        ctx.write_key("name", "Identifier");

        ctx.write_terminal_value(&node.name.range);
    }

    {
        ctx.write_indent();
        ctx.write_key("colon", "Colon");

        ctx.write_terminal_value(&node.colon.range);
    }

    {
        ctx.write_indent();
        ctx.write_key("value", "Expression");

        render_expression(ctx, &node.value);
    }

    ctx.depth -= 1;
}

pub fn render_named_argument_group(ctx: &mut RenderContext<'_>, node: &NamedArgumentGroupStruct) {
    ctx.write_nonterminal_start();
    ctx.depth += 1;

    {
        ctx.write_indent();
        ctx.write_key("open_brace", "OpenBrace");

        ctx.write_terminal_value(&node.open_brace.range);
    }

    {
        ctx.write_indent();
        ctx.write_key("arguments", "NamedArguments");

        render_named_arguments(ctx, &node.arguments);
    }

    {
        ctx.write_indent();
        ctx.write_key("close_brace", "CloseBrace");

        ctx.write_terminal_value(&node.close_brace.range);
    }

    ctx.depth -= 1;
}

pub fn render_named_arguments_declaration(
    ctx: &mut RenderContext<'_>,
    node: &NamedArgumentsDeclarationStruct,
) {
    ctx.write_nonterminal_start();
    ctx.depth += 1;

    {
        ctx.write_indent();
        ctx.write_key("open_paren", "OpenParen");

        ctx.write_terminal_value(&node.open_paren.range);
    }

    {
        ctx.write_indent();
        ctx.write_key("arguments", "NamedArgumentGroup");

        render_named_argument_group(ctx, &node.arguments);
    }

    {
        ctx.write_indent();
        ctx.write_key("close_paren", "CloseParen");

        ctx.write_terminal_value(&node.close_paren.range);
    }

    ctx.depth -= 1;
}

pub fn render_named_import(ctx: &mut RenderContext<'_>, node: &NamedImportStruct) {
    ctx.write_nonterminal_start();
    ctx.depth += 1;

    {
        ctx.write_indent();
        ctx.write_key("asterisk", "Asterisk");

        ctx.write_terminal_value(&node.asterisk.range);
    }

    {
        ctx.write_indent();
        ctx.write_key("alias", "ImportAlias");

        render_import_alias(ctx, &node.alias);
    }

    {
        ctx.write_indent();
        ctx.write_key("from_keyword", "FromKeyword");

        ctx.write_terminal_value(&node.from_keyword.range);
    }

    {
        ctx.write_indent();
        ctx.write_key("path", "StringLiteral");

        ctx.write_terminal_value(&node.path.range);
    }

    ctx.depth -= 1;
}

pub fn render_new_expression(ctx: &mut RenderContext<'_>, node: &NewExpressionStruct) {
    ctx.write_nonterminal_start();
    ctx.depth += 1;

    {
        ctx.write_indent();
        ctx.write_key("new_keyword", "NewKeyword");

        ctx.write_terminal_value(&node.new_keyword.range);
    }

    {
        ctx.write_indent();
        ctx.write_key("type_name", "TypeName");

        render_type_name(ctx, &node.type_name);
    }

    ctx.depth -= 1;
}

pub fn render_or_expression(ctx: &mut RenderContext<'_>, node: &OrExpressionStruct) {
    ctx.write_nonterminal_start();
    ctx.depth += 1;

    {
        ctx.write_indent();
        ctx.write_key("left_operand", "Expression");

        render_expression(ctx, &node.left_operand);
    }

    {
        ctx.write_indent();
        ctx.write_key("operator", "BarBar");

        ctx.write_terminal_value(&node.operator.range);
    }

    {
        ctx.write_indent();
        ctx.write_key("right_operand", "Expression");

        render_expression(ctx, &node.right_operand);
    }

    ctx.depth -= 1;
}

pub fn render_override_paths_declaration(
    ctx: &mut RenderContext<'_>,
    node: &OverridePathsDeclarationStruct,
) {
    ctx.write_nonterminal_start();
    ctx.depth += 1;

    {
        ctx.write_indent();
        ctx.write_key("open_paren", "OpenParen");

        ctx.write_terminal_value(&node.open_paren.range);
    }

    {
        ctx.write_indent();
        ctx.write_key("paths", "OverridePaths");

        render_override_paths(ctx, &node.paths);
    }

    {
        ctx.write_indent();
        ctx.write_key("close_paren", "CloseParen");

        ctx.write_terminal_value(&node.close_paren.range);
    }

    ctx.depth -= 1;
}

pub fn render_override_specifier(ctx: &mut RenderContext<'_>, node: &OverrideSpecifierStruct) {
    ctx.write_nonterminal_start();
    ctx.depth += 1;

    {
        ctx.write_indent();
        ctx.write_key("override_keyword", "OverrideKeyword");

        ctx.write_terminal_value(&node.override_keyword.range);
    }

    if let Some(ref overridden) = node.overridden {
        ctx.write_indent();
        ctx.write_key("overridden", "OverridePathsDeclaration");

        render_override_paths_declaration(ctx, overridden);
    }

    ctx.depth -= 1;
}

pub fn render_parameter(ctx: &mut RenderContext<'_>, node: &ParameterStruct) {
    ctx.write_nonterminal_start();
    ctx.depth += 1;

    {
        ctx.write_indent();
        ctx.write_key("type_name", "TypeName");

        render_type_name(ctx, &node.type_name);
    }

    if let Some(ref storage_location) = node.storage_location {
        ctx.write_indent();
        ctx.write_key("storage_location", "StorageLocation");

        render_storage_location(ctx, storage_location);
    }

    if let Some(ref name) = node.name {
        ctx.write_indent();
        ctx.write_key("name", "Identifier");

        ctx.write_terminal_value(&name.range);
    }

    ctx.depth -= 1;
}

pub fn render_parameters_declaration(
    ctx: &mut RenderContext<'_>,
    node: &ParametersDeclarationStruct,
) {
    ctx.write_nonterminal_start();
    ctx.depth += 1;

    {
        ctx.write_indent();
        ctx.write_key("open_paren", "OpenParen");

        ctx.write_terminal_value(&node.open_paren.range);
    }

    {
        ctx.write_indent();
        ctx.write_key("parameters", "Parameters");

        render_parameters(ctx, &node.parameters);
    }

    {
        ctx.write_indent();
        ctx.write_key("close_paren", "CloseParen");

        ctx.write_terminal_value(&node.close_paren.range);
    }

    ctx.depth -= 1;
}

pub fn render_path_import(ctx: &mut RenderContext<'_>, node: &PathImportStruct) {
    ctx.write_nonterminal_start();
    ctx.depth += 1;

    {
        ctx.write_indent();
        ctx.write_key("path", "StringLiteral");

        ctx.write_terminal_value(&node.path.range);
    }

    if let Some(ref alias) = node.alias {
        ctx.write_indent();
        ctx.write_key("alias", "ImportAlias");

        render_import_alias(ctx, alias);
    }

    ctx.depth -= 1;
}

pub fn render_positional_arguments_declaration(
    ctx: &mut RenderContext<'_>,
    node: &PositionalArgumentsDeclarationStruct,
) {
    ctx.write_nonterminal_start();
    ctx.depth += 1;

    {
        ctx.write_indent();
        ctx.write_key("open_paren", "OpenParen");

        ctx.write_terminal_value(&node.open_paren.range);
    }

    {
        ctx.write_indent();
        ctx.write_key("arguments", "PositionalArguments");

        render_positional_arguments(ctx, &node.arguments);
    }

    {
        ctx.write_indent();
        ctx.write_key("close_paren", "CloseParen");

        ctx.write_terminal_value(&node.close_paren.range);
    }

    ctx.depth -= 1;
}

pub fn render_postfix_expression(ctx: &mut RenderContext<'_>, node: &PostfixExpressionStruct) {
    ctx.write_nonterminal_start();
    ctx.depth += 1;

    {
        ctx.write_indent();
        ctx.write_key("operand", "Expression");

        render_expression(ctx, &node.operand);
    }

    {
        ctx.write_indent();
        ctx.write_key(
            "expression_postfix_expression_operator",
            "Expression_PostfixExpression_Operator",
        );

        render_expression_postfix_expression_operator(
            ctx,
            &node.expression_postfix_expression_operator,
        );
    }

    ctx.depth -= 1;
}

pub fn render_pragma_directive(ctx: &mut RenderContext<'_>, node: &PragmaDirectiveStruct) {
    ctx.write_nonterminal_start();
    ctx.depth += 1;

    {
        ctx.write_indent();
        ctx.write_key("pragma_keyword", "PragmaKeyword");

        ctx.write_terminal_value(&node.pragma_keyword.range);
    }

    {
        ctx.write_indent();
        ctx.write_key("pragma", "Pragma");

        render_pragma(ctx, &node.pragma);
    }

    {
        ctx.write_indent();
        ctx.write_key("semicolon", "PragmaSemicolon");

        ctx.write_terminal_value(&node.semicolon.range);
    }

    ctx.depth -= 1;
}

pub fn render_prefix_expression(ctx: &mut RenderContext<'_>, node: &PrefixExpressionStruct) {
    ctx.write_nonterminal_start();
    ctx.depth += 1;

    {
        ctx.write_indent();
        ctx.write_key(
            "expression_prefix_expression_operator",
            "Expression_PrefixExpression_Operator",
        );

        render_expression_prefix_expression_operator(
            ctx,
            &node.expression_prefix_expression_operator,
        );
    }

    {
        ctx.write_indent();
        ctx.write_key("operand", "Expression");

        render_expression(ctx, &node.operand);
    }

    ctx.depth -= 1;
}

pub fn render_receive_function_definition(
    ctx: &mut RenderContext<'_>,
    node: &ReceiveFunctionDefinitionStruct,
) {
    ctx.write_nonterminal_start();
    ctx.depth += 1;

    {
        ctx.write_indent();
        ctx.write_key("receive_keyword", "ReceiveKeyword");

        ctx.write_terminal_value(&node.receive_keyword.range);
    }

    {
        ctx.write_indent();
        ctx.write_key("parameters", "ParametersDeclaration");

        render_parameters_declaration(ctx, &node.parameters);
    }

    {
        ctx.write_indent();
        ctx.write_key("attributes", "ReceiveFunctionAttributes");

        render_receive_function_attributes(ctx, &node.attributes);
    }

    {
        ctx.write_indent();
        ctx.write_key("body", "FunctionBody");

        render_function_body(ctx, &node.body);
    }

    ctx.depth -= 1;
}

pub fn render_return_statement(ctx: &mut RenderContext<'_>, node: &ReturnStatementStruct) {
    ctx.write_nonterminal_start();
    ctx.depth += 1;

    {
        ctx.write_indent();
        ctx.write_key("return_keyword", "ReturnKeyword");

        ctx.write_terminal_value(&node.return_keyword.range);
    }

    if let Some(ref expression) = node.expression {
        ctx.write_indent();
        ctx.write_key("expression", "Expression");

        render_expression(ctx, expression);
    }

    {
        ctx.write_indent();
        ctx.write_key("semicolon", "Semicolon");

        ctx.write_terminal_value(&node.semicolon.range);
    }

    ctx.depth -= 1;
}

pub fn render_returns_declaration(ctx: &mut RenderContext<'_>, node: &ReturnsDeclarationStruct) {
    ctx.write_nonterminal_start();
    ctx.depth += 1;

    {
        ctx.write_indent();
        ctx.write_key("returns_keyword", "ReturnsKeyword");

        ctx.write_terminal_value(&node.returns_keyword.range);
    }

    {
        ctx.write_indent();
        ctx.write_key("variables", "ParametersDeclaration");

        render_parameters_declaration(ctx, &node.variables);
    }

    ctx.depth -= 1;
}

pub fn render_revert_statement(ctx: &mut RenderContext<'_>, node: &RevertStatementStruct) {
    ctx.write_nonterminal_start();
    ctx.depth += 1;

    {
        ctx.write_indent();
        ctx.write_key("revert_keyword", "RevertKeyword");

        ctx.write_terminal_value(&node.revert_keyword.range);
    }

    {
        ctx.write_indent();
        ctx.write_key("error", "IdentifierPath");

        render_identifier_path(ctx, &node.error);
    }

    {
        ctx.write_indent();
        ctx.write_key("arguments", "ArgumentsDeclaration");

        render_arguments_declaration(ctx, &node.arguments);
    }

    {
        ctx.write_indent();
        ctx.write_key("semicolon", "Semicolon");

        ctx.write_terminal_value(&node.semicolon.range);
    }

    ctx.depth -= 1;
}

pub fn render_shift_expression(ctx: &mut RenderContext<'_>, node: &ShiftExpressionStruct) {
    ctx.write_nonterminal_start();
    ctx.depth += 1;

    {
        ctx.write_indent();
        ctx.write_key("left_operand", "Expression");

        render_expression(ctx, &node.left_operand);
    }

    {
        ctx.write_indent();
        ctx.write_key(
            "expression_shift_expression_operator",
            "Expression_ShiftExpression_Operator",
        );

        render_expression_shift_expression_operator(
            ctx,
            &node.expression_shift_expression_operator,
        );
    }

    {
        ctx.write_indent();
        ctx.write_key("right_operand", "Expression");

        render_expression(ctx, &node.right_operand);
    }

    ctx.depth -= 1;
}

pub fn render_single_typed_declaration(
    ctx: &mut RenderContext<'_>,
    node: &SingleTypedDeclarationStruct,
) {
    ctx.write_nonterminal_start();
    ctx.depth += 1;

    {
        ctx.write_indent();
        ctx.write_key("declaration", "VariableDeclaration");

        render_variable_declaration(ctx, &node.declaration);
    }

    if let Some(ref value) = node.value {
        ctx.write_indent();
        ctx.write_key("value", "VariableDeclarationValue");

        render_variable_declaration_value(ctx, value);
    }

    ctx.depth -= 1;
}

pub fn render_source_unit(ctx: &mut RenderContext<'_>, node: &SourceUnitStruct) {
    ctx.write_nonterminal_start();
    ctx.depth += 1;

    {
        ctx.write_indent();
        ctx.write_key("members", "SourceUnitMembers");

        render_source_unit_members(ctx, &node.members);
    }

    ctx.depth -= 1;
}

pub fn render_state_variable_definition(
    ctx: &mut RenderContext<'_>,
    node: &StateVariableDefinitionStruct,
) {
    ctx.write_nonterminal_start();
    ctx.depth += 1;

    {
        ctx.write_indent();
        ctx.write_key("type_name", "TypeName");

        render_type_name(ctx, &node.type_name);
    }

    {
        ctx.write_indent();
        ctx.write_key("attributes", "StateVariableAttributes");

        render_state_variable_attributes(ctx, &node.attributes);
    }

    {
        ctx.write_indent();
        ctx.write_key("name", "Identifier");

        ctx.write_terminal_value(&node.name.range);
    }

    if let Some(ref value) = node.value {
        ctx.write_indent();
        ctx.write_key("value", "StateVariableDefinitionValue");

        render_state_variable_definition_value(ctx, value);
    }

    {
        ctx.write_indent();
        ctx.write_key("semicolon", "Semicolon");

        ctx.write_terminal_value(&node.semicolon.range);
    }

    ctx.depth -= 1;
}

pub fn render_state_variable_definition_value(
    ctx: &mut RenderContext<'_>,
    node: &StateVariableDefinitionValueStruct,
) {
    ctx.write_nonterminal_start();
    ctx.depth += 1;

    {
        ctx.write_indent();
        ctx.write_key("equal", "Equal");

        ctx.write_terminal_value(&node.equal.range);
    }

    {
        ctx.write_indent();
        ctx.write_key("value", "Expression");

        render_expression(ctx, &node.value);
    }

    ctx.depth -= 1;
}

pub fn render_storage_layout_specifier(
    ctx: &mut RenderContext<'_>,
    node: &StorageLayoutSpecifierStruct,
) {
    ctx.write_nonterminal_start();
    ctx.depth += 1;

    {
        ctx.write_indent();
        ctx.write_key("layout_keyword", "LayoutKeyword");

        ctx.write_terminal_value(&node.layout_keyword.range);
    }

    {
        ctx.write_indent();
        ctx.write_key("at_keyword", "AtKeyword");

        ctx.write_terminal_value(&node.at_keyword.range);
    }

    {
        ctx.write_indent();
        ctx.write_key("expression", "Expression");

        render_expression(ctx, &node.expression);
    }

    ctx.depth -= 1;
}

pub fn render_struct_definition(ctx: &mut RenderContext<'_>, node: &StructDefinitionStruct) {
    ctx.write_nonterminal_start();
    ctx.depth += 1;

    {
        ctx.write_indent();
        ctx.write_key("struct_keyword", "StructKeyword");

        ctx.write_terminal_value(&node.struct_keyword.range);
    }

    {
        ctx.write_indent();
        ctx.write_key("name", "Identifier");

        ctx.write_terminal_value(&node.name.range);
    }

    {
        ctx.write_indent();
        ctx.write_key("open_brace", "OpenBrace");

        ctx.write_terminal_value(&node.open_brace.range);
    }

    {
        ctx.write_indent();
        ctx.write_key("members", "StructMembers");

        render_struct_members(ctx, &node.members);
    }

    {
        ctx.write_indent();
        ctx.write_key("close_brace", "CloseBrace");

        ctx.write_terminal_value(&node.close_brace.range);
    }

    ctx.depth -= 1;
}

pub fn render_struct_member(ctx: &mut RenderContext<'_>, node: &StructMemberStruct) {
    ctx.write_nonterminal_start();
    ctx.depth += 1;

    {
        ctx.write_indent();
        ctx.write_key("type_name", "TypeName");

        render_type_name(ctx, &node.type_name);
    }

    {
        ctx.write_indent();
        ctx.write_key("name", "Identifier");

        ctx.write_terminal_value(&node.name.range);
    }

    {
        ctx.write_indent();
        ctx.write_key("semicolon", "Semicolon");

        ctx.write_terminal_value(&node.semicolon.range);
    }

    ctx.depth -= 1;
}

pub fn render_try_statement(ctx: &mut RenderContext<'_>, node: &TryStatementStruct) {
    ctx.write_nonterminal_start();
    ctx.depth += 1;

    {
        ctx.write_indent();
        ctx.write_key("try_keyword", "TryKeyword");

        ctx.write_terminal_value(&node.try_keyword.range);
    }

    {
        ctx.write_indent();
        ctx.write_key("expression", "Expression");

        render_expression(ctx, &node.expression);
    }

    if let Some(ref returns) = node.returns {
        ctx.write_indent();
        ctx.write_key("returns", "ReturnsDeclaration");

        render_returns_declaration(ctx, returns);
    }

    {
        ctx.write_indent();
        ctx.write_key("body", "Block");

        render_block(ctx, &node.body);
    }

    {
        ctx.write_indent();
        ctx.write_key("catch_clauses", "CatchClauses");

        render_catch_clauses(ctx, &node.catch_clauses);
    }

    ctx.depth -= 1;
}

pub fn render_tuple_expression(ctx: &mut RenderContext<'_>, node: &TupleExpressionStruct) {
    ctx.write_nonterminal_start();
    ctx.depth += 1;

    {
        ctx.write_indent();
        ctx.write_key("open_paren", "OpenParen");

        ctx.write_terminal_value(&node.open_paren.range);
    }

    {
        ctx.write_indent();
        ctx.write_key("items", "TupleValues");

        render_tuple_values(ctx, &node.items);
    }

    {
        ctx.write_indent();
        ctx.write_key("close_paren", "CloseParen");

        ctx.write_terminal_value(&node.close_paren.range);
    }

    ctx.depth -= 1;
}

pub fn render_tuple_value(ctx: &mut RenderContext<'_>, node: &TupleValueStruct) {
    ctx.write_nonterminal_start();
    ctx.depth += 1;

    let mut has_children = false;

    if let Some(ref expression) = node.expression {
        has_children = true;
        ctx.write_indent();
        ctx.write_key("expression", "Expression");

        render_expression(ctx, expression);
    }

    ctx.depth -= 1;

    ctx.finish_nonterminal(has_children);
}

pub fn render_type_expression(ctx: &mut RenderContext<'_>, node: &TypeExpressionStruct) {
    ctx.write_nonterminal_start();
    ctx.depth += 1;

    {
        ctx.write_indent();
        ctx.write_key("type_keyword", "TypeKeyword");

        ctx.write_terminal_value(&node.type_keyword.range);
    }

    {
        ctx.write_indent();
        ctx.write_key("open_paren", "OpenParen");

        ctx.write_terminal_value(&node.open_paren.range);
    }

    {
        ctx.write_indent();
        ctx.write_key("type_name", "TypeName");

        render_type_name(ctx, &node.type_name);
    }

    {
        ctx.write_indent();
        ctx.write_key("close_paren", "CloseParen");

        ctx.write_terminal_value(&node.close_paren.range);
    }

    ctx.depth -= 1;
}

pub fn render_unchecked_block(ctx: &mut RenderContext<'_>, node: &UncheckedBlockStruct) {
    ctx.write_nonterminal_start();
    ctx.depth += 1;

    {
        ctx.write_indent();
        ctx.write_key("unchecked_keyword", "UncheckedKeyword");

        ctx.write_terminal_value(&node.unchecked_keyword.range);
    }

    {
        ctx.write_indent();
        ctx.write_key("block", "Block");

        render_block(ctx, &node.block);
    }

    ctx.depth -= 1;
}

pub fn render_user_defined_value_type_definition(
    ctx: &mut RenderContext<'_>,
    node: &UserDefinedValueTypeDefinitionStruct,
) {
    ctx.write_nonterminal_start();
    ctx.depth += 1;

    {
        ctx.write_indent();
        ctx.write_key("type_keyword", "TypeKeyword");

        ctx.write_terminal_value(&node.type_keyword.range);
    }

    {
        ctx.write_indent();
        ctx.write_key("name", "Identifier");

        ctx.write_terminal_value(&node.name.range);
    }

    {
        ctx.write_indent();
        ctx.write_key("is_keyword", "IsKeyword");

        ctx.write_terminal_value(&node.is_keyword.range);
    }

    {
        ctx.write_indent();
        ctx.write_key("value_type", "ElementaryType");

        render_elementary_type(ctx, &node.value_type);
    }

    {
        ctx.write_indent();
        ctx.write_key("semicolon", "Semicolon");

        ctx.write_terminal_value(&node.semicolon.range);
    }

    ctx.depth -= 1;
}

pub fn render_using_alias(ctx: &mut RenderContext<'_>, node: &UsingAliasStruct) {
    ctx.write_nonterminal_start();
    ctx.depth += 1;

    {
        ctx.write_indent();
        ctx.write_key("as_keyword", "AsKeyword");

        ctx.write_terminal_value(&node.as_keyword.range);
    }

    {
        ctx.write_indent();
        ctx.write_key("operator", "UsingOperator");

        render_using_operator(ctx, &node.operator);
    }

    ctx.depth -= 1;
}

pub fn render_using_deconstruction(ctx: &mut RenderContext<'_>, node: &UsingDeconstructionStruct) {
    ctx.write_nonterminal_start();
    ctx.depth += 1;

    {
        ctx.write_indent();
        ctx.write_key("open_brace", "OpenBrace");

        ctx.write_terminal_value(&node.open_brace.range);
    }

    {
        ctx.write_indent();
        ctx.write_key("symbols", "UsingDeconstructionSymbols");

        render_using_deconstruction_symbols(ctx, &node.symbols);
    }

    {
        ctx.write_indent();
        ctx.write_key("close_brace", "CloseBrace");

        ctx.write_terminal_value(&node.close_brace.range);
    }

    ctx.depth -= 1;
}

pub fn render_using_deconstruction_symbol(
    ctx: &mut RenderContext<'_>,
    node: &UsingDeconstructionSymbolStruct,
) {
    ctx.write_nonterminal_start();
    ctx.depth += 1;

    {
        ctx.write_indent();
        ctx.write_key("name", "IdentifierPath");

        render_identifier_path(ctx, &node.name);
    }

    if let Some(ref alias) = node.alias {
        ctx.write_indent();
        ctx.write_key("alias", "UsingAlias");

        render_using_alias(ctx, alias);
    }

    ctx.depth -= 1;
}

pub fn render_using_directive(ctx: &mut RenderContext<'_>, node: &UsingDirectiveStruct) {
    ctx.write_nonterminal_start();
    ctx.depth += 1;

    {
        ctx.write_indent();
        ctx.write_key("using_keyword", "UsingKeyword");

        ctx.write_terminal_value(&node.using_keyword.range);
    }

    {
        ctx.write_indent();
        ctx.write_key("clause", "UsingClause");

        render_using_clause(ctx, &node.clause);
    }

    {
        ctx.write_indent();
        ctx.write_key("for_keyword", "ForKeyword");

        ctx.write_terminal_value(&node.for_keyword.range);
    }

    {
        ctx.write_indent();
        ctx.write_key("target", "UsingTarget");

        render_using_target(ctx, &node.target);
    }

    if let Some(ref global_keyword) = node.global_keyword {
        ctx.write_indent();
        ctx.write_key("global_keyword", "GlobalKeyword");

        ctx.write_terminal_value(&global_keyword.range);
    }

    {
        ctx.write_indent();
        ctx.write_key("semicolon", "Semicolon");

        ctx.write_terminal_value(&node.semicolon.range);
    }

    ctx.depth -= 1;
}

pub fn render_variable_declaration(ctx: &mut RenderContext<'_>, node: &VariableDeclarationStruct) {
    ctx.write_nonterminal_start();
    ctx.depth += 1;

    {
        ctx.write_indent();
        ctx.write_key("type_name", "TypeName");

        render_type_name(ctx, &node.type_name);
    }

    if let Some(ref storage_location) = node.storage_location {
        ctx.write_indent();
        ctx.write_key("storage_location", "StorageLocation");

        render_storage_location(ctx, storage_location);
    }

    {
        ctx.write_indent();
        ctx.write_key("name", "Identifier");

        ctx.write_terminal_value(&node.name.range);
    }

    ctx.depth -= 1;
}

pub fn render_variable_declaration_statement(
    ctx: &mut RenderContext<'_>,
    node: &VariableDeclarationStatementStruct,
) {
    ctx.write_nonterminal_start();
    ctx.depth += 1;

    {
        ctx.write_indent();
        ctx.write_key("target", "VariableDeclarationTarget");

        render_variable_declaration_target(ctx, &node.target);
    }

    {
        ctx.write_indent();
        ctx.write_key("semicolon", "Semicolon");

        ctx.write_terminal_value(&node.semicolon.range);
    }

    ctx.depth -= 1;
}

pub fn render_variable_declaration_value(
    ctx: &mut RenderContext<'_>,
    node: &VariableDeclarationValueStruct,
) {
    ctx.write_nonterminal_start();
    ctx.depth += 1;

    {
        ctx.write_indent();
        ctx.write_key("equal", "Equal");

        ctx.write_terminal_value(&node.equal.range);
    }

    {
        ctx.write_indent();
        ctx.write_key("expression", "Expression");

        render_expression(ctx, &node.expression);
    }

    ctx.depth -= 1;
}

pub fn render_version_pragma(ctx: &mut RenderContext<'_>, node: &VersionPragmaStruct) {
    ctx.write_nonterminal_start();
    ctx.depth += 1;

    {
        ctx.write_indent();
        ctx.write_key("solidity_keyword", "SolidityKeyword");

        ctx.write_terminal_value(&node.solidity_keyword.range);
    }

    {
        ctx.write_indent();
        ctx.write_key("sets", "VersionExpressionSets");

        render_version_expression_sets(ctx, &node.sets);
    }

    ctx.depth -= 1;
}

pub fn render_version_range(ctx: &mut RenderContext<'_>, node: &VersionRangeStruct) {
    ctx.write_nonterminal_start();
    ctx.depth += 1;

    {
        ctx.write_indent();
        ctx.write_key("start", "VersionLiteral");

        render_version_literal(ctx, &node.start);
    }

    {
        ctx.write_indent();
        ctx.write_key("minus", "PragmaMinus");

        ctx.write_terminal_value(&node.minus.range);
    }

    {
        ctx.write_indent();
        ctx.write_key("end", "VersionLiteral");

        render_version_literal(ctx, &node.end);
    }

    ctx.depth -= 1;
}

pub fn render_version_term(ctx: &mut RenderContext<'_>, node: &VersionTermStruct) {
    ctx.write_nonterminal_start();
    ctx.depth += 1;

    if let Some(ref operator) = node.operator {
        ctx.write_indent();
        ctx.write_key("operator", "VersionOperator");

        render_version_operator(ctx, operator);
    }

    {
        ctx.write_indent();
        ctx.write_key("literal", "VersionLiteral");

        render_version_literal(ctx, &node.literal);
    }

    ctx.depth -= 1;
}

pub fn render_while_statement(ctx: &mut RenderContext<'_>, node: &WhileStatementStruct) {
    ctx.write_nonterminal_start();
    ctx.depth += 1;

    {
        ctx.write_indent();
        ctx.write_key("while_keyword", "WhileKeyword");

        ctx.write_terminal_value(&node.while_keyword.range);
    }

    {
        ctx.write_indent();
        ctx.write_key("open_paren", "OpenParen");

        ctx.write_terminal_value(&node.open_paren.range);
    }

    {
        ctx.write_indent();
        ctx.write_key("condition", "Expression");

        render_expression(ctx, &node.condition);
    }

    {
        ctx.write_indent();
        ctx.write_key("close_paren", "CloseParen");

        ctx.write_terminal_value(&node.close_paren.range);
    }

    {
        ctx.write_indent();
        ctx.write_key("body", "Statement");

        render_statement(ctx, &node.body);
    }

    ctx.depth -= 1;
}

pub fn render_yul_block(ctx: &mut RenderContext<'_>, node: &YulBlockStruct) {
    ctx.write_nonterminal_start();
    ctx.depth += 1;

    {
        ctx.write_indent();
        ctx.write_key("open_brace", "YulOpenBrace");

        ctx.write_terminal_value(&node.open_brace.range);
    }

    {
        ctx.write_indent();
        ctx.write_key("statements", "YulStatements");

        render_yul_statements(ctx, &node.statements);
    }

    {
        ctx.write_indent();
        ctx.write_key("close_brace", "YulCloseBrace");

        ctx.write_terminal_value(&node.close_brace.range);
    }

    ctx.depth -= 1;
}

pub fn render_yul_break_statement(ctx: &mut RenderContext<'_>, node: &YulBreakStatementStruct) {
    ctx.write_nonterminal_start();
    ctx.depth += 1;

    {
        ctx.write_indent();
        ctx.write_key("break_keyword", "YulBreakKeyword");

        ctx.write_terminal_value(&node.break_keyword.range);
    }

    ctx.depth -= 1;
}

pub fn render_yul_continue_statement(
    ctx: &mut RenderContext<'_>,
    node: &YulContinueStatementStruct,
) {
    ctx.write_nonterminal_start();
    ctx.depth += 1;

    {
        ctx.write_indent();
        ctx.write_key("continue_keyword", "YulContinueKeyword");

        ctx.write_terminal_value(&node.continue_keyword.range);
    }

    ctx.depth -= 1;
}

pub fn render_yul_default_case(ctx: &mut RenderContext<'_>, node: &YulDefaultCaseStruct) {
    ctx.write_nonterminal_start();
    ctx.depth += 1;

    {
        ctx.write_indent();
        ctx.write_key("default_keyword", "YulDefaultKeyword");

        ctx.write_terminal_value(&node.default_keyword.range);
    }

    {
        ctx.write_indent();
        ctx.write_key("body", "YulBlock");

        render_yul_block(ctx, &node.body);
    }

    ctx.depth -= 1;
}

pub fn render_yul_flags_declaration(ctx: &mut RenderContext<'_>, node: &YulFlagsDeclarationStruct) {
    ctx.write_nonterminal_start();
    ctx.depth += 1;

    {
        ctx.write_indent();
        ctx.write_key("open_paren", "YulOpenParen");

        ctx.write_terminal_value(&node.open_paren.range);
    }

    {
        ctx.write_indent();
        ctx.write_key("flags", "YulFlags");

        render_yul_flags(ctx, &node.flags);
    }

    {
        ctx.write_indent();
        ctx.write_key("close_paren", "YulCloseParen");

        ctx.write_terminal_value(&node.close_paren.range);
    }

    ctx.depth -= 1;
}

pub fn render_yul_for_statement(ctx: &mut RenderContext<'_>, node: &YulForStatementStruct) {
    ctx.write_nonterminal_start();
    ctx.depth += 1;

    {
        ctx.write_indent();
        ctx.write_key("for_keyword", "YulForKeyword");

        ctx.write_terminal_value(&node.for_keyword.range);
    }

    {
        ctx.write_indent();
        ctx.write_key("initialization", "YulBlock");

        render_yul_block(ctx, &node.initialization);
    }

    {
        ctx.write_indent();
        ctx.write_key("condition", "YulExpression");

        render_yul_expression(ctx, &node.condition);
    }

    {
        ctx.write_indent();
        ctx.write_key("iterator", "YulBlock");

        render_yul_block(ctx, &node.iterator);
    }

    {
        ctx.write_indent();
        ctx.write_key("body", "YulBlock");

        render_yul_block(ctx, &node.body);
    }

    ctx.depth -= 1;
}

pub fn render_yul_function_call_expression(
    ctx: &mut RenderContext<'_>,
    node: &YulFunctionCallExpressionStruct,
) {
    ctx.write_nonterminal_start();
    ctx.depth += 1;

    {
        ctx.write_indent();
        ctx.write_key("operand", "YulExpression");

        render_yul_expression(ctx, &node.operand);
    }

    {
        ctx.write_indent();
        ctx.write_key("open_paren", "YulOpenParen");

        ctx.write_terminal_value(&node.open_paren.range);
    }

    {
        ctx.write_indent();
        ctx.write_key("arguments", "YulArguments");

        render_yul_arguments(ctx, &node.arguments);
    }

    {
        ctx.write_indent();
        ctx.write_key("close_paren", "YulCloseParen");

        ctx.write_terminal_value(&node.close_paren.range);
    }

    ctx.depth -= 1;
}

pub fn render_yul_function_definition(
    ctx: &mut RenderContext<'_>,
    node: &YulFunctionDefinitionStruct,
) {
    ctx.write_nonterminal_start();
    ctx.depth += 1;

    {
        ctx.write_indent();
        ctx.write_key("function_keyword", "YulFunctionKeyword");

        ctx.write_terminal_value(&node.function_keyword.range);
    }

    {
        ctx.write_indent();
        ctx.write_key("name", "YulIdentifier");

        ctx.write_terminal_value(&node.name.range);
    }

    {
        ctx.write_indent();
        ctx.write_key("parameters", "YulParametersDeclaration");

        render_yul_parameters_declaration(ctx, &node.parameters);
    }

    if let Some(ref returns) = node.returns {
        ctx.write_indent();
        ctx.write_key("returns", "YulReturnsDeclaration");

        render_yul_returns_declaration(ctx, returns);
    }

    {
        ctx.write_indent();
        ctx.write_key("body", "YulBlock");

        render_yul_block(ctx, &node.body);
    }

    ctx.depth -= 1;
}

pub fn render_yul_if_statement(ctx: &mut RenderContext<'_>, node: &YulIfStatementStruct) {
    ctx.write_nonterminal_start();
    ctx.depth += 1;

    {
        ctx.write_indent();
        ctx.write_key("if_keyword", "YulIfKeyword");

        ctx.write_terminal_value(&node.if_keyword.range);
    }

    {
        ctx.write_indent();
        ctx.write_key("condition", "YulExpression");

        render_yul_expression(ctx, &node.condition);
    }

    {
        ctx.write_indent();
        ctx.write_key("body", "YulBlock");

        render_yul_block(ctx, &node.body);
    }

    ctx.depth -= 1;
}

pub fn render_yul_leave_statement(ctx: &mut RenderContext<'_>, node: &YulLeaveStatementStruct) {
    ctx.write_nonterminal_start();
    ctx.depth += 1;

    {
        ctx.write_indent();
        ctx.write_key("leave_keyword", "YulLeaveKeyword");

        ctx.write_terminal_value(&node.leave_keyword.range);
    }

    ctx.depth -= 1;
}

pub fn render_yul_parameters_declaration(
    ctx: &mut RenderContext<'_>,
    node: &YulParametersDeclarationStruct,
) {
    ctx.write_nonterminal_start();
    ctx.depth += 1;

    {
        ctx.write_indent();
        ctx.write_key("open_paren", "YulOpenParen");

        ctx.write_terminal_value(&node.open_paren.range);
    }

    {
        ctx.write_indent();
        ctx.write_key("parameters", "YulParameters");

        render_yul_parameters(ctx, &node.parameters);
    }

    {
        ctx.write_indent();
        ctx.write_key("close_paren", "YulCloseParen");

        ctx.write_terminal_value(&node.close_paren.range);
    }

    ctx.depth -= 1;
}

pub fn render_yul_returns_declaration(
    ctx: &mut RenderContext<'_>,
    node: &YulReturnsDeclarationStruct,
) {
    ctx.write_nonterminal_start();
    ctx.depth += 1;

    {
        ctx.write_indent();
        ctx.write_key("minus_greater_than", "YulMinusGreaterThan");

        ctx.write_terminal_value(&node.minus_greater_than.range);
    }

    {
        ctx.write_indent();
        ctx.write_key("variables", "YulVariableNames");

        render_yul_variable_names(ctx, &node.variables);
    }

    ctx.depth -= 1;
}

pub fn render_yul_switch_statement(ctx: &mut RenderContext<'_>, node: &YulSwitchStatementStruct) {
    ctx.write_nonterminal_start();
    ctx.depth += 1;

    {
        ctx.write_indent();
        ctx.write_key("switch_keyword", "YulSwitchKeyword");

        ctx.write_terminal_value(&node.switch_keyword.range);
    }

    {
        ctx.write_indent();
        ctx.write_key("expression", "YulExpression");

        render_yul_expression(ctx, &node.expression);
    }

    {
        ctx.write_indent();
        ctx.write_key("cases", "YulSwitchCases");

        render_yul_switch_cases(ctx, &node.cases);
    }

    ctx.depth -= 1;
}

pub fn render_yul_value_case(ctx: &mut RenderContext<'_>, node: &YulValueCaseStruct) {
    ctx.write_nonterminal_start();
    ctx.depth += 1;

    {
        ctx.write_indent();
        ctx.write_key("case_keyword", "YulCaseKeyword");

        ctx.write_terminal_value(&node.case_keyword.range);
    }

    {
        ctx.write_indent();
        ctx.write_key("value", "YulLiteral");

        render_yul_literal(ctx, &node.value);
    }

    {
        ctx.write_indent();
        ctx.write_key("body", "YulBlock");

        render_yul_block(ctx, &node.body);
    }

    ctx.depth -= 1;
}

pub fn render_yul_variable_assignment_statement(
    ctx: &mut RenderContext<'_>,
    node: &YulVariableAssignmentStatementStruct,
) {
    ctx.write_nonterminal_start();
    ctx.depth += 1;

    {
        ctx.write_indent();
        ctx.write_key("variables", "YulPaths");

        render_yul_paths(ctx, &node.variables);
    }

    {
        ctx.write_indent();
        ctx.write_key("assignment", "YulColonEqual");

        ctx.write_terminal_value(&node.assignment.range);
    }

    {
        ctx.write_indent();
        ctx.write_key("expression", "YulExpression");

        render_yul_expression(ctx, &node.expression);
    }

    ctx.depth -= 1;
}

pub fn render_yul_variable_declaration_statement(
    ctx: &mut RenderContext<'_>,
    node: &YulVariableDeclarationStatementStruct,
) {
    ctx.write_nonterminal_start();
    ctx.depth += 1;

    {
        ctx.write_indent();
        ctx.write_key("let_keyword", "YulLetKeyword");

        ctx.write_terminal_value(&node.let_keyword.range);
    }

    {
        ctx.write_indent();
        ctx.write_key("variables", "YulVariableNames");

        render_yul_variable_names(ctx, &node.variables);
    }

    if let Some(ref value) = node.value {
        ctx.write_indent();
        ctx.write_key("value", "YulVariableDeclarationValue");

        render_yul_variable_declaration_value(ctx, value);
    }

    ctx.depth -= 1;
}

pub fn render_yul_variable_declaration_value(
    ctx: &mut RenderContext<'_>,
    node: &YulVariableDeclarationValueStruct,
) {
    ctx.write_nonterminal_start();
    ctx.depth += 1;

    {
        ctx.write_indent();
        ctx.write_key("assignment", "YulColonEqual");

        ctx.write_terminal_value(&node.assignment.range);
    }

    {
        ctx.write_indent();
        ctx.write_key("expression", "YulExpression");

        render_yul_expression(ctx, &node.expression);
    }

    ctx.depth -= 1;
}

//
// Choices:
//

pub fn render_abicoder_version(ctx: &mut RenderContext<'_>, node: &AbicoderVersion) {
    match node {
        AbicoderVersion::AbicoderV1Keyword(ref element) => {
            ctx.write_connector();
            ctx.write_key("abicoder_v1_keyword", "AbicoderV1Keyword");
            ctx.write_terminal_value(&element.range);
        }

        AbicoderVersion::AbicoderV2Keyword(ref element) => {
            ctx.write_connector();
            ctx.write_key("abicoder_v2_keyword", "AbicoderV2Keyword");
            ctx.write_terminal_value(&element.range);
        }
    }
}

pub fn render_arguments_declaration(ctx: &mut RenderContext<'_>, node: &ArgumentsDeclaration) {
    match node {
        ArgumentsDeclaration::PositionalArgumentsDeclaration(ref element) => {
            ctx.write_connector();
            ctx.write_key(
                "positional_arguments_declaration",
                "PositionalArgumentsDeclaration",
            );
            render_positional_arguments_declaration(ctx, element);
        }

        ArgumentsDeclaration::NamedArgumentsDeclaration(ref element) => {
            ctx.write_connector();
            ctx.write_key("named_arguments_declaration", "NamedArgumentsDeclaration");
            render_named_arguments_declaration(ctx, element);
        }
    }
}

pub fn render_constructor_attribute(ctx: &mut RenderContext<'_>, node: &ConstructorAttribute) {
    match node {
        ConstructorAttribute::ModifierInvocation(ref element) => {
            ctx.write_connector();
            ctx.write_key("modifier_invocation", "ModifierInvocation");
            render_modifier_invocation(ctx, element);
        }

        ConstructorAttribute::InternalKeyword(ref element) => {
            ctx.write_connector();
            ctx.write_key("internal_keyword", "InternalKeyword");
            ctx.write_terminal_value(&element.range);
        }

        ConstructorAttribute::PayableKeyword(ref element) => {
            ctx.write_connector();
            ctx.write_key("payable_keyword", "PayableKeyword");
            ctx.write_terminal_value(&element.range);
        }

        ConstructorAttribute::PublicKeyword(ref element) => {
            ctx.write_connector();
            ctx.write_key("public_keyword", "PublicKeyword");
            ctx.write_terminal_value(&element.range);
        }
    }
}

pub fn render_contract_member(ctx: &mut RenderContext<'_>, node: &ContractMember) {
    match node {
        ContractMember::UsingDirective(ref element) => {
            ctx.write_connector();
            ctx.write_key("using_directive", "UsingDirective");
            render_using_directive(ctx, element);
        }

        ContractMember::FunctionDefinition(ref element) => {
            ctx.write_connector();
            ctx.write_key("function_definition", "FunctionDefinition");
            render_function_definition(ctx, element);
        }

        ContractMember::ConstructorDefinition(ref element) => {
            ctx.write_connector();
            ctx.write_key("constructor_definition", "ConstructorDefinition");
            render_constructor_definition(ctx, element);
        }

        ContractMember::ReceiveFunctionDefinition(ref element) => {
            ctx.write_connector();
            ctx.write_key("receive_function_definition", "ReceiveFunctionDefinition");
            render_receive_function_definition(ctx, element);
        }

        ContractMember::FallbackFunctionDefinition(ref element) => {
            ctx.write_connector();
            ctx.write_key("fallback_function_definition", "FallbackFunctionDefinition");
            render_fallback_function_definition(ctx, element);
        }

        ContractMember::ModifierDefinition(ref element) => {
            ctx.write_connector();
            ctx.write_key("modifier_definition", "ModifierDefinition");
            render_modifier_definition(ctx, element);
        }

        ContractMember::StructDefinition(ref element) => {
            ctx.write_connector();
            ctx.write_key("struct_definition", "StructDefinition");
            render_struct_definition(ctx, element);
        }

        ContractMember::EnumDefinition(ref element) => {
            ctx.write_connector();
            ctx.write_key("enum_definition", "EnumDefinition");
            render_enum_definition(ctx, element);
        }

        ContractMember::EventDefinition(ref element) => {
            ctx.write_connector();
            ctx.write_key("event_definition", "EventDefinition");
            render_event_definition(ctx, element);
        }

        ContractMember::ErrorDefinition(ref element) => {
            ctx.write_connector();
            ctx.write_key("error_definition", "ErrorDefinition");
            render_error_definition(ctx, element);
        }

        ContractMember::UserDefinedValueTypeDefinition(ref element) => {
            ctx.write_connector();
            ctx.write_key(
                "user_defined_value_type_definition",
                "UserDefinedValueTypeDefinition",
            );
            render_user_defined_value_type_definition(ctx, element);
        }

        ContractMember::StateVariableDefinition(ref element) => {
            ctx.write_connector();
            ctx.write_key("state_variable_definition", "StateVariableDefinition");
            render_state_variable_definition(ctx, element);
        }
    }
}

pub fn render_contract_specifier(ctx: &mut RenderContext<'_>, node: &ContractSpecifier) {
    match node {
        ContractSpecifier::InheritanceSpecifier(ref element) => {
            ctx.write_connector();
            ctx.write_key("inheritance_specifier", "InheritanceSpecifier");
            render_inheritance_specifier(ctx, element);
        }

        ContractSpecifier::StorageLayoutSpecifier(ref element) => {
            ctx.write_connector();
            ctx.write_key("storage_layout_specifier", "StorageLayoutSpecifier");
            render_storage_layout_specifier(ctx, element);
        }
    }
}

pub fn render_elementary_type(ctx: &mut RenderContext<'_>, node: &ElementaryType) {
    match node {
        ElementaryType::BoolKeyword(ref element) => {
            ctx.write_connector();
            ctx.write_key("bool_keyword", "BoolKeyword");
            ctx.write_terminal_value(&element.range);
        }

        ElementaryType::StringKeyword(ref element) => {
            ctx.write_connector();
            ctx.write_key("string_keyword", "StringKeyword");
            ctx.write_terminal_value(&element.range);
        }

        ElementaryType::AddressType(ref element) => {
            ctx.write_connector();
            ctx.write_key("address_type", "AddressType");
            render_address_type(ctx, element);
        }

        ElementaryType::BytesKeyword(ref element) => {
            ctx.write_connector();
            ctx.write_key("bytes_keyword", "BytesKeyword");
            ctx.write_terminal_value(&element.range);
        }

        ElementaryType::IntKeyword(ref element) => {
            ctx.write_connector();
            ctx.write_key("int_keyword", "IntKeyword");
            ctx.write_terminal_value(&element.range);
        }

        ElementaryType::UintKeyword(ref element) => {
            ctx.write_connector();
            ctx.write_key("uint_keyword", "UintKeyword");
            ctx.write_terminal_value(&element.range);
        }

        ElementaryType::FixedKeyword(ref element) => {
            ctx.write_connector();
            ctx.write_key("fixed_keyword", "FixedKeyword");
            ctx.write_terminal_value(&element.range);
        }

        ElementaryType::UfixedKeyword(ref element) => {
            ctx.write_connector();
            ctx.write_key("ufixed_keyword", "UfixedKeyword");
            ctx.write_terminal_value(&element.range);
        }
    }
}

pub fn render_experimental_feature(ctx: &mut RenderContext<'_>, node: &ExperimentalFeature) {
    match node {
        ExperimentalFeature::ABIEncoderV2Keyword(ref element) => {
            ctx.write_connector();
            ctx.write_key("abi_encoder_v2_keyword", "ABIEncoderV2Keyword");
            ctx.write_terminal_value(&element.range);
        }

        ExperimentalFeature::SMTCheckerKeyword(ref element) => {
            ctx.write_connector();
            ctx.write_key("smt_checker_keyword", "SMTCheckerKeyword");
            ctx.write_terminal_value(&element.range);
        }

        ExperimentalFeature::PragmaStringLiteral(ref element) => {
            ctx.write_connector();
            ctx.write_key("pragma_string_literal", "PragmaStringLiteral");
            ctx.write_terminal_value(&element.range);
        }
    }
}

pub fn render_expression(ctx: &mut RenderContext<'_>, node: &Expression) {
    match node {
        Expression::AssignmentExpression(ref element) => {
            ctx.write_connector();
            ctx.write_key("assignment_expression", "AssignmentExpression");
            render_assignment_expression(ctx, element);
        }

        Expression::ConditionalExpression(ref element) => {
            ctx.write_connector();
            ctx.write_key("conditional_expression", "ConditionalExpression");
            render_conditional_expression(ctx, element);
        }

        Expression::OrExpression(ref element) => {
            ctx.write_connector();
            ctx.write_key("or_expression", "OrExpression");
            render_or_expression(ctx, element);
        }

        Expression::AndExpression(ref element) => {
            ctx.write_connector();
            ctx.write_key("and_expression", "AndExpression");
            render_and_expression(ctx, element);
        }

        Expression::EqualityExpression(ref element) => {
            ctx.write_connector();
            ctx.write_key("equality_expression", "EqualityExpression");
            render_equality_expression(ctx, element);
        }

        Expression::InequalityExpression(ref element) => {
            ctx.write_connector();
            ctx.write_key("inequality_expression", "InequalityExpression");
            render_inequality_expression(ctx, element);
        }

        Expression::BitwiseOrExpression(ref element) => {
            ctx.write_connector();
            ctx.write_key("bitwise_or_expression", "BitwiseOrExpression");
            render_bitwise_or_expression(ctx, element);
        }

        Expression::BitwiseXorExpression(ref element) => {
            ctx.write_connector();
            ctx.write_key("bitwise_xor_expression", "BitwiseXorExpression");
            render_bitwise_xor_expression(ctx, element);
        }

        Expression::BitwiseAndExpression(ref element) => {
            ctx.write_connector();
            ctx.write_key("bitwise_and_expression", "BitwiseAndExpression");
            render_bitwise_and_expression(ctx, element);
        }

        Expression::ShiftExpression(ref element) => {
            ctx.write_connector();
            ctx.write_key("shift_expression", "ShiftExpression");
            render_shift_expression(ctx, element);
        }

        Expression::AdditiveExpression(ref element) => {
            ctx.write_connector();
            ctx.write_key("additive_expression", "AdditiveExpression");
            render_additive_expression(ctx, element);
        }

        Expression::MultiplicativeExpression(ref element) => {
            ctx.write_connector();
            ctx.write_key("multiplicative_expression", "MultiplicativeExpression");
            render_multiplicative_expression(ctx, element);
        }

        Expression::ExponentiationExpression(ref element) => {
            ctx.write_connector();
            ctx.write_key("exponentiation_expression", "ExponentiationExpression");
            render_exponentiation_expression(ctx, element);
        }

        Expression::PostfixExpression(ref element) => {
            ctx.write_connector();
            ctx.write_key("postfix_expression", "PostfixExpression");
            render_postfix_expression(ctx, element);
        }

        Expression::PrefixExpression(ref element) => {
            ctx.write_connector();
            ctx.write_key("prefix_expression", "PrefixExpression");
            render_prefix_expression(ctx, element);
        }

        Expression::FunctionCallExpression(ref element) => {
            ctx.write_connector();
            ctx.write_key("function_call_expression", "FunctionCallExpression");
            render_function_call_expression(ctx, element);
        }

        Expression::CallOptionsExpression(ref element) => {
            ctx.write_connector();
            ctx.write_key("call_options_expression", "CallOptionsExpression");
            render_call_options_expression(ctx, element);
        }

        Expression::MemberAccessExpression(ref element) => {
            ctx.write_connector();
            ctx.write_key("member_access_expression", "MemberAccessExpression");
            render_member_access_expression(ctx, element);
        }

        Expression::IndexAccessExpression(ref element) => {
            ctx.write_connector();
            ctx.write_key("index_access_expression", "IndexAccessExpression");
            render_index_access_expression(ctx, element);
        }

        Expression::NewExpression(ref element) => {
            ctx.write_connector();
            ctx.write_key("new_expression", "NewExpression");
            render_new_expression(ctx, element);
        }

        Expression::TupleExpression(ref element) => {
            ctx.write_connector();
            ctx.write_key("tuple_expression", "TupleExpression");
            render_tuple_expression(ctx, element);
        }

        Expression::TypeExpression(ref element) => {
            ctx.write_connector();
            ctx.write_key("type_expression", "TypeExpression");
            render_type_expression(ctx, element);
        }

        Expression::ArrayExpression(ref element) => {
            ctx.write_connector();
            ctx.write_key("array_expression", "ArrayExpression");
            render_array_expression(ctx, element);
        }

        Expression::HexNumberExpression(ref element) => {
            ctx.write_connector();
            ctx.write_key("hex_number_expression", "HexNumberExpression");
            render_hex_number_expression(ctx, element);
        }

        Expression::DecimalNumberExpression(ref element) => {
            ctx.write_connector();
            ctx.write_key("decimal_number_expression", "DecimalNumberExpression");
            render_decimal_number_expression(ctx, element);
        }

        Expression::StringExpression(ref element) => {
            ctx.write_connector();
            ctx.write_key("string_expression", "StringExpression");
            render_string_expression(ctx, element);
        }

        Expression::ElementaryType(ref element) => {
            ctx.write_connector();
            ctx.write_key("elementary_type", "ElementaryType");
            render_elementary_type(ctx, element);
        }

        Expression::PayableKeyword(ref element) => {
            ctx.write_connector();
            ctx.write_key("payable_keyword", "PayableKeyword");
            ctx.write_terminal_value(&element.range);
        }

        Expression::ThisKeyword(ref element) => {
            ctx.write_connector();
            ctx.write_key("this_keyword", "ThisKeyword");
            ctx.write_terminal_value(&element.range);
        }

        Expression::SuperKeyword(ref element) => {
            ctx.write_connector();
            ctx.write_key("super_keyword", "SuperKeyword");
            ctx.write_terminal_value(&element.range);
        }

        Expression::TrueKeyword(ref element) => {
            ctx.write_connector();
            ctx.write_key("true_keyword", "TrueKeyword");
            ctx.write_terminal_value(&element.range);
        }

        Expression::FalseKeyword(ref element) => {
            ctx.write_connector();
            ctx.write_key("false_keyword", "FalseKeyword");
            ctx.write_terminal_value(&element.range);
        }

        Expression::Identifier(ref element) => {
            ctx.write_connector();
            ctx.write_key("identifier", "Identifier");
            ctx.write_terminal_value(&element.range);
        }
    }
}

pub fn render_expression_additive_expression_operator(
    ctx: &mut RenderContext<'_>,
    node: &Expression_AdditiveExpression_Operator,
) {
    match node {
        Expression_AdditiveExpression_Operator::Minus(ref element) => {
            ctx.write_connector();
            ctx.write_key("minus", "Minus");
            ctx.write_terminal_value(&element.range);
        }

        Expression_AdditiveExpression_Operator::Plus(ref element) => {
            ctx.write_connector();
            ctx.write_key("plus", "Plus");
            ctx.write_terminal_value(&element.range);
        }
    }
}

pub fn render_expression_assignment_expression_operator(
    ctx: &mut RenderContext<'_>,
    node: &Expression_AssignmentExpression_Operator,
) {
    match node {
        Expression_AssignmentExpression_Operator::AmpersandEqual(ref element) => {
            ctx.write_connector();
            ctx.write_key("ampersand_equal", "AmpersandEqual");
            ctx.write_terminal_value(&element.range);
        }

        Expression_AssignmentExpression_Operator::AsteriskEqual(ref element) => {
            ctx.write_connector();
            ctx.write_key("asterisk_equal", "AsteriskEqual");
            ctx.write_terminal_value(&element.range);
        }

        Expression_AssignmentExpression_Operator::BarEqual(ref element) => {
            ctx.write_connector();
            ctx.write_key("bar_equal", "BarEqual");
            ctx.write_terminal_value(&element.range);
        }

        Expression_AssignmentExpression_Operator::CaretEqual(ref element) => {
            ctx.write_connector();
            ctx.write_key("caret_equal", "CaretEqual");
            ctx.write_terminal_value(&element.range);
        }

        Expression_AssignmentExpression_Operator::Equal(ref element) => {
            ctx.write_connector();
            ctx.write_key("equal", "Equal");
            ctx.write_terminal_value(&element.range);
        }

        Expression_AssignmentExpression_Operator::GreaterThanGreaterThanEqual(ref element) => {
            ctx.write_connector();
            ctx.write_key(
                "greater_than_greater_than_equal",
                "GreaterThanGreaterThanEqual",
            );
            ctx.write_terminal_value(&element.range);
        }

        Expression_AssignmentExpression_Operator::GreaterThanGreaterThanGreaterThanEqual(
            ref element,
        ) => {
            ctx.write_connector();
            ctx.write_key(
                "greater_than_greater_than_greater_than_equal",
                "GreaterThanGreaterThanGreaterThanEqual",
            );
            ctx.write_terminal_value(&element.range);
        }

        Expression_AssignmentExpression_Operator::LessThanLessThanEqual(ref element) => {
            ctx.write_connector();
            ctx.write_key("less_than_less_than_equal", "LessThanLessThanEqual");
            ctx.write_terminal_value(&element.range);
        }

        Expression_AssignmentExpression_Operator::MinusEqual(ref element) => {
            ctx.write_connector();
            ctx.write_key("minus_equal", "MinusEqual");
            ctx.write_terminal_value(&element.range);
        }

        Expression_AssignmentExpression_Operator::PercentEqual(ref element) => {
            ctx.write_connector();
            ctx.write_key("percent_equal", "PercentEqual");
            ctx.write_terminal_value(&element.range);
        }

        Expression_AssignmentExpression_Operator::PlusEqual(ref element) => {
            ctx.write_connector();
            ctx.write_key("plus_equal", "PlusEqual");
            ctx.write_terminal_value(&element.range);
        }

        Expression_AssignmentExpression_Operator::SlashEqual(ref element) => {
            ctx.write_connector();
            ctx.write_key("slash_equal", "SlashEqual");
            ctx.write_terminal_value(&element.range);
        }
    }
}

pub fn render_expression_equality_expression_operator(
    ctx: &mut RenderContext<'_>,
    node: &Expression_EqualityExpression_Operator,
) {
    match node {
        Expression_EqualityExpression_Operator::BangEqual(ref element) => {
            ctx.write_connector();
            ctx.write_key("bang_equal", "BangEqual");
            ctx.write_terminal_value(&element.range);
        }

        Expression_EqualityExpression_Operator::EqualEqual(ref element) => {
            ctx.write_connector();
            ctx.write_key("equal_equal", "EqualEqual");
            ctx.write_terminal_value(&element.range);
        }
    }
}

pub fn render_expression_inequality_expression_operator(
    ctx: &mut RenderContext<'_>,
    node: &Expression_InequalityExpression_Operator,
) {
    match node {
        Expression_InequalityExpression_Operator::GreaterThan(ref element) => {
            ctx.write_connector();
            ctx.write_key("greater_than", "GreaterThan");
            ctx.write_terminal_value(&element.range);
        }

        Expression_InequalityExpression_Operator::GreaterThanEqual(ref element) => {
            ctx.write_connector();
            ctx.write_key("greater_than_equal", "GreaterThanEqual");
            ctx.write_terminal_value(&element.range);
        }

        Expression_InequalityExpression_Operator::LessThan(ref element) => {
            ctx.write_connector();
            ctx.write_key("less_than", "LessThan");
            ctx.write_terminal_value(&element.range);
        }

        Expression_InequalityExpression_Operator::LessThanEqual(ref element) => {
            ctx.write_connector();
            ctx.write_key("less_than_equal", "LessThanEqual");
            ctx.write_terminal_value(&element.range);
        }
    }
}

pub fn render_expression_multiplicative_expression_operator(
    ctx: &mut RenderContext<'_>,
    node: &Expression_MultiplicativeExpression_Operator,
) {
    match node {
        Expression_MultiplicativeExpression_Operator::Asterisk(ref element) => {
            ctx.write_connector();
            ctx.write_key("asterisk", "Asterisk");
            ctx.write_terminal_value(&element.range);
        }

        Expression_MultiplicativeExpression_Operator::Percent(ref element) => {
            ctx.write_connector();
            ctx.write_key("percent", "Percent");
            ctx.write_terminal_value(&element.range);
        }

        Expression_MultiplicativeExpression_Operator::Slash(ref element) => {
            ctx.write_connector();
            ctx.write_key("slash", "Slash");
            ctx.write_terminal_value(&element.range);
        }
    }
}

pub fn render_expression_postfix_expression_operator(
    ctx: &mut RenderContext<'_>,
    node: &Expression_PostfixExpression_Operator,
) {
    match node {
        Expression_PostfixExpression_Operator::MinusMinus(ref element) => {
            ctx.write_connector();
            ctx.write_key("minus_minus", "MinusMinus");
            ctx.write_terminal_value(&element.range);
        }

        Expression_PostfixExpression_Operator::PlusPlus(ref element) => {
            ctx.write_connector();
            ctx.write_key("plus_plus", "PlusPlus");
            ctx.write_terminal_value(&element.range);
        }
    }
}

pub fn render_expression_prefix_expression_operator(
    ctx: &mut RenderContext<'_>,
    node: &Expression_PrefixExpression_Operator,
) {
    match node {
        Expression_PrefixExpression_Operator::Bang(ref element) => {
            ctx.write_connector();
            ctx.write_key("bang", "Bang");
            ctx.write_terminal_value(&element.range);
        }

        Expression_PrefixExpression_Operator::DeleteKeyword(ref element) => {
            ctx.write_connector();
            ctx.write_key("delete_keyword", "DeleteKeyword");
            ctx.write_terminal_value(&element.range);
        }

        Expression_PrefixExpression_Operator::Minus(ref element) => {
            ctx.write_connector();
            ctx.write_key("minus", "Minus");
            ctx.write_terminal_value(&element.range);
        }

        Expression_PrefixExpression_Operator::MinusMinus(ref element) => {
            ctx.write_connector();
            ctx.write_key("minus_minus", "MinusMinus");
            ctx.write_terminal_value(&element.range);
        }

        Expression_PrefixExpression_Operator::PlusPlus(ref element) => {
            ctx.write_connector();
            ctx.write_key("plus_plus", "PlusPlus");
            ctx.write_terminal_value(&element.range);
        }

        Expression_PrefixExpression_Operator::Tilde(ref element) => {
            ctx.write_connector();
            ctx.write_key("tilde", "Tilde");
            ctx.write_terminal_value(&element.range);
        }
    }
}

pub fn render_expression_shift_expression_operator(
    ctx: &mut RenderContext<'_>,
    node: &Expression_ShiftExpression_Operator,
) {
    match node {
        Expression_ShiftExpression_Operator::GreaterThanGreaterThan(ref element) => {
            ctx.write_connector();
            ctx.write_key("greater_than_greater_than", "GreaterThanGreaterThan");
            ctx.write_terminal_value(&element.range);
        }

        Expression_ShiftExpression_Operator::GreaterThanGreaterThanGreaterThan(ref element) => {
            ctx.write_connector();
            ctx.write_key(
                "greater_than_greater_than_greater_than",
                "GreaterThanGreaterThanGreaterThan",
            );
            ctx.write_terminal_value(&element.range);
        }

        Expression_ShiftExpression_Operator::LessThanLessThan(ref element) => {
            ctx.write_connector();
            ctx.write_key("less_than_less_than", "LessThanLessThan");
            ctx.write_terminal_value(&element.range);
        }
    }
}

pub fn render_fallback_function_attribute(
    ctx: &mut RenderContext<'_>,
    node: &FallbackFunctionAttribute,
) {
    match node {
        FallbackFunctionAttribute::ModifierInvocation(ref element) => {
            ctx.write_connector();
            ctx.write_key("modifier_invocation", "ModifierInvocation");
            render_modifier_invocation(ctx, element);
        }

        FallbackFunctionAttribute::OverrideSpecifier(ref element) => {
            ctx.write_connector();
            ctx.write_key("override_specifier", "OverrideSpecifier");
            render_override_specifier(ctx, element);
        }

        FallbackFunctionAttribute::ExternalKeyword(ref element) => {
            ctx.write_connector();
            ctx.write_key("external_keyword", "ExternalKeyword");
            ctx.write_terminal_value(&element.range);
        }

        FallbackFunctionAttribute::PayableKeyword(ref element) => {
            ctx.write_connector();
            ctx.write_key("payable_keyword", "PayableKeyword");
            ctx.write_terminal_value(&element.range);
        }

        FallbackFunctionAttribute::PureKeyword(ref element) => {
            ctx.write_connector();
            ctx.write_key("pure_keyword", "PureKeyword");
            ctx.write_terminal_value(&element.range);
        }

        FallbackFunctionAttribute::ViewKeyword(ref element) => {
            ctx.write_connector();
            ctx.write_key("view_keyword", "ViewKeyword");
            ctx.write_terminal_value(&element.range);
        }

        FallbackFunctionAttribute::VirtualKeyword(ref element) => {
            ctx.write_connector();
            ctx.write_key("virtual_keyword", "VirtualKeyword");
            ctx.write_terminal_value(&element.range);
        }
    }
}

pub fn render_for_statement_condition(ctx: &mut RenderContext<'_>, node: &ForStatementCondition) {
    match node {
        ForStatementCondition::ExpressionStatement(ref element) => {
            ctx.write_connector();
            ctx.write_key("expression_statement", "ExpressionStatement");
            render_expression_statement(ctx, element);
        }

        ForStatementCondition::Semicolon(ref element) => {
            ctx.write_connector();
            ctx.write_key("semicolon", "Semicolon");
            ctx.write_terminal_value(&element.range);
        }
    }
}

pub fn render_for_statement_initialization(
    ctx: &mut RenderContext<'_>,
    node: &ForStatementInitialization,
) {
    match node {
        ForStatementInitialization::VariableDeclarationStatement(ref element) => {
            ctx.write_connector();
            ctx.write_key(
                "variable_declaration_statement",
                "VariableDeclarationStatement",
            );
            render_variable_declaration_statement(ctx, element);
        }

        ForStatementInitialization::ExpressionStatement(ref element) => {
            ctx.write_connector();
            ctx.write_key("expression_statement", "ExpressionStatement");
            render_expression_statement(ctx, element);
        }

        ForStatementInitialization::Semicolon(ref element) => {
            ctx.write_connector();
            ctx.write_key("semicolon", "Semicolon");
            ctx.write_terminal_value(&element.range);
        }
    }
}

pub fn render_function_attribute(ctx: &mut RenderContext<'_>, node: &FunctionAttribute) {
    match node {
        FunctionAttribute::ModifierInvocation(ref element) => {
            ctx.write_connector();
            ctx.write_key("modifier_invocation", "ModifierInvocation");
            render_modifier_invocation(ctx, element);
        }

        FunctionAttribute::OverrideSpecifier(ref element) => {
            ctx.write_connector();
            ctx.write_key("override_specifier", "OverrideSpecifier");
            render_override_specifier(ctx, element);
        }

        FunctionAttribute::ExternalKeyword(ref element) => {
            ctx.write_connector();
            ctx.write_key("external_keyword", "ExternalKeyword");
            ctx.write_terminal_value(&element.range);
        }

        FunctionAttribute::InternalKeyword(ref element) => {
            ctx.write_connector();
            ctx.write_key("internal_keyword", "InternalKeyword");
            ctx.write_terminal_value(&element.range);
        }

        FunctionAttribute::PayableKeyword(ref element) => {
            ctx.write_connector();
            ctx.write_key("payable_keyword", "PayableKeyword");
            ctx.write_terminal_value(&element.range);
        }

        FunctionAttribute::PrivateKeyword(ref element) => {
            ctx.write_connector();
            ctx.write_key("private_keyword", "PrivateKeyword");
            ctx.write_terminal_value(&element.range);
        }

        FunctionAttribute::PublicKeyword(ref element) => {
            ctx.write_connector();
            ctx.write_key("public_keyword", "PublicKeyword");
            ctx.write_terminal_value(&element.range);
        }

        FunctionAttribute::PureKeyword(ref element) => {
            ctx.write_connector();
            ctx.write_key("pure_keyword", "PureKeyword");
            ctx.write_terminal_value(&element.range);
        }

        FunctionAttribute::ViewKeyword(ref element) => {
            ctx.write_connector();
            ctx.write_key("view_keyword", "ViewKeyword");
            ctx.write_terminal_value(&element.range);
        }

        FunctionAttribute::VirtualKeyword(ref element) => {
            ctx.write_connector();
            ctx.write_key("virtual_keyword", "VirtualKeyword");
            ctx.write_terminal_value(&element.range);
        }
    }
}

pub fn render_function_body(ctx: &mut RenderContext<'_>, node: &FunctionBody) {
    match node {
        FunctionBody::Block(ref element) => {
            ctx.write_connector();
            ctx.write_key("block", "Block");
            render_block(ctx, element);
        }

        FunctionBody::Semicolon(ref element) => {
            ctx.write_connector();
            ctx.write_key("semicolon", "Semicolon");
            ctx.write_terminal_value(&element.range);
        }
    }
}

pub fn render_function_name(ctx: &mut RenderContext<'_>, node: &FunctionName) {
    match node {
        FunctionName::Identifier(ref element) => {
            ctx.write_connector();
            ctx.write_key("identifier", "Identifier");
            ctx.write_terminal_value(&element.range);
        }

        FunctionName::FallbackKeyword(ref element) => {
            ctx.write_connector();
            ctx.write_key("fallback_keyword", "FallbackKeyword");
            ctx.write_terminal_value(&element.range);
        }

        FunctionName::ReceiveKeyword(ref element) => {
            ctx.write_connector();
            ctx.write_key("receive_keyword", "ReceiveKeyword");
            ctx.write_terminal_value(&element.range);
        }
    }
}

pub fn render_function_type_attribute(ctx: &mut RenderContext<'_>, node: &FunctionTypeAttribute) {
    match node {
        FunctionTypeAttribute::InternalKeyword(ref element) => {
            ctx.write_connector();
            ctx.write_key("internal_keyword", "InternalKeyword");
            ctx.write_terminal_value(&element.range);
        }

        FunctionTypeAttribute::ExternalKeyword(ref element) => {
            ctx.write_connector();
            ctx.write_key("external_keyword", "ExternalKeyword");
            ctx.write_terminal_value(&element.range);
        }

        FunctionTypeAttribute::PrivateKeyword(ref element) => {
            ctx.write_connector();
            ctx.write_key("private_keyword", "PrivateKeyword");
            ctx.write_terminal_value(&element.range);
        }

        FunctionTypeAttribute::PublicKeyword(ref element) => {
            ctx.write_connector();
            ctx.write_key("public_keyword", "PublicKeyword");
            ctx.write_terminal_value(&element.range);
        }

        FunctionTypeAttribute::PureKeyword(ref element) => {
            ctx.write_connector();
            ctx.write_key("pure_keyword", "PureKeyword");
            ctx.write_terminal_value(&element.range);
        }

        FunctionTypeAttribute::ViewKeyword(ref element) => {
            ctx.write_connector();
            ctx.write_key("view_keyword", "ViewKeyword");
            ctx.write_terminal_value(&element.range);
        }

        FunctionTypeAttribute::PayableKeyword(ref element) => {
            ctx.write_connector();
            ctx.write_key("payable_keyword", "PayableKeyword");
            ctx.write_terminal_value(&element.range);
        }
    }
}

pub fn render_identifier_path_element(ctx: &mut RenderContext<'_>, node: &IdentifierPathElement) {
    match node {
        IdentifierPathElement::Identifier(ref element) => {
            ctx.write_connector();
            ctx.write_key("identifier", "Identifier");
            ctx.write_terminal_value(&element.range);
        }

        IdentifierPathElement::AddressKeyword(ref element) => {
            ctx.write_connector();
            ctx.write_key("address_keyword", "AddressKeyword");
            ctx.write_terminal_value(&element.range);
        }
    }
}

pub fn render_import_clause(ctx: &mut RenderContext<'_>, node: &ImportClause) {
    match node {
        ImportClause::PathImport(ref element) => {
            ctx.write_connector();
            ctx.write_key("path_import", "PathImport");
            render_path_import(ctx, element);
        }

        ImportClause::NamedImport(ref element) => {
            ctx.write_connector();
            ctx.write_key("named_import", "NamedImport");
            render_named_import(ctx, element);
        }

        ImportClause::ImportDeconstruction(ref element) => {
            ctx.write_connector();
            ctx.write_key("import_deconstruction", "ImportDeconstruction");
            render_import_deconstruction(ctx, element);
        }
    }
}

pub fn render_mapping_key_type(ctx: &mut RenderContext<'_>, node: &MappingKeyType) {
    match node {
        MappingKeyType::ElementaryType(ref element) => {
            ctx.write_connector();
            ctx.write_key("elementary_type", "ElementaryType");
            render_elementary_type(ctx, element);
        }

        MappingKeyType::IdentifierPath(ref element) => {
            ctx.write_connector();
            ctx.write_key("identifier_path", "IdentifierPath");
            render_identifier_path(ctx, element);
        }
    }
}

pub fn render_modifier_attribute(ctx: &mut RenderContext<'_>, node: &ModifierAttribute) {
    match node {
        ModifierAttribute::OverrideSpecifier(ref element) => {
            ctx.write_connector();
            ctx.write_key("override_specifier", "OverrideSpecifier");
            render_override_specifier(ctx, element);
        }

        ModifierAttribute::VirtualKeyword(ref element) => {
            ctx.write_connector();
            ctx.write_key("virtual_keyword", "VirtualKeyword");
            ctx.write_terminal_value(&element.range);
        }
    }
}

pub fn render_number_unit(ctx: &mut RenderContext<'_>, node: &NumberUnit) {
    match node {
        NumberUnit::WeiKeyword(ref element) => {
            ctx.write_connector();
            ctx.write_key("wei_keyword", "WeiKeyword");
            ctx.write_terminal_value(&element.range);
        }

        NumberUnit::GweiKeyword(ref element) => {
            ctx.write_connector();
            ctx.write_key("gwei_keyword", "GweiKeyword");
            ctx.write_terminal_value(&element.range);
        }

        NumberUnit::EtherKeyword(ref element) => {
            ctx.write_connector();
            ctx.write_key("ether_keyword", "EtherKeyword");
            ctx.write_terminal_value(&element.range);
        }

        NumberUnit::SecondsKeyword(ref element) => {
            ctx.write_connector();
            ctx.write_key("seconds_keyword", "SecondsKeyword");
            ctx.write_terminal_value(&element.range);
        }

        NumberUnit::MinutesKeyword(ref element) => {
            ctx.write_connector();
            ctx.write_key("minutes_keyword", "MinutesKeyword");
            ctx.write_terminal_value(&element.range);
        }

        NumberUnit::HoursKeyword(ref element) => {
            ctx.write_connector();
            ctx.write_key("hours_keyword", "HoursKeyword");
            ctx.write_terminal_value(&element.range);
        }

        NumberUnit::DaysKeyword(ref element) => {
            ctx.write_connector();
            ctx.write_key("days_keyword", "DaysKeyword");
            ctx.write_terminal_value(&element.range);
        }

        NumberUnit::WeeksKeyword(ref element) => {
            ctx.write_connector();
            ctx.write_key("weeks_keyword", "WeeksKeyword");
            ctx.write_terminal_value(&element.range);
        }
    }
}

pub fn render_pragma(ctx: &mut RenderContext<'_>, node: &Pragma) {
    match node {
        Pragma::VersionPragma(ref element) => {
            ctx.write_connector();
            ctx.write_key("version_pragma", "VersionPragma");
            render_version_pragma(ctx, element);
        }

        Pragma::AbicoderPragma(ref element) => {
            ctx.write_connector();
            ctx.write_key("abicoder_pragma", "AbicoderPragma");
            render_abicoder_pragma(ctx, element);
        }

        Pragma::ExperimentalPragma(ref element) => {
            ctx.write_connector();
            ctx.write_key("experimental_pragma", "ExperimentalPragma");
            render_experimental_pragma(ctx, element);
        }
    }
}

pub fn render_receive_function_attribute(
    ctx: &mut RenderContext<'_>,
    node: &ReceiveFunctionAttribute,
) {
    match node {
        ReceiveFunctionAttribute::ModifierInvocation(ref element) => {
            ctx.write_connector();
            ctx.write_key("modifier_invocation", "ModifierInvocation");
            render_modifier_invocation(ctx, element);
        }

        ReceiveFunctionAttribute::OverrideSpecifier(ref element) => {
            ctx.write_connector();
            ctx.write_key("override_specifier", "OverrideSpecifier");
            render_override_specifier(ctx, element);
        }

        ReceiveFunctionAttribute::ExternalKeyword(ref element) => {
            ctx.write_connector();
            ctx.write_key("external_keyword", "ExternalKeyword");
            ctx.write_terminal_value(&element.range);
        }

        ReceiveFunctionAttribute::PayableKeyword(ref element) => {
            ctx.write_connector();
            ctx.write_key("payable_keyword", "PayableKeyword");
            ctx.write_terminal_value(&element.range);
        }

        ReceiveFunctionAttribute::VirtualKeyword(ref element) => {
            ctx.write_connector();
            ctx.write_key("virtual_keyword", "VirtualKeyword");
            ctx.write_terminal_value(&element.range);
        }
    }
}

pub fn render_source_unit_member(ctx: &mut RenderContext<'_>, node: &SourceUnitMember) {
    match node {
        SourceUnitMember::PragmaDirective(ref element) => {
            ctx.write_connector();
            ctx.write_key("pragma_directive", "PragmaDirective");
            render_pragma_directive(ctx, element);
        }

        SourceUnitMember::ImportDirective(ref element) => {
            ctx.write_connector();
            ctx.write_key("import_directive", "ImportDirective");
            render_import_directive(ctx, element);
        }

        SourceUnitMember::ContractDefinition(ref element) => {
            ctx.write_connector();
            ctx.write_key("contract_definition", "ContractDefinition");
            render_contract_definition(ctx, element);
        }

        SourceUnitMember::InterfaceDefinition(ref element) => {
            ctx.write_connector();
            ctx.write_key("interface_definition", "InterfaceDefinition");
            render_interface_definition(ctx, element);
        }

        SourceUnitMember::LibraryDefinition(ref element) => {
            ctx.write_connector();
            ctx.write_key("library_definition", "LibraryDefinition");
            render_library_definition(ctx, element);
        }

        SourceUnitMember::StructDefinition(ref element) => {
            ctx.write_connector();
            ctx.write_key("struct_definition", "StructDefinition");
            render_struct_definition(ctx, element);
        }

        SourceUnitMember::EnumDefinition(ref element) => {
            ctx.write_connector();
            ctx.write_key("enum_definition", "EnumDefinition");
            render_enum_definition(ctx, element);
        }

        SourceUnitMember::FunctionDefinition(ref element) => {
            ctx.write_connector();
            ctx.write_key("function_definition", "FunctionDefinition");
            render_function_definition(ctx, element);
        }

        SourceUnitMember::ErrorDefinition(ref element) => {
            ctx.write_connector();
            ctx.write_key("error_definition", "ErrorDefinition");
            render_error_definition(ctx, element);
        }

        SourceUnitMember::UserDefinedValueTypeDefinition(ref element) => {
            ctx.write_connector();
            ctx.write_key(
                "user_defined_value_type_definition",
                "UserDefinedValueTypeDefinition",
            );
            render_user_defined_value_type_definition(ctx, element);
        }

        SourceUnitMember::UsingDirective(ref element) => {
            ctx.write_connector();
            ctx.write_key("using_directive", "UsingDirective");
            render_using_directive(ctx, element);
        }

        SourceUnitMember::EventDefinition(ref element) => {
            ctx.write_connector();
            ctx.write_key("event_definition", "EventDefinition");
            render_event_definition(ctx, element);
        }

        SourceUnitMember::ConstantDefinition(ref element) => {
            ctx.write_connector();
            ctx.write_key("constant_definition", "ConstantDefinition");
            render_constant_definition(ctx, element);
        }
    }
}

pub fn render_state_variable_attribute(ctx: &mut RenderContext<'_>, node: &StateVariableAttribute) {
    match node {
        StateVariableAttribute::OverrideSpecifier(ref element) => {
            ctx.write_connector();
            ctx.write_key("override_specifier", "OverrideSpecifier");
            render_override_specifier(ctx, element);
        }

        StateVariableAttribute::ConstantKeyword(ref element) => {
            ctx.write_connector();
            ctx.write_key("constant_keyword", "ConstantKeyword");
            ctx.write_terminal_value(&element.range);
        }

        StateVariableAttribute::InternalKeyword(ref element) => {
            ctx.write_connector();
            ctx.write_key("internal_keyword", "InternalKeyword");
            ctx.write_terminal_value(&element.range);
        }

        StateVariableAttribute::PrivateKeyword(ref element) => {
            ctx.write_connector();
            ctx.write_key("private_keyword", "PrivateKeyword");
            ctx.write_terminal_value(&element.range);
        }

        StateVariableAttribute::PublicKeyword(ref element) => {
            ctx.write_connector();
            ctx.write_key("public_keyword", "PublicKeyword");
            ctx.write_terminal_value(&element.range);
        }

        StateVariableAttribute::ImmutableKeyword(ref element) => {
            ctx.write_connector();
            ctx.write_key("immutable_keyword", "ImmutableKeyword");
            ctx.write_terminal_value(&element.range);
        }

        StateVariableAttribute::TransientKeyword(ref element) => {
            ctx.write_connector();
            ctx.write_key("transient_keyword", "TransientKeyword");
            ctx.write_terminal_value(&element.range);
        }
    }
}

pub fn render_statement(ctx: &mut RenderContext<'_>, node: &Statement) {
    match node {
        Statement::IfStatement(ref element) => {
            ctx.write_connector();
            ctx.write_key("if_statement", "IfStatement");
            render_if_statement(ctx, element);
        }

        Statement::ForStatement(ref element) => {
            ctx.write_connector();
            ctx.write_key("for_statement", "ForStatement");
            render_for_statement(ctx, element);
        }

        Statement::WhileStatement(ref element) => {
            ctx.write_connector();
            ctx.write_key("while_statement", "WhileStatement");
            render_while_statement(ctx, element);
        }

        Statement::DoWhileStatement(ref element) => {
            ctx.write_connector();
            ctx.write_key("do_while_statement", "DoWhileStatement");
            render_do_while_statement(ctx, element);
        }

        Statement::ContinueStatement(ref element) => {
            ctx.write_connector();
            ctx.write_key("continue_statement", "ContinueStatement");
            render_continue_statement(ctx, element);
        }

        Statement::BreakStatement(ref element) => {
            ctx.write_connector();
            ctx.write_key("break_statement", "BreakStatement");
            render_break_statement(ctx, element);
        }

        Statement::ReturnStatement(ref element) => {
            ctx.write_connector();
            ctx.write_key("return_statement", "ReturnStatement");
            render_return_statement(ctx, element);
        }

        Statement::EmitStatement(ref element) => {
            ctx.write_connector();
            ctx.write_key("emit_statement", "EmitStatement");
            render_emit_statement(ctx, element);
        }

        Statement::TryStatement(ref element) => {
            ctx.write_connector();
            ctx.write_key("try_statement", "TryStatement");
            render_try_statement(ctx, element);
        }

        Statement::RevertStatement(ref element) => {
            ctx.write_connector();
            ctx.write_key("revert_statement", "RevertStatement");
            render_revert_statement(ctx, element);
        }

        Statement::AssemblyStatement(ref element) => {
            ctx.write_connector();
            ctx.write_key("assembly_statement", "AssemblyStatement");
            render_assembly_statement(ctx, element);
        }

        Statement::Block(ref element) => {
            ctx.write_connector();
            ctx.write_key("block", "Block");
            render_block(ctx, element);
        }

        Statement::UncheckedBlock(ref element) => {
            ctx.write_connector();
            ctx.write_key("unchecked_block", "UncheckedBlock");
            render_unchecked_block(ctx, element);
        }

        Statement::VariableDeclarationStatement(ref element) => {
            ctx.write_connector();
            ctx.write_key(
                "variable_declaration_statement",
                "VariableDeclarationStatement",
            );
            render_variable_declaration_statement(ctx, element);
        }

        Statement::ExpressionStatement(ref element) => {
            ctx.write_connector();
            ctx.write_key("expression_statement", "ExpressionStatement");
            render_expression_statement(ctx, element);
        }
    }
}

pub fn render_storage_location(ctx: &mut RenderContext<'_>, node: &StorageLocation) {
    match node {
        StorageLocation::MemoryKeyword(ref element) => {
            ctx.write_connector();
            ctx.write_key("memory_keyword", "MemoryKeyword");
            ctx.write_terminal_value(&element.range);
        }

        StorageLocation::StorageKeyword(ref element) => {
            ctx.write_connector();
            ctx.write_key("storage_keyword", "StorageKeyword");
            ctx.write_terminal_value(&element.range);
        }

        StorageLocation::CallDataKeyword(ref element) => {
            ctx.write_connector();
            ctx.write_key("call_data_keyword", "CallDataKeyword");
            ctx.write_terminal_value(&element.range);
        }
    }
}

pub fn render_string_expression(ctx: &mut RenderContext<'_>, node: &StringExpression) {
    match node {
        StringExpression::StringLiterals(ref element) => {
            ctx.write_connector();
            ctx.write_key("string_literals", "StringLiterals");
            render_string_literals(ctx, element);
        }

        StringExpression::HexStringLiterals(ref element) => {
            ctx.write_connector();
            ctx.write_key("hex_string_literals", "HexStringLiterals");
            render_hex_string_literals(ctx, element);
        }

        StringExpression::UnicodeStringLiterals(ref element) => {
            ctx.write_connector();
            ctx.write_key("unicode_string_literals", "UnicodeStringLiterals");
            render_unicode_string_literals(ctx, element);
        }
    }
}

pub fn render_type_name(ctx: &mut RenderContext<'_>, node: &TypeName) {
    match node {
        TypeName::ArrayTypeName(ref element) => {
            ctx.write_connector();
            ctx.write_key("array_type_name", "ArrayTypeName");
            render_array_type_name(ctx, element);
        }

        TypeName::FunctionType(ref element) => {
            ctx.write_connector();
            ctx.write_key("function_type", "FunctionType");
            render_function_type(ctx, element);
        }

        TypeName::MappingType(ref element) => {
            ctx.write_connector();
            ctx.write_key("mapping_type", "MappingType");
            render_mapping_type(ctx, element);
        }

        TypeName::ElementaryType(ref element) => {
            ctx.write_connector();
            ctx.write_key("elementary_type", "ElementaryType");
            render_elementary_type(ctx, element);
        }

        TypeName::IdentifierPath(ref element) => {
            ctx.write_connector();
            ctx.write_key("identifier_path", "IdentifierPath");
            render_identifier_path(ctx, element);
        }
    }
}

pub fn render_using_clause(ctx: &mut RenderContext<'_>, node: &UsingClause) {
    match node {
        UsingClause::IdentifierPath(ref element) => {
            ctx.write_connector();
            ctx.write_key("identifier_path", "IdentifierPath");
            render_identifier_path(ctx, element);
        }

        UsingClause::UsingDeconstruction(ref element) => {
            ctx.write_connector();
            ctx.write_key("using_deconstruction", "UsingDeconstruction");
            render_using_deconstruction(ctx, element);
        }
    }
}

pub fn render_using_operator(ctx: &mut RenderContext<'_>, node: &UsingOperator) {
    match node {
        UsingOperator::Ampersand(ref element) => {
            ctx.write_connector();
            ctx.write_key("ampersand", "Ampersand");
            ctx.write_terminal_value(&element.range);
        }

        UsingOperator::Asterisk(ref element) => {
            ctx.write_connector();
            ctx.write_key("asterisk", "Asterisk");
            ctx.write_terminal_value(&element.range);
        }

        UsingOperator::BangEqual(ref element) => {
            ctx.write_connector();
            ctx.write_key("bang_equal", "BangEqual");
            ctx.write_terminal_value(&element.range);
        }

        UsingOperator::Bar(ref element) => {
            ctx.write_connector();
            ctx.write_key("bar", "Bar");
            ctx.write_terminal_value(&element.range);
        }

        UsingOperator::Caret(ref element) => {
            ctx.write_connector();
            ctx.write_key("caret", "Caret");
            ctx.write_terminal_value(&element.range);
        }

        UsingOperator::EqualEqual(ref element) => {
            ctx.write_connector();
            ctx.write_key("equal_equal", "EqualEqual");
            ctx.write_terminal_value(&element.range);
        }

        UsingOperator::GreaterThan(ref element) => {
            ctx.write_connector();
            ctx.write_key("greater_than", "GreaterThan");
            ctx.write_terminal_value(&element.range);
        }

        UsingOperator::GreaterThanEqual(ref element) => {
            ctx.write_connector();
            ctx.write_key("greater_than_equal", "GreaterThanEqual");
            ctx.write_terminal_value(&element.range);
        }

        UsingOperator::LessThan(ref element) => {
            ctx.write_connector();
            ctx.write_key("less_than", "LessThan");
            ctx.write_terminal_value(&element.range);
        }

        UsingOperator::LessThanEqual(ref element) => {
            ctx.write_connector();
            ctx.write_key("less_than_equal", "LessThanEqual");
            ctx.write_terminal_value(&element.range);
        }

        UsingOperator::Minus(ref element) => {
            ctx.write_connector();
            ctx.write_key("minus", "Minus");
            ctx.write_terminal_value(&element.range);
        }

        UsingOperator::Percent(ref element) => {
            ctx.write_connector();
            ctx.write_key("percent", "Percent");
            ctx.write_terminal_value(&element.range);
        }

        UsingOperator::Plus(ref element) => {
            ctx.write_connector();
            ctx.write_key("plus", "Plus");
            ctx.write_terminal_value(&element.range);
        }

        UsingOperator::Slash(ref element) => {
            ctx.write_connector();
            ctx.write_key("slash", "Slash");
            ctx.write_terminal_value(&element.range);
        }

        UsingOperator::Tilde(ref element) => {
            ctx.write_connector();
            ctx.write_key("tilde", "Tilde");
            ctx.write_terminal_value(&element.range);
        }
    }
}

pub fn render_using_target(ctx: &mut RenderContext<'_>, node: &UsingTarget) {
    match node {
        UsingTarget::TypeName(ref element) => {
            ctx.write_connector();
            ctx.write_key("type_name", "TypeName");
            render_type_name(ctx, element);
        }

        UsingTarget::Asterisk(ref element) => {
            ctx.write_connector();
            ctx.write_key("asterisk", "Asterisk");
            ctx.write_terminal_value(&element.range);
        }
    }
}

pub fn render_variable_declaration_target(
    ctx: &mut RenderContext<'_>,
    node: &VariableDeclarationTarget,
) {
    match node {
        VariableDeclarationTarget::SingleTypedDeclaration(ref element) => {
            ctx.write_connector();
            ctx.write_key("single_typed_declaration", "SingleTypedDeclaration");
            render_single_typed_declaration(ctx, element);
        }

        VariableDeclarationTarget::MultiTypedDeclaration(ref element) => {
            ctx.write_connector();
            ctx.write_key("multi_typed_declaration", "MultiTypedDeclaration");
            render_multi_typed_declaration(ctx, element);
        }
    }
}

pub fn render_version_expression(ctx: &mut RenderContext<'_>, node: &VersionExpression) {
    match node {
        VersionExpression::VersionRange(ref element) => {
            ctx.write_connector();
            ctx.write_key("version_range", "VersionRange");
            render_version_range(ctx, element);
        }

        VersionExpression::VersionTerm(ref element) => {
            ctx.write_connector();
            ctx.write_key("version_term", "VersionTerm");
            render_version_term(ctx, element);
        }
    }
}

pub fn render_version_literal(ctx: &mut RenderContext<'_>, node: &VersionLiteral) {
    match node {
        VersionLiteral::SimpleVersionLiteral(ref element) => {
            ctx.write_connector();
            ctx.write_key("simple_version_literal", "SimpleVersionLiteral");
            render_simple_version_literal(ctx, element);
        }

        VersionLiteral::PragmaStringLiteral(ref element) => {
            ctx.write_connector();
            ctx.write_key("pragma_string_literal", "PragmaStringLiteral");
            ctx.write_terminal_value(&element.range);
        }
    }
}

pub fn render_version_operator(ctx: &mut RenderContext<'_>, node: &VersionOperator) {
    match node {
        VersionOperator::PragmaCaret(ref element) => {
            ctx.write_connector();
            ctx.write_key("pragma_caret", "PragmaCaret");
            ctx.write_terminal_value(&element.range);
        }

        VersionOperator::PragmaTilde(ref element) => {
            ctx.write_connector();
            ctx.write_key("pragma_tilde", "PragmaTilde");
            ctx.write_terminal_value(&element.range);
        }

        VersionOperator::PragmaEqual(ref element) => {
            ctx.write_connector();
            ctx.write_key("pragma_equal", "PragmaEqual");
            ctx.write_terminal_value(&element.range);
        }

        VersionOperator::PragmaLessThan(ref element) => {
            ctx.write_connector();
            ctx.write_key("pragma_less_than", "PragmaLessThan");
            ctx.write_terminal_value(&element.range);
        }

        VersionOperator::PragmaGreaterThan(ref element) => {
            ctx.write_connector();
            ctx.write_key("pragma_greater_than", "PragmaGreaterThan");
            ctx.write_terminal_value(&element.range);
        }

        VersionOperator::PragmaLessThanEqual(ref element) => {
            ctx.write_connector();
            ctx.write_key("pragma_less_than_equal", "PragmaLessThanEqual");
            ctx.write_terminal_value(&element.range);
        }

        VersionOperator::PragmaGreaterThanEqual(ref element) => {
            ctx.write_connector();
            ctx.write_key("pragma_greater_than_equal", "PragmaGreaterThanEqual");
            ctx.write_terminal_value(&element.range);
        }
    }
}

pub fn render_yul_expression(ctx: &mut RenderContext<'_>, node: &YulExpression) {
    match node {
        YulExpression::YulFunctionCallExpression(ref element) => {
            ctx.write_connector();
            ctx.write_key("yul_function_call_expression", "YulFunctionCallExpression");
            render_yul_function_call_expression(ctx, element);
        }

        YulExpression::YulLiteral(ref element) => {
            ctx.write_connector();
            ctx.write_key("yul_literal", "YulLiteral");
            render_yul_literal(ctx, element);
        }

        YulExpression::YulPath(ref element) => {
            ctx.write_connector();
            ctx.write_key("yul_path", "YulPath");
            render_yul_path(ctx, element);
        }
    }
}

pub fn render_yul_literal(ctx: &mut RenderContext<'_>, node: &YulLiteral) {
    match node {
        YulLiteral::YulTrueKeyword(ref element) => {
            ctx.write_connector();
            ctx.write_key("yul_true_keyword", "YulTrueKeyword");
            ctx.write_terminal_value(&element.range);
        }

        YulLiteral::YulFalseKeyword(ref element) => {
            ctx.write_connector();
            ctx.write_key("yul_false_keyword", "YulFalseKeyword");
            ctx.write_terminal_value(&element.range);
        }

        YulLiteral::YulDecimalLiteral(ref element) => {
            ctx.write_connector();
            ctx.write_key("yul_decimal_literal", "YulDecimalLiteral");
            ctx.write_terminal_value(&element.range);
        }

        YulLiteral::YulHexLiteral(ref element) => {
            ctx.write_connector();
            ctx.write_key("yul_hex_literal", "YulHexLiteral");
            ctx.write_terminal_value(&element.range);
        }

        YulLiteral::YulHexStringLiteral(ref element) => {
            ctx.write_connector();
            ctx.write_key("yul_hex_string_literal", "YulHexStringLiteral");
            ctx.write_terminal_value(&element.range);
        }

        YulLiteral::YulStringLiteral(ref element) => {
            ctx.write_connector();
            ctx.write_key("yul_string_literal", "YulStringLiteral");
            ctx.write_terminal_value(&element.range);
        }
    }
}

pub fn render_yul_statement(ctx: &mut RenderContext<'_>, node: &YulStatement) {
    match node {
        YulStatement::YulBlock(ref element) => {
            ctx.write_connector();
            ctx.write_key("yul_block", "YulBlock");
            render_yul_block(ctx, element);
        }

        YulStatement::YulFunctionDefinition(ref element) => {
            ctx.write_connector();
            ctx.write_key("yul_function_definition", "YulFunctionDefinition");
            render_yul_function_definition(ctx, element);
        }

        YulStatement::YulIfStatement(ref element) => {
            ctx.write_connector();
            ctx.write_key("yul_if_statement", "YulIfStatement");
            render_yul_if_statement(ctx, element);
        }

        YulStatement::YulForStatement(ref element) => {
            ctx.write_connector();
            ctx.write_key("yul_for_statement", "YulForStatement");
            render_yul_for_statement(ctx, element);
        }

        YulStatement::YulSwitchStatement(ref element) => {
            ctx.write_connector();
            ctx.write_key("yul_switch_statement", "YulSwitchStatement");
            render_yul_switch_statement(ctx, element);
        }

        YulStatement::YulLeaveStatement(ref element) => {
            ctx.write_connector();
            ctx.write_key("yul_leave_statement", "YulLeaveStatement");
            render_yul_leave_statement(ctx, element);
        }

        YulStatement::YulBreakStatement(ref element) => {
            ctx.write_connector();
            ctx.write_key("yul_break_statement", "YulBreakStatement");
            render_yul_break_statement(ctx, element);
        }

        YulStatement::YulContinueStatement(ref element) => {
            ctx.write_connector();
            ctx.write_key("yul_continue_statement", "YulContinueStatement");
            render_yul_continue_statement(ctx, element);
        }

        YulStatement::YulVariableAssignmentStatement(ref element) => {
            ctx.write_connector();
            ctx.write_key(
                "yul_variable_assignment_statement",
                "YulVariableAssignmentStatement",
            );
            render_yul_variable_assignment_statement(ctx, element);
        }

        YulStatement::YulVariableDeclarationStatement(ref element) => {
            ctx.write_connector();
            ctx.write_key(
                "yul_variable_declaration_statement",
                "YulVariableDeclarationStatement",
            );
            render_yul_variable_declaration_statement(ctx, element);
        }

        YulStatement::YulExpression(ref element) => {
            ctx.write_connector();
            ctx.write_key("yul_expression", "YulExpression");
            render_yul_expression(ctx, element);
        }
    }
}

pub fn render_yul_switch_case(ctx: &mut RenderContext<'_>, node: &YulSwitchCase) {
    match node {
        YulSwitchCase::YulDefaultCase(ref element) => {
            ctx.write_connector();
            ctx.write_key("yul_default_case", "YulDefaultCase");
            render_yul_default_case(ctx, element);
        }

        YulSwitchCase::YulValueCase(ref element) => {
            ctx.write_connector();
            ctx.write_key("yul_value_case", "YulValueCase");
            render_yul_value_case(ctx, element);
        }
    }
}

//
// Repeated & Separated
//

pub fn render_array_values(ctx: &mut RenderContext<'_>, node: &ArrayValues) {
    if node.elements.is_empty() {
        ctx.write_empty_collection();
    } else {
        ctx.write_nonterminal_start();
        ctx.depth += 1;
        for element in &node.elements {
            ctx.write_indent();
            ctx.write_key("item", "Expression");

            render_expression(ctx, element);
        }
        ctx.depth -= 1;
    }
}

pub fn render_call_options(ctx: &mut RenderContext<'_>, node: &CallOptions) {
    if node.elements.is_empty() {
        ctx.write_empty_collection();
    } else {
        ctx.write_nonterminal_start();
        ctx.depth += 1;
        for element in &node.elements {
            ctx.write_indent();
            ctx.write_key("item", "NamedArgument");

            render_named_argument(ctx, element);
        }
        ctx.depth -= 1;
    }
}

pub fn render_catch_clauses(ctx: &mut RenderContext<'_>, node: &CatchClauses) {
    if node.elements.is_empty() {
        ctx.write_empty_collection();
    } else {
        ctx.write_nonterminal_start();
        ctx.depth += 1;
        for element in &node.elements {
            ctx.write_indent();
            ctx.write_key("item", "CatchClause");

            render_catch_clause(ctx, element);
        }
        ctx.depth -= 1;
    }
}

pub fn render_constructor_attributes(ctx: &mut RenderContext<'_>, node: &ConstructorAttributes) {
    if node.elements.is_empty() {
        ctx.write_empty_collection();
    } else {
        ctx.write_nonterminal_start();
        ctx.depth += 1;
        for element in &node.elements {
            ctx.write_indent();
            ctx.write_key("item", "ConstructorAttribute");

            render_constructor_attribute(ctx, element);
        }
        ctx.depth -= 1;
    }
}

pub fn render_contract_members(ctx: &mut RenderContext<'_>, node: &ContractMembers) {
    if node.elements.is_empty() {
        ctx.write_empty_collection();
    } else {
        ctx.write_nonterminal_start();
        ctx.depth += 1;
        for element in &node.elements {
            ctx.write_indent();
            ctx.write_key("item", "ContractMember");

            render_contract_member(ctx, element);
        }
        ctx.depth -= 1;
    }
}

pub fn render_contract_specifiers(ctx: &mut RenderContext<'_>, node: &ContractSpecifiers) {
    if node.elements.is_empty() {
        ctx.write_empty_collection();
    } else {
        ctx.write_nonterminal_start();
        ctx.depth += 1;
        for element in &node.elements {
            ctx.write_indent();
            ctx.write_key("item", "ContractSpecifier");

            render_contract_specifier(ctx, element);
        }
        ctx.depth -= 1;
    }
}

pub fn render_enum_members(ctx: &mut RenderContext<'_>, node: &EnumMembers) {
    if node.elements.is_empty() {
        ctx.write_empty_collection();
    } else {
        ctx.write_nonterminal_start();
        ctx.depth += 1;
        for element in &node.elements {
            ctx.write_indent();
            ctx.write_key("item", "Identifier");

            ctx.write_terminal_value(&element.range);
        }
        ctx.depth -= 1;
    }
}

pub fn render_error_parameters(ctx: &mut RenderContext<'_>, node: &ErrorParameters) {
    if node.elements.is_empty() {
        ctx.write_empty_collection();
    } else {
        ctx.write_nonterminal_start();
        ctx.depth += 1;
        for element in &node.elements {
            ctx.write_indent();
            ctx.write_key("item", "ErrorParameter");

            render_error_parameter(ctx, element);
        }
        ctx.depth -= 1;
    }
}

pub fn render_event_parameters(ctx: &mut RenderContext<'_>, node: &EventParameters) {
    if node.elements.is_empty() {
        ctx.write_empty_collection();
    } else {
        ctx.write_nonterminal_start();
        ctx.depth += 1;
        for element in &node.elements {
            ctx.write_indent();
            ctx.write_key("item", "EventParameter");

            render_event_parameter(ctx, element);
        }
        ctx.depth -= 1;
    }
}

pub fn render_fallback_function_attributes(
    ctx: &mut RenderContext<'_>,
    node: &FallbackFunctionAttributes,
) {
    if node.elements.is_empty() {
        ctx.write_empty_collection();
    } else {
        ctx.write_nonterminal_start();
        ctx.depth += 1;
        for element in &node.elements {
            ctx.write_indent();
            ctx.write_key("item", "FallbackFunctionAttribute");

            render_fallback_function_attribute(ctx, element);
        }
        ctx.depth -= 1;
    }
}

pub fn render_function_attributes(ctx: &mut RenderContext<'_>, node: &FunctionAttributes) {
    if node.elements.is_empty() {
        ctx.write_empty_collection();
    } else {
        ctx.write_nonterminal_start();
        ctx.depth += 1;
        for element in &node.elements {
            ctx.write_indent();
            ctx.write_key("item", "FunctionAttribute");

            render_function_attribute(ctx, element);
        }
        ctx.depth -= 1;
    }
}

pub fn render_function_type_attributes(ctx: &mut RenderContext<'_>, node: &FunctionTypeAttributes) {
    if node.elements.is_empty() {
        ctx.write_empty_collection();
    } else {
        ctx.write_nonterminal_start();
        ctx.depth += 1;
        for element in &node.elements {
            ctx.write_indent();
            ctx.write_key("item", "FunctionTypeAttribute");

            render_function_type_attribute(ctx, element);
        }
        ctx.depth -= 1;
    }
}

pub fn render_hex_string_literals(ctx: &mut RenderContext<'_>, node: &HexStringLiterals) {
    if node.elements.is_empty() {
        ctx.write_empty_collection();
    } else {
        ctx.write_nonterminal_start();
        ctx.depth += 1;
        for element in &node.elements {
            ctx.write_indent();
            ctx.write_key("item", "HexStringLiteral");

            ctx.write_terminal_value(&element.range);
        }
        ctx.depth -= 1;
    }
}

pub fn render_identifier_path(ctx: &mut RenderContext<'_>, node: &IdentifierPath) {
    if node.elements.is_empty() {
        ctx.write_empty_collection();
    } else {
        ctx.write_nonterminal_start();
        ctx.depth += 1;
        for element in &node.elements {
            ctx.write_indent();
            ctx.write_key("item", "IdentifierPathElement");

            render_identifier_path_element(ctx, element);
        }
        ctx.depth -= 1;
    }
}

pub fn render_import_deconstruction_symbols(
    ctx: &mut RenderContext<'_>,
    node: &ImportDeconstructionSymbols,
) {
    if node.elements.is_empty() {
        ctx.write_empty_collection();
    } else {
        ctx.write_nonterminal_start();
        ctx.depth += 1;
        for element in &node.elements {
            ctx.write_indent();
            ctx.write_key("item", "ImportDeconstructionSymbol");

            render_import_deconstruction_symbol(ctx, element);
        }
        ctx.depth -= 1;
    }
}

pub fn render_inheritance_types(ctx: &mut RenderContext<'_>, node: &InheritanceTypes) {
    if node.elements.is_empty() {
        ctx.write_empty_collection();
    } else {
        ctx.write_nonterminal_start();
        ctx.depth += 1;
        for element in &node.elements {
            ctx.write_indent();
            ctx.write_key("item", "InheritanceType");

            render_inheritance_type(ctx, element);
        }
        ctx.depth -= 1;
    }
}

pub fn render_interface_members(ctx: &mut RenderContext<'_>, node: &InterfaceMembers) {
    if node.elements.is_empty() {
        ctx.write_empty_collection();
    } else {
        ctx.write_nonterminal_start();
        ctx.depth += 1;
        for element in &node.elements {
            ctx.write_indent();
            ctx.write_key("item", "ContractMember");

            render_contract_member(ctx, element);
        }
        ctx.depth -= 1;
    }
}

pub fn render_library_members(ctx: &mut RenderContext<'_>, node: &LibraryMembers) {
    if node.elements.is_empty() {
        ctx.write_empty_collection();
    } else {
        ctx.write_nonterminal_start();
        ctx.depth += 1;
        for element in &node.elements {
            ctx.write_indent();
            ctx.write_key("item", "ContractMember");

            render_contract_member(ctx, element);
        }
        ctx.depth -= 1;
    }
}

pub fn render_modifier_attributes(ctx: &mut RenderContext<'_>, node: &ModifierAttributes) {
    if node.elements.is_empty() {
        ctx.write_empty_collection();
    } else {
        ctx.write_nonterminal_start();
        ctx.depth += 1;
        for element in &node.elements {
            ctx.write_indent();
            ctx.write_key("item", "ModifierAttribute");

            render_modifier_attribute(ctx, element);
        }
        ctx.depth -= 1;
    }
}

pub fn render_multi_typed_declaration_elements(
    ctx: &mut RenderContext<'_>,
    node: &MultiTypedDeclarationElements,
) {
    if node.elements.is_empty() {
        ctx.write_empty_collection();
    } else {
        ctx.write_nonterminal_start();
        ctx.depth += 1;
        for element in &node.elements {
            ctx.write_indent();
            ctx.write_key("item", "MultiTypedDeclarationElement");

            render_multi_typed_declaration_element(ctx, element);
        }
        ctx.depth -= 1;
    }
}

pub fn render_named_arguments(ctx: &mut RenderContext<'_>, node: &NamedArguments) {
    if node.elements.is_empty() {
        ctx.write_empty_collection();
    } else {
        ctx.write_nonterminal_start();
        ctx.depth += 1;
        for element in &node.elements {
            ctx.write_indent();
            ctx.write_key("item", "NamedArgument");

            render_named_argument(ctx, element);
        }
        ctx.depth -= 1;
    }
}

pub fn render_override_paths(ctx: &mut RenderContext<'_>, node: &OverridePaths) {
    if node.elements.is_empty() {
        ctx.write_empty_collection();
    } else {
        ctx.write_nonterminal_start();
        ctx.depth += 1;
        for element in &node.elements {
            ctx.write_indent();
            ctx.write_key("item", "IdentifierPath");

            render_identifier_path(ctx, element);
        }
        ctx.depth -= 1;
    }
}

pub fn render_parameters(ctx: &mut RenderContext<'_>, node: &Parameters) {
    if node.elements.is_empty() {
        ctx.write_empty_collection();
    } else {
        ctx.write_nonterminal_start();
        ctx.depth += 1;
        for element in &node.elements {
            ctx.write_indent();
            ctx.write_key("item", "Parameter");

            render_parameter(ctx, element);
        }
        ctx.depth -= 1;
    }
}

pub fn render_positional_arguments(ctx: &mut RenderContext<'_>, node: &PositionalArguments) {
    if node.elements.is_empty() {
        ctx.write_empty_collection();
    } else {
        ctx.write_nonterminal_start();
        ctx.depth += 1;
        for element in &node.elements {
            ctx.write_indent();
            ctx.write_key("item", "Expression");

            render_expression(ctx, element);
        }
        ctx.depth -= 1;
    }
}

pub fn render_receive_function_attributes(
    ctx: &mut RenderContext<'_>,
    node: &ReceiveFunctionAttributes,
) {
    if node.elements.is_empty() {
        ctx.write_empty_collection();
    } else {
        ctx.write_nonterminal_start();
        ctx.depth += 1;
        for element in &node.elements {
            ctx.write_indent();
            ctx.write_key("item", "ReceiveFunctionAttribute");

            render_receive_function_attribute(ctx, element);
        }
        ctx.depth -= 1;
    }
}

pub fn render_simple_version_literal(ctx: &mut RenderContext<'_>, node: &SimpleVersionLiteral) {
    if node.elements.is_empty() {
        ctx.write_empty_collection();
    } else {
        ctx.write_nonterminal_start();
        ctx.depth += 1;
        for element in &node.elements {
            ctx.write_indent();
            ctx.write_key("item", "VersionSpecifier");

            ctx.write_terminal_value(&element.range);
        }
        ctx.depth -= 1;
    }
}

pub fn render_source_unit_members(ctx: &mut RenderContext<'_>, node: &SourceUnitMembers) {
    if node.elements.is_empty() {
        ctx.write_empty_collection();
    } else {
        ctx.write_nonterminal_start();
        ctx.depth += 1;
        for element in &node.elements {
            ctx.write_indent();
            ctx.write_key("item", "SourceUnitMember");

            render_source_unit_member(ctx, element);
        }
        ctx.depth -= 1;
    }
}

pub fn render_state_variable_attributes(
    ctx: &mut RenderContext<'_>,
    node: &StateVariableAttributes,
) {
    if node.elements.is_empty() {
        ctx.write_empty_collection();
    } else {
        ctx.write_nonterminal_start();
        ctx.depth += 1;
        for element in &node.elements {
            ctx.write_indent();
            ctx.write_key("item", "StateVariableAttribute");

            render_state_variable_attribute(ctx, element);
        }
        ctx.depth -= 1;
    }
}

pub fn render_statements(ctx: &mut RenderContext<'_>, node: &Statements) {
    if node.elements.is_empty() {
        ctx.write_empty_collection();
    } else {
        ctx.write_nonterminal_start();
        ctx.depth += 1;
        for element in &node.elements {
            ctx.write_indent();
            ctx.write_key("item", "Statement");

            render_statement(ctx, element);
        }
        ctx.depth -= 1;
    }
}

pub fn render_string_literals(ctx: &mut RenderContext<'_>, node: &StringLiterals) {
    if node.elements.is_empty() {
        ctx.write_empty_collection();
    } else {
        ctx.write_nonterminal_start();
        ctx.depth += 1;
        for element in &node.elements {
            ctx.write_indent();
            ctx.write_key("item", "StringLiteral");

            ctx.write_terminal_value(&element.range);
        }
        ctx.depth -= 1;
    }
}

pub fn render_struct_members(ctx: &mut RenderContext<'_>, node: &StructMembers) {
    if node.elements.is_empty() {
        ctx.write_empty_collection();
    } else {
        ctx.write_nonterminal_start();
        ctx.depth += 1;
        for element in &node.elements {
            ctx.write_indent();
            ctx.write_key("item", "StructMember");

            render_struct_member(ctx, element);
        }
        ctx.depth -= 1;
    }
}

pub fn render_tuple_values(ctx: &mut RenderContext<'_>, node: &TupleValues) {
    if node.elements.is_empty() {
        ctx.write_empty_collection();
    } else {
        ctx.write_nonterminal_start();
        ctx.depth += 1;
        for element in &node.elements {
            ctx.write_indent();
            ctx.write_key("item", "TupleValue");

            render_tuple_value(ctx, element);
        }
        ctx.depth -= 1;
    }
}

pub fn render_unicode_string_literals(ctx: &mut RenderContext<'_>, node: &UnicodeStringLiterals) {
    if node.elements.is_empty() {
        ctx.write_empty_collection();
    } else {
        ctx.write_nonterminal_start();
        ctx.depth += 1;
        for element in &node.elements {
            ctx.write_indent();
            ctx.write_key("item", "UnicodeStringLiteral");

            ctx.write_terminal_value(&element.range);
        }
        ctx.depth -= 1;
    }
}

pub fn render_using_deconstruction_symbols(
    ctx: &mut RenderContext<'_>,
    node: &UsingDeconstructionSymbols,
) {
    if node.elements.is_empty() {
        ctx.write_empty_collection();
    } else {
        ctx.write_nonterminal_start();
        ctx.depth += 1;
        for element in &node.elements {
            ctx.write_indent();
            ctx.write_key("item", "UsingDeconstructionSymbol");

            render_using_deconstruction_symbol(ctx, element);
        }
        ctx.depth -= 1;
    }
}

pub fn render_version_expression_set(ctx: &mut RenderContext<'_>, node: &VersionExpressionSet) {
    if node.elements.is_empty() {
        ctx.write_empty_collection();
    } else {
        ctx.write_nonterminal_start();
        ctx.depth += 1;
        for element in &node.elements {
            ctx.write_indent();
            ctx.write_key("item", "VersionExpression");

            render_version_expression(ctx, element);
        }
        ctx.depth -= 1;
    }
}

pub fn render_version_expression_sets(ctx: &mut RenderContext<'_>, node: &VersionExpressionSets) {
    if node.elements.is_empty() {
        ctx.write_empty_collection();
    } else {
        ctx.write_nonterminal_start();
        ctx.depth += 1;
        for element in &node.elements {
            ctx.write_indent();
            ctx.write_key("item", "VersionExpressionSet");

            render_version_expression_set(ctx, element);
        }
        ctx.depth -= 1;
    }
}

pub fn render_yul_arguments(ctx: &mut RenderContext<'_>, node: &YulArguments) {
    if node.elements.is_empty() {
        ctx.write_empty_collection();
    } else {
        ctx.write_nonterminal_start();
        ctx.depth += 1;
        for element in &node.elements {
            ctx.write_indent();
            ctx.write_key("item", "YulExpression");

            render_yul_expression(ctx, element);
        }
        ctx.depth -= 1;
    }
}

pub fn render_yul_flags(ctx: &mut RenderContext<'_>, node: &YulFlags) {
    if node.elements.is_empty() {
        ctx.write_empty_collection();
    } else {
        ctx.write_nonterminal_start();
        ctx.depth += 1;
        for element in &node.elements {
            ctx.write_indent();
            ctx.write_key("item", "YulStringLiteral");

            ctx.write_terminal_value(&element.range);
        }
        ctx.depth -= 1;
    }
}

pub fn render_yul_parameters(ctx: &mut RenderContext<'_>, node: &YulParameters) {
    if node.elements.is_empty() {
        ctx.write_empty_collection();
    } else {
        ctx.write_nonterminal_start();
        ctx.depth += 1;
        for element in &node.elements {
            ctx.write_indent();
            ctx.write_key("item", "YulIdentifier");

            ctx.write_terminal_value(&element.range);
        }
        ctx.depth -= 1;
    }
}

pub fn render_yul_path(ctx: &mut RenderContext<'_>, node: &YulPath) {
    if node.elements.is_empty() {
        ctx.write_empty_collection();
    } else {
        ctx.write_nonterminal_start();
        ctx.depth += 1;
        for element in &node.elements {
            ctx.write_indent();
            ctx.write_key("item", "YulIdentifier");

            ctx.write_terminal_value(&element.range);
        }
        ctx.depth -= 1;
    }
}

pub fn render_yul_paths(ctx: &mut RenderContext<'_>, node: &YulPaths) {
    if node.elements.is_empty() {
        ctx.write_empty_collection();
    } else {
        ctx.write_nonterminal_start();
        ctx.depth += 1;
        for element in &node.elements {
            ctx.write_indent();
            ctx.write_key("item", "YulPath");

            render_yul_path(ctx, element);
        }
        ctx.depth -= 1;
    }
}

pub fn render_yul_statements(ctx: &mut RenderContext<'_>, node: &YulStatements) {
    if node.elements.is_empty() {
        ctx.write_empty_collection();
    } else {
        ctx.write_nonterminal_start();
        ctx.depth += 1;
        for element in &node.elements {
            ctx.write_indent();
            ctx.write_key("item", "YulStatement");

            render_yul_statement(ctx, element);
        }
        ctx.depth -= 1;
    }
}

pub fn render_yul_switch_cases(ctx: &mut RenderContext<'_>, node: &YulSwitchCases) {
    if node.elements.is_empty() {
        ctx.write_empty_collection();
    } else {
        ctx.write_nonterminal_start();
        ctx.depth += 1;
        for element in &node.elements {
            ctx.write_indent();
            ctx.write_key("item", "YulSwitchCase");

            render_yul_switch_case(ctx, element);
        }
        ctx.depth -= 1;
    }
}

pub fn render_yul_variable_names(ctx: &mut RenderContext<'_>, node: &YulVariableNames) {
    if node.elements.is_empty() {
        ctx.write_empty_collection();
    } else {
        ctx.write_nonterminal_start();
        ctx.depth += 1;
        for element in &node.elements {
            ctx.write_indent();
            ctx.write_key("item", "YulIdentifier");

            ctx.write_terminal_value(&element.range);
        }
        ctx.depth -= 1;
    }
}

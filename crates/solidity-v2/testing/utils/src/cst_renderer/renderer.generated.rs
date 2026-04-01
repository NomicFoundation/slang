// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

#![allow(clippy::too_many_lines)]
#![allow(unused_variables)]

#[allow(clippy::wildcard_imports)]
use slang_solidity_v2_cst::structured_cst::nodes::*;

use crate::cst_renderer::{render_terminal, ChildrenAccumulator, RenderedOutput};

//
// Sequences:
//

pub fn render_abicoder_pragma(
    source: &str,
    node: &AbicoderPragmaStruct,
    depth: usize,
) -> RenderedOutput {
    let mut acc = ChildrenAccumulator::new(source, depth + 1);

    acc.add(
        "abicoder_keyword",
        "AbicoderKeyword",
        render_terminal(source, &node.abicoder_keyword.range),
    );

    acc.add(
        "version",
        "AbicoderVersion",
        render_abicoder_version(source, &node.version, depth + 1),
    );

    acc.finish()
}

pub fn render_additive_expression(
    source: &str,
    node: &AdditiveExpressionStruct,
    depth: usize,
) -> RenderedOutput {
    let mut acc = ChildrenAccumulator::new(source, depth + 1);

    acc.add(
        "left_operand",
        "Expression",
        render_expression(source, &node.left_operand, depth + 1),
    );

    acc.add(
        "expression_additive_expression_operator",
        "Expression_AdditiveExpression_Operator",
        render_expression_additive_expression_operator(
            source,
            &node.expression_additive_expression_operator,
            depth + 1,
        ),
    );

    acc.add(
        "right_operand",
        "Expression",
        render_expression(source, &node.right_operand, depth + 1),
    );

    acc.finish()
}

pub fn render_address_type(source: &str, node: &AddressTypeStruct, depth: usize) -> RenderedOutput {
    let mut acc = ChildrenAccumulator::new(source, depth + 1);

    acc.add(
        "address_keyword",
        "AddressKeyword",
        render_terminal(source, &node.address_keyword.range),
    );

    if let Some(ref payable_keyword) = node.payable_keyword {
        acc.add(
            "payable_keyword",
            "PayableKeyword",
            render_terminal(source, &payable_keyword.range),
        );
    }

    acc.finish()
}

pub fn render_and_expression(
    source: &str,
    node: &AndExpressionStruct,
    depth: usize,
) -> RenderedOutput {
    let mut acc = ChildrenAccumulator::new(source, depth + 1);

    acc.add(
        "left_operand",
        "Expression",
        render_expression(source, &node.left_operand, depth + 1),
    );

    acc.add(
        "operator",
        "AmpersandAmpersand",
        render_terminal(source, &node.operator.range),
    );

    acc.add(
        "right_operand",
        "Expression",
        render_expression(source, &node.right_operand, depth + 1),
    );

    acc.finish()
}

pub fn render_array_expression(
    source: &str,
    node: &ArrayExpressionStruct,
    depth: usize,
) -> RenderedOutput {
    let mut acc = ChildrenAccumulator::new(source, depth + 1);

    acc.add(
        "open_bracket",
        "OpenBracket",
        render_terminal(source, &node.open_bracket.range),
    );

    acc.add(
        "items",
        "ArrayValues",
        render_array_values(source, &node.items, depth + 1),
    );

    acc.add(
        "close_bracket",
        "CloseBracket",
        render_terminal(source, &node.close_bracket.range),
    );

    acc.finish()
}

pub fn render_array_type_name(
    source: &str,
    node: &ArrayTypeNameStruct,
    depth: usize,
) -> RenderedOutput {
    let mut acc = ChildrenAccumulator::new(source, depth + 1);

    acc.add(
        "operand",
        "TypeName",
        render_type_name(source, &node.operand, depth + 1),
    );

    acc.add(
        "open_bracket",
        "OpenBracket",
        render_terminal(source, &node.open_bracket.range),
    );

    if let Some(ref index) = node.index {
        acc.add(
            "index",
            "Expression",
            render_expression(source, index, depth + 1),
        );
    }

    acc.add(
        "close_bracket",
        "CloseBracket",
        render_terminal(source, &node.close_bracket.range),
    );

    acc.finish()
}

pub fn render_assembly_statement(
    source: &str,
    node: &AssemblyStatementStruct,
    depth: usize,
) -> RenderedOutput {
    let mut acc = ChildrenAccumulator::new(source, depth + 1);

    acc.add(
        "assembly_keyword",
        "AssemblyKeyword",
        render_terminal(source, &node.assembly_keyword.range),
    );

    if let Some(ref label) = node.label {
        acc.add(
            "label",
            "YulStringLiteral",
            render_terminal(source, &label.range),
        );
    }

    if let Some(ref flags) = node.flags {
        acc.add(
            "flags",
            "YulFlagsDeclaration",
            render_yul_flags_declaration(source, flags, depth + 1),
        );
    }

    acc.add(
        "body",
        "YulBlock",
        render_yul_block(source, &node.body, depth + 1),
    );

    acc.finish()
}

pub fn render_assignment_expression(
    source: &str,
    node: &AssignmentExpressionStruct,
    depth: usize,
) -> RenderedOutput {
    let mut acc = ChildrenAccumulator::new(source, depth + 1);

    acc.add(
        "left_operand",
        "Expression",
        render_expression(source, &node.left_operand, depth + 1),
    );

    acc.add(
        "expression_assignment_expression_operator",
        "Expression_AssignmentExpression_Operator",
        render_expression_assignment_expression_operator(
            source,
            &node.expression_assignment_expression_operator,
            depth + 1,
        ),
    );

    acc.add(
        "right_operand",
        "Expression",
        render_expression(source, &node.right_operand, depth + 1),
    );

    acc.finish()
}

pub fn render_bitwise_and_expression(
    source: &str,
    node: &BitwiseAndExpressionStruct,
    depth: usize,
) -> RenderedOutput {
    let mut acc = ChildrenAccumulator::new(source, depth + 1);

    acc.add(
        "left_operand",
        "Expression",
        render_expression(source, &node.left_operand, depth + 1),
    );

    acc.add(
        "operator",
        "Ampersand",
        render_terminal(source, &node.operator.range),
    );

    acc.add(
        "right_operand",
        "Expression",
        render_expression(source, &node.right_operand, depth + 1),
    );

    acc.finish()
}

pub fn render_bitwise_or_expression(
    source: &str,
    node: &BitwiseOrExpressionStruct,
    depth: usize,
) -> RenderedOutput {
    let mut acc = ChildrenAccumulator::new(source, depth + 1);

    acc.add(
        "left_operand",
        "Expression",
        render_expression(source, &node.left_operand, depth + 1),
    );

    acc.add(
        "operator",
        "Bar",
        render_terminal(source, &node.operator.range),
    );

    acc.add(
        "right_operand",
        "Expression",
        render_expression(source, &node.right_operand, depth + 1),
    );

    acc.finish()
}

pub fn render_bitwise_xor_expression(
    source: &str,
    node: &BitwiseXorExpressionStruct,
    depth: usize,
) -> RenderedOutput {
    let mut acc = ChildrenAccumulator::new(source, depth + 1);

    acc.add(
        "left_operand",
        "Expression",
        render_expression(source, &node.left_operand, depth + 1),
    );

    acc.add(
        "operator",
        "Caret",
        render_terminal(source, &node.operator.range),
    );

    acc.add(
        "right_operand",
        "Expression",
        render_expression(source, &node.right_operand, depth + 1),
    );

    acc.finish()
}

pub fn render_block(source: &str, node: &BlockStruct, depth: usize) -> RenderedOutput {
    let mut acc = ChildrenAccumulator::new(source, depth + 1);

    acc.add(
        "open_brace",
        "OpenBrace",
        render_terminal(source, &node.open_brace.range),
    );

    acc.add(
        "statements",
        "Statements",
        render_statements(source, &node.statements, depth + 1),
    );

    acc.add(
        "close_brace",
        "CloseBrace",
        render_terminal(source, &node.close_brace.range),
    );

    acc.finish()
}

pub fn render_break_statement(
    source: &str,
    node: &BreakStatementStruct,
    depth: usize,
) -> RenderedOutput {
    let mut acc = ChildrenAccumulator::new(source, depth + 1);

    acc.add(
        "break_keyword",
        "BreakKeyword",
        render_terminal(source, &node.break_keyword.range),
    );

    acc.add(
        "semicolon",
        "Semicolon",
        render_terminal(source, &node.semicolon.range),
    );

    acc.finish()
}

pub fn render_call_options_expression(
    source: &str,
    node: &CallOptionsExpressionStruct,
    depth: usize,
) -> RenderedOutput {
    let mut acc = ChildrenAccumulator::new(source, depth + 1);

    acc.add(
        "operand",
        "Expression",
        render_expression(source, &node.operand, depth + 1),
    );

    acc.add(
        "open_brace",
        "OpenBrace",
        render_terminal(source, &node.open_brace.range),
    );

    acc.add(
        "options",
        "CallOptions",
        render_call_options(source, &node.options, depth + 1),
    );

    acc.add(
        "close_brace",
        "CloseBrace",
        render_terminal(source, &node.close_brace.range),
    );

    acc.finish()
}

pub fn render_catch_clause(source: &str, node: &CatchClauseStruct, depth: usize) -> RenderedOutput {
    let mut acc = ChildrenAccumulator::new(source, depth + 1);

    acc.add(
        "catch_keyword",
        "CatchKeyword",
        render_terminal(source, &node.catch_keyword.range),
    );

    if let Some(ref error) = node.error {
        acc.add(
            "error",
            "CatchClauseError",
            render_catch_clause_error(source, error, depth + 1),
        );
    }

    acc.add("body", "Block", render_block(source, &node.body, depth + 1));

    acc.finish()
}

pub fn render_catch_clause_error(
    source: &str,
    node: &CatchClauseErrorStruct,
    depth: usize,
) -> RenderedOutput {
    let mut acc = ChildrenAccumulator::new(source, depth + 1);

    if let Some(ref name) = node.name {
        acc.add("name", "Identifier", render_terminal(source, &name.range));
    }

    acc.add(
        "parameters",
        "ParametersDeclaration",
        render_parameters_declaration(source, &node.parameters, depth + 1),
    );

    acc.finish()
}

pub fn render_conditional_expression(
    source: &str,
    node: &ConditionalExpressionStruct,
    depth: usize,
) -> RenderedOutput {
    let mut acc = ChildrenAccumulator::new(source, depth + 1);

    acc.add(
        "operand",
        "Expression",
        render_expression(source, &node.operand, depth + 1),
    );

    acc.add(
        "question_mark",
        "QuestionMark",
        render_terminal(source, &node.question_mark.range),
    );

    acc.add(
        "true_expression",
        "Expression",
        render_expression(source, &node.true_expression, depth + 1),
    );

    acc.add("colon", "Colon", render_terminal(source, &node.colon.range));

    acc.add(
        "false_expression",
        "Expression",
        render_expression(source, &node.false_expression, depth + 1),
    );

    acc.finish()
}

pub fn render_constant_definition(
    source: &str,
    node: &ConstantDefinitionStruct,
    depth: usize,
) -> RenderedOutput {
    let mut acc = ChildrenAccumulator::new(source, depth + 1);

    acc.add(
        "type_name",
        "TypeName",
        render_type_name(source, &node.type_name, depth + 1),
    );

    acc.add(
        "constant_keyword",
        "ConstantKeyword",
        render_terminal(source, &node.constant_keyword.range),
    );

    acc.add(
        "name",
        "Identifier",
        render_terminal(source, &node.name.range),
    );

    acc.add("equal", "Equal", render_terminal(source, &node.equal.range));

    acc.add(
        "value",
        "Expression",
        render_expression(source, &node.value, depth + 1),
    );

    acc.add(
        "semicolon",
        "Semicolon",
        render_terminal(source, &node.semicolon.range),
    );

    acc.finish()
}

pub fn render_constructor_definition(
    source: &str,
    node: &ConstructorDefinitionStruct,
    depth: usize,
) -> RenderedOutput {
    let mut acc = ChildrenAccumulator::new(source, depth + 1);

    acc.add(
        "constructor_keyword",
        "ConstructorKeyword",
        render_terminal(source, &node.constructor_keyword.range),
    );

    acc.add(
        "parameters",
        "ParametersDeclaration",
        render_parameters_declaration(source, &node.parameters, depth + 1),
    );

    acc.add(
        "attributes",
        "ConstructorAttributes",
        render_constructor_attributes(source, &node.attributes, depth + 1),
    );

    acc.add("body", "Block", render_block(source, &node.body, depth + 1));

    acc.finish()
}

pub fn render_continue_statement(
    source: &str,
    node: &ContinueStatementStruct,
    depth: usize,
) -> RenderedOutput {
    let mut acc = ChildrenAccumulator::new(source, depth + 1);

    acc.add(
        "continue_keyword",
        "ContinueKeyword",
        render_terminal(source, &node.continue_keyword.range),
    );

    acc.add(
        "semicolon",
        "Semicolon",
        render_terminal(source, &node.semicolon.range),
    );

    acc.finish()
}

pub fn render_contract_definition(
    source: &str,
    node: &ContractDefinitionStruct,
    depth: usize,
) -> RenderedOutput {
    let mut acc = ChildrenAccumulator::new(source, depth + 1);

    if let Some(ref abstract_keyword) = node.abstract_keyword {
        acc.add(
            "abstract_keyword",
            "AbstractKeyword",
            render_terminal(source, &abstract_keyword.range),
        );
    }

    acc.add(
        "contract_keyword",
        "ContractKeyword",
        render_terminal(source, &node.contract_keyword.range),
    );

    acc.add(
        "name",
        "Identifier",
        render_terminal(source, &node.name.range),
    );

    acc.add(
        "specifiers",
        "ContractSpecifiers",
        render_contract_specifiers(source, &node.specifiers, depth + 1),
    );

    acc.add(
        "open_brace",
        "OpenBrace",
        render_terminal(source, &node.open_brace.range),
    );

    acc.add(
        "members",
        "ContractMembers",
        render_contract_members(source, &node.members, depth + 1),
    );

    acc.add(
        "close_brace",
        "CloseBrace",
        render_terminal(source, &node.close_brace.range),
    );

    acc.finish()
}

pub fn render_decimal_number_expression(
    source: &str,
    node: &DecimalNumberExpressionStruct,
    depth: usize,
) -> RenderedOutput {
    let mut acc = ChildrenAccumulator::new(source, depth + 1);

    acc.add(
        "literal",
        "DecimalLiteral",
        render_terminal(source, &node.literal.range),
    );

    if let Some(ref unit) = node.unit {
        acc.add(
            "unit",
            "NumberUnit",
            render_number_unit(source, unit, depth + 1),
        );
    }

    acc.finish()
}

pub fn render_do_while_statement(
    source: &str,
    node: &DoWhileStatementStruct,
    depth: usize,
) -> RenderedOutput {
    let mut acc = ChildrenAccumulator::new(source, depth + 1);

    acc.add(
        "do_keyword",
        "DoKeyword",
        render_terminal(source, &node.do_keyword.range),
    );

    acc.add(
        "body",
        "Statement",
        render_statement(source, &node.body, depth + 1),
    );

    acc.add(
        "while_keyword",
        "WhileKeyword",
        render_terminal(source, &node.while_keyword.range),
    );

    acc.add(
        "open_paren",
        "OpenParen",
        render_terminal(source, &node.open_paren.range),
    );

    acc.add(
        "condition",
        "Expression",
        render_expression(source, &node.condition, depth + 1),
    );

    acc.add(
        "close_paren",
        "CloseParen",
        render_terminal(source, &node.close_paren.range),
    );

    acc.add(
        "semicolon",
        "Semicolon",
        render_terminal(source, &node.semicolon.range),
    );

    acc.finish()
}

pub fn render_else_branch(source: &str, node: &ElseBranchStruct, depth: usize) -> RenderedOutput {
    let mut acc = ChildrenAccumulator::new(source, depth + 1);

    acc.add(
        "else_keyword",
        "ElseKeyword",
        render_terminal(source, &node.else_keyword.range),
    );

    acc.add(
        "body",
        "Statement",
        render_statement(source, &node.body, depth + 1),
    );

    acc.finish()
}

pub fn render_emit_statement(
    source: &str,
    node: &EmitStatementStruct,
    depth: usize,
) -> RenderedOutput {
    let mut acc = ChildrenAccumulator::new(source, depth + 1);

    acc.add(
        "emit_keyword",
        "EmitKeyword",
        render_terminal(source, &node.emit_keyword.range),
    );

    acc.add(
        "event",
        "IdentifierPath",
        render_identifier_path(source, &node.event, depth + 1),
    );

    acc.add(
        "arguments",
        "ArgumentsDeclaration",
        render_arguments_declaration(source, &node.arguments, depth + 1),
    );

    acc.add(
        "semicolon",
        "Semicolon",
        render_terminal(source, &node.semicolon.range),
    );

    acc.finish()
}

pub fn render_enum_definition(
    source: &str,
    node: &EnumDefinitionStruct,
    depth: usize,
) -> RenderedOutput {
    let mut acc = ChildrenAccumulator::new(source, depth + 1);

    acc.add(
        "enum_keyword",
        "EnumKeyword",
        render_terminal(source, &node.enum_keyword.range),
    );

    acc.add(
        "name",
        "Identifier",
        render_terminal(source, &node.name.range),
    );

    acc.add(
        "open_brace",
        "OpenBrace",
        render_terminal(source, &node.open_brace.range),
    );

    acc.add(
        "members",
        "EnumMembers",
        render_enum_members(source, &node.members, depth + 1),
    );

    acc.add(
        "close_brace",
        "CloseBrace",
        render_terminal(source, &node.close_brace.range),
    );

    acc.finish()
}

pub fn render_equality_expression(
    source: &str,
    node: &EqualityExpressionStruct,
    depth: usize,
) -> RenderedOutput {
    let mut acc = ChildrenAccumulator::new(source, depth + 1);

    acc.add(
        "left_operand",
        "Expression",
        render_expression(source, &node.left_operand, depth + 1),
    );

    acc.add(
        "expression_equality_expression_operator",
        "Expression_EqualityExpression_Operator",
        render_expression_equality_expression_operator(
            source,
            &node.expression_equality_expression_operator,
            depth + 1,
        ),
    );

    acc.add(
        "right_operand",
        "Expression",
        render_expression(source, &node.right_operand, depth + 1),
    );

    acc.finish()
}

pub fn render_error_definition(
    source: &str,
    node: &ErrorDefinitionStruct,
    depth: usize,
) -> RenderedOutput {
    let mut acc = ChildrenAccumulator::new(source, depth + 1);

    acc.add(
        "error_keyword",
        "ErrorKeyword",
        render_terminal(source, &node.error_keyword.range),
    );

    acc.add(
        "name",
        "Identifier",
        render_terminal(source, &node.name.range),
    );

    acc.add(
        "members",
        "ErrorParametersDeclaration",
        render_error_parameters_declaration(source, &node.members, depth + 1),
    );

    acc.add(
        "semicolon",
        "Semicolon",
        render_terminal(source, &node.semicolon.range),
    );

    acc.finish()
}

pub fn render_error_parameter(
    source: &str,
    node: &ErrorParameterStruct,
    depth: usize,
) -> RenderedOutput {
    let mut acc = ChildrenAccumulator::new(source, depth + 1);

    acc.add(
        "type_name",
        "TypeName",
        render_type_name(source, &node.type_name, depth + 1),
    );

    if let Some(ref name) = node.name {
        acc.add("name", "Identifier", render_terminal(source, &name.range));
    }

    acc.finish()
}

pub fn render_error_parameters_declaration(
    source: &str,
    node: &ErrorParametersDeclarationStruct,
    depth: usize,
) -> RenderedOutput {
    let mut acc = ChildrenAccumulator::new(source, depth + 1);

    acc.add(
        "open_paren",
        "OpenParen",
        render_terminal(source, &node.open_paren.range),
    );

    acc.add(
        "parameters",
        "ErrorParameters",
        render_error_parameters(source, &node.parameters, depth + 1),
    );

    acc.add(
        "close_paren",
        "CloseParen",
        render_terminal(source, &node.close_paren.range),
    );

    acc.finish()
}

pub fn render_event_definition(
    source: &str,
    node: &EventDefinitionStruct,
    depth: usize,
) -> RenderedOutput {
    let mut acc = ChildrenAccumulator::new(source, depth + 1);

    acc.add(
        "event_keyword",
        "EventKeyword",
        render_terminal(source, &node.event_keyword.range),
    );

    acc.add(
        "name",
        "Identifier",
        render_terminal(source, &node.name.range),
    );

    acc.add(
        "parameters",
        "EventParametersDeclaration",
        render_event_parameters_declaration(source, &node.parameters, depth + 1),
    );

    if let Some(ref anonymous_keyword) = node.anonymous_keyword {
        acc.add(
            "anonymous_keyword",
            "AnonymousKeyword",
            render_terminal(source, &anonymous_keyword.range),
        );
    }

    acc.add(
        "semicolon",
        "Semicolon",
        render_terminal(source, &node.semicolon.range),
    );

    acc.finish()
}

pub fn render_event_parameter(
    source: &str,
    node: &EventParameterStruct,
    depth: usize,
) -> RenderedOutput {
    let mut acc = ChildrenAccumulator::new(source, depth + 1);

    acc.add(
        "type_name",
        "TypeName",
        render_type_name(source, &node.type_name, depth + 1),
    );

    if let Some(ref indexed_keyword) = node.indexed_keyword {
        acc.add(
            "indexed_keyword",
            "IndexedKeyword",
            render_terminal(source, &indexed_keyword.range),
        );
    }

    if let Some(ref name) = node.name {
        acc.add("name", "Identifier", render_terminal(source, &name.range));
    }

    acc.finish()
}

pub fn render_event_parameters_declaration(
    source: &str,
    node: &EventParametersDeclarationStruct,
    depth: usize,
) -> RenderedOutput {
    let mut acc = ChildrenAccumulator::new(source, depth + 1);

    acc.add(
        "open_paren",
        "OpenParen",
        render_terminal(source, &node.open_paren.range),
    );

    acc.add(
        "parameters",
        "EventParameters",
        render_event_parameters(source, &node.parameters, depth + 1),
    );

    acc.add(
        "close_paren",
        "CloseParen",
        render_terminal(source, &node.close_paren.range),
    );

    acc.finish()
}

pub fn render_experimental_pragma(
    source: &str,
    node: &ExperimentalPragmaStruct,
    depth: usize,
) -> RenderedOutput {
    let mut acc = ChildrenAccumulator::new(source, depth + 1);

    acc.add(
        "experimental_keyword",
        "ExperimentalKeyword",
        render_terminal(source, &node.experimental_keyword.range),
    );

    acc.add(
        "feature",
        "ExperimentalFeature",
        render_experimental_feature(source, &node.feature, depth + 1),
    );

    acc.finish()
}

pub fn render_exponentiation_expression(
    source: &str,
    node: &ExponentiationExpressionStruct,
    depth: usize,
) -> RenderedOutput {
    let mut acc = ChildrenAccumulator::new(source, depth + 1);

    acc.add(
        "left_operand",
        "Expression",
        render_expression(source, &node.left_operand, depth + 1),
    );

    acc.add(
        "operator",
        "AsteriskAsterisk",
        render_terminal(source, &node.operator.range),
    );

    acc.add(
        "right_operand",
        "Expression",
        render_expression(source, &node.right_operand, depth + 1),
    );

    acc.finish()
}

pub fn render_expression_statement(
    source: &str,
    node: &ExpressionStatementStruct,
    depth: usize,
) -> RenderedOutput {
    let mut acc = ChildrenAccumulator::new(source, depth + 1);

    acc.add(
        "expression",
        "Expression",
        render_expression(source, &node.expression, depth + 1),
    );

    acc.add(
        "semicolon",
        "Semicolon",
        render_terminal(source, &node.semicolon.range),
    );

    acc.finish()
}

pub fn render_fallback_function_definition(
    source: &str,
    node: &FallbackFunctionDefinitionStruct,
    depth: usize,
) -> RenderedOutput {
    let mut acc = ChildrenAccumulator::new(source, depth + 1);

    acc.add(
        "fallback_keyword",
        "FallbackKeyword",
        render_terminal(source, &node.fallback_keyword.range),
    );

    acc.add(
        "parameters",
        "ParametersDeclaration",
        render_parameters_declaration(source, &node.parameters, depth + 1),
    );

    acc.add(
        "attributes",
        "FallbackFunctionAttributes",
        render_fallback_function_attributes(source, &node.attributes, depth + 1),
    );

    if let Some(ref returns) = node.returns {
        acc.add(
            "returns",
            "ReturnsDeclaration",
            render_returns_declaration(source, returns, depth + 1),
        );
    }

    acc.add(
        "body",
        "FunctionBody",
        render_function_body(source, &node.body, depth + 1),
    );

    acc.finish()
}

pub fn render_for_statement(
    source: &str,
    node: &ForStatementStruct,
    depth: usize,
) -> RenderedOutput {
    let mut acc = ChildrenAccumulator::new(source, depth + 1);

    acc.add(
        "for_keyword",
        "ForKeyword",
        render_terminal(source, &node.for_keyword.range),
    );

    acc.add(
        "open_paren",
        "OpenParen",
        render_terminal(source, &node.open_paren.range),
    );

    acc.add(
        "initialization",
        "ForStatementInitialization",
        render_for_statement_initialization(source, &node.initialization, depth + 1),
    );

    acc.add(
        "condition",
        "ForStatementCondition",
        render_for_statement_condition(source, &node.condition, depth + 1),
    );

    if let Some(ref iterator) = node.iterator {
        acc.add(
            "iterator",
            "Expression",
            render_expression(source, iterator, depth + 1),
        );
    }

    acc.add(
        "close_paren",
        "CloseParen",
        render_terminal(source, &node.close_paren.range),
    );

    acc.add(
        "body",
        "Statement",
        render_statement(source, &node.body, depth + 1),
    );

    acc.finish()
}

pub fn render_function_call_expression(
    source: &str,
    node: &FunctionCallExpressionStruct,
    depth: usize,
) -> RenderedOutput {
    let mut acc = ChildrenAccumulator::new(source, depth + 1);

    acc.add(
        "operand",
        "Expression",
        render_expression(source, &node.operand, depth + 1),
    );

    acc.add(
        "arguments",
        "ArgumentsDeclaration",
        render_arguments_declaration(source, &node.arguments, depth + 1),
    );

    acc.finish()
}

pub fn render_function_definition(
    source: &str,
    node: &FunctionDefinitionStruct,
    depth: usize,
) -> RenderedOutput {
    let mut acc = ChildrenAccumulator::new(source, depth + 1);

    acc.add(
        "function_keyword",
        "FunctionKeyword",
        render_terminal(source, &node.function_keyword.range),
    );

    acc.add(
        "name",
        "FunctionName",
        render_function_name(source, &node.name, depth + 1),
    );

    acc.add(
        "parameters",
        "ParametersDeclaration",
        render_parameters_declaration(source, &node.parameters, depth + 1),
    );

    acc.add(
        "attributes",
        "FunctionAttributes",
        render_function_attributes(source, &node.attributes, depth + 1),
    );

    if let Some(ref returns) = node.returns {
        acc.add(
            "returns",
            "ReturnsDeclaration",
            render_returns_declaration(source, returns, depth + 1),
        );
    }

    acc.add(
        "body",
        "FunctionBody",
        render_function_body(source, &node.body, depth + 1),
    );

    acc.finish()
}

pub fn render_function_type(
    source: &str,
    node: &FunctionTypeStruct,
    depth: usize,
) -> RenderedOutput {
    let mut acc = ChildrenAccumulator::new(source, depth + 1);

    acc.add(
        "function_keyword",
        "FunctionKeyword",
        render_terminal(source, &node.function_keyword.range),
    );

    acc.add(
        "parameters",
        "ParametersDeclaration",
        render_parameters_declaration(source, &node.parameters, depth + 1),
    );

    acc.add(
        "attributes",
        "FunctionTypeAttributes",
        render_function_type_attributes(source, &node.attributes, depth + 1),
    );

    if let Some(ref returns) = node.returns {
        acc.add(
            "returns",
            "ReturnsDeclaration",
            render_returns_declaration(source, returns, depth + 1),
        );
    }

    acc.finish()
}

pub fn render_hex_number_expression(
    source: &str,
    node: &HexNumberExpressionStruct,
    depth: usize,
) -> RenderedOutput {
    let mut acc = ChildrenAccumulator::new(source, depth + 1);

    acc.add(
        "literal",
        "HexLiteral",
        render_terminal(source, &node.literal.range),
    );

    acc.finish()
}

pub fn render_if_statement(source: &str, node: &IfStatementStruct, depth: usize) -> RenderedOutput {
    let mut acc = ChildrenAccumulator::new(source, depth + 1);

    acc.add(
        "if_keyword",
        "IfKeyword",
        render_terminal(source, &node.if_keyword.range),
    );

    acc.add(
        "open_paren",
        "OpenParen",
        render_terminal(source, &node.open_paren.range),
    );

    acc.add(
        "condition",
        "Expression",
        render_expression(source, &node.condition, depth + 1),
    );

    acc.add(
        "close_paren",
        "CloseParen",
        render_terminal(source, &node.close_paren.range),
    );

    acc.add(
        "body",
        "Statement",
        render_statement(source, &node.body, depth + 1),
    );

    if let Some(ref else_branch) = node.else_branch {
        acc.add(
            "else_branch",
            "ElseBranch",
            render_else_branch(source, else_branch, depth + 1),
        );
    }

    acc.finish()
}

pub fn render_import_alias(source: &str, node: &ImportAliasStruct, depth: usize) -> RenderedOutput {
    let mut acc = ChildrenAccumulator::new(source, depth + 1);

    acc.add(
        "as_keyword",
        "AsKeyword",
        render_terminal(source, &node.as_keyword.range),
    );

    acc.add(
        "identifier",
        "Identifier",
        render_terminal(source, &node.identifier.range),
    );

    acc.finish()
}

pub fn render_import_deconstruction(
    source: &str,
    node: &ImportDeconstructionStruct,
    depth: usize,
) -> RenderedOutput {
    let mut acc = ChildrenAccumulator::new(source, depth + 1);

    acc.add(
        "open_brace",
        "OpenBrace",
        render_terminal(source, &node.open_brace.range),
    );

    acc.add(
        "symbols",
        "ImportDeconstructionSymbols",
        render_import_deconstruction_symbols(source, &node.symbols, depth + 1),
    );

    acc.add(
        "close_brace",
        "CloseBrace",
        render_terminal(source, &node.close_brace.range),
    );

    acc.add(
        "from_keyword",
        "FromKeyword",
        render_terminal(source, &node.from_keyword.range),
    );

    acc.add(
        "path",
        "StringLiteral",
        render_terminal(source, &node.path.range),
    );

    acc.finish()
}

pub fn render_import_deconstruction_symbol(
    source: &str,
    node: &ImportDeconstructionSymbolStruct,
    depth: usize,
) -> RenderedOutput {
    let mut acc = ChildrenAccumulator::new(source, depth + 1);

    acc.add(
        "name",
        "Identifier",
        render_terminal(source, &node.name.range),
    );

    if let Some(ref alias) = node.alias {
        acc.add(
            "alias",
            "ImportAlias",
            render_import_alias(source, alias, depth + 1),
        );
    }

    acc.finish()
}

pub fn render_import_directive(
    source: &str,
    node: &ImportDirectiveStruct,
    depth: usize,
) -> RenderedOutput {
    let mut acc = ChildrenAccumulator::new(source, depth + 1);

    acc.add(
        "import_keyword",
        "ImportKeyword",
        render_terminal(source, &node.import_keyword.range),
    );

    acc.add(
        "clause",
        "ImportClause",
        render_import_clause(source, &node.clause, depth + 1),
    );

    acc.add(
        "semicolon",
        "Semicolon",
        render_terminal(source, &node.semicolon.range),
    );

    acc.finish()
}

pub fn render_index_access_end(
    source: &str,
    node: &IndexAccessEndStruct,
    depth: usize,
) -> RenderedOutput {
    let mut acc = ChildrenAccumulator::new(source, depth + 1);

    acc.add("colon", "Colon", render_terminal(source, &node.colon.range));

    if let Some(ref end) = node.end {
        acc.add(
            "end",
            "Expression",
            render_expression(source, end, depth + 1),
        );
    }

    acc.finish()
}

pub fn render_index_access_expression(
    source: &str,
    node: &IndexAccessExpressionStruct,
    depth: usize,
) -> RenderedOutput {
    let mut acc = ChildrenAccumulator::new(source, depth + 1);

    acc.add(
        "operand",
        "Expression",
        render_expression(source, &node.operand, depth + 1),
    );

    acc.add(
        "open_bracket",
        "OpenBracket",
        render_terminal(source, &node.open_bracket.range),
    );

    if let Some(ref start) = node.start {
        acc.add(
            "start",
            "Expression",
            render_expression(source, start, depth + 1),
        );
    }

    if let Some(ref end) = node.end {
        acc.add(
            "end",
            "IndexAccessEnd",
            render_index_access_end(source, end, depth + 1),
        );
    }

    acc.add(
        "close_bracket",
        "CloseBracket",
        render_terminal(source, &node.close_bracket.range),
    );

    acc.finish()
}

pub fn render_inequality_expression(
    source: &str,
    node: &InequalityExpressionStruct,
    depth: usize,
) -> RenderedOutput {
    let mut acc = ChildrenAccumulator::new(source, depth + 1);

    acc.add(
        "left_operand",
        "Expression",
        render_expression(source, &node.left_operand, depth + 1),
    );

    acc.add(
        "expression_inequality_expression_operator",
        "Expression_InequalityExpression_Operator",
        render_expression_inequality_expression_operator(
            source,
            &node.expression_inequality_expression_operator,
            depth + 1,
        ),
    );

    acc.add(
        "right_operand",
        "Expression",
        render_expression(source, &node.right_operand, depth + 1),
    );

    acc.finish()
}

pub fn render_inheritance_specifier(
    source: &str,
    node: &InheritanceSpecifierStruct,
    depth: usize,
) -> RenderedOutput {
    let mut acc = ChildrenAccumulator::new(source, depth + 1);

    acc.add(
        "is_keyword",
        "IsKeyword",
        render_terminal(source, &node.is_keyword.range),
    );

    acc.add(
        "types",
        "InheritanceTypes",
        render_inheritance_types(source, &node.types, depth + 1),
    );

    acc.finish()
}

pub fn render_inheritance_type(
    source: &str,
    node: &InheritanceTypeStruct,
    depth: usize,
) -> RenderedOutput {
    let mut acc = ChildrenAccumulator::new(source, depth + 1);

    acc.add(
        "type_name",
        "IdentifierPath",
        render_identifier_path(source, &node.type_name, depth + 1),
    );

    if let Some(ref arguments) = node.arguments {
        acc.add(
            "arguments",
            "ArgumentsDeclaration",
            render_arguments_declaration(source, arguments, depth + 1),
        );
    }

    acc.finish()
}

pub fn render_interface_definition(
    source: &str,
    node: &InterfaceDefinitionStruct,
    depth: usize,
) -> RenderedOutput {
    let mut acc = ChildrenAccumulator::new(source, depth + 1);

    acc.add(
        "interface_keyword",
        "InterfaceKeyword",
        render_terminal(source, &node.interface_keyword.range),
    );

    acc.add(
        "name",
        "Identifier",
        render_terminal(source, &node.name.range),
    );

    if let Some(ref inheritance) = node.inheritance {
        acc.add(
            "inheritance",
            "InheritanceSpecifier",
            render_inheritance_specifier(source, inheritance, depth + 1),
        );
    }

    acc.add(
        "open_brace",
        "OpenBrace",
        render_terminal(source, &node.open_brace.range),
    );

    acc.add(
        "members",
        "InterfaceMembers",
        render_interface_members(source, &node.members, depth + 1),
    );

    acc.add(
        "close_brace",
        "CloseBrace",
        render_terminal(source, &node.close_brace.range),
    );

    acc.finish()
}

pub fn render_library_definition(
    source: &str,
    node: &LibraryDefinitionStruct,
    depth: usize,
) -> RenderedOutput {
    let mut acc = ChildrenAccumulator::new(source, depth + 1);

    acc.add(
        "library_keyword",
        "LibraryKeyword",
        render_terminal(source, &node.library_keyword.range),
    );

    acc.add(
        "name",
        "Identifier",
        render_terminal(source, &node.name.range),
    );

    acc.add(
        "open_brace",
        "OpenBrace",
        render_terminal(source, &node.open_brace.range),
    );

    acc.add(
        "members",
        "LibraryMembers",
        render_library_members(source, &node.members, depth + 1),
    );

    acc.add(
        "close_brace",
        "CloseBrace",
        render_terminal(source, &node.close_brace.range),
    );

    acc.finish()
}

pub fn render_mapping_key(source: &str, node: &MappingKeyStruct, depth: usize) -> RenderedOutput {
    let mut acc = ChildrenAccumulator::new(source, depth + 1);

    acc.add(
        "key_type",
        "MappingKeyType",
        render_mapping_key_type(source, &node.key_type, depth + 1),
    );

    if let Some(ref name) = node.name {
        acc.add("name", "Identifier", render_terminal(source, &name.range));
    }

    acc.finish()
}

pub fn render_mapping_type(source: &str, node: &MappingTypeStruct, depth: usize) -> RenderedOutput {
    let mut acc = ChildrenAccumulator::new(source, depth + 1);

    acc.add(
        "mapping_keyword",
        "MappingKeyword",
        render_terminal(source, &node.mapping_keyword.range),
    );

    acc.add(
        "open_paren",
        "OpenParen",
        render_terminal(source, &node.open_paren.range),
    );

    acc.add(
        "key_type",
        "MappingKey",
        render_mapping_key(source, &node.key_type, depth + 1),
    );

    acc.add(
        "equal_greater_than",
        "EqualGreaterThan",
        render_terminal(source, &node.equal_greater_than.range),
    );

    acc.add(
        "value_type",
        "MappingValue",
        render_mapping_value(source, &node.value_type, depth + 1),
    );

    acc.add(
        "close_paren",
        "CloseParen",
        render_terminal(source, &node.close_paren.range),
    );

    acc.finish()
}

pub fn render_mapping_value(
    source: &str,
    node: &MappingValueStruct,
    depth: usize,
) -> RenderedOutput {
    let mut acc = ChildrenAccumulator::new(source, depth + 1);

    acc.add(
        "type_name",
        "TypeName",
        render_type_name(source, &node.type_name, depth + 1),
    );

    if let Some(ref name) = node.name {
        acc.add("name", "Identifier", render_terminal(source, &name.range));
    }

    acc.finish()
}

pub fn render_member_access_expression(
    source: &str,
    node: &MemberAccessExpressionStruct,
    depth: usize,
) -> RenderedOutput {
    let mut acc = ChildrenAccumulator::new(source, depth + 1);

    acc.add(
        "operand",
        "Expression",
        render_expression(source, &node.operand, depth + 1),
    );

    acc.add(
        "period",
        "Period",
        render_terminal(source, &node.period.range),
    );

    acc.add(
        "member",
        "IdentifierPathElement",
        render_identifier_path_element(source, &node.member, depth + 1),
    );

    acc.finish()
}

pub fn render_modifier_definition(
    source: &str,
    node: &ModifierDefinitionStruct,
    depth: usize,
) -> RenderedOutput {
    let mut acc = ChildrenAccumulator::new(source, depth + 1);

    acc.add(
        "modifier_keyword",
        "ModifierKeyword",
        render_terminal(source, &node.modifier_keyword.range),
    );

    acc.add(
        "name",
        "Identifier",
        render_terminal(source, &node.name.range),
    );

    if let Some(ref parameters) = node.parameters {
        acc.add(
            "parameters",
            "ParametersDeclaration",
            render_parameters_declaration(source, parameters, depth + 1),
        );
    }

    acc.add(
        "attributes",
        "ModifierAttributes",
        render_modifier_attributes(source, &node.attributes, depth + 1),
    );

    acc.add(
        "body",
        "FunctionBody",
        render_function_body(source, &node.body, depth + 1),
    );

    acc.finish()
}

pub fn render_modifier_invocation(
    source: &str,
    node: &ModifierInvocationStruct,
    depth: usize,
) -> RenderedOutput {
    let mut acc = ChildrenAccumulator::new(source, depth + 1);

    acc.add(
        "name",
        "IdentifierPath",
        render_identifier_path(source, &node.name, depth + 1),
    );

    if let Some(ref arguments) = node.arguments {
        acc.add(
            "arguments",
            "ArgumentsDeclaration",
            render_arguments_declaration(source, arguments, depth + 1),
        );
    }

    acc.finish()
}

pub fn render_multi_typed_declaration(
    source: &str,
    node: &MultiTypedDeclarationStruct,
    depth: usize,
) -> RenderedOutput {
    let mut acc = ChildrenAccumulator::new(source, depth + 1);

    acc.add(
        "open_paren",
        "OpenParen",
        render_terminal(source, &node.open_paren.range),
    );

    acc.add(
        "elements",
        "MultiTypedDeclarationElements",
        render_multi_typed_declaration_elements(source, &node.elements, depth + 1),
    );

    acc.add(
        "close_paren",
        "CloseParen",
        render_terminal(source, &node.close_paren.range),
    );

    acc.add(
        "value",
        "VariableDeclarationValue",
        render_variable_declaration_value(source, &node.value, depth + 1),
    );

    acc.finish()
}

pub fn render_multi_typed_declaration_element(
    source: &str,
    node: &MultiTypedDeclarationElementStruct,
    depth: usize,
) -> RenderedOutput {
    let mut acc = ChildrenAccumulator::new(source, depth + 1);

    if let Some(ref member) = node.member {
        acc.add(
            "member",
            "VariableDeclaration",
            render_variable_declaration(source, member, depth + 1),
        );
    }

    acc.finish()
}

pub fn render_multiplicative_expression(
    source: &str,
    node: &MultiplicativeExpressionStruct,
    depth: usize,
) -> RenderedOutput {
    let mut acc = ChildrenAccumulator::new(source, depth + 1);

    acc.add(
        "left_operand",
        "Expression",
        render_expression(source, &node.left_operand, depth + 1),
    );

    acc.add(
        "expression_multiplicative_expression_operator",
        "Expression_MultiplicativeExpression_Operator",
        render_expression_multiplicative_expression_operator(
            source,
            &node.expression_multiplicative_expression_operator,
            depth + 1,
        ),
    );

    acc.add(
        "right_operand",
        "Expression",
        render_expression(source, &node.right_operand, depth + 1),
    );

    acc.finish()
}

pub fn render_named_argument(
    source: &str,
    node: &NamedArgumentStruct,
    depth: usize,
) -> RenderedOutput {
    let mut acc = ChildrenAccumulator::new(source, depth + 1);

    acc.add(
        "name",
        "Identifier",
        render_terminal(source, &node.name.range),
    );

    acc.add("colon", "Colon", render_terminal(source, &node.colon.range));

    acc.add(
        "value",
        "Expression",
        render_expression(source, &node.value, depth + 1),
    );

    acc.finish()
}

pub fn render_named_argument_group(
    source: &str,
    node: &NamedArgumentGroupStruct,
    depth: usize,
) -> RenderedOutput {
    let mut acc = ChildrenAccumulator::new(source, depth + 1);

    acc.add(
        "open_brace",
        "OpenBrace",
        render_terminal(source, &node.open_brace.range),
    );

    acc.add(
        "arguments",
        "NamedArguments",
        render_named_arguments(source, &node.arguments, depth + 1),
    );

    acc.add(
        "close_brace",
        "CloseBrace",
        render_terminal(source, &node.close_brace.range),
    );

    acc.finish()
}

pub fn render_named_arguments_declaration(
    source: &str,
    node: &NamedArgumentsDeclarationStruct,
    depth: usize,
) -> RenderedOutput {
    let mut acc = ChildrenAccumulator::new(source, depth + 1);

    acc.add(
        "open_paren",
        "OpenParen",
        render_terminal(source, &node.open_paren.range),
    );

    acc.add(
        "arguments",
        "NamedArgumentGroup",
        render_named_argument_group(source, &node.arguments, depth + 1),
    );

    acc.add(
        "close_paren",
        "CloseParen",
        render_terminal(source, &node.close_paren.range),
    );

    acc.finish()
}

pub fn render_named_import(source: &str, node: &NamedImportStruct, depth: usize) -> RenderedOutput {
    let mut acc = ChildrenAccumulator::new(source, depth + 1);

    acc.add(
        "asterisk",
        "Asterisk",
        render_terminal(source, &node.asterisk.range),
    );

    acc.add(
        "alias",
        "ImportAlias",
        render_import_alias(source, &node.alias, depth + 1),
    );

    acc.add(
        "from_keyword",
        "FromKeyword",
        render_terminal(source, &node.from_keyword.range),
    );

    acc.add(
        "path",
        "StringLiteral",
        render_terminal(source, &node.path.range),
    );

    acc.finish()
}

pub fn render_new_expression(
    source: &str,
    node: &NewExpressionStruct,
    depth: usize,
) -> RenderedOutput {
    let mut acc = ChildrenAccumulator::new(source, depth + 1);

    acc.add(
        "new_keyword",
        "NewKeyword",
        render_terminal(source, &node.new_keyword.range),
    );

    acc.add(
        "type_name",
        "TypeName",
        render_type_name(source, &node.type_name, depth + 1),
    );

    acc.finish()
}

pub fn render_or_expression(
    source: &str,
    node: &OrExpressionStruct,
    depth: usize,
) -> RenderedOutput {
    let mut acc = ChildrenAccumulator::new(source, depth + 1);

    acc.add(
        "left_operand",
        "Expression",
        render_expression(source, &node.left_operand, depth + 1),
    );

    acc.add(
        "operator",
        "BarBar",
        render_terminal(source, &node.operator.range),
    );

    acc.add(
        "right_operand",
        "Expression",
        render_expression(source, &node.right_operand, depth + 1),
    );

    acc.finish()
}

pub fn render_override_paths_declaration(
    source: &str,
    node: &OverridePathsDeclarationStruct,
    depth: usize,
) -> RenderedOutput {
    let mut acc = ChildrenAccumulator::new(source, depth + 1);

    acc.add(
        "open_paren",
        "OpenParen",
        render_terminal(source, &node.open_paren.range),
    );

    acc.add(
        "paths",
        "OverridePaths",
        render_override_paths(source, &node.paths, depth + 1),
    );

    acc.add(
        "close_paren",
        "CloseParen",
        render_terminal(source, &node.close_paren.range),
    );

    acc.finish()
}

pub fn render_override_specifier(
    source: &str,
    node: &OverrideSpecifierStruct,
    depth: usize,
) -> RenderedOutput {
    let mut acc = ChildrenAccumulator::new(source, depth + 1);

    acc.add(
        "override_keyword",
        "OverrideKeyword",
        render_terminal(source, &node.override_keyword.range),
    );

    if let Some(ref overridden) = node.overridden {
        acc.add(
            "overridden",
            "OverridePathsDeclaration",
            render_override_paths_declaration(source, overridden, depth + 1),
        );
    }

    acc.finish()
}

pub fn render_parameter(source: &str, node: &ParameterStruct, depth: usize) -> RenderedOutput {
    let mut acc = ChildrenAccumulator::new(source, depth + 1);

    acc.add(
        "type_name",
        "TypeName",
        render_type_name(source, &node.type_name, depth + 1),
    );

    if let Some(ref storage_location) = node.storage_location {
        acc.add(
            "storage_location",
            "StorageLocation",
            render_storage_location(source, storage_location, depth + 1),
        );
    }

    if let Some(ref name) = node.name {
        acc.add("name", "Identifier", render_terminal(source, &name.range));
    }

    acc.finish()
}

pub fn render_parameters_declaration(
    source: &str,
    node: &ParametersDeclarationStruct,
    depth: usize,
) -> RenderedOutput {
    let mut acc = ChildrenAccumulator::new(source, depth + 1);

    acc.add(
        "open_paren",
        "OpenParen",
        render_terminal(source, &node.open_paren.range),
    );

    acc.add(
        "parameters",
        "Parameters",
        render_parameters(source, &node.parameters, depth + 1),
    );

    acc.add(
        "close_paren",
        "CloseParen",
        render_terminal(source, &node.close_paren.range),
    );

    acc.finish()
}

pub fn render_path_import(source: &str, node: &PathImportStruct, depth: usize) -> RenderedOutput {
    let mut acc = ChildrenAccumulator::new(source, depth + 1);

    acc.add(
        "path",
        "StringLiteral",
        render_terminal(source, &node.path.range),
    );

    if let Some(ref alias) = node.alias {
        acc.add(
            "alias",
            "ImportAlias",
            render_import_alias(source, alias, depth + 1),
        );
    }

    acc.finish()
}

pub fn render_positional_arguments_declaration(
    source: &str,
    node: &PositionalArgumentsDeclarationStruct,
    depth: usize,
) -> RenderedOutput {
    let mut acc = ChildrenAccumulator::new(source, depth + 1);

    acc.add(
        "open_paren",
        "OpenParen",
        render_terminal(source, &node.open_paren.range),
    );

    acc.add(
        "arguments",
        "PositionalArguments",
        render_positional_arguments(source, &node.arguments, depth + 1),
    );

    acc.add(
        "close_paren",
        "CloseParen",
        render_terminal(source, &node.close_paren.range),
    );

    acc.finish()
}

pub fn render_postfix_expression(
    source: &str,
    node: &PostfixExpressionStruct,
    depth: usize,
) -> RenderedOutput {
    let mut acc = ChildrenAccumulator::new(source, depth + 1);

    acc.add(
        "operand",
        "Expression",
        render_expression(source, &node.operand, depth + 1),
    );

    acc.add(
        "expression_postfix_expression_operator",
        "Expression_PostfixExpression_Operator",
        render_expression_postfix_expression_operator(
            source,
            &node.expression_postfix_expression_operator,
            depth + 1,
        ),
    );

    acc.finish()
}

pub fn render_pragma_directive(
    source: &str,
    node: &PragmaDirectiveStruct,
    depth: usize,
) -> RenderedOutput {
    let mut acc = ChildrenAccumulator::new(source, depth + 1);

    acc.add(
        "pragma_keyword",
        "PragmaKeyword",
        render_terminal(source, &node.pragma_keyword.range),
    );

    acc.add(
        "pragma",
        "Pragma",
        render_pragma(source, &node.pragma, depth + 1),
    );

    acc.add(
        "semicolon",
        "PragmaSemicolon",
        render_terminal(source, &node.semicolon.range),
    );

    acc.finish()
}

pub fn render_prefix_expression(
    source: &str,
    node: &PrefixExpressionStruct,
    depth: usize,
) -> RenderedOutput {
    let mut acc = ChildrenAccumulator::new(source, depth + 1);

    acc.add(
        "expression_prefix_expression_operator",
        "Expression_PrefixExpression_Operator",
        render_expression_prefix_expression_operator(
            source,
            &node.expression_prefix_expression_operator,
            depth + 1,
        ),
    );

    acc.add(
        "operand",
        "Expression",
        render_expression(source, &node.operand, depth + 1),
    );

    acc.finish()
}

pub fn render_receive_function_definition(
    source: &str,
    node: &ReceiveFunctionDefinitionStruct,
    depth: usize,
) -> RenderedOutput {
    let mut acc = ChildrenAccumulator::new(source, depth + 1);

    acc.add(
        "receive_keyword",
        "ReceiveKeyword",
        render_terminal(source, &node.receive_keyword.range),
    );

    acc.add(
        "parameters",
        "ParametersDeclaration",
        render_parameters_declaration(source, &node.parameters, depth + 1),
    );

    acc.add(
        "attributes",
        "ReceiveFunctionAttributes",
        render_receive_function_attributes(source, &node.attributes, depth + 1),
    );

    acc.add(
        "body",
        "FunctionBody",
        render_function_body(source, &node.body, depth + 1),
    );

    acc.finish()
}

pub fn render_return_statement(
    source: &str,
    node: &ReturnStatementStruct,
    depth: usize,
) -> RenderedOutput {
    let mut acc = ChildrenAccumulator::new(source, depth + 1);

    acc.add(
        "return_keyword",
        "ReturnKeyword",
        render_terminal(source, &node.return_keyword.range),
    );

    if let Some(ref expression) = node.expression {
        acc.add(
            "expression",
            "Expression",
            render_expression(source, expression, depth + 1),
        );
    }

    acc.add(
        "semicolon",
        "Semicolon",
        render_terminal(source, &node.semicolon.range),
    );

    acc.finish()
}

pub fn render_returns_declaration(
    source: &str,
    node: &ReturnsDeclarationStruct,
    depth: usize,
) -> RenderedOutput {
    let mut acc = ChildrenAccumulator::new(source, depth + 1);

    acc.add(
        "returns_keyword",
        "ReturnsKeyword",
        render_terminal(source, &node.returns_keyword.range),
    );

    acc.add(
        "variables",
        "ParametersDeclaration",
        render_parameters_declaration(source, &node.variables, depth + 1),
    );

    acc.finish()
}

pub fn render_revert_statement(
    source: &str,
    node: &RevertStatementStruct,
    depth: usize,
) -> RenderedOutput {
    let mut acc = ChildrenAccumulator::new(source, depth + 1);

    acc.add(
        "revert_keyword",
        "RevertKeyword",
        render_terminal(source, &node.revert_keyword.range),
    );

    acc.add(
        "error",
        "IdentifierPath",
        render_identifier_path(source, &node.error, depth + 1),
    );

    acc.add(
        "arguments",
        "ArgumentsDeclaration",
        render_arguments_declaration(source, &node.arguments, depth + 1),
    );

    acc.add(
        "semicolon",
        "Semicolon",
        render_terminal(source, &node.semicolon.range),
    );

    acc.finish()
}

pub fn render_shift_expression(
    source: &str,
    node: &ShiftExpressionStruct,
    depth: usize,
) -> RenderedOutput {
    let mut acc = ChildrenAccumulator::new(source, depth + 1);

    acc.add(
        "left_operand",
        "Expression",
        render_expression(source, &node.left_operand, depth + 1),
    );

    acc.add(
        "expression_shift_expression_operator",
        "Expression_ShiftExpression_Operator",
        render_expression_shift_expression_operator(
            source,
            &node.expression_shift_expression_operator,
            depth + 1,
        ),
    );

    acc.add(
        "right_operand",
        "Expression",
        render_expression(source, &node.right_operand, depth + 1),
    );

    acc.finish()
}

pub fn render_single_typed_declaration(
    source: &str,
    node: &SingleTypedDeclarationStruct,
    depth: usize,
) -> RenderedOutput {
    let mut acc = ChildrenAccumulator::new(source, depth + 1);

    acc.add(
        "declaration",
        "VariableDeclaration",
        render_variable_declaration(source, &node.declaration, depth + 1),
    );

    if let Some(ref value) = node.value {
        acc.add(
            "value",
            "VariableDeclarationValue",
            render_variable_declaration_value(source, value, depth + 1),
        );
    }

    acc.finish()
}

pub fn render_source_unit(source: &str, node: &SourceUnitStruct, depth: usize) -> RenderedOutput {
    let mut acc = ChildrenAccumulator::new(source, depth + 1);

    acc.add(
        "members",
        "SourceUnitMembers",
        render_source_unit_members(source, &node.members, depth + 1),
    );

    acc.finish()
}

pub fn render_state_variable_definition(
    source: &str,
    node: &StateVariableDefinitionStruct,
    depth: usize,
) -> RenderedOutput {
    let mut acc = ChildrenAccumulator::new(source, depth + 1);

    acc.add(
        "type_name",
        "TypeName",
        render_type_name(source, &node.type_name, depth + 1),
    );

    acc.add(
        "attributes",
        "StateVariableAttributes",
        render_state_variable_attributes(source, &node.attributes, depth + 1),
    );

    acc.add(
        "name",
        "Identifier",
        render_terminal(source, &node.name.range),
    );

    if let Some(ref value) = node.value {
        acc.add(
            "value",
            "StateVariableDefinitionValue",
            render_state_variable_definition_value(source, value, depth + 1),
        );
    }

    acc.add(
        "semicolon",
        "Semicolon",
        render_terminal(source, &node.semicolon.range),
    );

    acc.finish()
}

pub fn render_state_variable_definition_value(
    source: &str,
    node: &StateVariableDefinitionValueStruct,
    depth: usize,
) -> RenderedOutput {
    let mut acc = ChildrenAccumulator::new(source, depth + 1);

    acc.add("equal", "Equal", render_terminal(source, &node.equal.range));

    acc.add(
        "value",
        "Expression",
        render_expression(source, &node.value, depth + 1),
    );

    acc.finish()
}

pub fn render_storage_layout_specifier(
    source: &str,
    node: &StorageLayoutSpecifierStruct,
    depth: usize,
) -> RenderedOutput {
    let mut acc = ChildrenAccumulator::new(source, depth + 1);

    acc.add(
        "layout_keyword",
        "LayoutKeyword",
        render_terminal(source, &node.layout_keyword.range),
    );

    acc.add(
        "at_keyword",
        "AtKeyword",
        render_terminal(source, &node.at_keyword.range),
    );

    acc.add(
        "expression",
        "Expression",
        render_expression(source, &node.expression, depth + 1),
    );

    acc.finish()
}

pub fn render_struct_definition(
    source: &str,
    node: &StructDefinitionStruct,
    depth: usize,
) -> RenderedOutput {
    let mut acc = ChildrenAccumulator::new(source, depth + 1);

    acc.add(
        "struct_keyword",
        "StructKeyword",
        render_terminal(source, &node.struct_keyword.range),
    );

    acc.add(
        "name",
        "Identifier",
        render_terminal(source, &node.name.range),
    );

    acc.add(
        "open_brace",
        "OpenBrace",
        render_terminal(source, &node.open_brace.range),
    );

    acc.add(
        "members",
        "StructMembers",
        render_struct_members(source, &node.members, depth + 1),
    );

    acc.add(
        "close_brace",
        "CloseBrace",
        render_terminal(source, &node.close_brace.range),
    );

    acc.finish()
}

pub fn render_struct_member(
    source: &str,
    node: &StructMemberStruct,
    depth: usize,
) -> RenderedOutput {
    let mut acc = ChildrenAccumulator::new(source, depth + 1);

    acc.add(
        "type_name",
        "TypeName",
        render_type_name(source, &node.type_name, depth + 1),
    );

    acc.add(
        "name",
        "Identifier",
        render_terminal(source, &node.name.range),
    );

    acc.add(
        "semicolon",
        "Semicolon",
        render_terminal(source, &node.semicolon.range),
    );

    acc.finish()
}

pub fn render_try_statement(
    source: &str,
    node: &TryStatementStruct,
    depth: usize,
) -> RenderedOutput {
    let mut acc = ChildrenAccumulator::new(source, depth + 1);

    acc.add(
        "try_keyword",
        "TryKeyword",
        render_terminal(source, &node.try_keyword.range),
    );

    acc.add(
        "expression",
        "Expression",
        render_expression(source, &node.expression, depth + 1),
    );

    if let Some(ref returns) = node.returns {
        acc.add(
            "returns",
            "ReturnsDeclaration",
            render_returns_declaration(source, returns, depth + 1),
        );
    }

    acc.add("body", "Block", render_block(source, &node.body, depth + 1));

    acc.add(
        "catch_clauses",
        "CatchClauses",
        render_catch_clauses(source, &node.catch_clauses, depth + 1),
    );

    acc.finish()
}

pub fn render_tuple_expression(
    source: &str,
    node: &TupleExpressionStruct,
    depth: usize,
) -> RenderedOutput {
    let mut acc = ChildrenAccumulator::new(source, depth + 1);

    acc.add(
        "open_paren",
        "OpenParen",
        render_terminal(source, &node.open_paren.range),
    );

    acc.add(
        "items",
        "TupleValues",
        render_tuple_values(source, &node.items, depth + 1),
    );

    acc.add(
        "close_paren",
        "CloseParen",
        render_terminal(source, &node.close_paren.range),
    );

    acc.finish()
}

pub fn render_tuple_value(source: &str, node: &TupleValueStruct, depth: usize) -> RenderedOutput {
    let mut acc = ChildrenAccumulator::new(source, depth + 1);

    if let Some(ref expression) = node.expression {
        acc.add(
            "expression",
            "Expression",
            render_expression(source, expression, depth + 1),
        );
    }

    acc.finish()
}

pub fn render_type_expression(
    source: &str,
    node: &TypeExpressionStruct,
    depth: usize,
) -> RenderedOutput {
    let mut acc = ChildrenAccumulator::new(source, depth + 1);

    acc.add(
        "type_keyword",
        "TypeKeyword",
        render_terminal(source, &node.type_keyword.range),
    );

    acc.add(
        "open_paren",
        "OpenParen",
        render_terminal(source, &node.open_paren.range),
    );

    acc.add(
        "type_name",
        "TypeName",
        render_type_name(source, &node.type_name, depth + 1),
    );

    acc.add(
        "close_paren",
        "CloseParen",
        render_terminal(source, &node.close_paren.range),
    );

    acc.finish()
}

pub fn render_unchecked_block(
    source: &str,
    node: &UncheckedBlockStruct,
    depth: usize,
) -> RenderedOutput {
    let mut acc = ChildrenAccumulator::new(source, depth + 1);

    acc.add(
        "unchecked_keyword",
        "UncheckedKeyword",
        render_terminal(source, &node.unchecked_keyword.range),
    );

    acc.add(
        "block",
        "Block",
        render_block(source, &node.block, depth + 1),
    );

    acc.finish()
}

pub fn render_user_defined_value_type_definition(
    source: &str,
    node: &UserDefinedValueTypeDefinitionStruct,
    depth: usize,
) -> RenderedOutput {
    let mut acc = ChildrenAccumulator::new(source, depth + 1);

    acc.add(
        "type_keyword",
        "TypeKeyword",
        render_terminal(source, &node.type_keyword.range),
    );

    acc.add(
        "name",
        "Identifier",
        render_terminal(source, &node.name.range),
    );

    acc.add(
        "is_keyword",
        "IsKeyword",
        render_terminal(source, &node.is_keyword.range),
    );

    acc.add(
        "value_type",
        "ElementaryType",
        render_elementary_type(source, &node.value_type, depth + 1),
    );

    acc.add(
        "semicolon",
        "Semicolon",
        render_terminal(source, &node.semicolon.range),
    );

    acc.finish()
}

pub fn render_using_alias(source: &str, node: &UsingAliasStruct, depth: usize) -> RenderedOutput {
    let mut acc = ChildrenAccumulator::new(source, depth + 1);

    acc.add(
        "as_keyword",
        "AsKeyword",
        render_terminal(source, &node.as_keyword.range),
    );

    acc.add(
        "operator",
        "UsingOperator",
        render_using_operator(source, &node.operator, depth + 1),
    );

    acc.finish()
}

pub fn render_using_deconstruction(
    source: &str,
    node: &UsingDeconstructionStruct,
    depth: usize,
) -> RenderedOutput {
    let mut acc = ChildrenAccumulator::new(source, depth + 1);

    acc.add(
        "open_brace",
        "OpenBrace",
        render_terminal(source, &node.open_brace.range),
    );

    acc.add(
        "symbols",
        "UsingDeconstructionSymbols",
        render_using_deconstruction_symbols(source, &node.symbols, depth + 1),
    );

    acc.add(
        "close_brace",
        "CloseBrace",
        render_terminal(source, &node.close_brace.range),
    );

    acc.finish()
}

pub fn render_using_deconstruction_symbol(
    source: &str,
    node: &UsingDeconstructionSymbolStruct,
    depth: usize,
) -> RenderedOutput {
    let mut acc = ChildrenAccumulator::new(source, depth + 1);

    acc.add(
        "name",
        "IdentifierPath",
        render_identifier_path(source, &node.name, depth + 1),
    );

    if let Some(ref alias) = node.alias {
        acc.add(
            "alias",
            "UsingAlias",
            render_using_alias(source, alias, depth + 1),
        );
    }

    acc.finish()
}

pub fn render_using_directive(
    source: &str,
    node: &UsingDirectiveStruct,
    depth: usize,
) -> RenderedOutput {
    let mut acc = ChildrenAccumulator::new(source, depth + 1);

    acc.add(
        "using_keyword",
        "UsingKeyword",
        render_terminal(source, &node.using_keyword.range),
    );

    acc.add(
        "clause",
        "UsingClause",
        render_using_clause(source, &node.clause, depth + 1),
    );

    acc.add(
        "for_keyword",
        "ForKeyword",
        render_terminal(source, &node.for_keyword.range),
    );

    acc.add(
        "target",
        "UsingTarget",
        render_using_target(source, &node.target, depth + 1),
    );

    if let Some(ref global_keyword) = node.global_keyword {
        acc.add(
            "global_keyword",
            "GlobalKeyword",
            render_terminal(source, &global_keyword.range),
        );
    }

    acc.add(
        "semicolon",
        "Semicolon",
        render_terminal(source, &node.semicolon.range),
    );

    acc.finish()
}

pub fn render_variable_declaration(
    source: &str,
    node: &VariableDeclarationStruct,
    depth: usize,
) -> RenderedOutput {
    let mut acc = ChildrenAccumulator::new(source, depth + 1);

    acc.add(
        "type_name",
        "TypeName",
        render_type_name(source, &node.type_name, depth + 1),
    );

    if let Some(ref storage_location) = node.storage_location {
        acc.add(
            "storage_location",
            "StorageLocation",
            render_storage_location(source, storage_location, depth + 1),
        );
    }

    acc.add(
        "name",
        "Identifier",
        render_terminal(source, &node.name.range),
    );

    acc.finish()
}

pub fn render_variable_declaration_statement(
    source: &str,
    node: &VariableDeclarationStatementStruct,
    depth: usize,
) -> RenderedOutput {
    let mut acc = ChildrenAccumulator::new(source, depth + 1);

    acc.add(
        "target",
        "VariableDeclarationTarget",
        render_variable_declaration_target(source, &node.target, depth + 1),
    );

    acc.add(
        "semicolon",
        "Semicolon",
        render_terminal(source, &node.semicolon.range),
    );

    acc.finish()
}

pub fn render_variable_declaration_value(
    source: &str,
    node: &VariableDeclarationValueStruct,
    depth: usize,
) -> RenderedOutput {
    let mut acc = ChildrenAccumulator::new(source, depth + 1);

    acc.add("equal", "Equal", render_terminal(source, &node.equal.range));

    acc.add(
        "expression",
        "Expression",
        render_expression(source, &node.expression, depth + 1),
    );

    acc.finish()
}

pub fn render_version_pragma(
    source: &str,
    node: &VersionPragmaStruct,
    depth: usize,
) -> RenderedOutput {
    let mut acc = ChildrenAccumulator::new(source, depth + 1);

    acc.add(
        "solidity_keyword",
        "SolidityKeyword",
        render_terminal(source, &node.solidity_keyword.range),
    );

    acc.add(
        "sets",
        "VersionExpressionSets",
        render_version_expression_sets(source, &node.sets, depth + 1),
    );

    acc.finish()
}

pub fn render_version_range(
    source: &str,
    node: &VersionRangeStruct,
    depth: usize,
) -> RenderedOutput {
    let mut acc = ChildrenAccumulator::new(source, depth + 1);

    acc.add(
        "start",
        "VersionLiteral",
        render_version_literal(source, &node.start, depth + 1),
    );

    acc.add(
        "minus",
        "PragmaMinus",
        render_terminal(source, &node.minus.range),
    );

    acc.add(
        "end",
        "VersionLiteral",
        render_version_literal(source, &node.end, depth + 1),
    );

    acc.finish()
}

pub fn render_version_term(source: &str, node: &VersionTermStruct, depth: usize) -> RenderedOutput {
    let mut acc = ChildrenAccumulator::new(source, depth + 1);

    if let Some(ref operator) = node.operator {
        acc.add(
            "operator",
            "VersionOperator",
            render_version_operator(source, operator, depth + 1),
        );
    }

    acc.add(
        "literal",
        "VersionLiteral",
        render_version_literal(source, &node.literal, depth + 1),
    );

    acc.finish()
}

pub fn render_while_statement(
    source: &str,
    node: &WhileStatementStruct,
    depth: usize,
) -> RenderedOutput {
    let mut acc = ChildrenAccumulator::new(source, depth + 1);

    acc.add(
        "while_keyword",
        "WhileKeyword",
        render_terminal(source, &node.while_keyword.range),
    );

    acc.add(
        "open_paren",
        "OpenParen",
        render_terminal(source, &node.open_paren.range),
    );

    acc.add(
        "condition",
        "Expression",
        render_expression(source, &node.condition, depth + 1),
    );

    acc.add(
        "close_paren",
        "CloseParen",
        render_terminal(source, &node.close_paren.range),
    );

    acc.add(
        "body",
        "Statement",
        render_statement(source, &node.body, depth + 1),
    );

    acc.finish()
}

pub fn render_yul_block(source: &str, node: &YulBlockStruct, depth: usize) -> RenderedOutput {
    let mut acc = ChildrenAccumulator::new(source, depth + 1);

    acc.add(
        "open_brace",
        "YulOpenBrace",
        render_terminal(source, &node.open_brace.range),
    );

    acc.add(
        "statements",
        "YulStatements",
        render_yul_statements(source, &node.statements, depth + 1),
    );

    acc.add(
        "close_brace",
        "YulCloseBrace",
        render_terminal(source, &node.close_brace.range),
    );

    acc.finish()
}

pub fn render_yul_break_statement(
    source: &str,
    node: &YulBreakStatementStruct,
    depth: usize,
) -> RenderedOutput {
    let mut acc = ChildrenAccumulator::new(source, depth + 1);

    acc.add(
        "break_keyword",
        "YulBreakKeyword",
        render_terminal(source, &node.break_keyword.range),
    );

    acc.finish()
}

pub fn render_yul_continue_statement(
    source: &str,
    node: &YulContinueStatementStruct,
    depth: usize,
) -> RenderedOutput {
    let mut acc = ChildrenAccumulator::new(source, depth + 1);

    acc.add(
        "continue_keyword",
        "YulContinueKeyword",
        render_terminal(source, &node.continue_keyword.range),
    );

    acc.finish()
}

pub fn render_yul_default_case(
    source: &str,
    node: &YulDefaultCaseStruct,
    depth: usize,
) -> RenderedOutput {
    let mut acc = ChildrenAccumulator::new(source, depth + 1);

    acc.add(
        "default_keyword",
        "YulDefaultKeyword",
        render_terminal(source, &node.default_keyword.range),
    );

    acc.add(
        "body",
        "YulBlock",
        render_yul_block(source, &node.body, depth + 1),
    );

    acc.finish()
}

pub fn render_yul_flags_declaration(
    source: &str,
    node: &YulFlagsDeclarationStruct,
    depth: usize,
) -> RenderedOutput {
    let mut acc = ChildrenAccumulator::new(source, depth + 1);

    acc.add(
        "open_paren",
        "YulOpenParen",
        render_terminal(source, &node.open_paren.range),
    );

    acc.add(
        "flags",
        "YulFlags",
        render_yul_flags(source, &node.flags, depth + 1),
    );

    acc.add(
        "close_paren",
        "YulCloseParen",
        render_terminal(source, &node.close_paren.range),
    );

    acc.finish()
}

pub fn render_yul_for_statement(
    source: &str,
    node: &YulForStatementStruct,
    depth: usize,
) -> RenderedOutput {
    let mut acc = ChildrenAccumulator::new(source, depth + 1);

    acc.add(
        "for_keyword",
        "YulForKeyword",
        render_terminal(source, &node.for_keyword.range),
    );

    acc.add(
        "initialization",
        "YulBlock",
        render_yul_block(source, &node.initialization, depth + 1),
    );

    acc.add(
        "condition",
        "YulExpression",
        render_yul_expression(source, &node.condition, depth + 1),
    );

    acc.add(
        "iterator",
        "YulBlock",
        render_yul_block(source, &node.iterator, depth + 1),
    );

    acc.add(
        "body",
        "YulBlock",
        render_yul_block(source, &node.body, depth + 1),
    );

    acc.finish()
}

pub fn render_yul_function_call_expression(
    source: &str,
    node: &YulFunctionCallExpressionStruct,
    depth: usize,
) -> RenderedOutput {
    let mut acc = ChildrenAccumulator::new(source, depth + 1);

    acc.add(
        "operand",
        "YulExpression",
        render_yul_expression(source, &node.operand, depth + 1),
    );

    acc.add(
        "open_paren",
        "YulOpenParen",
        render_terminal(source, &node.open_paren.range),
    );

    acc.add(
        "arguments",
        "YulArguments",
        render_yul_arguments(source, &node.arguments, depth + 1),
    );

    acc.add(
        "close_paren",
        "YulCloseParen",
        render_terminal(source, &node.close_paren.range),
    );

    acc.finish()
}

pub fn render_yul_function_definition(
    source: &str,
    node: &YulFunctionDefinitionStruct,
    depth: usize,
) -> RenderedOutput {
    let mut acc = ChildrenAccumulator::new(source, depth + 1);

    acc.add(
        "function_keyword",
        "YulFunctionKeyword",
        render_terminal(source, &node.function_keyword.range),
    );

    acc.add(
        "name",
        "YulIdentifier",
        render_terminal(source, &node.name.range),
    );

    acc.add(
        "parameters",
        "YulParametersDeclaration",
        render_yul_parameters_declaration(source, &node.parameters, depth + 1),
    );

    if let Some(ref returns) = node.returns {
        acc.add(
            "returns",
            "YulReturnsDeclaration",
            render_yul_returns_declaration(source, returns, depth + 1),
        );
    }

    acc.add(
        "body",
        "YulBlock",
        render_yul_block(source, &node.body, depth + 1),
    );

    acc.finish()
}

pub fn render_yul_if_statement(
    source: &str,
    node: &YulIfStatementStruct,
    depth: usize,
) -> RenderedOutput {
    let mut acc = ChildrenAccumulator::new(source, depth + 1);

    acc.add(
        "if_keyword",
        "YulIfKeyword",
        render_terminal(source, &node.if_keyword.range),
    );

    acc.add(
        "condition",
        "YulExpression",
        render_yul_expression(source, &node.condition, depth + 1),
    );

    acc.add(
        "body",
        "YulBlock",
        render_yul_block(source, &node.body, depth + 1),
    );

    acc.finish()
}

pub fn render_yul_leave_statement(
    source: &str,
    node: &YulLeaveStatementStruct,
    depth: usize,
) -> RenderedOutput {
    let mut acc = ChildrenAccumulator::new(source, depth + 1);

    acc.add(
        "leave_keyword",
        "YulLeaveKeyword",
        render_terminal(source, &node.leave_keyword.range),
    );

    acc.finish()
}

pub fn render_yul_parameters_declaration(
    source: &str,
    node: &YulParametersDeclarationStruct,
    depth: usize,
) -> RenderedOutput {
    let mut acc = ChildrenAccumulator::new(source, depth + 1);

    acc.add(
        "open_paren",
        "YulOpenParen",
        render_terminal(source, &node.open_paren.range),
    );

    acc.add(
        "parameters",
        "YulParameters",
        render_yul_parameters(source, &node.parameters, depth + 1),
    );

    acc.add(
        "close_paren",
        "YulCloseParen",
        render_terminal(source, &node.close_paren.range),
    );

    acc.finish()
}

pub fn render_yul_returns_declaration(
    source: &str,
    node: &YulReturnsDeclarationStruct,
    depth: usize,
) -> RenderedOutput {
    let mut acc = ChildrenAccumulator::new(source, depth + 1);

    acc.add(
        "minus_greater_than",
        "YulMinusGreaterThan",
        render_terminal(source, &node.minus_greater_than.range),
    );

    acc.add(
        "variables",
        "YulVariableNames",
        render_yul_variable_names(source, &node.variables, depth + 1),
    );

    acc.finish()
}

pub fn render_yul_switch_statement(
    source: &str,
    node: &YulSwitchStatementStruct,
    depth: usize,
) -> RenderedOutput {
    let mut acc = ChildrenAccumulator::new(source, depth + 1);

    acc.add(
        "switch_keyword",
        "YulSwitchKeyword",
        render_terminal(source, &node.switch_keyword.range),
    );

    acc.add(
        "expression",
        "YulExpression",
        render_yul_expression(source, &node.expression, depth + 1),
    );

    acc.add(
        "cases",
        "YulSwitchCases",
        render_yul_switch_cases(source, &node.cases, depth + 1),
    );

    acc.finish()
}

pub fn render_yul_value_case(
    source: &str,
    node: &YulValueCaseStruct,
    depth: usize,
) -> RenderedOutput {
    let mut acc = ChildrenAccumulator::new(source, depth + 1);

    acc.add(
        "case_keyword",
        "YulCaseKeyword",
        render_terminal(source, &node.case_keyword.range),
    );

    acc.add(
        "value",
        "YulLiteral",
        render_yul_literal(source, &node.value, depth + 1),
    );

    acc.add(
        "body",
        "YulBlock",
        render_yul_block(source, &node.body, depth + 1),
    );

    acc.finish()
}

pub fn render_yul_variable_assignment_statement(
    source: &str,
    node: &YulVariableAssignmentStatementStruct,
    depth: usize,
) -> RenderedOutput {
    let mut acc = ChildrenAccumulator::new(source, depth + 1);

    acc.add(
        "variables",
        "YulPaths",
        render_yul_paths(source, &node.variables, depth + 1),
    );

    acc.add(
        "assignment",
        "YulColonEqual",
        render_terminal(source, &node.assignment.range),
    );

    acc.add(
        "expression",
        "YulExpression",
        render_yul_expression(source, &node.expression, depth + 1),
    );

    acc.finish()
}

pub fn render_yul_variable_declaration_statement(
    source: &str,
    node: &YulVariableDeclarationStatementStruct,
    depth: usize,
) -> RenderedOutput {
    let mut acc = ChildrenAccumulator::new(source, depth + 1);

    acc.add(
        "let_keyword",
        "YulLetKeyword",
        render_terminal(source, &node.let_keyword.range),
    );

    acc.add(
        "variables",
        "YulVariableNames",
        render_yul_variable_names(source, &node.variables, depth + 1),
    );

    if let Some(ref value) = node.value {
        acc.add(
            "value",
            "YulVariableDeclarationValue",
            render_yul_variable_declaration_value(source, value, depth + 1),
        );
    }

    acc.finish()
}

pub fn render_yul_variable_declaration_value(
    source: &str,
    node: &YulVariableDeclarationValueStruct,
    depth: usize,
) -> RenderedOutput {
    let mut acc = ChildrenAccumulator::new(source, depth + 1);

    acc.add(
        "assignment",
        "YulColonEqual",
        render_terminal(source, &node.assignment.range),
    );

    acc.add(
        "expression",
        "YulExpression",
        render_yul_expression(source, &node.expression, depth + 1),
    );

    acc.finish()
}

//
// Choices:
//

pub fn render_abicoder_version(
    source: &str,
    node: &AbicoderVersion,
    depth: usize,
) -> RenderedOutput {
    match node {
        AbicoderVersion::AbicoderV1Keyword(ref element) => {
            let (range, mut frags) = render_terminal(source, &element.range);

            frags.insert(
                0,
                " \u{25ba} (abicoder_v1_keyword\u{a789} AbicoderV1Keyword)".to_string(),
            );
            (range, frags)
        }

        AbicoderVersion::AbicoderV2Keyword(ref element) => {
            let (range, mut frags) = render_terminal(source, &element.range);

            frags.insert(
                0,
                " \u{25ba} (abicoder_v2_keyword\u{a789} AbicoderV2Keyword)".to_string(),
            );
            (range, frags)
        }
    }
}

pub fn render_arguments_declaration(
    source: &str,
    node: &ArgumentsDeclaration,
    depth: usize,
) -> RenderedOutput {
    match node {
        ArgumentsDeclaration::PositionalArgumentsDeclaration(ref element) => {
            let (range, mut frags) =
                render_positional_arguments_declaration(source, element, depth);

            frags.insert(0, " \u{25ba} (positional_arguments_declaration\u{a789} PositionalArgumentsDeclaration)".to_string());
            (range, frags)
        }

        ArgumentsDeclaration::NamedArgumentsDeclaration(ref element) => {
            let (range, mut frags) = render_named_arguments_declaration(source, element, depth);

            frags.insert(
                0,
                " \u{25ba} (named_arguments_declaration\u{a789} NamedArgumentsDeclaration)"
                    .to_string(),
            );
            (range, frags)
        }
    }
}

pub fn render_constructor_attribute(
    source: &str,
    node: &ConstructorAttribute,
    depth: usize,
) -> RenderedOutput {
    match node {
        ConstructorAttribute::ModifierInvocation(ref element) => {
            let (range, mut frags) = render_modifier_invocation(source, element, depth);

            frags.insert(
                0,
                " \u{25ba} (modifier_invocation\u{a789} ModifierInvocation)".to_string(),
            );
            (range, frags)
        }

        ConstructorAttribute::InternalKeyword(ref element) => {
            let (range, mut frags) = render_terminal(source, &element.range);

            frags.insert(
                0,
                " \u{25ba} (internal_keyword\u{a789} InternalKeyword)".to_string(),
            );
            (range, frags)
        }

        ConstructorAttribute::PayableKeyword(ref element) => {
            let (range, mut frags) = render_terminal(source, &element.range);

            frags.insert(
                0,
                " \u{25ba} (payable_keyword\u{a789} PayableKeyword)".to_string(),
            );
            (range, frags)
        }

        ConstructorAttribute::PublicKeyword(ref element) => {
            let (range, mut frags) = render_terminal(source, &element.range);

            frags.insert(
                0,
                " \u{25ba} (public_keyword\u{a789} PublicKeyword)".to_string(),
            );
            (range, frags)
        }
    }
}

pub fn render_contract_member(source: &str, node: &ContractMember, depth: usize) -> RenderedOutput {
    match node {
        ContractMember::UsingDirective(ref element) => {
            let (range, mut frags) = render_using_directive(source, element, depth);

            frags.insert(
                0,
                " \u{25ba} (using_directive\u{a789} UsingDirective)".to_string(),
            );
            (range, frags)
        }

        ContractMember::FunctionDefinition(ref element) => {
            let (range, mut frags) = render_function_definition(source, element, depth);

            frags.insert(
                0,
                " \u{25ba} (function_definition\u{a789} FunctionDefinition)".to_string(),
            );
            (range, frags)
        }

        ContractMember::ConstructorDefinition(ref element) => {
            let (range, mut frags) = render_constructor_definition(source, element, depth);

            frags.insert(
                0,
                " \u{25ba} (constructor_definition\u{a789} ConstructorDefinition)".to_string(),
            );
            (range, frags)
        }

        ContractMember::ReceiveFunctionDefinition(ref element) => {
            let (range, mut frags) = render_receive_function_definition(source, element, depth);

            frags.insert(
                0,
                " \u{25ba} (receive_function_definition\u{a789} ReceiveFunctionDefinition)"
                    .to_string(),
            );
            (range, frags)
        }

        ContractMember::FallbackFunctionDefinition(ref element) => {
            let (range, mut frags) = render_fallback_function_definition(source, element, depth);

            frags.insert(
                0,
                " \u{25ba} (fallback_function_definition\u{a789} FallbackFunctionDefinition)"
                    .to_string(),
            );
            (range, frags)
        }

        ContractMember::ModifierDefinition(ref element) => {
            let (range, mut frags) = render_modifier_definition(source, element, depth);

            frags.insert(
                0,
                " \u{25ba} (modifier_definition\u{a789} ModifierDefinition)".to_string(),
            );
            (range, frags)
        }

        ContractMember::StructDefinition(ref element) => {
            let (range, mut frags) = render_struct_definition(source, element, depth);

            frags.insert(
                0,
                " \u{25ba} (struct_definition\u{a789} StructDefinition)".to_string(),
            );
            (range, frags)
        }

        ContractMember::EnumDefinition(ref element) => {
            let (range, mut frags) = render_enum_definition(source, element, depth);

            frags.insert(
                0,
                " \u{25ba} (enum_definition\u{a789} EnumDefinition)".to_string(),
            );
            (range, frags)
        }

        ContractMember::EventDefinition(ref element) => {
            let (range, mut frags) = render_event_definition(source, element, depth);

            frags.insert(
                0,
                " \u{25ba} (event_definition\u{a789} EventDefinition)".to_string(),
            );
            (range, frags)
        }

        ContractMember::ErrorDefinition(ref element) => {
            let (range, mut frags) = render_error_definition(source, element, depth);

            frags.insert(
                0,
                " \u{25ba} (error_definition\u{a789} ErrorDefinition)".to_string(),
            );
            (range, frags)
        }

        ContractMember::UserDefinedValueTypeDefinition(ref element) => {
            let (range, mut frags) =
                render_user_defined_value_type_definition(source, element, depth);

            frags.insert(0, " \u{25ba} (user_defined_value_type_definition\u{a789} UserDefinedValueTypeDefinition)".to_string());
            (range, frags)
        }

        ContractMember::StateVariableDefinition(ref element) => {
            let (range, mut frags) = render_state_variable_definition(source, element, depth);

            frags.insert(
                0,
                " \u{25ba} (state_variable_definition\u{a789} StateVariableDefinition)".to_string(),
            );
            (range, frags)
        }
    }
}

pub fn render_contract_specifier(
    source: &str,
    node: &ContractSpecifier,
    depth: usize,
) -> RenderedOutput {
    match node {
        ContractSpecifier::InheritanceSpecifier(ref element) => {
            let (range, mut frags) = render_inheritance_specifier(source, element, depth);

            frags.insert(
                0,
                " \u{25ba} (inheritance_specifier\u{a789} InheritanceSpecifier)".to_string(),
            );
            (range, frags)
        }

        ContractSpecifier::StorageLayoutSpecifier(ref element) => {
            let (range, mut frags) = render_storage_layout_specifier(source, element, depth);

            frags.insert(
                0,
                " \u{25ba} (storage_layout_specifier\u{a789} StorageLayoutSpecifier)".to_string(),
            );
            (range, frags)
        }
    }
}

pub fn render_elementary_type(source: &str, node: &ElementaryType, depth: usize) -> RenderedOutput {
    match node {
        ElementaryType::BoolKeyword(ref element) => {
            let (range, mut frags) = render_terminal(source, &element.range);

            frags.insert(
                0,
                " \u{25ba} (bool_keyword\u{a789} BoolKeyword)".to_string(),
            );
            (range, frags)
        }

        ElementaryType::StringKeyword(ref element) => {
            let (range, mut frags) = render_terminal(source, &element.range);

            frags.insert(
                0,
                " \u{25ba} (string_keyword\u{a789} StringKeyword)".to_string(),
            );
            (range, frags)
        }

        ElementaryType::AddressType(ref element) => {
            let (range, mut frags) = render_address_type(source, element, depth);

            frags.insert(
                0,
                " \u{25ba} (address_type\u{a789} AddressType)".to_string(),
            );
            (range, frags)
        }

        ElementaryType::BytesKeyword(ref element) => {
            let (range, mut frags) = render_terminal(source, &element.range);

            frags.insert(
                0,
                " \u{25ba} (bytes_keyword\u{a789} BytesKeyword)".to_string(),
            );
            (range, frags)
        }

        ElementaryType::IntKeyword(ref element) => {
            let (range, mut frags) = render_terminal(source, &element.range);

            frags.insert(0, " \u{25ba} (int_keyword\u{a789} IntKeyword)".to_string());
            (range, frags)
        }

        ElementaryType::UintKeyword(ref element) => {
            let (range, mut frags) = render_terminal(source, &element.range);

            frags.insert(
                0,
                " \u{25ba} (uint_keyword\u{a789} UintKeyword)".to_string(),
            );
            (range, frags)
        }

        ElementaryType::FixedKeyword(ref element) => {
            let (range, mut frags) = render_terminal(source, &element.range);

            frags.insert(
                0,
                " \u{25ba} (fixed_keyword\u{a789} FixedKeyword)".to_string(),
            );
            (range, frags)
        }

        ElementaryType::UfixedKeyword(ref element) => {
            let (range, mut frags) = render_terminal(source, &element.range);

            frags.insert(
                0,
                " \u{25ba} (ufixed_keyword\u{a789} UfixedKeyword)".to_string(),
            );
            (range, frags)
        }
    }
}

pub fn render_experimental_feature(
    source: &str,
    node: &ExperimentalFeature,
    depth: usize,
) -> RenderedOutput {
    match node {
        ExperimentalFeature::ABIEncoderV2Keyword(ref element) => {
            let (range, mut frags) = render_terminal(source, &element.range);

            frags.insert(
                0,
                " \u{25ba} (abi_encoder_v2_keyword\u{a789} ABIEncoderV2Keyword)".to_string(),
            );
            (range, frags)
        }

        ExperimentalFeature::SMTCheckerKeyword(ref element) => {
            let (range, mut frags) = render_terminal(source, &element.range);

            frags.insert(
                0,
                " \u{25ba} (smt_checker_keyword\u{a789} SMTCheckerKeyword)".to_string(),
            );
            (range, frags)
        }

        ExperimentalFeature::PragmaStringLiteral(ref element) => {
            let (range, mut frags) = render_terminal(source, &element.range);

            frags.insert(
                0,
                " \u{25ba} (pragma_string_literal\u{a789} PragmaStringLiteral)".to_string(),
            );
            (range, frags)
        }
    }
}

pub fn render_expression(source: &str, node: &Expression, depth: usize) -> RenderedOutput {
    match node {
        Expression::AssignmentExpression(ref element) => {
            let (range, mut frags) = render_assignment_expression(source, element, depth);

            frags.insert(
                0,
                " \u{25ba} (assignment_expression\u{a789} AssignmentExpression)".to_string(),
            );
            (range, frags)
        }

        Expression::ConditionalExpression(ref element) => {
            let (range, mut frags) = render_conditional_expression(source, element, depth);

            frags.insert(
                0,
                " \u{25ba} (conditional_expression\u{a789} ConditionalExpression)".to_string(),
            );
            (range, frags)
        }

        Expression::OrExpression(ref element) => {
            let (range, mut frags) = render_or_expression(source, element, depth);

            frags.insert(
                0,
                " \u{25ba} (or_expression\u{a789} OrExpression)".to_string(),
            );
            (range, frags)
        }

        Expression::AndExpression(ref element) => {
            let (range, mut frags) = render_and_expression(source, element, depth);

            frags.insert(
                0,
                " \u{25ba} (and_expression\u{a789} AndExpression)".to_string(),
            );
            (range, frags)
        }

        Expression::EqualityExpression(ref element) => {
            let (range, mut frags) = render_equality_expression(source, element, depth);

            frags.insert(
                0,
                " \u{25ba} (equality_expression\u{a789} EqualityExpression)".to_string(),
            );
            (range, frags)
        }

        Expression::InequalityExpression(ref element) => {
            let (range, mut frags) = render_inequality_expression(source, element, depth);

            frags.insert(
                0,
                " \u{25ba} (inequality_expression\u{a789} InequalityExpression)".to_string(),
            );
            (range, frags)
        }

        Expression::BitwiseOrExpression(ref element) => {
            let (range, mut frags) = render_bitwise_or_expression(source, element, depth);

            frags.insert(
                0,
                " \u{25ba} (bitwise_or_expression\u{a789} BitwiseOrExpression)".to_string(),
            );
            (range, frags)
        }

        Expression::BitwiseXorExpression(ref element) => {
            let (range, mut frags) = render_bitwise_xor_expression(source, element, depth);

            frags.insert(
                0,
                " \u{25ba} (bitwise_xor_expression\u{a789} BitwiseXorExpression)".to_string(),
            );
            (range, frags)
        }

        Expression::BitwiseAndExpression(ref element) => {
            let (range, mut frags) = render_bitwise_and_expression(source, element, depth);

            frags.insert(
                0,
                " \u{25ba} (bitwise_and_expression\u{a789} BitwiseAndExpression)".to_string(),
            );
            (range, frags)
        }

        Expression::ShiftExpression(ref element) => {
            let (range, mut frags) = render_shift_expression(source, element, depth);

            frags.insert(
                0,
                " \u{25ba} (shift_expression\u{a789} ShiftExpression)".to_string(),
            );
            (range, frags)
        }

        Expression::AdditiveExpression(ref element) => {
            let (range, mut frags) = render_additive_expression(source, element, depth);

            frags.insert(
                0,
                " \u{25ba} (additive_expression\u{a789} AdditiveExpression)".to_string(),
            );
            (range, frags)
        }

        Expression::MultiplicativeExpression(ref element) => {
            let (range, mut frags) = render_multiplicative_expression(source, element, depth);

            frags.insert(
                0,
                " \u{25ba} (multiplicative_expression\u{a789} MultiplicativeExpression)"
                    .to_string(),
            );
            (range, frags)
        }

        Expression::ExponentiationExpression(ref element) => {
            let (range, mut frags) = render_exponentiation_expression(source, element, depth);

            frags.insert(
                0,
                " \u{25ba} (exponentiation_expression\u{a789} ExponentiationExpression)"
                    .to_string(),
            );
            (range, frags)
        }

        Expression::PostfixExpression(ref element) => {
            let (range, mut frags) = render_postfix_expression(source, element, depth);

            frags.insert(
                0,
                " \u{25ba} (postfix_expression\u{a789} PostfixExpression)".to_string(),
            );
            (range, frags)
        }

        Expression::PrefixExpression(ref element) => {
            let (range, mut frags) = render_prefix_expression(source, element, depth);

            frags.insert(
                0,
                " \u{25ba} (prefix_expression\u{a789} PrefixExpression)".to_string(),
            );
            (range, frags)
        }

        Expression::FunctionCallExpression(ref element) => {
            let (range, mut frags) = render_function_call_expression(source, element, depth);

            frags.insert(
                0,
                " \u{25ba} (function_call_expression\u{a789} FunctionCallExpression)".to_string(),
            );
            (range, frags)
        }

        Expression::CallOptionsExpression(ref element) => {
            let (range, mut frags) = render_call_options_expression(source, element, depth);

            frags.insert(
                0,
                " \u{25ba} (call_options_expression\u{a789} CallOptionsExpression)".to_string(),
            );
            (range, frags)
        }

        Expression::MemberAccessExpression(ref element) => {
            let (range, mut frags) = render_member_access_expression(source, element, depth);

            frags.insert(
                0,
                " \u{25ba} (member_access_expression\u{a789} MemberAccessExpression)".to_string(),
            );
            (range, frags)
        }

        Expression::IndexAccessExpression(ref element) => {
            let (range, mut frags) = render_index_access_expression(source, element, depth);

            frags.insert(
                0,
                " \u{25ba} (index_access_expression\u{a789} IndexAccessExpression)".to_string(),
            );
            (range, frags)
        }

        Expression::NewExpression(ref element) => {
            let (range, mut frags) = render_new_expression(source, element, depth);

            frags.insert(
                0,
                " \u{25ba} (new_expression\u{a789} NewExpression)".to_string(),
            );
            (range, frags)
        }

        Expression::TupleExpression(ref element) => {
            let (range, mut frags) = render_tuple_expression(source, element, depth);

            frags.insert(
                0,
                " \u{25ba} (tuple_expression\u{a789} TupleExpression)".to_string(),
            );
            (range, frags)
        }

        Expression::TypeExpression(ref element) => {
            let (range, mut frags) = render_type_expression(source, element, depth);

            frags.insert(
                0,
                " \u{25ba} (type_expression\u{a789} TypeExpression)".to_string(),
            );
            (range, frags)
        }

        Expression::ArrayExpression(ref element) => {
            let (range, mut frags) = render_array_expression(source, element, depth);

            frags.insert(
                0,
                " \u{25ba} (array_expression\u{a789} ArrayExpression)".to_string(),
            );
            (range, frags)
        }

        Expression::HexNumberExpression(ref element) => {
            let (range, mut frags) = render_hex_number_expression(source, element, depth);

            frags.insert(
                0,
                " \u{25ba} (hex_number_expression\u{a789} HexNumberExpression)".to_string(),
            );
            (range, frags)
        }

        Expression::DecimalNumberExpression(ref element) => {
            let (range, mut frags) = render_decimal_number_expression(source, element, depth);

            frags.insert(
                0,
                " \u{25ba} (decimal_number_expression\u{a789} DecimalNumberExpression)".to_string(),
            );
            (range, frags)
        }

        Expression::StringExpression(ref element) => {
            let (range, mut frags) = render_string_expression(source, element, depth);

            frags.insert(
                0,
                " \u{25ba} (string_expression\u{a789} StringExpression)".to_string(),
            );
            (range, frags)
        }

        Expression::ElementaryType(ref element) => {
            let (range, mut frags) = render_elementary_type(source, element, depth);

            frags.insert(
                0,
                " \u{25ba} (elementary_type\u{a789} ElementaryType)".to_string(),
            );
            (range, frags)
        }

        Expression::PayableKeyword(ref element) => {
            let (range, mut frags) = render_terminal(source, &element.range);

            frags.insert(
                0,
                " \u{25ba} (payable_keyword\u{a789} PayableKeyword)".to_string(),
            );
            (range, frags)
        }

        Expression::ThisKeyword(ref element) => {
            let (range, mut frags) = render_terminal(source, &element.range);

            frags.insert(
                0,
                " \u{25ba} (this_keyword\u{a789} ThisKeyword)".to_string(),
            );
            (range, frags)
        }

        Expression::SuperKeyword(ref element) => {
            let (range, mut frags) = render_terminal(source, &element.range);

            frags.insert(
                0,
                " \u{25ba} (super_keyword\u{a789} SuperKeyword)".to_string(),
            );
            (range, frags)
        }

        Expression::TrueKeyword(ref element) => {
            let (range, mut frags) = render_terminal(source, &element.range);

            frags.insert(
                0,
                " \u{25ba} (true_keyword\u{a789} TrueKeyword)".to_string(),
            );
            (range, frags)
        }

        Expression::FalseKeyword(ref element) => {
            let (range, mut frags) = render_terminal(source, &element.range);

            frags.insert(
                0,
                " \u{25ba} (false_keyword\u{a789} FalseKeyword)".to_string(),
            );
            (range, frags)
        }

        Expression::Identifier(ref element) => {
            let (range, mut frags) = render_terminal(source, &element.range);

            frags.insert(0, " \u{25ba} (identifier\u{a789} Identifier)".to_string());
            (range, frags)
        }
    }
}

pub fn render_expression_additive_expression_operator(
    source: &str,
    node: &Expression_AdditiveExpression_Operator,
    depth: usize,
) -> RenderedOutput {
    match node {
        Expression_AdditiveExpression_Operator::Minus(ref element) => {
            let (range, mut frags) = render_terminal(source, &element.range);

            frags.insert(0, " \u{25ba} (minus\u{a789} Minus)".to_string());
            (range, frags)
        }

        Expression_AdditiveExpression_Operator::Plus(ref element) => {
            let (range, mut frags) = render_terminal(source, &element.range);

            frags.insert(0, " \u{25ba} (plus\u{a789} Plus)".to_string());
            (range, frags)
        }
    }
}

pub fn render_expression_assignment_expression_operator(
    source: &str,
    node: &Expression_AssignmentExpression_Operator,
    depth: usize,
) -> RenderedOutput {
    match node {
        Expression_AssignmentExpression_Operator::AmpersandEqual(ref element) => {
            let (range, mut frags) = render_terminal(source, &element.range);

            frags.insert(
                0,
                " \u{25ba} (ampersand_equal\u{a789} AmpersandEqual)".to_string(),
            );
            (range, frags)
        }

        Expression_AssignmentExpression_Operator::AsteriskEqual(ref element) => {
            let (range, mut frags) = render_terminal(source, &element.range);

            frags.insert(
                0,
                " \u{25ba} (asterisk_equal\u{a789} AsteriskEqual)".to_string(),
            );
            (range, frags)
        }

        Expression_AssignmentExpression_Operator::BarEqual(ref element) => {
            let (range, mut frags) = render_terminal(source, &element.range);

            frags.insert(0, " \u{25ba} (bar_equal\u{a789} BarEqual)".to_string());
            (range, frags)
        }

        Expression_AssignmentExpression_Operator::CaretEqual(ref element) => {
            let (range, mut frags) = render_terminal(source, &element.range);

            frags.insert(0, " \u{25ba} (caret_equal\u{a789} CaretEqual)".to_string());
            (range, frags)
        }

        Expression_AssignmentExpression_Operator::Equal(ref element) => {
            let (range, mut frags) = render_terminal(source, &element.range);

            frags.insert(0, " \u{25ba} (equal\u{a789} Equal)".to_string());
            (range, frags)
        }

        Expression_AssignmentExpression_Operator::GreaterThanGreaterThanEqual(ref element) => {
            let (range, mut frags) = render_terminal(source, &element.range);

            frags.insert(
                0,
                " \u{25ba} (greater_than_greater_than_equal\u{a789} GreaterThanGreaterThanEqual)"
                    .to_string(),
            );
            (range, frags)
        }

        Expression_AssignmentExpression_Operator::GreaterThanGreaterThanGreaterThanEqual(
            ref element,
        ) => {
            let (range, mut frags) = render_terminal(source, &element.range);

            frags.insert(0, " \u{25ba} (greater_than_greater_than_greater_than_equal\u{a789} GreaterThanGreaterThanGreaterThanEqual)".to_string());
            (range, frags)
        }

        Expression_AssignmentExpression_Operator::LessThanLessThanEqual(ref element) => {
            let (range, mut frags) = render_terminal(source, &element.range);

            frags.insert(
                0,
                " \u{25ba} (less_than_less_than_equal\u{a789} LessThanLessThanEqual)".to_string(),
            );
            (range, frags)
        }

        Expression_AssignmentExpression_Operator::MinusEqual(ref element) => {
            let (range, mut frags) = render_terminal(source, &element.range);

            frags.insert(0, " \u{25ba} (minus_equal\u{a789} MinusEqual)".to_string());
            (range, frags)
        }

        Expression_AssignmentExpression_Operator::PercentEqual(ref element) => {
            let (range, mut frags) = render_terminal(source, &element.range);

            frags.insert(
                0,
                " \u{25ba} (percent_equal\u{a789} PercentEqual)".to_string(),
            );
            (range, frags)
        }

        Expression_AssignmentExpression_Operator::PlusEqual(ref element) => {
            let (range, mut frags) = render_terminal(source, &element.range);

            frags.insert(0, " \u{25ba} (plus_equal\u{a789} PlusEqual)".to_string());
            (range, frags)
        }

        Expression_AssignmentExpression_Operator::SlashEqual(ref element) => {
            let (range, mut frags) = render_terminal(source, &element.range);

            frags.insert(0, " \u{25ba} (slash_equal\u{a789} SlashEqual)".to_string());
            (range, frags)
        }
    }
}

pub fn render_expression_equality_expression_operator(
    source: &str,
    node: &Expression_EqualityExpression_Operator,
    depth: usize,
) -> RenderedOutput {
    match node {
        Expression_EqualityExpression_Operator::BangEqual(ref element) => {
            let (range, mut frags) = render_terminal(source, &element.range);

            frags.insert(0, " \u{25ba} (bang_equal\u{a789} BangEqual)".to_string());
            (range, frags)
        }

        Expression_EqualityExpression_Operator::EqualEqual(ref element) => {
            let (range, mut frags) = render_terminal(source, &element.range);

            frags.insert(0, " \u{25ba} (equal_equal\u{a789} EqualEqual)".to_string());
            (range, frags)
        }
    }
}

pub fn render_expression_inequality_expression_operator(
    source: &str,
    node: &Expression_InequalityExpression_Operator,
    depth: usize,
) -> RenderedOutput {
    match node {
        Expression_InequalityExpression_Operator::GreaterThan(ref element) => {
            let (range, mut frags) = render_terminal(source, &element.range);

            frags.insert(
                0,
                " \u{25ba} (greater_than\u{a789} GreaterThan)".to_string(),
            );
            (range, frags)
        }

        Expression_InequalityExpression_Operator::GreaterThanEqual(ref element) => {
            let (range, mut frags) = render_terminal(source, &element.range);

            frags.insert(
                0,
                " \u{25ba} (greater_than_equal\u{a789} GreaterThanEqual)".to_string(),
            );
            (range, frags)
        }

        Expression_InequalityExpression_Operator::LessThan(ref element) => {
            let (range, mut frags) = render_terminal(source, &element.range);

            frags.insert(0, " \u{25ba} (less_than\u{a789} LessThan)".to_string());
            (range, frags)
        }

        Expression_InequalityExpression_Operator::LessThanEqual(ref element) => {
            let (range, mut frags) = render_terminal(source, &element.range);

            frags.insert(
                0,
                " \u{25ba} (less_than_equal\u{a789} LessThanEqual)".to_string(),
            );
            (range, frags)
        }
    }
}

pub fn render_expression_multiplicative_expression_operator(
    source: &str,
    node: &Expression_MultiplicativeExpression_Operator,
    depth: usize,
) -> RenderedOutput {
    match node {
        Expression_MultiplicativeExpression_Operator::Asterisk(ref element) => {
            let (range, mut frags) = render_terminal(source, &element.range);

            frags.insert(0, " \u{25ba} (asterisk\u{a789} Asterisk)".to_string());
            (range, frags)
        }

        Expression_MultiplicativeExpression_Operator::Percent(ref element) => {
            let (range, mut frags) = render_terminal(source, &element.range);

            frags.insert(0, " \u{25ba} (percent\u{a789} Percent)".to_string());
            (range, frags)
        }

        Expression_MultiplicativeExpression_Operator::Slash(ref element) => {
            let (range, mut frags) = render_terminal(source, &element.range);

            frags.insert(0, " \u{25ba} (slash\u{a789} Slash)".to_string());
            (range, frags)
        }
    }
}

pub fn render_expression_postfix_expression_operator(
    source: &str,
    node: &Expression_PostfixExpression_Operator,
    depth: usize,
) -> RenderedOutput {
    match node {
        Expression_PostfixExpression_Operator::MinusMinus(ref element) => {
            let (range, mut frags) = render_terminal(source, &element.range);

            frags.insert(0, " \u{25ba} (minus_minus\u{a789} MinusMinus)".to_string());
            (range, frags)
        }

        Expression_PostfixExpression_Operator::PlusPlus(ref element) => {
            let (range, mut frags) = render_terminal(source, &element.range);

            frags.insert(0, " \u{25ba} (plus_plus\u{a789} PlusPlus)".to_string());
            (range, frags)
        }
    }
}

pub fn render_expression_prefix_expression_operator(
    source: &str,
    node: &Expression_PrefixExpression_Operator,
    depth: usize,
) -> RenderedOutput {
    match node {
        Expression_PrefixExpression_Operator::Bang(ref element) => {
            let (range, mut frags) = render_terminal(source, &element.range);

            frags.insert(0, " \u{25ba} (bang\u{a789} Bang)".to_string());
            (range, frags)
        }

        Expression_PrefixExpression_Operator::DeleteKeyword(ref element) => {
            let (range, mut frags) = render_terminal(source, &element.range);

            frags.insert(
                0,
                " \u{25ba} (delete_keyword\u{a789} DeleteKeyword)".to_string(),
            );
            (range, frags)
        }

        Expression_PrefixExpression_Operator::Minus(ref element) => {
            let (range, mut frags) = render_terminal(source, &element.range);

            frags.insert(0, " \u{25ba} (minus\u{a789} Minus)".to_string());
            (range, frags)
        }

        Expression_PrefixExpression_Operator::MinusMinus(ref element) => {
            let (range, mut frags) = render_terminal(source, &element.range);

            frags.insert(0, " \u{25ba} (minus_minus\u{a789} MinusMinus)".to_string());
            (range, frags)
        }

        Expression_PrefixExpression_Operator::PlusPlus(ref element) => {
            let (range, mut frags) = render_terminal(source, &element.range);

            frags.insert(0, " \u{25ba} (plus_plus\u{a789} PlusPlus)".to_string());
            (range, frags)
        }

        Expression_PrefixExpression_Operator::Tilde(ref element) => {
            let (range, mut frags) = render_terminal(source, &element.range);

            frags.insert(0, " \u{25ba} (tilde\u{a789} Tilde)".to_string());
            (range, frags)
        }
    }
}

pub fn render_expression_shift_expression_operator(
    source: &str,
    node: &Expression_ShiftExpression_Operator,
    depth: usize,
) -> RenderedOutput {
    match node {
        Expression_ShiftExpression_Operator::GreaterThanGreaterThan(ref element) => {
            let (range, mut frags) = render_terminal(source, &element.range);

            frags.insert(
                0,
                " \u{25ba} (greater_than_greater_than\u{a789} GreaterThanGreaterThan)".to_string(),
            );
            (range, frags)
        }

        Expression_ShiftExpression_Operator::GreaterThanGreaterThanGreaterThan(ref element) => {
            let (range, mut frags) = render_terminal(source, &element.range);

            frags.insert(0, " \u{25ba} (greater_than_greater_than_greater_than\u{a789} GreaterThanGreaterThanGreaterThan)".to_string());
            (range, frags)
        }

        Expression_ShiftExpression_Operator::LessThanLessThan(ref element) => {
            let (range, mut frags) = render_terminal(source, &element.range);

            frags.insert(
                0,
                " \u{25ba} (less_than_less_than\u{a789} LessThanLessThan)".to_string(),
            );
            (range, frags)
        }
    }
}

pub fn render_fallback_function_attribute(
    source: &str,
    node: &FallbackFunctionAttribute,
    depth: usize,
) -> RenderedOutput {
    match node {
        FallbackFunctionAttribute::ModifierInvocation(ref element) => {
            let (range, mut frags) = render_modifier_invocation(source, element, depth);

            frags.insert(
                0,
                " \u{25ba} (modifier_invocation\u{a789} ModifierInvocation)".to_string(),
            );
            (range, frags)
        }

        FallbackFunctionAttribute::OverrideSpecifier(ref element) => {
            let (range, mut frags) = render_override_specifier(source, element, depth);

            frags.insert(
                0,
                " \u{25ba} (override_specifier\u{a789} OverrideSpecifier)".to_string(),
            );
            (range, frags)
        }

        FallbackFunctionAttribute::ExternalKeyword(ref element) => {
            let (range, mut frags) = render_terminal(source, &element.range);

            frags.insert(
                0,
                " \u{25ba} (external_keyword\u{a789} ExternalKeyword)".to_string(),
            );
            (range, frags)
        }

        FallbackFunctionAttribute::PayableKeyword(ref element) => {
            let (range, mut frags) = render_terminal(source, &element.range);

            frags.insert(
                0,
                " \u{25ba} (payable_keyword\u{a789} PayableKeyword)".to_string(),
            );
            (range, frags)
        }

        FallbackFunctionAttribute::PureKeyword(ref element) => {
            let (range, mut frags) = render_terminal(source, &element.range);

            frags.insert(
                0,
                " \u{25ba} (pure_keyword\u{a789} PureKeyword)".to_string(),
            );
            (range, frags)
        }

        FallbackFunctionAttribute::ViewKeyword(ref element) => {
            let (range, mut frags) = render_terminal(source, &element.range);

            frags.insert(
                0,
                " \u{25ba} (view_keyword\u{a789} ViewKeyword)".to_string(),
            );
            (range, frags)
        }

        FallbackFunctionAttribute::VirtualKeyword(ref element) => {
            let (range, mut frags) = render_terminal(source, &element.range);

            frags.insert(
                0,
                " \u{25ba} (virtual_keyword\u{a789} VirtualKeyword)".to_string(),
            );
            (range, frags)
        }
    }
}

pub fn render_for_statement_condition(
    source: &str,
    node: &ForStatementCondition,
    depth: usize,
) -> RenderedOutput {
    match node {
        ForStatementCondition::ExpressionStatement(ref element) => {
            let (range, mut frags) = render_expression_statement(source, element, depth);

            frags.insert(
                0,
                " \u{25ba} (expression_statement\u{a789} ExpressionStatement)".to_string(),
            );
            (range, frags)
        }

        ForStatementCondition::Semicolon(ref element) => {
            let (range, mut frags) = render_terminal(source, &element.range);

            frags.insert(0, " \u{25ba} (semicolon\u{a789} Semicolon)".to_string());
            (range, frags)
        }
    }
}

pub fn render_for_statement_initialization(
    source: &str,
    node: &ForStatementInitialization,
    depth: usize,
) -> RenderedOutput {
    match node {
        ForStatementInitialization::VariableDeclarationStatement(ref element) => {
            let (range, mut frags) = render_variable_declaration_statement(source, element, depth);

            frags.insert(
                0,
                " \u{25ba} (variable_declaration_statement\u{a789} VariableDeclarationStatement)"
                    .to_string(),
            );
            (range, frags)
        }

        ForStatementInitialization::ExpressionStatement(ref element) => {
            let (range, mut frags) = render_expression_statement(source, element, depth);

            frags.insert(
                0,
                " \u{25ba} (expression_statement\u{a789} ExpressionStatement)".to_string(),
            );
            (range, frags)
        }

        ForStatementInitialization::Semicolon(ref element) => {
            let (range, mut frags) = render_terminal(source, &element.range);

            frags.insert(0, " \u{25ba} (semicolon\u{a789} Semicolon)".to_string());
            (range, frags)
        }
    }
}

pub fn render_function_attribute(
    source: &str,
    node: &FunctionAttribute,
    depth: usize,
) -> RenderedOutput {
    match node {
        FunctionAttribute::ModifierInvocation(ref element) => {
            let (range, mut frags) = render_modifier_invocation(source, element, depth);

            frags.insert(
                0,
                " \u{25ba} (modifier_invocation\u{a789} ModifierInvocation)".to_string(),
            );
            (range, frags)
        }

        FunctionAttribute::OverrideSpecifier(ref element) => {
            let (range, mut frags) = render_override_specifier(source, element, depth);

            frags.insert(
                0,
                " \u{25ba} (override_specifier\u{a789} OverrideSpecifier)".to_string(),
            );
            (range, frags)
        }

        FunctionAttribute::ExternalKeyword(ref element) => {
            let (range, mut frags) = render_terminal(source, &element.range);

            frags.insert(
                0,
                " \u{25ba} (external_keyword\u{a789} ExternalKeyword)".to_string(),
            );
            (range, frags)
        }

        FunctionAttribute::InternalKeyword(ref element) => {
            let (range, mut frags) = render_terminal(source, &element.range);

            frags.insert(
                0,
                " \u{25ba} (internal_keyword\u{a789} InternalKeyword)".to_string(),
            );
            (range, frags)
        }

        FunctionAttribute::PayableKeyword(ref element) => {
            let (range, mut frags) = render_terminal(source, &element.range);

            frags.insert(
                0,
                " \u{25ba} (payable_keyword\u{a789} PayableKeyword)".to_string(),
            );
            (range, frags)
        }

        FunctionAttribute::PrivateKeyword(ref element) => {
            let (range, mut frags) = render_terminal(source, &element.range);

            frags.insert(
                0,
                " \u{25ba} (private_keyword\u{a789} PrivateKeyword)".to_string(),
            );
            (range, frags)
        }

        FunctionAttribute::PublicKeyword(ref element) => {
            let (range, mut frags) = render_terminal(source, &element.range);

            frags.insert(
                0,
                " \u{25ba} (public_keyword\u{a789} PublicKeyword)".to_string(),
            );
            (range, frags)
        }

        FunctionAttribute::PureKeyword(ref element) => {
            let (range, mut frags) = render_terminal(source, &element.range);

            frags.insert(
                0,
                " \u{25ba} (pure_keyword\u{a789} PureKeyword)".to_string(),
            );
            (range, frags)
        }

        FunctionAttribute::ViewKeyword(ref element) => {
            let (range, mut frags) = render_terminal(source, &element.range);

            frags.insert(
                0,
                " \u{25ba} (view_keyword\u{a789} ViewKeyword)".to_string(),
            );
            (range, frags)
        }

        FunctionAttribute::VirtualKeyword(ref element) => {
            let (range, mut frags) = render_terminal(source, &element.range);

            frags.insert(
                0,
                " \u{25ba} (virtual_keyword\u{a789} VirtualKeyword)".to_string(),
            );
            (range, frags)
        }
    }
}

pub fn render_function_body(source: &str, node: &FunctionBody, depth: usize) -> RenderedOutput {
    match node {
        FunctionBody::Block(ref element) => {
            let (range, mut frags) = render_block(source, element, depth);

            frags.insert(0, " \u{25ba} (block\u{a789} Block)".to_string());
            (range, frags)
        }

        FunctionBody::Semicolon(ref element) => {
            let (range, mut frags) = render_terminal(source, &element.range);

            frags.insert(0, " \u{25ba} (semicolon\u{a789} Semicolon)".to_string());
            (range, frags)
        }
    }
}

pub fn render_function_name(source: &str, node: &FunctionName, depth: usize) -> RenderedOutput {
    match node {
        FunctionName::Identifier(ref element) => {
            let (range, mut frags) = render_terminal(source, &element.range);

            frags.insert(0, " \u{25ba} (identifier\u{a789} Identifier)".to_string());
            (range, frags)
        }

        FunctionName::FallbackKeyword(ref element) => {
            let (range, mut frags) = render_terminal(source, &element.range);

            frags.insert(
                0,
                " \u{25ba} (fallback_keyword\u{a789} FallbackKeyword)".to_string(),
            );
            (range, frags)
        }

        FunctionName::ReceiveKeyword(ref element) => {
            let (range, mut frags) = render_terminal(source, &element.range);

            frags.insert(
                0,
                " \u{25ba} (receive_keyword\u{a789} ReceiveKeyword)".to_string(),
            );
            (range, frags)
        }
    }
}

pub fn render_function_type_attribute(
    source: &str,
    node: &FunctionTypeAttribute,
    depth: usize,
) -> RenderedOutput {
    match node {
        FunctionTypeAttribute::InternalKeyword(ref element) => {
            let (range, mut frags) = render_terminal(source, &element.range);

            frags.insert(
                0,
                " \u{25ba} (internal_keyword\u{a789} InternalKeyword)".to_string(),
            );
            (range, frags)
        }

        FunctionTypeAttribute::ExternalKeyword(ref element) => {
            let (range, mut frags) = render_terminal(source, &element.range);

            frags.insert(
                0,
                " \u{25ba} (external_keyword\u{a789} ExternalKeyword)".to_string(),
            );
            (range, frags)
        }

        FunctionTypeAttribute::PrivateKeyword(ref element) => {
            let (range, mut frags) = render_terminal(source, &element.range);

            frags.insert(
                0,
                " \u{25ba} (private_keyword\u{a789} PrivateKeyword)".to_string(),
            );
            (range, frags)
        }

        FunctionTypeAttribute::PublicKeyword(ref element) => {
            let (range, mut frags) = render_terminal(source, &element.range);

            frags.insert(
                0,
                " \u{25ba} (public_keyword\u{a789} PublicKeyword)".to_string(),
            );
            (range, frags)
        }

        FunctionTypeAttribute::PureKeyword(ref element) => {
            let (range, mut frags) = render_terminal(source, &element.range);

            frags.insert(
                0,
                " \u{25ba} (pure_keyword\u{a789} PureKeyword)".to_string(),
            );
            (range, frags)
        }

        FunctionTypeAttribute::ViewKeyword(ref element) => {
            let (range, mut frags) = render_terminal(source, &element.range);

            frags.insert(
                0,
                " \u{25ba} (view_keyword\u{a789} ViewKeyword)".to_string(),
            );
            (range, frags)
        }

        FunctionTypeAttribute::PayableKeyword(ref element) => {
            let (range, mut frags) = render_terminal(source, &element.range);

            frags.insert(
                0,
                " \u{25ba} (payable_keyword\u{a789} PayableKeyword)".to_string(),
            );
            (range, frags)
        }
    }
}

pub fn render_identifier_path_element(
    source: &str,
    node: &IdentifierPathElement,
    depth: usize,
) -> RenderedOutput {
    match node {
        IdentifierPathElement::Identifier(ref element) => {
            let (range, mut frags) = render_terminal(source, &element.range);

            frags.insert(0, " \u{25ba} (identifier\u{a789} Identifier)".to_string());
            (range, frags)
        }

        IdentifierPathElement::AddressKeyword(ref element) => {
            let (range, mut frags) = render_terminal(source, &element.range);

            frags.insert(
                0,
                " \u{25ba} (address_keyword\u{a789} AddressKeyword)".to_string(),
            );
            (range, frags)
        }
    }
}

pub fn render_import_clause(source: &str, node: &ImportClause, depth: usize) -> RenderedOutput {
    match node {
        ImportClause::PathImport(ref element) => {
            let (range, mut frags) = render_path_import(source, element, depth);

            frags.insert(0, " \u{25ba} (path_import\u{a789} PathImport)".to_string());
            (range, frags)
        }

        ImportClause::NamedImport(ref element) => {
            let (range, mut frags) = render_named_import(source, element, depth);

            frags.insert(
                0,
                " \u{25ba} (named_import\u{a789} NamedImport)".to_string(),
            );
            (range, frags)
        }

        ImportClause::ImportDeconstruction(ref element) => {
            let (range, mut frags) = render_import_deconstruction(source, element, depth);

            frags.insert(
                0,
                " \u{25ba} (import_deconstruction\u{a789} ImportDeconstruction)".to_string(),
            );
            (range, frags)
        }
    }
}

pub fn render_mapping_key_type(
    source: &str,
    node: &MappingKeyType,
    depth: usize,
) -> RenderedOutput {
    match node {
        MappingKeyType::ElementaryType(ref element) => {
            let (range, mut frags) = render_elementary_type(source, element, depth);

            frags.insert(
                0,
                " \u{25ba} (elementary_type\u{a789} ElementaryType)".to_string(),
            );
            (range, frags)
        }

        MappingKeyType::IdentifierPath(ref element) => {
            let (range, mut frags) = render_identifier_path(source, element, depth);

            frags.insert(
                0,
                " \u{25ba} (identifier_path\u{a789} IdentifierPath)".to_string(),
            );
            (range, frags)
        }
    }
}

pub fn render_modifier_attribute(
    source: &str,
    node: &ModifierAttribute,
    depth: usize,
) -> RenderedOutput {
    match node {
        ModifierAttribute::OverrideSpecifier(ref element) => {
            let (range, mut frags) = render_override_specifier(source, element, depth);

            frags.insert(
                0,
                " \u{25ba} (override_specifier\u{a789} OverrideSpecifier)".to_string(),
            );
            (range, frags)
        }

        ModifierAttribute::VirtualKeyword(ref element) => {
            let (range, mut frags) = render_terminal(source, &element.range);

            frags.insert(
                0,
                " \u{25ba} (virtual_keyword\u{a789} VirtualKeyword)".to_string(),
            );
            (range, frags)
        }
    }
}

pub fn render_number_unit(source: &str, node: &NumberUnit, depth: usize) -> RenderedOutput {
    match node {
        NumberUnit::WeiKeyword(ref element) => {
            let (range, mut frags) = render_terminal(source, &element.range);

            frags.insert(0, " \u{25ba} (wei_keyword\u{a789} WeiKeyword)".to_string());
            (range, frags)
        }

        NumberUnit::GweiKeyword(ref element) => {
            let (range, mut frags) = render_terminal(source, &element.range);

            frags.insert(
                0,
                " \u{25ba} (gwei_keyword\u{a789} GweiKeyword)".to_string(),
            );
            (range, frags)
        }

        NumberUnit::EtherKeyword(ref element) => {
            let (range, mut frags) = render_terminal(source, &element.range);

            frags.insert(
                0,
                " \u{25ba} (ether_keyword\u{a789} EtherKeyword)".to_string(),
            );
            (range, frags)
        }

        NumberUnit::SecondsKeyword(ref element) => {
            let (range, mut frags) = render_terminal(source, &element.range);

            frags.insert(
                0,
                " \u{25ba} (seconds_keyword\u{a789} SecondsKeyword)".to_string(),
            );
            (range, frags)
        }

        NumberUnit::MinutesKeyword(ref element) => {
            let (range, mut frags) = render_terminal(source, &element.range);

            frags.insert(
                0,
                " \u{25ba} (minutes_keyword\u{a789} MinutesKeyword)".to_string(),
            );
            (range, frags)
        }

        NumberUnit::HoursKeyword(ref element) => {
            let (range, mut frags) = render_terminal(source, &element.range);

            frags.insert(
                0,
                " \u{25ba} (hours_keyword\u{a789} HoursKeyword)".to_string(),
            );
            (range, frags)
        }

        NumberUnit::DaysKeyword(ref element) => {
            let (range, mut frags) = render_terminal(source, &element.range);

            frags.insert(
                0,
                " \u{25ba} (days_keyword\u{a789} DaysKeyword)".to_string(),
            );
            (range, frags)
        }

        NumberUnit::WeeksKeyword(ref element) => {
            let (range, mut frags) = render_terminal(source, &element.range);

            frags.insert(
                0,
                " \u{25ba} (weeks_keyword\u{a789} WeeksKeyword)".to_string(),
            );
            (range, frags)
        }
    }
}

pub fn render_pragma(source: &str, node: &Pragma, depth: usize) -> RenderedOutput {
    match node {
        Pragma::VersionPragma(ref element) => {
            let (range, mut frags) = render_version_pragma(source, element, depth);

            frags.insert(
                0,
                " \u{25ba} (version_pragma\u{a789} VersionPragma)".to_string(),
            );
            (range, frags)
        }

        Pragma::AbicoderPragma(ref element) => {
            let (range, mut frags) = render_abicoder_pragma(source, element, depth);

            frags.insert(
                0,
                " \u{25ba} (abicoder_pragma\u{a789} AbicoderPragma)".to_string(),
            );
            (range, frags)
        }

        Pragma::ExperimentalPragma(ref element) => {
            let (range, mut frags) = render_experimental_pragma(source, element, depth);

            frags.insert(
                0,
                " \u{25ba} (experimental_pragma\u{a789} ExperimentalPragma)".to_string(),
            );
            (range, frags)
        }
    }
}

pub fn render_receive_function_attribute(
    source: &str,
    node: &ReceiveFunctionAttribute,
    depth: usize,
) -> RenderedOutput {
    match node {
        ReceiveFunctionAttribute::ModifierInvocation(ref element) => {
            let (range, mut frags) = render_modifier_invocation(source, element, depth);

            frags.insert(
                0,
                " \u{25ba} (modifier_invocation\u{a789} ModifierInvocation)".to_string(),
            );
            (range, frags)
        }

        ReceiveFunctionAttribute::OverrideSpecifier(ref element) => {
            let (range, mut frags) = render_override_specifier(source, element, depth);

            frags.insert(
                0,
                " \u{25ba} (override_specifier\u{a789} OverrideSpecifier)".to_string(),
            );
            (range, frags)
        }

        ReceiveFunctionAttribute::ExternalKeyword(ref element) => {
            let (range, mut frags) = render_terminal(source, &element.range);

            frags.insert(
                0,
                " \u{25ba} (external_keyword\u{a789} ExternalKeyword)".to_string(),
            );
            (range, frags)
        }

        ReceiveFunctionAttribute::PayableKeyword(ref element) => {
            let (range, mut frags) = render_terminal(source, &element.range);

            frags.insert(
                0,
                " \u{25ba} (payable_keyword\u{a789} PayableKeyword)".to_string(),
            );
            (range, frags)
        }

        ReceiveFunctionAttribute::VirtualKeyword(ref element) => {
            let (range, mut frags) = render_terminal(source, &element.range);

            frags.insert(
                0,
                " \u{25ba} (virtual_keyword\u{a789} VirtualKeyword)".to_string(),
            );
            (range, frags)
        }
    }
}

pub fn render_source_unit_member(
    source: &str,
    node: &SourceUnitMember,
    depth: usize,
) -> RenderedOutput {
    match node {
        SourceUnitMember::PragmaDirective(ref element) => {
            let (range, mut frags) = render_pragma_directive(source, element, depth);

            frags.insert(
                0,
                " \u{25ba} (pragma_directive\u{a789} PragmaDirective)".to_string(),
            );
            (range, frags)
        }

        SourceUnitMember::ImportDirective(ref element) => {
            let (range, mut frags) = render_import_directive(source, element, depth);

            frags.insert(
                0,
                " \u{25ba} (import_directive\u{a789} ImportDirective)".to_string(),
            );
            (range, frags)
        }

        SourceUnitMember::ContractDefinition(ref element) => {
            let (range, mut frags) = render_contract_definition(source, element, depth);

            frags.insert(
                0,
                " \u{25ba} (contract_definition\u{a789} ContractDefinition)".to_string(),
            );
            (range, frags)
        }

        SourceUnitMember::InterfaceDefinition(ref element) => {
            let (range, mut frags) = render_interface_definition(source, element, depth);

            frags.insert(
                0,
                " \u{25ba} (interface_definition\u{a789} InterfaceDefinition)".to_string(),
            );
            (range, frags)
        }

        SourceUnitMember::LibraryDefinition(ref element) => {
            let (range, mut frags) = render_library_definition(source, element, depth);

            frags.insert(
                0,
                " \u{25ba} (library_definition\u{a789} LibraryDefinition)".to_string(),
            );
            (range, frags)
        }

        SourceUnitMember::StructDefinition(ref element) => {
            let (range, mut frags) = render_struct_definition(source, element, depth);

            frags.insert(
                0,
                " \u{25ba} (struct_definition\u{a789} StructDefinition)".to_string(),
            );
            (range, frags)
        }

        SourceUnitMember::EnumDefinition(ref element) => {
            let (range, mut frags) = render_enum_definition(source, element, depth);

            frags.insert(
                0,
                " \u{25ba} (enum_definition\u{a789} EnumDefinition)".to_string(),
            );
            (range, frags)
        }

        SourceUnitMember::FunctionDefinition(ref element) => {
            let (range, mut frags) = render_function_definition(source, element, depth);

            frags.insert(
                0,
                " \u{25ba} (function_definition\u{a789} FunctionDefinition)".to_string(),
            );
            (range, frags)
        }

        SourceUnitMember::ErrorDefinition(ref element) => {
            let (range, mut frags) = render_error_definition(source, element, depth);

            frags.insert(
                0,
                " \u{25ba} (error_definition\u{a789} ErrorDefinition)".to_string(),
            );
            (range, frags)
        }

        SourceUnitMember::UserDefinedValueTypeDefinition(ref element) => {
            let (range, mut frags) =
                render_user_defined_value_type_definition(source, element, depth);

            frags.insert(0, " \u{25ba} (user_defined_value_type_definition\u{a789} UserDefinedValueTypeDefinition)".to_string());
            (range, frags)
        }

        SourceUnitMember::UsingDirective(ref element) => {
            let (range, mut frags) = render_using_directive(source, element, depth);

            frags.insert(
                0,
                " \u{25ba} (using_directive\u{a789} UsingDirective)".to_string(),
            );
            (range, frags)
        }

        SourceUnitMember::EventDefinition(ref element) => {
            let (range, mut frags) = render_event_definition(source, element, depth);

            frags.insert(
                0,
                " \u{25ba} (event_definition\u{a789} EventDefinition)".to_string(),
            );
            (range, frags)
        }

        SourceUnitMember::ConstantDefinition(ref element) => {
            let (range, mut frags) = render_constant_definition(source, element, depth);

            frags.insert(
                0,
                " \u{25ba} (constant_definition\u{a789} ConstantDefinition)".to_string(),
            );
            (range, frags)
        }
    }
}

pub fn render_state_variable_attribute(
    source: &str,
    node: &StateVariableAttribute,
    depth: usize,
) -> RenderedOutput {
    match node {
        StateVariableAttribute::OverrideSpecifier(ref element) => {
            let (range, mut frags) = render_override_specifier(source, element, depth);

            frags.insert(
                0,
                " \u{25ba} (override_specifier\u{a789} OverrideSpecifier)".to_string(),
            );
            (range, frags)
        }

        StateVariableAttribute::ConstantKeyword(ref element) => {
            let (range, mut frags) = render_terminal(source, &element.range);

            frags.insert(
                0,
                " \u{25ba} (constant_keyword\u{a789} ConstantKeyword)".to_string(),
            );
            (range, frags)
        }

        StateVariableAttribute::InternalKeyword(ref element) => {
            let (range, mut frags) = render_terminal(source, &element.range);

            frags.insert(
                0,
                " \u{25ba} (internal_keyword\u{a789} InternalKeyword)".to_string(),
            );
            (range, frags)
        }

        StateVariableAttribute::PrivateKeyword(ref element) => {
            let (range, mut frags) = render_terminal(source, &element.range);

            frags.insert(
                0,
                " \u{25ba} (private_keyword\u{a789} PrivateKeyword)".to_string(),
            );
            (range, frags)
        }

        StateVariableAttribute::PublicKeyword(ref element) => {
            let (range, mut frags) = render_terminal(source, &element.range);

            frags.insert(
                0,
                " \u{25ba} (public_keyword\u{a789} PublicKeyword)".to_string(),
            );
            (range, frags)
        }

        StateVariableAttribute::ImmutableKeyword(ref element) => {
            let (range, mut frags) = render_terminal(source, &element.range);

            frags.insert(
                0,
                " \u{25ba} (immutable_keyword\u{a789} ImmutableKeyword)".to_string(),
            );
            (range, frags)
        }

        StateVariableAttribute::TransientKeyword(ref element) => {
            let (range, mut frags) = render_terminal(source, &element.range);

            frags.insert(
                0,
                " \u{25ba} (transient_keyword\u{a789} TransientKeyword)".to_string(),
            );
            (range, frags)
        }
    }
}

pub fn render_statement(source: &str, node: &Statement, depth: usize) -> RenderedOutput {
    match node {
        Statement::IfStatement(ref element) => {
            let (range, mut frags) = render_if_statement(source, element, depth);

            frags.insert(
                0,
                " \u{25ba} (if_statement\u{a789} IfStatement)".to_string(),
            );
            (range, frags)
        }

        Statement::ForStatement(ref element) => {
            let (range, mut frags) = render_for_statement(source, element, depth);

            frags.insert(
                0,
                " \u{25ba} (for_statement\u{a789} ForStatement)".to_string(),
            );
            (range, frags)
        }

        Statement::WhileStatement(ref element) => {
            let (range, mut frags) = render_while_statement(source, element, depth);

            frags.insert(
                0,
                " \u{25ba} (while_statement\u{a789} WhileStatement)".to_string(),
            );
            (range, frags)
        }

        Statement::DoWhileStatement(ref element) => {
            let (range, mut frags) = render_do_while_statement(source, element, depth);

            frags.insert(
                0,
                " \u{25ba} (do_while_statement\u{a789} DoWhileStatement)".to_string(),
            );
            (range, frags)
        }

        Statement::ContinueStatement(ref element) => {
            let (range, mut frags) = render_continue_statement(source, element, depth);

            frags.insert(
                0,
                " \u{25ba} (continue_statement\u{a789} ContinueStatement)".to_string(),
            );
            (range, frags)
        }

        Statement::BreakStatement(ref element) => {
            let (range, mut frags) = render_break_statement(source, element, depth);

            frags.insert(
                0,
                " \u{25ba} (break_statement\u{a789} BreakStatement)".to_string(),
            );
            (range, frags)
        }

        Statement::ReturnStatement(ref element) => {
            let (range, mut frags) = render_return_statement(source, element, depth);

            frags.insert(
                0,
                " \u{25ba} (return_statement\u{a789} ReturnStatement)".to_string(),
            );
            (range, frags)
        }

        Statement::EmitStatement(ref element) => {
            let (range, mut frags) = render_emit_statement(source, element, depth);

            frags.insert(
                0,
                " \u{25ba} (emit_statement\u{a789} EmitStatement)".to_string(),
            );
            (range, frags)
        }

        Statement::TryStatement(ref element) => {
            let (range, mut frags) = render_try_statement(source, element, depth);

            frags.insert(
                0,
                " \u{25ba} (try_statement\u{a789} TryStatement)".to_string(),
            );
            (range, frags)
        }

        Statement::RevertStatement(ref element) => {
            let (range, mut frags) = render_revert_statement(source, element, depth);

            frags.insert(
                0,
                " \u{25ba} (revert_statement\u{a789} RevertStatement)".to_string(),
            );
            (range, frags)
        }

        Statement::AssemblyStatement(ref element) => {
            let (range, mut frags) = render_assembly_statement(source, element, depth);

            frags.insert(
                0,
                " \u{25ba} (assembly_statement\u{a789} AssemblyStatement)".to_string(),
            );
            (range, frags)
        }

        Statement::Block(ref element) => {
            let (range, mut frags) = render_block(source, element, depth);

            frags.insert(0, " \u{25ba} (block\u{a789} Block)".to_string());
            (range, frags)
        }

        Statement::UncheckedBlock(ref element) => {
            let (range, mut frags) = render_unchecked_block(source, element, depth);

            frags.insert(
                0,
                " \u{25ba} (unchecked_block\u{a789} UncheckedBlock)".to_string(),
            );
            (range, frags)
        }

        Statement::VariableDeclarationStatement(ref element) => {
            let (range, mut frags) = render_variable_declaration_statement(source, element, depth);

            frags.insert(
                0,
                " \u{25ba} (variable_declaration_statement\u{a789} VariableDeclarationStatement)"
                    .to_string(),
            );
            (range, frags)
        }

        Statement::ExpressionStatement(ref element) => {
            let (range, mut frags) = render_expression_statement(source, element, depth);

            frags.insert(
                0,
                " \u{25ba} (expression_statement\u{a789} ExpressionStatement)".to_string(),
            );
            (range, frags)
        }
    }
}

pub fn render_storage_location(
    source: &str,
    node: &StorageLocation,
    depth: usize,
) -> RenderedOutput {
    match node {
        StorageLocation::MemoryKeyword(ref element) => {
            let (range, mut frags) = render_terminal(source, &element.range);

            frags.insert(
                0,
                " \u{25ba} (memory_keyword\u{a789} MemoryKeyword)".to_string(),
            );
            (range, frags)
        }

        StorageLocation::StorageKeyword(ref element) => {
            let (range, mut frags) = render_terminal(source, &element.range);

            frags.insert(
                0,
                " \u{25ba} (storage_keyword\u{a789} StorageKeyword)".to_string(),
            );
            (range, frags)
        }

        StorageLocation::CallDataKeyword(ref element) => {
            let (range, mut frags) = render_terminal(source, &element.range);

            frags.insert(
                0,
                " \u{25ba} (call_data_keyword\u{a789} CallDataKeyword)".to_string(),
            );
            (range, frags)
        }
    }
}

pub fn render_string_expression(
    source: &str,
    node: &StringExpression,
    depth: usize,
) -> RenderedOutput {
    match node {
        StringExpression::StringLiterals(ref element) => {
            let (range, mut frags) = render_string_literals(source, element, depth);

            frags.insert(
                0,
                " \u{25ba} (string_literals\u{a789} StringLiterals)".to_string(),
            );
            (range, frags)
        }

        StringExpression::HexStringLiterals(ref element) => {
            let (range, mut frags) = render_hex_string_literals(source, element, depth);

            frags.insert(
                0,
                " \u{25ba} (hex_string_literals\u{a789} HexStringLiterals)".to_string(),
            );
            (range, frags)
        }

        StringExpression::UnicodeStringLiterals(ref element) => {
            let (range, mut frags) = render_unicode_string_literals(source, element, depth);

            frags.insert(
                0,
                " \u{25ba} (unicode_string_literals\u{a789} UnicodeStringLiterals)".to_string(),
            );
            (range, frags)
        }
    }
}

pub fn render_type_name(source: &str, node: &TypeName, depth: usize) -> RenderedOutput {
    match node {
        TypeName::ArrayTypeName(ref element) => {
            let (range, mut frags) = render_array_type_name(source, element, depth);

            frags.insert(
                0,
                " \u{25ba} (array_type_name\u{a789} ArrayTypeName)".to_string(),
            );
            (range, frags)
        }

        TypeName::FunctionType(ref element) => {
            let (range, mut frags) = render_function_type(source, element, depth);

            frags.insert(
                0,
                " \u{25ba} (function_type\u{a789} FunctionType)".to_string(),
            );
            (range, frags)
        }

        TypeName::MappingType(ref element) => {
            let (range, mut frags) = render_mapping_type(source, element, depth);

            frags.insert(
                0,
                " \u{25ba} (mapping_type\u{a789} MappingType)".to_string(),
            );
            (range, frags)
        }

        TypeName::ElementaryType(ref element) => {
            let (range, mut frags) = render_elementary_type(source, element, depth);

            frags.insert(
                0,
                " \u{25ba} (elementary_type\u{a789} ElementaryType)".to_string(),
            );
            (range, frags)
        }

        TypeName::IdentifierPath(ref element) => {
            let (range, mut frags) = render_identifier_path(source, element, depth);

            frags.insert(
                0,
                " \u{25ba} (identifier_path\u{a789} IdentifierPath)".to_string(),
            );
            (range, frags)
        }
    }
}

pub fn render_using_clause(source: &str, node: &UsingClause, depth: usize) -> RenderedOutput {
    match node {
        UsingClause::IdentifierPath(ref element) => {
            let (range, mut frags) = render_identifier_path(source, element, depth);

            frags.insert(
                0,
                " \u{25ba} (identifier_path\u{a789} IdentifierPath)".to_string(),
            );
            (range, frags)
        }

        UsingClause::UsingDeconstruction(ref element) => {
            let (range, mut frags) = render_using_deconstruction(source, element, depth);

            frags.insert(
                0,
                " \u{25ba} (using_deconstruction\u{a789} UsingDeconstruction)".to_string(),
            );
            (range, frags)
        }
    }
}

pub fn render_using_operator(source: &str, node: &UsingOperator, depth: usize) -> RenderedOutput {
    match node {
        UsingOperator::Ampersand(ref element) => {
            let (range, mut frags) = render_terminal(source, &element.range);

            frags.insert(0, " \u{25ba} (ampersand\u{a789} Ampersand)".to_string());
            (range, frags)
        }

        UsingOperator::Asterisk(ref element) => {
            let (range, mut frags) = render_terminal(source, &element.range);

            frags.insert(0, " \u{25ba} (asterisk\u{a789} Asterisk)".to_string());
            (range, frags)
        }

        UsingOperator::BangEqual(ref element) => {
            let (range, mut frags) = render_terminal(source, &element.range);

            frags.insert(0, " \u{25ba} (bang_equal\u{a789} BangEqual)".to_string());
            (range, frags)
        }

        UsingOperator::Bar(ref element) => {
            let (range, mut frags) = render_terminal(source, &element.range);

            frags.insert(0, " \u{25ba} (bar\u{a789} Bar)".to_string());
            (range, frags)
        }

        UsingOperator::Caret(ref element) => {
            let (range, mut frags) = render_terminal(source, &element.range);

            frags.insert(0, " \u{25ba} (caret\u{a789} Caret)".to_string());
            (range, frags)
        }

        UsingOperator::EqualEqual(ref element) => {
            let (range, mut frags) = render_terminal(source, &element.range);

            frags.insert(0, " \u{25ba} (equal_equal\u{a789} EqualEqual)".to_string());
            (range, frags)
        }

        UsingOperator::GreaterThan(ref element) => {
            let (range, mut frags) = render_terminal(source, &element.range);

            frags.insert(
                0,
                " \u{25ba} (greater_than\u{a789} GreaterThan)".to_string(),
            );
            (range, frags)
        }

        UsingOperator::GreaterThanEqual(ref element) => {
            let (range, mut frags) = render_terminal(source, &element.range);

            frags.insert(
                0,
                " \u{25ba} (greater_than_equal\u{a789} GreaterThanEqual)".to_string(),
            );
            (range, frags)
        }

        UsingOperator::LessThan(ref element) => {
            let (range, mut frags) = render_terminal(source, &element.range);

            frags.insert(0, " \u{25ba} (less_than\u{a789} LessThan)".to_string());
            (range, frags)
        }

        UsingOperator::LessThanEqual(ref element) => {
            let (range, mut frags) = render_terminal(source, &element.range);

            frags.insert(
                0,
                " \u{25ba} (less_than_equal\u{a789} LessThanEqual)".to_string(),
            );
            (range, frags)
        }

        UsingOperator::Minus(ref element) => {
            let (range, mut frags) = render_terminal(source, &element.range);

            frags.insert(0, " \u{25ba} (minus\u{a789} Minus)".to_string());
            (range, frags)
        }

        UsingOperator::Percent(ref element) => {
            let (range, mut frags) = render_terminal(source, &element.range);

            frags.insert(0, " \u{25ba} (percent\u{a789} Percent)".to_string());
            (range, frags)
        }

        UsingOperator::Plus(ref element) => {
            let (range, mut frags) = render_terminal(source, &element.range);

            frags.insert(0, " \u{25ba} (plus\u{a789} Plus)".to_string());
            (range, frags)
        }

        UsingOperator::Slash(ref element) => {
            let (range, mut frags) = render_terminal(source, &element.range);

            frags.insert(0, " \u{25ba} (slash\u{a789} Slash)".to_string());
            (range, frags)
        }

        UsingOperator::Tilde(ref element) => {
            let (range, mut frags) = render_terminal(source, &element.range);

            frags.insert(0, " \u{25ba} (tilde\u{a789} Tilde)".to_string());
            (range, frags)
        }
    }
}

pub fn render_using_target(source: &str, node: &UsingTarget, depth: usize) -> RenderedOutput {
    match node {
        UsingTarget::TypeName(ref element) => {
            let (range, mut frags) = render_type_name(source, element, depth);

            frags.insert(0, " \u{25ba} (type_name\u{a789} TypeName)".to_string());
            (range, frags)
        }

        UsingTarget::Asterisk(ref element) => {
            let (range, mut frags) = render_terminal(source, &element.range);

            frags.insert(0, " \u{25ba} (asterisk\u{a789} Asterisk)".to_string());
            (range, frags)
        }
    }
}

pub fn render_variable_declaration_target(
    source: &str,
    node: &VariableDeclarationTarget,
    depth: usize,
) -> RenderedOutput {
    match node {
        VariableDeclarationTarget::SingleTypedDeclaration(ref element) => {
            let (range, mut frags) = render_single_typed_declaration(source, element, depth);

            frags.insert(
                0,
                " \u{25ba} (single_typed_declaration\u{a789} SingleTypedDeclaration)".to_string(),
            );
            (range, frags)
        }

        VariableDeclarationTarget::MultiTypedDeclaration(ref element) => {
            let (range, mut frags) = render_multi_typed_declaration(source, element, depth);

            frags.insert(
                0,
                " \u{25ba} (multi_typed_declaration\u{a789} MultiTypedDeclaration)".to_string(),
            );
            (range, frags)
        }
    }
}

pub fn render_version_expression(
    source: &str,
    node: &VersionExpression,
    depth: usize,
) -> RenderedOutput {
    match node {
        VersionExpression::VersionRange(ref element) => {
            let (range, mut frags) = render_version_range(source, element, depth);

            frags.insert(
                0,
                " \u{25ba} (version_range\u{a789} VersionRange)".to_string(),
            );
            (range, frags)
        }

        VersionExpression::VersionTerm(ref element) => {
            let (range, mut frags) = render_version_term(source, element, depth);

            frags.insert(
                0,
                " \u{25ba} (version_term\u{a789} VersionTerm)".to_string(),
            );
            (range, frags)
        }
    }
}

pub fn render_version_literal(source: &str, node: &VersionLiteral, depth: usize) -> RenderedOutput {
    match node {
        VersionLiteral::SimpleVersionLiteral(ref element) => {
            let (range, mut frags) = render_simple_version_literal(source, element, depth);

            frags.insert(
                0,
                " \u{25ba} (simple_version_literal\u{a789} SimpleVersionLiteral)".to_string(),
            );
            (range, frags)
        }

        VersionLiteral::PragmaStringLiteral(ref element) => {
            let (range, mut frags) = render_terminal(source, &element.range);

            frags.insert(
                0,
                " \u{25ba} (pragma_string_literal\u{a789} PragmaStringLiteral)".to_string(),
            );
            (range, frags)
        }
    }
}

pub fn render_version_operator(
    source: &str,
    node: &VersionOperator,
    depth: usize,
) -> RenderedOutput {
    match node {
        VersionOperator::PragmaCaret(ref element) => {
            let (range, mut frags) = render_terminal(source, &element.range);

            frags.insert(
                0,
                " \u{25ba} (pragma_caret\u{a789} PragmaCaret)".to_string(),
            );
            (range, frags)
        }

        VersionOperator::PragmaTilde(ref element) => {
            let (range, mut frags) = render_terminal(source, &element.range);

            frags.insert(
                0,
                " \u{25ba} (pragma_tilde\u{a789} PragmaTilde)".to_string(),
            );
            (range, frags)
        }

        VersionOperator::PragmaEqual(ref element) => {
            let (range, mut frags) = render_terminal(source, &element.range);

            frags.insert(
                0,
                " \u{25ba} (pragma_equal\u{a789} PragmaEqual)".to_string(),
            );
            (range, frags)
        }

        VersionOperator::PragmaLessThan(ref element) => {
            let (range, mut frags) = render_terminal(source, &element.range);

            frags.insert(
                0,
                " \u{25ba} (pragma_less_than\u{a789} PragmaLessThan)".to_string(),
            );
            (range, frags)
        }

        VersionOperator::PragmaGreaterThan(ref element) => {
            let (range, mut frags) = render_terminal(source, &element.range);

            frags.insert(
                0,
                " \u{25ba} (pragma_greater_than\u{a789} PragmaGreaterThan)".to_string(),
            );
            (range, frags)
        }

        VersionOperator::PragmaLessThanEqual(ref element) => {
            let (range, mut frags) = render_terminal(source, &element.range);

            frags.insert(
                0,
                " \u{25ba} (pragma_less_than_equal\u{a789} PragmaLessThanEqual)".to_string(),
            );
            (range, frags)
        }

        VersionOperator::PragmaGreaterThanEqual(ref element) => {
            let (range, mut frags) = render_terminal(source, &element.range);

            frags.insert(
                0,
                " \u{25ba} (pragma_greater_than_equal\u{a789} PragmaGreaterThanEqual)".to_string(),
            );
            (range, frags)
        }
    }
}

pub fn render_yul_expression(source: &str, node: &YulExpression, depth: usize) -> RenderedOutput {
    match node {
        YulExpression::YulFunctionCallExpression(ref element) => {
            let (range, mut frags) = render_yul_function_call_expression(source, element, depth);

            frags.insert(
                0,
                " \u{25ba} (yul_function_call_expression\u{a789} YulFunctionCallExpression)"
                    .to_string(),
            );
            (range, frags)
        }

        YulExpression::YulLiteral(ref element) => {
            let (range, mut frags) = render_yul_literal(source, element, depth);

            frags.insert(0, " \u{25ba} (yul_literal\u{a789} YulLiteral)".to_string());
            (range, frags)
        }

        YulExpression::YulPath(ref element) => {
            let (range, mut frags) = render_yul_path(source, element, depth);

            frags.insert(0, " \u{25ba} (yul_path\u{a789} YulPath)".to_string());
            (range, frags)
        }
    }
}

pub fn render_yul_literal(source: &str, node: &YulLiteral, depth: usize) -> RenderedOutput {
    match node {
        YulLiteral::YulTrueKeyword(ref element) => {
            let (range, mut frags) = render_terminal(source, &element.range);

            frags.insert(
                0,
                " \u{25ba} (yul_true_keyword\u{a789} YulTrueKeyword)".to_string(),
            );
            (range, frags)
        }

        YulLiteral::YulFalseKeyword(ref element) => {
            let (range, mut frags) = render_terminal(source, &element.range);

            frags.insert(
                0,
                " \u{25ba} (yul_false_keyword\u{a789} YulFalseKeyword)".to_string(),
            );
            (range, frags)
        }

        YulLiteral::YulDecimalLiteral(ref element) => {
            let (range, mut frags) = render_terminal(source, &element.range);

            frags.insert(
                0,
                " \u{25ba} (yul_decimal_literal\u{a789} YulDecimalLiteral)".to_string(),
            );
            (range, frags)
        }

        YulLiteral::YulHexLiteral(ref element) => {
            let (range, mut frags) = render_terminal(source, &element.range);

            frags.insert(
                0,
                " \u{25ba} (yul_hex_literal\u{a789} YulHexLiteral)".to_string(),
            );
            (range, frags)
        }

        YulLiteral::YulHexStringLiteral(ref element) => {
            let (range, mut frags) = render_terminal(source, &element.range);

            frags.insert(
                0,
                " \u{25ba} (yul_hex_string_literal\u{a789} YulHexStringLiteral)".to_string(),
            );
            (range, frags)
        }

        YulLiteral::YulStringLiteral(ref element) => {
            let (range, mut frags) = render_terminal(source, &element.range);

            frags.insert(
                0,
                " \u{25ba} (yul_string_literal\u{a789} YulStringLiteral)".to_string(),
            );
            (range, frags)
        }
    }
}

pub fn render_yul_statement(source: &str, node: &YulStatement, depth: usize) -> RenderedOutput {
    match node {
        YulStatement::YulBlock(ref element) => {
            let (range, mut frags) = render_yul_block(source, element, depth);

            frags.insert(0, " \u{25ba} (yul_block\u{a789} YulBlock)".to_string());
            (range, frags)
        }

        YulStatement::YulFunctionDefinition(ref element) => {
            let (range, mut frags) = render_yul_function_definition(source, element, depth);

            frags.insert(
                0,
                " \u{25ba} (yul_function_definition\u{a789} YulFunctionDefinition)".to_string(),
            );
            (range, frags)
        }

        YulStatement::YulIfStatement(ref element) => {
            let (range, mut frags) = render_yul_if_statement(source, element, depth);

            frags.insert(
                0,
                " \u{25ba} (yul_if_statement\u{a789} YulIfStatement)".to_string(),
            );
            (range, frags)
        }

        YulStatement::YulForStatement(ref element) => {
            let (range, mut frags) = render_yul_for_statement(source, element, depth);

            frags.insert(
                0,
                " \u{25ba} (yul_for_statement\u{a789} YulForStatement)".to_string(),
            );
            (range, frags)
        }

        YulStatement::YulSwitchStatement(ref element) => {
            let (range, mut frags) = render_yul_switch_statement(source, element, depth);

            frags.insert(
                0,
                " \u{25ba} (yul_switch_statement\u{a789} YulSwitchStatement)".to_string(),
            );
            (range, frags)
        }

        YulStatement::YulLeaveStatement(ref element) => {
            let (range, mut frags) = render_yul_leave_statement(source, element, depth);

            frags.insert(
                0,
                " \u{25ba} (yul_leave_statement\u{a789} YulLeaveStatement)".to_string(),
            );
            (range, frags)
        }

        YulStatement::YulBreakStatement(ref element) => {
            let (range, mut frags) = render_yul_break_statement(source, element, depth);

            frags.insert(
                0,
                " \u{25ba} (yul_break_statement\u{a789} YulBreakStatement)".to_string(),
            );
            (range, frags)
        }

        YulStatement::YulContinueStatement(ref element) => {
            let (range, mut frags) = render_yul_continue_statement(source, element, depth);

            frags.insert(
                0,
                " \u{25ba} (yul_continue_statement\u{a789} YulContinueStatement)".to_string(),
            );
            (range, frags)
        }

        YulStatement::YulVariableAssignmentStatement(ref element) => {
            let (range, mut frags) =
                render_yul_variable_assignment_statement(source, element, depth);

            frags.insert(0, " \u{25ba} (yul_variable_assignment_statement\u{a789} YulVariableAssignmentStatement)".to_string());
            (range, frags)
        }

        YulStatement::YulVariableDeclarationStatement(ref element) => {
            let (range, mut frags) =
                render_yul_variable_declaration_statement(source, element, depth);

            frags.insert(0, " \u{25ba} (yul_variable_declaration_statement\u{a789} YulVariableDeclarationStatement)".to_string());
            (range, frags)
        }

        YulStatement::YulExpression(ref element) => {
            let (range, mut frags) = render_yul_expression(source, element, depth);

            frags.insert(
                0,
                " \u{25ba} (yul_expression\u{a789} YulExpression)".to_string(),
            );
            (range, frags)
        }
    }
}

pub fn render_yul_switch_case(source: &str, node: &YulSwitchCase, depth: usize) -> RenderedOutput {
    match node {
        YulSwitchCase::YulDefaultCase(ref element) => {
            let (range, mut frags) = render_yul_default_case(source, element, depth);

            frags.insert(
                0,
                " \u{25ba} (yul_default_case\u{a789} YulDefaultCase)".to_string(),
            );
            (range, frags)
        }

        YulSwitchCase::YulValueCase(ref element) => {
            let (range, mut frags) = render_yul_value_case(source, element, depth);

            frags.insert(
                0,
                " \u{25ba} (yul_value_case\u{a789} YulValueCase)".to_string(),
            );
            (range, frags)
        }
    }
}

//
// Repeated & Separated
//

pub fn render_array_values(source: &str, node: &ArrayValues, depth: usize) -> RenderedOutput {
    if node.elements.is_empty() {
        return (None, vec![": []\n".to_string()]);
    }
    let mut acc = ChildrenAccumulator::new(source, depth + 1);
    for element in &node.elements {
        acc.add(
            "item",
            "Expression",
            render_expression(source, element, depth + 1),
        );
    }
    acc.finish()
}

pub fn render_call_options(source: &str, node: &CallOptions, depth: usize) -> RenderedOutput {
    if node.elements.is_empty() {
        return (None, vec![": []\n".to_string()]);
    }
    let mut acc = ChildrenAccumulator::new(source, depth + 1);
    for element in &node.elements {
        acc.add(
            "item",
            "NamedArgument",
            render_named_argument(source, element, depth + 1),
        );
    }
    acc.finish()
}

pub fn render_catch_clauses(source: &str, node: &CatchClauses, depth: usize) -> RenderedOutput {
    if node.elements.is_empty() {
        return (None, vec![": []\n".to_string()]);
    }
    let mut acc = ChildrenAccumulator::new(source, depth + 1);
    for element in &node.elements {
        acc.add(
            "item",
            "CatchClause",
            render_catch_clause(source, element, depth + 1),
        );
    }
    acc.finish()
}

pub fn render_constructor_attributes(
    source: &str,
    node: &ConstructorAttributes,
    depth: usize,
) -> RenderedOutput {
    if node.elements.is_empty() {
        return (None, vec![": []\n".to_string()]);
    }
    let mut acc = ChildrenAccumulator::new(source, depth + 1);
    for element in &node.elements {
        acc.add(
            "item",
            "ConstructorAttribute",
            render_constructor_attribute(source, element, depth + 1),
        );
    }
    acc.finish()
}

pub fn render_contract_members(
    source: &str,
    node: &ContractMembers,
    depth: usize,
) -> RenderedOutput {
    if node.elements.is_empty() {
        return (None, vec![": []\n".to_string()]);
    }
    let mut acc = ChildrenAccumulator::new(source, depth + 1);
    for element in &node.elements {
        acc.add(
            "item",
            "ContractMember",
            render_contract_member(source, element, depth + 1),
        );
    }
    acc.finish()
}

pub fn render_contract_specifiers(
    source: &str,
    node: &ContractSpecifiers,
    depth: usize,
) -> RenderedOutput {
    if node.elements.is_empty() {
        return (None, vec![": []\n".to_string()]);
    }
    let mut acc = ChildrenAccumulator::new(source, depth + 1);
    for element in &node.elements {
        acc.add(
            "item",
            "ContractSpecifier",
            render_contract_specifier(source, element, depth + 1),
        );
    }
    acc.finish()
}

pub fn render_enum_members(source: &str, node: &EnumMembers, depth: usize) -> RenderedOutput {
    if node.elements.is_empty() {
        return (None, vec![": []\n".to_string()]);
    }
    let mut acc = ChildrenAccumulator::new(source, depth + 1);
    for element in &node.elements {
        acc.add(
            "item",
            "Identifier",
            render_terminal(source, &element.range),
        );
    }
    acc.finish()
}

pub fn render_error_parameters(
    source: &str,
    node: &ErrorParameters,
    depth: usize,
) -> RenderedOutput {
    if node.elements.is_empty() {
        return (None, vec![": []\n".to_string()]);
    }
    let mut acc = ChildrenAccumulator::new(source, depth + 1);
    for element in &node.elements {
        acc.add(
            "item",
            "ErrorParameter",
            render_error_parameter(source, element, depth + 1),
        );
    }
    acc.finish()
}

pub fn render_event_parameters(
    source: &str,
    node: &EventParameters,
    depth: usize,
) -> RenderedOutput {
    if node.elements.is_empty() {
        return (None, vec![": []\n".to_string()]);
    }
    let mut acc = ChildrenAccumulator::new(source, depth + 1);
    for element in &node.elements {
        acc.add(
            "item",
            "EventParameter",
            render_event_parameter(source, element, depth + 1),
        );
    }
    acc.finish()
}

pub fn render_fallback_function_attributes(
    source: &str,
    node: &FallbackFunctionAttributes,
    depth: usize,
) -> RenderedOutput {
    if node.elements.is_empty() {
        return (None, vec![": []\n".to_string()]);
    }
    let mut acc = ChildrenAccumulator::new(source, depth + 1);
    for element in &node.elements {
        acc.add(
            "item",
            "FallbackFunctionAttribute",
            render_fallback_function_attribute(source, element, depth + 1),
        );
    }
    acc.finish()
}

pub fn render_function_attributes(
    source: &str,
    node: &FunctionAttributes,
    depth: usize,
) -> RenderedOutput {
    if node.elements.is_empty() {
        return (None, vec![": []\n".to_string()]);
    }
    let mut acc = ChildrenAccumulator::new(source, depth + 1);
    for element in &node.elements {
        acc.add(
            "item",
            "FunctionAttribute",
            render_function_attribute(source, element, depth + 1),
        );
    }
    acc.finish()
}

pub fn render_function_type_attributes(
    source: &str,
    node: &FunctionTypeAttributes,
    depth: usize,
) -> RenderedOutput {
    if node.elements.is_empty() {
        return (None, vec![": []\n".to_string()]);
    }
    let mut acc = ChildrenAccumulator::new(source, depth + 1);
    for element in &node.elements {
        acc.add(
            "item",
            "FunctionTypeAttribute",
            render_function_type_attribute(source, element, depth + 1),
        );
    }
    acc.finish()
}

pub fn render_hex_string_literals(
    source: &str,
    node: &HexStringLiterals,
    depth: usize,
) -> RenderedOutput {
    if node.elements.is_empty() {
        return (None, vec![": []\n".to_string()]);
    }
    let mut acc = ChildrenAccumulator::new(source, depth + 1);
    for element in &node.elements {
        acc.add(
            "item",
            "HexStringLiteral",
            render_terminal(source, &element.range),
        );
    }
    acc.finish()
}

pub fn render_identifier_path(source: &str, node: &IdentifierPath, depth: usize) -> RenderedOutput {
    if node.elements.is_empty() {
        return (None, vec![": []\n".to_string()]);
    }
    let mut acc = ChildrenAccumulator::new(source, depth + 1);
    for element in &node.elements {
        acc.add(
            "item",
            "IdentifierPathElement",
            render_identifier_path_element(source, element, depth + 1),
        );
    }
    acc.finish()
}

pub fn render_import_deconstruction_symbols(
    source: &str,
    node: &ImportDeconstructionSymbols,
    depth: usize,
) -> RenderedOutput {
    if node.elements.is_empty() {
        return (None, vec![": []\n".to_string()]);
    }
    let mut acc = ChildrenAccumulator::new(source, depth + 1);
    for element in &node.elements {
        acc.add(
            "item",
            "ImportDeconstructionSymbol",
            render_import_deconstruction_symbol(source, element, depth + 1),
        );
    }
    acc.finish()
}

pub fn render_inheritance_types(
    source: &str,
    node: &InheritanceTypes,
    depth: usize,
) -> RenderedOutput {
    if node.elements.is_empty() {
        return (None, vec![": []\n".to_string()]);
    }
    let mut acc = ChildrenAccumulator::new(source, depth + 1);
    for element in &node.elements {
        acc.add(
            "item",
            "InheritanceType",
            render_inheritance_type(source, element, depth + 1),
        );
    }
    acc.finish()
}

pub fn render_interface_members(
    source: &str,
    node: &InterfaceMembers,
    depth: usize,
) -> RenderedOutput {
    if node.elements.is_empty() {
        return (None, vec![": []\n".to_string()]);
    }
    let mut acc = ChildrenAccumulator::new(source, depth + 1);
    for element in &node.elements {
        acc.add(
            "item",
            "ContractMember",
            render_contract_member(source, element, depth + 1),
        );
    }
    acc.finish()
}

pub fn render_library_members(source: &str, node: &LibraryMembers, depth: usize) -> RenderedOutput {
    if node.elements.is_empty() {
        return (None, vec![": []\n".to_string()]);
    }
    let mut acc = ChildrenAccumulator::new(source, depth + 1);
    for element in &node.elements {
        acc.add(
            "item",
            "ContractMember",
            render_contract_member(source, element, depth + 1),
        );
    }
    acc.finish()
}

pub fn render_modifier_attributes(
    source: &str,
    node: &ModifierAttributes,
    depth: usize,
) -> RenderedOutput {
    if node.elements.is_empty() {
        return (None, vec![": []\n".to_string()]);
    }
    let mut acc = ChildrenAccumulator::new(source, depth + 1);
    for element in &node.elements {
        acc.add(
            "item",
            "ModifierAttribute",
            render_modifier_attribute(source, element, depth + 1),
        );
    }
    acc.finish()
}

pub fn render_multi_typed_declaration_elements(
    source: &str,
    node: &MultiTypedDeclarationElements,
    depth: usize,
) -> RenderedOutput {
    if node.elements.is_empty() {
        return (None, vec![": []\n".to_string()]);
    }
    let mut acc = ChildrenAccumulator::new(source, depth + 1);
    for element in &node.elements {
        acc.add(
            "item",
            "MultiTypedDeclarationElement",
            render_multi_typed_declaration_element(source, element, depth + 1),
        );
    }
    acc.finish()
}

pub fn render_named_arguments(source: &str, node: &NamedArguments, depth: usize) -> RenderedOutput {
    if node.elements.is_empty() {
        return (None, vec![": []\n".to_string()]);
    }
    let mut acc = ChildrenAccumulator::new(source, depth + 1);
    for element in &node.elements {
        acc.add(
            "item",
            "NamedArgument",
            render_named_argument(source, element, depth + 1),
        );
    }
    acc.finish()
}

pub fn render_override_paths(source: &str, node: &OverridePaths, depth: usize) -> RenderedOutput {
    if node.elements.is_empty() {
        return (None, vec![": []\n".to_string()]);
    }
    let mut acc = ChildrenAccumulator::new(source, depth + 1);
    for element in &node.elements {
        acc.add(
            "item",
            "IdentifierPath",
            render_identifier_path(source, element, depth + 1),
        );
    }
    acc.finish()
}

pub fn render_parameters(source: &str, node: &Parameters, depth: usize) -> RenderedOutput {
    if node.elements.is_empty() {
        return (None, vec![": []\n".to_string()]);
    }
    let mut acc = ChildrenAccumulator::new(source, depth + 1);
    for element in &node.elements {
        acc.add(
            "item",
            "Parameter",
            render_parameter(source, element, depth + 1),
        );
    }
    acc.finish()
}

pub fn render_positional_arguments(
    source: &str,
    node: &PositionalArguments,
    depth: usize,
) -> RenderedOutput {
    if node.elements.is_empty() {
        return (None, vec![": []\n".to_string()]);
    }
    let mut acc = ChildrenAccumulator::new(source, depth + 1);
    for element in &node.elements {
        acc.add(
            "item",
            "Expression",
            render_expression(source, element, depth + 1),
        );
    }
    acc.finish()
}

pub fn render_receive_function_attributes(
    source: &str,
    node: &ReceiveFunctionAttributes,
    depth: usize,
) -> RenderedOutput {
    if node.elements.is_empty() {
        return (None, vec![": []\n".to_string()]);
    }
    let mut acc = ChildrenAccumulator::new(source, depth + 1);
    for element in &node.elements {
        acc.add(
            "item",
            "ReceiveFunctionAttribute",
            render_receive_function_attribute(source, element, depth + 1),
        );
    }
    acc.finish()
}

pub fn render_simple_version_literal(
    source: &str,
    node: &SimpleVersionLiteral,
    depth: usize,
) -> RenderedOutput {
    if node.elements.is_empty() {
        return (None, vec![": []\n".to_string()]);
    }
    let mut acc = ChildrenAccumulator::new(source, depth + 1);
    for element in &node.elements {
        acc.add(
            "item",
            "VersionSpecifier",
            render_terminal(source, &element.range),
        );
    }
    acc.finish()
}

pub fn render_source_unit_members(
    source: &str,
    node: &SourceUnitMembers,
    depth: usize,
) -> RenderedOutput {
    if node.elements.is_empty() {
        return (None, vec![": []\n".to_string()]);
    }
    let mut acc = ChildrenAccumulator::new(source, depth + 1);
    for element in &node.elements {
        acc.add(
            "item",
            "SourceUnitMember",
            render_source_unit_member(source, element, depth + 1),
        );
    }
    acc.finish()
}

pub fn render_state_variable_attributes(
    source: &str,
    node: &StateVariableAttributes,
    depth: usize,
) -> RenderedOutput {
    if node.elements.is_empty() {
        return (None, vec![": []\n".to_string()]);
    }
    let mut acc = ChildrenAccumulator::new(source, depth + 1);
    for element in &node.elements {
        acc.add(
            "item",
            "StateVariableAttribute",
            render_state_variable_attribute(source, element, depth + 1),
        );
    }
    acc.finish()
}

pub fn render_statements(source: &str, node: &Statements, depth: usize) -> RenderedOutput {
    if node.elements.is_empty() {
        return (None, vec![": []\n".to_string()]);
    }
    let mut acc = ChildrenAccumulator::new(source, depth + 1);
    for element in &node.elements {
        acc.add(
            "item",
            "Statement",
            render_statement(source, element, depth + 1),
        );
    }
    acc.finish()
}

pub fn render_string_literals(source: &str, node: &StringLiterals, depth: usize) -> RenderedOutput {
    if node.elements.is_empty() {
        return (None, vec![": []\n".to_string()]);
    }
    let mut acc = ChildrenAccumulator::new(source, depth + 1);
    for element in &node.elements {
        acc.add(
            "item",
            "StringLiteral",
            render_terminal(source, &element.range),
        );
    }
    acc.finish()
}

pub fn render_struct_members(source: &str, node: &StructMembers, depth: usize) -> RenderedOutput {
    if node.elements.is_empty() {
        return (None, vec![": []\n".to_string()]);
    }
    let mut acc = ChildrenAccumulator::new(source, depth + 1);
    for element in &node.elements {
        acc.add(
            "item",
            "StructMember",
            render_struct_member(source, element, depth + 1),
        );
    }
    acc.finish()
}

pub fn render_tuple_values(source: &str, node: &TupleValues, depth: usize) -> RenderedOutput {
    if node.elements.is_empty() {
        return (None, vec![": []\n".to_string()]);
    }
    let mut acc = ChildrenAccumulator::new(source, depth + 1);
    for element in &node.elements {
        acc.add(
            "item",
            "TupleValue",
            render_tuple_value(source, element, depth + 1),
        );
    }
    acc.finish()
}

pub fn render_unicode_string_literals(
    source: &str,
    node: &UnicodeStringLiterals,
    depth: usize,
) -> RenderedOutput {
    if node.elements.is_empty() {
        return (None, vec![": []\n".to_string()]);
    }
    let mut acc = ChildrenAccumulator::new(source, depth + 1);
    for element in &node.elements {
        acc.add(
            "item",
            "UnicodeStringLiteral",
            render_terminal(source, &element.range),
        );
    }
    acc.finish()
}

pub fn render_using_deconstruction_symbols(
    source: &str,
    node: &UsingDeconstructionSymbols,
    depth: usize,
) -> RenderedOutput {
    if node.elements.is_empty() {
        return (None, vec![": []\n".to_string()]);
    }
    let mut acc = ChildrenAccumulator::new(source, depth + 1);
    for element in &node.elements {
        acc.add(
            "item",
            "UsingDeconstructionSymbol",
            render_using_deconstruction_symbol(source, element, depth + 1),
        );
    }
    acc.finish()
}

pub fn render_version_expression_set(
    source: &str,
    node: &VersionExpressionSet,
    depth: usize,
) -> RenderedOutput {
    if node.elements.is_empty() {
        return (None, vec![": []\n".to_string()]);
    }
    let mut acc = ChildrenAccumulator::new(source, depth + 1);
    for element in &node.elements {
        acc.add(
            "item",
            "VersionExpression",
            render_version_expression(source, element, depth + 1),
        );
    }
    acc.finish()
}

pub fn render_version_expression_sets(
    source: &str,
    node: &VersionExpressionSets,
    depth: usize,
) -> RenderedOutput {
    if node.elements.is_empty() {
        return (None, vec![": []\n".to_string()]);
    }
    let mut acc = ChildrenAccumulator::new(source, depth + 1);
    for element in &node.elements {
        acc.add(
            "item",
            "VersionExpressionSet",
            render_version_expression_set(source, element, depth + 1),
        );
    }
    acc.finish()
}

pub fn render_yul_arguments(source: &str, node: &YulArguments, depth: usize) -> RenderedOutput {
    if node.elements.is_empty() {
        return (None, vec![": []\n".to_string()]);
    }
    let mut acc = ChildrenAccumulator::new(source, depth + 1);
    for element in &node.elements {
        acc.add(
            "item",
            "YulExpression",
            render_yul_expression(source, element, depth + 1),
        );
    }
    acc.finish()
}

pub fn render_yul_flags(source: &str, node: &YulFlags, depth: usize) -> RenderedOutput {
    if node.elements.is_empty() {
        return (None, vec![": []\n".to_string()]);
    }
    let mut acc = ChildrenAccumulator::new(source, depth + 1);
    for element in &node.elements {
        acc.add(
            "item",
            "YulStringLiteral",
            render_terminal(source, &element.range),
        );
    }
    acc.finish()
}

pub fn render_yul_parameters(source: &str, node: &YulParameters, depth: usize) -> RenderedOutput {
    if node.elements.is_empty() {
        return (None, vec![": []\n".to_string()]);
    }
    let mut acc = ChildrenAccumulator::new(source, depth + 1);
    for element in &node.elements {
        acc.add(
            "item",
            "YulIdentifier",
            render_terminal(source, &element.range),
        );
    }
    acc.finish()
}

pub fn render_yul_path(source: &str, node: &YulPath, depth: usize) -> RenderedOutput {
    if node.elements.is_empty() {
        return (None, vec![": []\n".to_string()]);
    }
    let mut acc = ChildrenAccumulator::new(source, depth + 1);
    for element in &node.elements {
        acc.add(
            "item",
            "YulIdentifier",
            render_terminal(source, &element.range),
        );
    }
    acc.finish()
}

pub fn render_yul_paths(source: &str, node: &YulPaths, depth: usize) -> RenderedOutput {
    if node.elements.is_empty() {
        return (None, vec![": []\n".to_string()]);
    }
    let mut acc = ChildrenAccumulator::new(source, depth + 1);
    for element in &node.elements {
        acc.add(
            "item",
            "YulPath",
            render_yul_path(source, element, depth + 1),
        );
    }
    acc.finish()
}

pub fn render_yul_statements(source: &str, node: &YulStatements, depth: usize) -> RenderedOutput {
    if node.elements.is_empty() {
        return (None, vec![": []\n".to_string()]);
    }
    let mut acc = ChildrenAccumulator::new(source, depth + 1);
    for element in &node.elements {
        acc.add(
            "item",
            "YulStatement",
            render_yul_statement(source, element, depth + 1),
        );
    }
    acc.finish()
}

pub fn render_yul_switch_cases(
    source: &str,
    node: &YulSwitchCases,
    depth: usize,
) -> RenderedOutput {
    if node.elements.is_empty() {
        return (None, vec![": []\n".to_string()]);
    }
    let mut acc = ChildrenAccumulator::new(source, depth + 1);
    for element in &node.elements {
        acc.add(
            "item",
            "YulSwitchCase",
            render_yul_switch_case(source, element, depth + 1),
        );
    }
    acc.finish()
}

pub fn render_yul_variable_names(
    source: &str,
    node: &YulVariableNames,
    depth: usize,
) -> RenderedOutput {
    if node.elements.is_empty() {
        return (None, vec![": []\n".to_string()]);
    }
    let mut acc = ChildrenAccumulator::new(source, depth + 1);
    for element in &node.elements {
        acc.add(
            "item",
            "YulIdentifier",
            render_terminal(source, &element.range),
        );
    }
    acc.finish()
}

// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

#![allow(clippy::too_many_lines)]
#![allow(unused_variables)]

#[allow(clippy::wildcard_imports)]
use slang_solidity_v2_cst::structured_cst::nodes::*;

use crate::cst_renderer::{
    format_label_kind, render_terminal, ChildrenAccumulator, RenderedOutput,
};

//
// Sequences:
//

pub fn render_abicoder_pragma(source: &str, node: &AbicoderPragma, depth: usize) -> RenderedOutput {
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
    node: &AdditiveExpression,
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

pub fn render_address_type(source: &str, node: &AddressType, depth: usize) -> RenderedOutput {
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

pub fn render_and_expression(source: &str, node: &AndExpression, depth: usize) -> RenderedOutput {
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
    node: &ArrayExpression,
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

pub fn render_array_type_name(source: &str, node: &ArrayTypeName, depth: usize) -> RenderedOutput {
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
    node: &AssemblyStatement,
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
    node: &AssignmentExpression,
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
    node: &BitwiseAndExpression,
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
    node: &BitwiseOrExpression,
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
    node: &BitwiseXorExpression,
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

pub fn render_block(source: &str, node: &Block, depth: usize) -> RenderedOutput {
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

pub fn render_break_statement(source: &str, node: &BreakStatement, depth: usize) -> RenderedOutput {
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
    node: &CallOptionsExpression,
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

pub fn render_catch_clause(source: &str, node: &CatchClause, depth: usize) -> RenderedOutput {
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
    node: &CatchClauseError,
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
    node: &ConditionalExpression,
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
    node: &ConstantDefinition,
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
    node: &ConstructorDefinition,
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
    node: &ContinueStatement,
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
    node: &ContractDefinition,
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
    node: &DecimalNumberExpression,
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
    node: &DoWhileStatement,
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

pub fn render_else_branch(source: &str, node: &ElseBranch, depth: usize) -> RenderedOutput {
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

pub fn render_emit_statement(source: &str, node: &EmitStatement, depth: usize) -> RenderedOutput {
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

pub fn render_enum_definition(source: &str, node: &EnumDefinition, depth: usize) -> RenderedOutput {
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
    node: &EqualityExpression,
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
    node: &ErrorDefinition,
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

pub fn render_error_parameter(source: &str, node: &ErrorParameter, depth: usize) -> RenderedOutput {
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
    node: &ErrorParametersDeclaration,
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
    node: &EventDefinition,
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

pub fn render_event_parameter(source: &str, node: &EventParameter, depth: usize) -> RenderedOutput {
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
    node: &EventParametersDeclaration,
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
    node: &ExperimentalPragma,
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
    node: &ExponentiationExpression,
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
    node: &ExpressionStatement,
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
    node: &FallbackFunctionDefinition,
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

pub fn render_for_statement(source: &str, node: &ForStatement, depth: usize) -> RenderedOutput {
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
    node: &FunctionCallExpression,
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
    node: &FunctionDefinition,
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

pub fn render_function_type(source: &str, node: &FunctionType, depth: usize) -> RenderedOutput {
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
    node: &HexNumberExpression,
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

pub fn render_if_statement(source: &str, node: &IfStatement, depth: usize) -> RenderedOutput {
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

pub fn render_import_alias(source: &str, node: &ImportAlias, depth: usize) -> RenderedOutput {
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
    node: &ImportDeconstruction,
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
    node: &ImportDeconstructionSymbol,
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
    node: &ImportDirective,
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
    node: &IndexAccessEnd,
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
    node: &IndexAccessExpression,
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
    node: &InequalityExpression,
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
    node: &InheritanceSpecifier,
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
    node: &InheritanceType,
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
    node: &InterfaceDefinition,
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
    node: &LibraryDefinition,
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

pub fn render_mapping_key(source: &str, node: &MappingKey, depth: usize) -> RenderedOutput {
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

pub fn render_mapping_type(source: &str, node: &MappingType, depth: usize) -> RenderedOutput {
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

pub fn render_mapping_value(source: &str, node: &MappingValue, depth: usize) -> RenderedOutput {
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
    node: &MemberAccessExpression,
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
    node: &ModifierDefinition,
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
    node: &ModifierInvocation,
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
    node: &MultiTypedDeclaration,
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
    node: &MultiTypedDeclarationElement,
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
    node: &MultiplicativeExpression,
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

pub fn render_named_argument(source: &str, node: &NamedArgument, depth: usize) -> RenderedOutput {
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
    node: &NamedArgumentGroup,
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
    node: &NamedArgumentsDeclaration,
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

pub fn render_named_import(source: &str, node: &NamedImport, depth: usize) -> RenderedOutput {
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

pub fn render_new_expression(source: &str, node: &NewExpression, depth: usize) -> RenderedOutput {
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

pub fn render_or_expression(source: &str, node: &OrExpression, depth: usize) -> RenderedOutput {
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
    node: &OverridePathsDeclaration,
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
    node: &OverrideSpecifier,
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

pub fn render_parameter(source: &str, node: &Parameter, depth: usize) -> RenderedOutput {
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
    node: &ParametersDeclaration,
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

pub fn render_path_import(source: &str, node: &PathImport, depth: usize) -> RenderedOutput {
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
    node: &PositionalArgumentsDeclaration,
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
    node: &PostfixExpression,
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
    node: &PragmaDirective,
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
    node: &PrefixExpression,
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
    node: &ReceiveFunctionDefinition,
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
    node: &ReturnStatement,
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
    node: &ReturnsDeclaration,
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
    node: &RevertStatement,
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
    node: &ShiftExpression,
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
    node: &SingleTypedDeclaration,
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

pub fn render_source_unit(source: &str, node: &SourceUnit, depth: usize) -> RenderedOutput {
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
    node: &StateVariableDefinition,
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
    node: &StateVariableDefinitionValue,
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
    node: &StorageLayoutSpecifier,
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
    node: &StructDefinition,
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

pub fn render_struct_member(source: &str, node: &StructMember, depth: usize) -> RenderedOutput {
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

pub fn render_try_statement(source: &str, node: &TryStatement, depth: usize) -> RenderedOutput {
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
    node: &TupleExpression,
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

pub fn render_tuple_value(source: &str, node: &TupleValue, depth: usize) -> RenderedOutput {
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

pub fn render_type_expression(source: &str, node: &TypeExpression, depth: usize) -> RenderedOutput {
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

pub fn render_unchecked_block(source: &str, node: &UncheckedBlock, depth: usize) -> RenderedOutput {
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
    node: &UserDefinedValueTypeDefinition,
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

pub fn render_using_alias(source: &str, node: &UsingAlias, depth: usize) -> RenderedOutput {
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
    node: &UsingDeconstruction,
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
    node: &UsingDeconstructionSymbol,
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

pub fn render_using_directive(source: &str, node: &UsingDirective, depth: usize) -> RenderedOutput {
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
    node: &VariableDeclaration,
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
    node: &VariableDeclarationStatement,
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
    node: &VariableDeclarationValue,
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

pub fn render_version_pragma(source: &str, node: &VersionPragma, depth: usize) -> RenderedOutput {
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

pub fn render_version_range(source: &str, node: &VersionRange, depth: usize) -> RenderedOutput {
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

pub fn render_version_term(source: &str, node: &VersionTerm, depth: usize) -> RenderedOutput {
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

pub fn render_while_statement(source: &str, node: &WhileStatement, depth: usize) -> RenderedOutput {
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

pub fn render_yul_block(source: &str, node: &YulBlock, depth: usize) -> RenderedOutput {
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
    node: &YulBreakStatement,
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
    node: &YulContinueStatement,
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
    node: &YulDefaultCase,
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
    node: &YulFlagsDeclaration,
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
    node: &YulForStatement,
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
    node: &YulFunctionCallExpression,
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
    node: &YulFunctionDefinition,
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
    node: &YulIfStatement,
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
    node: &YulLeaveStatement,
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
    node: &YulParametersDeclaration,
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
    node: &YulReturnsDeclaration,
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
    node: &YulSwitchStatement,
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

pub fn render_yul_value_case(source: &str, node: &YulValueCase, depth: usize) -> RenderedOutput {
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
    node: &YulVariableAssignmentStatement,
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
    node: &YulVariableDeclarationStatement,
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
    node: &YulVariableDeclarationValue,
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
                format!(
                    " \u{25ba} {}",
                    format_label_kind("abicoder_v1_keyword", "AbicoderV1Keyword")
                ),
            );
            (range, frags)
        }

        AbicoderVersion::AbicoderV2Keyword(ref element) => {
            let (range, mut frags) = render_terminal(source, &element.range);

            frags.insert(
                0,
                format!(
                    " \u{25ba} {}",
                    format_label_kind("abicoder_v2_keyword", "AbicoderV2Keyword")
                ),
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

            frags.insert(
                0,
                format!(
                    " \u{25ba} {}",
                    format_label_kind(
                        "positional_arguments_declaration",
                        "PositionalArgumentsDeclaration"
                    )
                ),
            );
            (range, frags)
        }

        ArgumentsDeclaration::NamedArgumentsDeclaration(ref element) => {
            let (range, mut frags) = render_named_arguments_declaration(source, element, depth);

            frags.insert(
                0,
                format!(
                    " \u{25ba} {}",
                    format_label_kind("named_arguments_declaration", "NamedArgumentsDeclaration")
                ),
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
                format!(
                    " \u{25ba} {}",
                    format_label_kind("modifier_invocation", "ModifierInvocation")
                ),
            );
            (range, frags)
        }

        ConstructorAttribute::InternalKeyword(ref element) => {
            let (range, mut frags) = render_terminal(source, &element.range);

            frags.insert(
                0,
                format!(
                    " \u{25ba} {}",
                    format_label_kind("internal_keyword", "InternalKeyword")
                ),
            );
            (range, frags)
        }

        ConstructorAttribute::PayableKeyword(ref element) => {
            let (range, mut frags) = render_terminal(source, &element.range);

            frags.insert(
                0,
                format!(
                    " \u{25ba} {}",
                    format_label_kind("payable_keyword", "PayableKeyword")
                ),
            );
            (range, frags)
        }

        ConstructorAttribute::PublicKeyword(ref element) => {
            let (range, mut frags) = render_terminal(source, &element.range);

            frags.insert(
                0,
                format!(
                    " \u{25ba} {}",
                    format_label_kind("public_keyword", "PublicKeyword")
                ),
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
                format!(
                    " \u{25ba} {}",
                    format_label_kind("using_directive", "UsingDirective")
                ),
            );
            (range, frags)
        }

        ContractMember::FunctionDefinition(ref element) => {
            let (range, mut frags) = render_function_definition(source, element, depth);

            frags.insert(
                0,
                format!(
                    " \u{25ba} {}",
                    format_label_kind("function_definition", "FunctionDefinition")
                ),
            );
            (range, frags)
        }

        ContractMember::ConstructorDefinition(ref element) => {
            let (range, mut frags) = render_constructor_definition(source, element, depth);

            frags.insert(
                0,
                format!(
                    " \u{25ba} {}",
                    format_label_kind("constructor_definition", "ConstructorDefinition")
                ),
            );
            (range, frags)
        }

        ContractMember::ReceiveFunctionDefinition(ref element) => {
            let (range, mut frags) = render_receive_function_definition(source, element, depth);

            frags.insert(
                0,
                format!(
                    " \u{25ba} {}",
                    format_label_kind("receive_function_definition", "ReceiveFunctionDefinition")
                ),
            );
            (range, frags)
        }

        ContractMember::FallbackFunctionDefinition(ref element) => {
            let (range, mut frags) = render_fallback_function_definition(source, element, depth);

            frags.insert(
                0,
                format!(
                    " \u{25ba} {}",
                    format_label_kind("fallback_function_definition", "FallbackFunctionDefinition")
                ),
            );
            (range, frags)
        }

        ContractMember::ModifierDefinition(ref element) => {
            let (range, mut frags) = render_modifier_definition(source, element, depth);

            frags.insert(
                0,
                format!(
                    " \u{25ba} {}",
                    format_label_kind("modifier_definition", "ModifierDefinition")
                ),
            );
            (range, frags)
        }

        ContractMember::StructDefinition(ref element) => {
            let (range, mut frags) = render_struct_definition(source, element, depth);

            frags.insert(
                0,
                format!(
                    " \u{25ba} {}",
                    format_label_kind("struct_definition", "StructDefinition")
                ),
            );
            (range, frags)
        }

        ContractMember::EnumDefinition(ref element) => {
            let (range, mut frags) = render_enum_definition(source, element, depth);

            frags.insert(
                0,
                format!(
                    " \u{25ba} {}",
                    format_label_kind("enum_definition", "EnumDefinition")
                ),
            );
            (range, frags)
        }

        ContractMember::EventDefinition(ref element) => {
            let (range, mut frags) = render_event_definition(source, element, depth);

            frags.insert(
                0,
                format!(
                    " \u{25ba} {}",
                    format_label_kind("event_definition", "EventDefinition")
                ),
            );
            (range, frags)
        }

        ContractMember::ErrorDefinition(ref element) => {
            let (range, mut frags) = render_error_definition(source, element, depth);

            frags.insert(
                0,
                format!(
                    " \u{25ba} {}",
                    format_label_kind("error_definition", "ErrorDefinition")
                ),
            );
            (range, frags)
        }

        ContractMember::UserDefinedValueTypeDefinition(ref element) => {
            let (range, mut frags) =
                render_user_defined_value_type_definition(source, element, depth);

            frags.insert(
                0,
                format!(
                    " \u{25ba} {}",
                    format_label_kind(
                        "user_defined_value_type_definition",
                        "UserDefinedValueTypeDefinition"
                    )
                ),
            );
            (range, frags)
        }

        ContractMember::StateVariableDefinition(ref element) => {
            let (range, mut frags) = render_state_variable_definition(source, element, depth);

            frags.insert(
                0,
                format!(
                    " \u{25ba} {}",
                    format_label_kind("state_variable_definition", "StateVariableDefinition")
                ),
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
                format!(
                    " \u{25ba} {}",
                    format_label_kind("inheritance_specifier", "InheritanceSpecifier")
                ),
            );
            (range, frags)
        }

        ContractSpecifier::StorageLayoutSpecifier(ref element) => {
            let (range, mut frags) = render_storage_layout_specifier(source, element, depth);

            frags.insert(
                0,
                format!(
                    " \u{25ba} {}",
                    format_label_kind("storage_layout_specifier", "StorageLayoutSpecifier")
                ),
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
                format!(
                    " \u{25ba} {}",
                    format_label_kind("bool_keyword", "BoolKeyword")
                ),
            );
            (range, frags)
        }

        ElementaryType::StringKeyword(ref element) => {
            let (range, mut frags) = render_terminal(source, &element.range);

            frags.insert(
                0,
                format!(
                    " \u{25ba} {}",
                    format_label_kind("string_keyword", "StringKeyword")
                ),
            );
            (range, frags)
        }

        ElementaryType::AddressType(ref element) => {
            let (range, mut frags) = render_address_type(source, element, depth);

            frags.insert(
                0,
                format!(
                    " \u{25ba} {}",
                    format_label_kind("address_type", "AddressType")
                ),
            );
            (range, frags)
        }

        ElementaryType::BytesKeyword(ref element) => {
            let (range, mut frags) = render_terminal(source, &element.range);

            frags.insert(
                0,
                format!(
                    " \u{25ba} {}",
                    format_label_kind("bytes_keyword", "BytesKeyword")
                ),
            );
            (range, frags)
        }

        ElementaryType::IntKeyword(ref element) => {
            let (range, mut frags) = render_terminal(source, &element.range);

            frags.insert(
                0,
                format!(
                    " \u{25ba} {}",
                    format_label_kind("int_keyword", "IntKeyword")
                ),
            );
            (range, frags)
        }

        ElementaryType::UintKeyword(ref element) => {
            let (range, mut frags) = render_terminal(source, &element.range);

            frags.insert(
                0,
                format!(
                    " \u{25ba} {}",
                    format_label_kind("uint_keyword", "UintKeyword")
                ),
            );
            (range, frags)
        }

        ElementaryType::FixedKeyword(ref element) => {
            let (range, mut frags) = render_terminal(source, &element.range);

            frags.insert(
                0,
                format!(
                    " \u{25ba} {}",
                    format_label_kind("fixed_keyword", "FixedKeyword")
                ),
            );
            (range, frags)
        }

        ElementaryType::UfixedKeyword(ref element) => {
            let (range, mut frags) = render_terminal(source, &element.range);

            frags.insert(
                0,
                format!(
                    " \u{25ba} {}",
                    format_label_kind("ufixed_keyword", "UfixedKeyword")
                ),
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
                format!(
                    " \u{25ba} {}",
                    format_label_kind("abi_encoder_v2_keyword", "ABIEncoderV2Keyword")
                ),
            );
            (range, frags)
        }

        ExperimentalFeature::SMTCheckerKeyword(ref element) => {
            let (range, mut frags) = render_terminal(source, &element.range);

            frags.insert(
                0,
                format!(
                    " \u{25ba} {}",
                    format_label_kind("smt_checker_keyword", "SMTCheckerKeyword")
                ),
            );
            (range, frags)
        }

        ExperimentalFeature::PragmaStringLiteral(ref element) => {
            let (range, mut frags) = render_terminal(source, &element.range);

            frags.insert(
                0,
                format!(
                    " \u{25ba} {}",
                    format_label_kind("pragma_string_literal", "PragmaStringLiteral")
                ),
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
                format!(
                    " \u{25ba} {}",
                    format_label_kind("assignment_expression", "AssignmentExpression")
                ),
            );
            (range, frags)
        }

        Expression::ConditionalExpression(ref element) => {
            let (range, mut frags) = render_conditional_expression(source, element, depth);

            frags.insert(
                0,
                format!(
                    " \u{25ba} {}",
                    format_label_kind("conditional_expression", "ConditionalExpression")
                ),
            );
            (range, frags)
        }

        Expression::OrExpression(ref element) => {
            let (range, mut frags) = render_or_expression(source, element, depth);

            frags.insert(
                0,
                format!(
                    " \u{25ba} {}",
                    format_label_kind("or_expression", "OrExpression")
                ),
            );
            (range, frags)
        }

        Expression::AndExpression(ref element) => {
            let (range, mut frags) = render_and_expression(source, element, depth);

            frags.insert(
                0,
                format!(
                    " \u{25ba} {}",
                    format_label_kind("and_expression", "AndExpression")
                ),
            );
            (range, frags)
        }

        Expression::EqualityExpression(ref element) => {
            let (range, mut frags) = render_equality_expression(source, element, depth);

            frags.insert(
                0,
                format!(
                    " \u{25ba} {}",
                    format_label_kind("equality_expression", "EqualityExpression")
                ),
            );
            (range, frags)
        }

        Expression::InequalityExpression(ref element) => {
            let (range, mut frags) = render_inequality_expression(source, element, depth);

            frags.insert(
                0,
                format!(
                    " \u{25ba} {}",
                    format_label_kind("inequality_expression", "InequalityExpression")
                ),
            );
            (range, frags)
        }

        Expression::BitwiseOrExpression(ref element) => {
            let (range, mut frags) = render_bitwise_or_expression(source, element, depth);

            frags.insert(
                0,
                format!(
                    " \u{25ba} {}",
                    format_label_kind("bitwise_or_expression", "BitwiseOrExpression")
                ),
            );
            (range, frags)
        }

        Expression::BitwiseXorExpression(ref element) => {
            let (range, mut frags) = render_bitwise_xor_expression(source, element, depth);

            frags.insert(
                0,
                format!(
                    " \u{25ba} {}",
                    format_label_kind("bitwise_xor_expression", "BitwiseXorExpression")
                ),
            );
            (range, frags)
        }

        Expression::BitwiseAndExpression(ref element) => {
            let (range, mut frags) = render_bitwise_and_expression(source, element, depth);

            frags.insert(
                0,
                format!(
                    " \u{25ba} {}",
                    format_label_kind("bitwise_and_expression", "BitwiseAndExpression")
                ),
            );
            (range, frags)
        }

        Expression::ShiftExpression(ref element) => {
            let (range, mut frags) = render_shift_expression(source, element, depth);

            frags.insert(
                0,
                format!(
                    " \u{25ba} {}",
                    format_label_kind("shift_expression", "ShiftExpression")
                ),
            );
            (range, frags)
        }

        Expression::AdditiveExpression(ref element) => {
            let (range, mut frags) = render_additive_expression(source, element, depth);

            frags.insert(
                0,
                format!(
                    " \u{25ba} {}",
                    format_label_kind("additive_expression", "AdditiveExpression")
                ),
            );
            (range, frags)
        }

        Expression::MultiplicativeExpression(ref element) => {
            let (range, mut frags) = render_multiplicative_expression(source, element, depth);

            frags.insert(
                0,
                format!(
                    " \u{25ba} {}",
                    format_label_kind("multiplicative_expression", "MultiplicativeExpression")
                ),
            );
            (range, frags)
        }

        Expression::ExponentiationExpression(ref element) => {
            let (range, mut frags) = render_exponentiation_expression(source, element, depth);

            frags.insert(
                0,
                format!(
                    " \u{25ba} {}",
                    format_label_kind("exponentiation_expression", "ExponentiationExpression")
                ),
            );
            (range, frags)
        }

        Expression::PostfixExpression(ref element) => {
            let (range, mut frags) = render_postfix_expression(source, element, depth);

            frags.insert(
                0,
                format!(
                    " \u{25ba} {}",
                    format_label_kind("postfix_expression", "PostfixExpression")
                ),
            );
            (range, frags)
        }

        Expression::PrefixExpression(ref element) => {
            let (range, mut frags) = render_prefix_expression(source, element, depth);

            frags.insert(
                0,
                format!(
                    " \u{25ba} {}",
                    format_label_kind("prefix_expression", "PrefixExpression")
                ),
            );
            (range, frags)
        }

        Expression::FunctionCallExpression(ref element) => {
            let (range, mut frags) = render_function_call_expression(source, element, depth);

            frags.insert(
                0,
                format!(
                    " \u{25ba} {}",
                    format_label_kind("function_call_expression", "FunctionCallExpression")
                ),
            );
            (range, frags)
        }

        Expression::CallOptionsExpression(ref element) => {
            let (range, mut frags) = render_call_options_expression(source, element, depth);

            frags.insert(
                0,
                format!(
                    " \u{25ba} {}",
                    format_label_kind("call_options_expression", "CallOptionsExpression")
                ),
            );
            (range, frags)
        }

        Expression::MemberAccessExpression(ref element) => {
            let (range, mut frags) = render_member_access_expression(source, element, depth);

            frags.insert(
                0,
                format!(
                    " \u{25ba} {}",
                    format_label_kind("member_access_expression", "MemberAccessExpression")
                ),
            );
            (range, frags)
        }

        Expression::IndexAccessExpression(ref element) => {
            let (range, mut frags) = render_index_access_expression(source, element, depth);

            frags.insert(
                0,
                format!(
                    " \u{25ba} {}",
                    format_label_kind("index_access_expression", "IndexAccessExpression")
                ),
            );
            (range, frags)
        }

        Expression::NewExpression(ref element) => {
            let (range, mut frags) = render_new_expression(source, element, depth);

            frags.insert(
                0,
                format!(
                    " \u{25ba} {}",
                    format_label_kind("new_expression", "NewExpression")
                ),
            );
            (range, frags)
        }

        Expression::TupleExpression(ref element) => {
            let (range, mut frags) = render_tuple_expression(source, element, depth);

            frags.insert(
                0,
                format!(
                    " \u{25ba} {}",
                    format_label_kind("tuple_expression", "TupleExpression")
                ),
            );
            (range, frags)
        }

        Expression::TypeExpression(ref element) => {
            let (range, mut frags) = render_type_expression(source, element, depth);

            frags.insert(
                0,
                format!(
                    " \u{25ba} {}",
                    format_label_kind("type_expression", "TypeExpression")
                ),
            );
            (range, frags)
        }

        Expression::ArrayExpression(ref element) => {
            let (range, mut frags) = render_array_expression(source, element, depth);

            frags.insert(
                0,
                format!(
                    " \u{25ba} {}",
                    format_label_kind("array_expression", "ArrayExpression")
                ),
            );
            (range, frags)
        }

        Expression::HexNumberExpression(ref element) => {
            let (range, mut frags) = render_hex_number_expression(source, element, depth);

            frags.insert(
                0,
                format!(
                    " \u{25ba} {}",
                    format_label_kind("hex_number_expression", "HexNumberExpression")
                ),
            );
            (range, frags)
        }

        Expression::DecimalNumberExpression(ref element) => {
            let (range, mut frags) = render_decimal_number_expression(source, element, depth);

            frags.insert(
                0,
                format!(
                    " \u{25ba} {}",
                    format_label_kind("decimal_number_expression", "DecimalNumberExpression")
                ),
            );
            (range, frags)
        }

        Expression::StringExpression(ref element) => {
            let (range, mut frags) = render_string_expression(source, element, depth);

            frags.insert(
                0,
                format!(
                    " \u{25ba} {}",
                    format_label_kind("string_expression", "StringExpression")
                ),
            );
            (range, frags)
        }

        Expression::ElementaryType(ref element) => {
            let (range, mut frags) = render_elementary_type(source, element, depth);

            frags.insert(
                0,
                format!(
                    " \u{25ba} {}",
                    format_label_kind("elementary_type", "ElementaryType")
                ),
            );
            (range, frags)
        }

        Expression::PayableKeyword(ref element) => {
            let (range, mut frags) = render_terminal(source, &element.range);

            frags.insert(
                0,
                format!(
                    " \u{25ba} {}",
                    format_label_kind("payable_keyword", "PayableKeyword")
                ),
            );
            (range, frags)
        }

        Expression::ThisKeyword(ref element) => {
            let (range, mut frags) = render_terminal(source, &element.range);

            frags.insert(
                0,
                format!(
                    " \u{25ba} {}",
                    format_label_kind("this_keyword", "ThisKeyword")
                ),
            );
            (range, frags)
        }

        Expression::SuperKeyword(ref element) => {
            let (range, mut frags) = render_terminal(source, &element.range);

            frags.insert(
                0,
                format!(
                    " \u{25ba} {}",
                    format_label_kind("super_keyword", "SuperKeyword")
                ),
            );
            (range, frags)
        }

        Expression::TrueKeyword(ref element) => {
            let (range, mut frags) = render_terminal(source, &element.range);

            frags.insert(
                0,
                format!(
                    " \u{25ba} {}",
                    format_label_kind("true_keyword", "TrueKeyword")
                ),
            );
            (range, frags)
        }

        Expression::FalseKeyword(ref element) => {
            let (range, mut frags) = render_terminal(source, &element.range);

            frags.insert(
                0,
                format!(
                    " \u{25ba} {}",
                    format_label_kind("false_keyword", "FalseKeyword")
                ),
            );
            (range, frags)
        }

        Expression::Identifier(ref element) => {
            let (range, mut frags) = render_terminal(source, &element.range);

            frags.insert(
                0,
                format!(
                    " \u{25ba} {}",
                    format_label_kind("identifier", "Identifier")
                ),
            );
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

            frags.insert(
                0,
                format!(" \u{25ba} {}", format_label_kind("minus", "Minus")),
            );
            (range, frags)
        }

        Expression_AdditiveExpression_Operator::Plus(ref element) => {
            let (range, mut frags) = render_terminal(source, &element.range);

            frags.insert(
                0,
                format!(" \u{25ba} {}", format_label_kind("plus", "Plus")),
            );
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
                format!(
                    " \u{25ba} {}",
                    format_label_kind("ampersand_equal", "AmpersandEqual")
                ),
            );
            (range, frags)
        }

        Expression_AssignmentExpression_Operator::AsteriskEqual(ref element) => {
            let (range, mut frags) = render_terminal(source, &element.range);

            frags.insert(
                0,
                format!(
                    " \u{25ba} {}",
                    format_label_kind("asterisk_equal", "AsteriskEqual")
                ),
            );
            (range, frags)
        }

        Expression_AssignmentExpression_Operator::BarEqual(ref element) => {
            let (range, mut frags) = render_terminal(source, &element.range);

            frags.insert(
                0,
                format!(" \u{25ba} {}", format_label_kind("bar_equal", "BarEqual")),
            );
            (range, frags)
        }

        Expression_AssignmentExpression_Operator::CaretEqual(ref element) => {
            let (range, mut frags) = render_terminal(source, &element.range);

            frags.insert(
                0,
                format!(
                    " \u{25ba} {}",
                    format_label_kind("caret_equal", "CaretEqual")
                ),
            );
            (range, frags)
        }

        Expression_AssignmentExpression_Operator::Equal(ref element) => {
            let (range, mut frags) = render_terminal(source, &element.range);

            frags.insert(
                0,
                format!(" \u{25ba} {}", format_label_kind("equal", "Equal")),
            );
            (range, frags)
        }

        Expression_AssignmentExpression_Operator::GreaterThanGreaterThanEqual(ref element) => {
            let (range, mut frags) = render_terminal(source, &element.range);

            frags.insert(
                0,
                format!(
                    " \u{25ba} {}",
                    format_label_kind(
                        "greater_than_greater_than_equal",
                        "GreaterThanGreaterThanEqual"
                    )
                ),
            );
            (range, frags)
        }

        Expression_AssignmentExpression_Operator::GreaterThanGreaterThanGreaterThanEqual(
            ref element,
        ) => {
            let (range, mut frags) = render_terminal(source, &element.range);

            frags.insert(
                0,
                format!(
                    " \u{25ba} {}",
                    format_label_kind(
                        "greater_than_greater_than_greater_than_equal",
                        "GreaterThanGreaterThanGreaterThanEqual"
                    )
                ),
            );
            (range, frags)
        }

        Expression_AssignmentExpression_Operator::LessThanLessThanEqual(ref element) => {
            let (range, mut frags) = render_terminal(source, &element.range);

            frags.insert(
                0,
                format!(
                    " \u{25ba} {}",
                    format_label_kind("less_than_less_than_equal", "LessThanLessThanEqual")
                ),
            );
            (range, frags)
        }

        Expression_AssignmentExpression_Operator::MinusEqual(ref element) => {
            let (range, mut frags) = render_terminal(source, &element.range);

            frags.insert(
                0,
                format!(
                    " \u{25ba} {}",
                    format_label_kind("minus_equal", "MinusEqual")
                ),
            );
            (range, frags)
        }

        Expression_AssignmentExpression_Operator::PercentEqual(ref element) => {
            let (range, mut frags) = render_terminal(source, &element.range);

            frags.insert(
                0,
                format!(
                    " \u{25ba} {}",
                    format_label_kind("percent_equal", "PercentEqual")
                ),
            );
            (range, frags)
        }

        Expression_AssignmentExpression_Operator::PlusEqual(ref element) => {
            let (range, mut frags) = render_terminal(source, &element.range);

            frags.insert(
                0,
                format!(" \u{25ba} {}", format_label_kind("plus_equal", "PlusEqual")),
            );
            (range, frags)
        }

        Expression_AssignmentExpression_Operator::SlashEqual(ref element) => {
            let (range, mut frags) = render_terminal(source, &element.range);

            frags.insert(
                0,
                format!(
                    " \u{25ba} {}",
                    format_label_kind("slash_equal", "SlashEqual")
                ),
            );
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

            frags.insert(
                0,
                format!(" \u{25ba} {}", format_label_kind("bang_equal", "BangEqual")),
            );
            (range, frags)
        }

        Expression_EqualityExpression_Operator::EqualEqual(ref element) => {
            let (range, mut frags) = render_terminal(source, &element.range);

            frags.insert(
                0,
                format!(
                    " \u{25ba} {}",
                    format_label_kind("equal_equal", "EqualEqual")
                ),
            );
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
                format!(
                    " \u{25ba} {}",
                    format_label_kind("greater_than", "GreaterThan")
                ),
            );
            (range, frags)
        }

        Expression_InequalityExpression_Operator::GreaterThanEqual(ref element) => {
            let (range, mut frags) = render_terminal(source, &element.range);

            frags.insert(
                0,
                format!(
                    " \u{25ba} {}",
                    format_label_kind("greater_than_equal", "GreaterThanEqual")
                ),
            );
            (range, frags)
        }

        Expression_InequalityExpression_Operator::LessThan(ref element) => {
            let (range, mut frags) = render_terminal(source, &element.range);

            frags.insert(
                0,
                format!(" \u{25ba} {}", format_label_kind("less_than", "LessThan")),
            );
            (range, frags)
        }

        Expression_InequalityExpression_Operator::LessThanEqual(ref element) => {
            let (range, mut frags) = render_terminal(source, &element.range);

            frags.insert(
                0,
                format!(
                    " \u{25ba} {}",
                    format_label_kind("less_than_equal", "LessThanEqual")
                ),
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

            frags.insert(
                0,
                format!(" \u{25ba} {}", format_label_kind("asterisk", "Asterisk")),
            );
            (range, frags)
        }

        Expression_MultiplicativeExpression_Operator::Percent(ref element) => {
            let (range, mut frags) = render_terminal(source, &element.range);

            frags.insert(
                0,
                format!(" \u{25ba} {}", format_label_kind("percent", "Percent")),
            );
            (range, frags)
        }

        Expression_MultiplicativeExpression_Operator::Slash(ref element) => {
            let (range, mut frags) = render_terminal(source, &element.range);

            frags.insert(
                0,
                format!(" \u{25ba} {}", format_label_kind("slash", "Slash")),
            );
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

            frags.insert(
                0,
                format!(
                    " \u{25ba} {}",
                    format_label_kind("minus_minus", "MinusMinus")
                ),
            );
            (range, frags)
        }

        Expression_PostfixExpression_Operator::PlusPlus(ref element) => {
            let (range, mut frags) = render_terminal(source, &element.range);

            frags.insert(
                0,
                format!(" \u{25ba} {}", format_label_kind("plus_plus", "PlusPlus")),
            );
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

            frags.insert(
                0,
                format!(" \u{25ba} {}", format_label_kind("bang", "Bang")),
            );
            (range, frags)
        }

        Expression_PrefixExpression_Operator::DeleteKeyword(ref element) => {
            let (range, mut frags) = render_terminal(source, &element.range);

            frags.insert(
                0,
                format!(
                    " \u{25ba} {}",
                    format_label_kind("delete_keyword", "DeleteKeyword")
                ),
            );
            (range, frags)
        }

        Expression_PrefixExpression_Operator::Minus(ref element) => {
            let (range, mut frags) = render_terminal(source, &element.range);

            frags.insert(
                0,
                format!(" \u{25ba} {}", format_label_kind("minus", "Minus")),
            );
            (range, frags)
        }

        Expression_PrefixExpression_Operator::MinusMinus(ref element) => {
            let (range, mut frags) = render_terminal(source, &element.range);

            frags.insert(
                0,
                format!(
                    " \u{25ba} {}",
                    format_label_kind("minus_minus", "MinusMinus")
                ),
            );
            (range, frags)
        }

        Expression_PrefixExpression_Operator::PlusPlus(ref element) => {
            let (range, mut frags) = render_terminal(source, &element.range);

            frags.insert(
                0,
                format!(" \u{25ba} {}", format_label_kind("plus_plus", "PlusPlus")),
            );
            (range, frags)
        }

        Expression_PrefixExpression_Operator::Tilde(ref element) => {
            let (range, mut frags) = render_terminal(source, &element.range);

            frags.insert(
                0,
                format!(" \u{25ba} {}", format_label_kind("tilde", "Tilde")),
            );
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
                format!(
                    " \u{25ba} {}",
                    format_label_kind("greater_than_greater_than", "GreaterThanGreaterThan")
                ),
            );
            (range, frags)
        }

        Expression_ShiftExpression_Operator::GreaterThanGreaterThanGreaterThan(ref element) => {
            let (range, mut frags) = render_terminal(source, &element.range);

            frags.insert(
                0,
                format!(
                    " \u{25ba} {}",
                    format_label_kind(
                        "greater_than_greater_than_greater_than",
                        "GreaterThanGreaterThanGreaterThan"
                    )
                ),
            );
            (range, frags)
        }

        Expression_ShiftExpression_Operator::LessThanLessThan(ref element) => {
            let (range, mut frags) = render_terminal(source, &element.range);

            frags.insert(
                0,
                format!(
                    " \u{25ba} {}",
                    format_label_kind("less_than_less_than", "LessThanLessThan")
                ),
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
                format!(
                    " \u{25ba} {}",
                    format_label_kind("modifier_invocation", "ModifierInvocation")
                ),
            );
            (range, frags)
        }

        FallbackFunctionAttribute::OverrideSpecifier(ref element) => {
            let (range, mut frags) = render_override_specifier(source, element, depth);

            frags.insert(
                0,
                format!(
                    " \u{25ba} {}",
                    format_label_kind("override_specifier", "OverrideSpecifier")
                ),
            );
            (range, frags)
        }

        FallbackFunctionAttribute::ExternalKeyword(ref element) => {
            let (range, mut frags) = render_terminal(source, &element.range);

            frags.insert(
                0,
                format!(
                    " \u{25ba} {}",
                    format_label_kind("external_keyword", "ExternalKeyword")
                ),
            );
            (range, frags)
        }

        FallbackFunctionAttribute::PayableKeyword(ref element) => {
            let (range, mut frags) = render_terminal(source, &element.range);

            frags.insert(
                0,
                format!(
                    " \u{25ba} {}",
                    format_label_kind("payable_keyword", "PayableKeyword")
                ),
            );
            (range, frags)
        }

        FallbackFunctionAttribute::PureKeyword(ref element) => {
            let (range, mut frags) = render_terminal(source, &element.range);

            frags.insert(
                0,
                format!(
                    " \u{25ba} {}",
                    format_label_kind("pure_keyword", "PureKeyword")
                ),
            );
            (range, frags)
        }

        FallbackFunctionAttribute::ViewKeyword(ref element) => {
            let (range, mut frags) = render_terminal(source, &element.range);

            frags.insert(
                0,
                format!(
                    " \u{25ba} {}",
                    format_label_kind("view_keyword", "ViewKeyword")
                ),
            );
            (range, frags)
        }

        FallbackFunctionAttribute::VirtualKeyword(ref element) => {
            let (range, mut frags) = render_terminal(source, &element.range);

            frags.insert(
                0,
                format!(
                    " \u{25ba} {}",
                    format_label_kind("virtual_keyword", "VirtualKeyword")
                ),
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
                format!(
                    " \u{25ba} {}",
                    format_label_kind("expression_statement", "ExpressionStatement")
                ),
            );
            (range, frags)
        }

        ForStatementCondition::Semicolon(ref element) => {
            let (range, mut frags) = render_terminal(source, &element.range);

            frags.insert(
                0,
                format!(" \u{25ba} {}", format_label_kind("semicolon", "Semicolon")),
            );
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
                format!(
                    " \u{25ba} {}",
                    format_label_kind(
                        "variable_declaration_statement",
                        "VariableDeclarationStatement"
                    )
                ),
            );
            (range, frags)
        }

        ForStatementInitialization::ExpressionStatement(ref element) => {
            let (range, mut frags) = render_expression_statement(source, element, depth);

            frags.insert(
                0,
                format!(
                    " \u{25ba} {}",
                    format_label_kind("expression_statement", "ExpressionStatement")
                ),
            );
            (range, frags)
        }

        ForStatementInitialization::Semicolon(ref element) => {
            let (range, mut frags) = render_terminal(source, &element.range);

            frags.insert(
                0,
                format!(" \u{25ba} {}", format_label_kind("semicolon", "Semicolon")),
            );
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
                format!(
                    " \u{25ba} {}",
                    format_label_kind("modifier_invocation", "ModifierInvocation")
                ),
            );
            (range, frags)
        }

        FunctionAttribute::OverrideSpecifier(ref element) => {
            let (range, mut frags) = render_override_specifier(source, element, depth);

            frags.insert(
                0,
                format!(
                    " \u{25ba} {}",
                    format_label_kind("override_specifier", "OverrideSpecifier")
                ),
            );
            (range, frags)
        }

        FunctionAttribute::ExternalKeyword(ref element) => {
            let (range, mut frags) = render_terminal(source, &element.range);

            frags.insert(
                0,
                format!(
                    " \u{25ba} {}",
                    format_label_kind("external_keyword", "ExternalKeyword")
                ),
            );
            (range, frags)
        }

        FunctionAttribute::InternalKeyword(ref element) => {
            let (range, mut frags) = render_terminal(source, &element.range);

            frags.insert(
                0,
                format!(
                    " \u{25ba} {}",
                    format_label_kind("internal_keyword", "InternalKeyword")
                ),
            );
            (range, frags)
        }

        FunctionAttribute::PayableKeyword(ref element) => {
            let (range, mut frags) = render_terminal(source, &element.range);

            frags.insert(
                0,
                format!(
                    " \u{25ba} {}",
                    format_label_kind("payable_keyword", "PayableKeyword")
                ),
            );
            (range, frags)
        }

        FunctionAttribute::PrivateKeyword(ref element) => {
            let (range, mut frags) = render_terminal(source, &element.range);

            frags.insert(
                0,
                format!(
                    " \u{25ba} {}",
                    format_label_kind("private_keyword", "PrivateKeyword")
                ),
            );
            (range, frags)
        }

        FunctionAttribute::PublicKeyword(ref element) => {
            let (range, mut frags) = render_terminal(source, &element.range);

            frags.insert(
                0,
                format!(
                    " \u{25ba} {}",
                    format_label_kind("public_keyword", "PublicKeyword")
                ),
            );
            (range, frags)
        }

        FunctionAttribute::PureKeyword(ref element) => {
            let (range, mut frags) = render_terminal(source, &element.range);

            frags.insert(
                0,
                format!(
                    " \u{25ba} {}",
                    format_label_kind("pure_keyword", "PureKeyword")
                ),
            );
            (range, frags)
        }

        FunctionAttribute::ViewKeyword(ref element) => {
            let (range, mut frags) = render_terminal(source, &element.range);

            frags.insert(
                0,
                format!(
                    " \u{25ba} {}",
                    format_label_kind("view_keyword", "ViewKeyword")
                ),
            );
            (range, frags)
        }

        FunctionAttribute::VirtualKeyword(ref element) => {
            let (range, mut frags) = render_terminal(source, &element.range);

            frags.insert(
                0,
                format!(
                    " \u{25ba} {}",
                    format_label_kind("virtual_keyword", "VirtualKeyword")
                ),
            );
            (range, frags)
        }
    }
}

pub fn render_function_body(source: &str, node: &FunctionBody, depth: usize) -> RenderedOutput {
    match node {
        FunctionBody::Block(ref element) => {
            let (range, mut frags) = render_block(source, element, depth);

            frags.insert(
                0,
                format!(" \u{25ba} {}", format_label_kind("block", "Block")),
            );
            (range, frags)
        }

        FunctionBody::Semicolon(ref element) => {
            let (range, mut frags) = render_terminal(source, &element.range);

            frags.insert(
                0,
                format!(" \u{25ba} {}", format_label_kind("semicolon", "Semicolon")),
            );
            (range, frags)
        }
    }
}

pub fn render_function_name(source: &str, node: &FunctionName, depth: usize) -> RenderedOutput {
    match node {
        FunctionName::Identifier(ref element) => {
            let (range, mut frags) = render_terminal(source, &element.range);

            frags.insert(
                0,
                format!(
                    " \u{25ba} {}",
                    format_label_kind("identifier", "Identifier")
                ),
            );
            (range, frags)
        }

        FunctionName::FallbackKeyword(ref element) => {
            let (range, mut frags) = render_terminal(source, &element.range);

            frags.insert(
                0,
                format!(
                    " \u{25ba} {}",
                    format_label_kind("fallback_keyword", "FallbackKeyword")
                ),
            );
            (range, frags)
        }

        FunctionName::ReceiveKeyword(ref element) => {
            let (range, mut frags) = render_terminal(source, &element.range);

            frags.insert(
                0,
                format!(
                    " \u{25ba} {}",
                    format_label_kind("receive_keyword", "ReceiveKeyword")
                ),
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
                format!(
                    " \u{25ba} {}",
                    format_label_kind("internal_keyword", "InternalKeyword")
                ),
            );
            (range, frags)
        }

        FunctionTypeAttribute::ExternalKeyword(ref element) => {
            let (range, mut frags) = render_terminal(source, &element.range);

            frags.insert(
                0,
                format!(
                    " \u{25ba} {}",
                    format_label_kind("external_keyword", "ExternalKeyword")
                ),
            );
            (range, frags)
        }

        FunctionTypeAttribute::PrivateKeyword(ref element) => {
            let (range, mut frags) = render_terminal(source, &element.range);

            frags.insert(
                0,
                format!(
                    " \u{25ba} {}",
                    format_label_kind("private_keyword", "PrivateKeyword")
                ),
            );
            (range, frags)
        }

        FunctionTypeAttribute::PublicKeyword(ref element) => {
            let (range, mut frags) = render_terminal(source, &element.range);

            frags.insert(
                0,
                format!(
                    " \u{25ba} {}",
                    format_label_kind("public_keyword", "PublicKeyword")
                ),
            );
            (range, frags)
        }

        FunctionTypeAttribute::PureKeyword(ref element) => {
            let (range, mut frags) = render_terminal(source, &element.range);

            frags.insert(
                0,
                format!(
                    " \u{25ba} {}",
                    format_label_kind("pure_keyword", "PureKeyword")
                ),
            );
            (range, frags)
        }

        FunctionTypeAttribute::ViewKeyword(ref element) => {
            let (range, mut frags) = render_terminal(source, &element.range);

            frags.insert(
                0,
                format!(
                    " \u{25ba} {}",
                    format_label_kind("view_keyword", "ViewKeyword")
                ),
            );
            (range, frags)
        }

        FunctionTypeAttribute::PayableKeyword(ref element) => {
            let (range, mut frags) = render_terminal(source, &element.range);

            frags.insert(
                0,
                format!(
                    " \u{25ba} {}",
                    format_label_kind("payable_keyword", "PayableKeyword")
                ),
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

            frags.insert(
                0,
                format!(
                    " \u{25ba} {}",
                    format_label_kind("identifier", "Identifier")
                ),
            );
            (range, frags)
        }

        IdentifierPathElement::AddressKeyword(ref element) => {
            let (range, mut frags) = render_terminal(source, &element.range);

            frags.insert(
                0,
                format!(
                    " \u{25ba} {}",
                    format_label_kind("address_keyword", "AddressKeyword")
                ),
            );
            (range, frags)
        }
    }
}

pub fn render_import_clause(source: &str, node: &ImportClause, depth: usize) -> RenderedOutput {
    match node {
        ImportClause::PathImport(ref element) => {
            let (range, mut frags) = render_path_import(source, element, depth);

            frags.insert(
                0,
                format!(
                    " \u{25ba} {}",
                    format_label_kind("path_import", "PathImport")
                ),
            );
            (range, frags)
        }

        ImportClause::NamedImport(ref element) => {
            let (range, mut frags) = render_named_import(source, element, depth);

            frags.insert(
                0,
                format!(
                    " \u{25ba} {}",
                    format_label_kind("named_import", "NamedImport")
                ),
            );
            (range, frags)
        }

        ImportClause::ImportDeconstruction(ref element) => {
            let (range, mut frags) = render_import_deconstruction(source, element, depth);

            frags.insert(
                0,
                format!(
                    " \u{25ba} {}",
                    format_label_kind("import_deconstruction", "ImportDeconstruction")
                ),
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
                format!(
                    " \u{25ba} {}",
                    format_label_kind("elementary_type", "ElementaryType")
                ),
            );
            (range, frags)
        }

        MappingKeyType::IdentifierPath(ref element) => {
            let (range, mut frags) = render_identifier_path(source, element, depth);

            frags.insert(
                0,
                format!(
                    " \u{25ba} {}",
                    format_label_kind("identifier_path", "IdentifierPath")
                ),
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
                format!(
                    " \u{25ba} {}",
                    format_label_kind("override_specifier", "OverrideSpecifier")
                ),
            );
            (range, frags)
        }

        ModifierAttribute::VirtualKeyword(ref element) => {
            let (range, mut frags) = render_terminal(source, &element.range);

            frags.insert(
                0,
                format!(
                    " \u{25ba} {}",
                    format_label_kind("virtual_keyword", "VirtualKeyword")
                ),
            );
            (range, frags)
        }
    }
}

pub fn render_number_unit(source: &str, node: &NumberUnit, depth: usize) -> RenderedOutput {
    match node {
        NumberUnit::WeiKeyword(ref element) => {
            let (range, mut frags) = render_terminal(source, &element.range);

            frags.insert(
                0,
                format!(
                    " \u{25ba} {}",
                    format_label_kind("wei_keyword", "WeiKeyword")
                ),
            );
            (range, frags)
        }

        NumberUnit::GweiKeyword(ref element) => {
            let (range, mut frags) = render_terminal(source, &element.range);

            frags.insert(
                0,
                format!(
                    " \u{25ba} {}",
                    format_label_kind("gwei_keyword", "GweiKeyword")
                ),
            );
            (range, frags)
        }

        NumberUnit::EtherKeyword(ref element) => {
            let (range, mut frags) = render_terminal(source, &element.range);

            frags.insert(
                0,
                format!(
                    " \u{25ba} {}",
                    format_label_kind("ether_keyword", "EtherKeyword")
                ),
            );
            (range, frags)
        }

        NumberUnit::SecondsKeyword(ref element) => {
            let (range, mut frags) = render_terminal(source, &element.range);

            frags.insert(
                0,
                format!(
                    " \u{25ba} {}",
                    format_label_kind("seconds_keyword", "SecondsKeyword")
                ),
            );
            (range, frags)
        }

        NumberUnit::MinutesKeyword(ref element) => {
            let (range, mut frags) = render_terminal(source, &element.range);

            frags.insert(
                0,
                format!(
                    " \u{25ba} {}",
                    format_label_kind("minutes_keyword", "MinutesKeyword")
                ),
            );
            (range, frags)
        }

        NumberUnit::HoursKeyword(ref element) => {
            let (range, mut frags) = render_terminal(source, &element.range);

            frags.insert(
                0,
                format!(
                    " \u{25ba} {}",
                    format_label_kind("hours_keyword", "HoursKeyword")
                ),
            );
            (range, frags)
        }

        NumberUnit::DaysKeyword(ref element) => {
            let (range, mut frags) = render_terminal(source, &element.range);

            frags.insert(
                0,
                format!(
                    " \u{25ba} {}",
                    format_label_kind("days_keyword", "DaysKeyword")
                ),
            );
            (range, frags)
        }

        NumberUnit::WeeksKeyword(ref element) => {
            let (range, mut frags) = render_terminal(source, &element.range);

            frags.insert(
                0,
                format!(
                    " \u{25ba} {}",
                    format_label_kind("weeks_keyword", "WeeksKeyword")
                ),
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
                format!(
                    " \u{25ba} {}",
                    format_label_kind("version_pragma", "VersionPragma")
                ),
            );
            (range, frags)
        }

        Pragma::AbicoderPragma(ref element) => {
            let (range, mut frags) = render_abicoder_pragma(source, element, depth);

            frags.insert(
                0,
                format!(
                    " \u{25ba} {}",
                    format_label_kind("abicoder_pragma", "AbicoderPragma")
                ),
            );
            (range, frags)
        }

        Pragma::ExperimentalPragma(ref element) => {
            let (range, mut frags) = render_experimental_pragma(source, element, depth);

            frags.insert(
                0,
                format!(
                    " \u{25ba} {}",
                    format_label_kind("experimental_pragma", "ExperimentalPragma")
                ),
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
                format!(
                    " \u{25ba} {}",
                    format_label_kind("modifier_invocation", "ModifierInvocation")
                ),
            );
            (range, frags)
        }

        ReceiveFunctionAttribute::OverrideSpecifier(ref element) => {
            let (range, mut frags) = render_override_specifier(source, element, depth);

            frags.insert(
                0,
                format!(
                    " \u{25ba} {}",
                    format_label_kind("override_specifier", "OverrideSpecifier")
                ),
            );
            (range, frags)
        }

        ReceiveFunctionAttribute::ExternalKeyword(ref element) => {
            let (range, mut frags) = render_terminal(source, &element.range);

            frags.insert(
                0,
                format!(
                    " \u{25ba} {}",
                    format_label_kind("external_keyword", "ExternalKeyword")
                ),
            );
            (range, frags)
        }

        ReceiveFunctionAttribute::PayableKeyword(ref element) => {
            let (range, mut frags) = render_terminal(source, &element.range);

            frags.insert(
                0,
                format!(
                    " \u{25ba} {}",
                    format_label_kind("payable_keyword", "PayableKeyword")
                ),
            );
            (range, frags)
        }

        ReceiveFunctionAttribute::VirtualKeyword(ref element) => {
            let (range, mut frags) = render_terminal(source, &element.range);

            frags.insert(
                0,
                format!(
                    " \u{25ba} {}",
                    format_label_kind("virtual_keyword", "VirtualKeyword")
                ),
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
                format!(
                    " \u{25ba} {}",
                    format_label_kind("pragma_directive", "PragmaDirective")
                ),
            );
            (range, frags)
        }

        SourceUnitMember::ImportDirective(ref element) => {
            let (range, mut frags) = render_import_directive(source, element, depth);

            frags.insert(
                0,
                format!(
                    " \u{25ba} {}",
                    format_label_kind("import_directive", "ImportDirective")
                ),
            );
            (range, frags)
        }

        SourceUnitMember::ContractDefinition(ref element) => {
            let (range, mut frags) = render_contract_definition(source, element, depth);

            frags.insert(
                0,
                format!(
                    " \u{25ba} {}",
                    format_label_kind("contract_definition", "ContractDefinition")
                ),
            );
            (range, frags)
        }

        SourceUnitMember::InterfaceDefinition(ref element) => {
            let (range, mut frags) = render_interface_definition(source, element, depth);

            frags.insert(
                0,
                format!(
                    " \u{25ba} {}",
                    format_label_kind("interface_definition", "InterfaceDefinition")
                ),
            );
            (range, frags)
        }

        SourceUnitMember::LibraryDefinition(ref element) => {
            let (range, mut frags) = render_library_definition(source, element, depth);

            frags.insert(
                0,
                format!(
                    " \u{25ba} {}",
                    format_label_kind("library_definition", "LibraryDefinition")
                ),
            );
            (range, frags)
        }

        SourceUnitMember::StructDefinition(ref element) => {
            let (range, mut frags) = render_struct_definition(source, element, depth);

            frags.insert(
                0,
                format!(
                    " \u{25ba} {}",
                    format_label_kind("struct_definition", "StructDefinition")
                ),
            );
            (range, frags)
        }

        SourceUnitMember::EnumDefinition(ref element) => {
            let (range, mut frags) = render_enum_definition(source, element, depth);

            frags.insert(
                0,
                format!(
                    " \u{25ba} {}",
                    format_label_kind("enum_definition", "EnumDefinition")
                ),
            );
            (range, frags)
        }

        SourceUnitMember::FunctionDefinition(ref element) => {
            let (range, mut frags) = render_function_definition(source, element, depth);

            frags.insert(
                0,
                format!(
                    " \u{25ba} {}",
                    format_label_kind("function_definition", "FunctionDefinition")
                ),
            );
            (range, frags)
        }

        SourceUnitMember::ErrorDefinition(ref element) => {
            let (range, mut frags) = render_error_definition(source, element, depth);

            frags.insert(
                0,
                format!(
                    " \u{25ba} {}",
                    format_label_kind("error_definition", "ErrorDefinition")
                ),
            );
            (range, frags)
        }

        SourceUnitMember::UserDefinedValueTypeDefinition(ref element) => {
            let (range, mut frags) =
                render_user_defined_value_type_definition(source, element, depth);

            frags.insert(
                0,
                format!(
                    " \u{25ba} {}",
                    format_label_kind(
                        "user_defined_value_type_definition",
                        "UserDefinedValueTypeDefinition"
                    )
                ),
            );
            (range, frags)
        }

        SourceUnitMember::UsingDirective(ref element) => {
            let (range, mut frags) = render_using_directive(source, element, depth);

            frags.insert(
                0,
                format!(
                    " \u{25ba} {}",
                    format_label_kind("using_directive", "UsingDirective")
                ),
            );
            (range, frags)
        }

        SourceUnitMember::EventDefinition(ref element) => {
            let (range, mut frags) = render_event_definition(source, element, depth);

            frags.insert(
                0,
                format!(
                    " \u{25ba} {}",
                    format_label_kind("event_definition", "EventDefinition")
                ),
            );
            (range, frags)
        }

        SourceUnitMember::ConstantDefinition(ref element) => {
            let (range, mut frags) = render_constant_definition(source, element, depth);

            frags.insert(
                0,
                format!(
                    " \u{25ba} {}",
                    format_label_kind("constant_definition", "ConstantDefinition")
                ),
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
                format!(
                    " \u{25ba} {}",
                    format_label_kind("override_specifier", "OverrideSpecifier")
                ),
            );
            (range, frags)
        }

        StateVariableAttribute::ConstantKeyword(ref element) => {
            let (range, mut frags) = render_terminal(source, &element.range);

            frags.insert(
                0,
                format!(
                    " \u{25ba} {}",
                    format_label_kind("constant_keyword", "ConstantKeyword")
                ),
            );
            (range, frags)
        }

        StateVariableAttribute::InternalKeyword(ref element) => {
            let (range, mut frags) = render_terminal(source, &element.range);

            frags.insert(
                0,
                format!(
                    " \u{25ba} {}",
                    format_label_kind("internal_keyword", "InternalKeyword")
                ),
            );
            (range, frags)
        }

        StateVariableAttribute::PrivateKeyword(ref element) => {
            let (range, mut frags) = render_terminal(source, &element.range);

            frags.insert(
                0,
                format!(
                    " \u{25ba} {}",
                    format_label_kind("private_keyword", "PrivateKeyword")
                ),
            );
            (range, frags)
        }

        StateVariableAttribute::PublicKeyword(ref element) => {
            let (range, mut frags) = render_terminal(source, &element.range);

            frags.insert(
                0,
                format!(
                    " \u{25ba} {}",
                    format_label_kind("public_keyword", "PublicKeyword")
                ),
            );
            (range, frags)
        }

        StateVariableAttribute::ImmutableKeyword(ref element) => {
            let (range, mut frags) = render_terminal(source, &element.range);

            frags.insert(
                0,
                format!(
                    " \u{25ba} {}",
                    format_label_kind("immutable_keyword", "ImmutableKeyword")
                ),
            );
            (range, frags)
        }

        StateVariableAttribute::TransientKeyword(ref element) => {
            let (range, mut frags) = render_terminal(source, &element.range);

            frags.insert(
                0,
                format!(
                    " \u{25ba} {}",
                    format_label_kind("transient_keyword", "TransientKeyword")
                ),
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
                format!(
                    " \u{25ba} {}",
                    format_label_kind("if_statement", "IfStatement")
                ),
            );
            (range, frags)
        }

        Statement::ForStatement(ref element) => {
            let (range, mut frags) = render_for_statement(source, element, depth);

            frags.insert(
                0,
                format!(
                    " \u{25ba} {}",
                    format_label_kind("for_statement", "ForStatement")
                ),
            );
            (range, frags)
        }

        Statement::WhileStatement(ref element) => {
            let (range, mut frags) = render_while_statement(source, element, depth);

            frags.insert(
                0,
                format!(
                    " \u{25ba} {}",
                    format_label_kind("while_statement", "WhileStatement")
                ),
            );
            (range, frags)
        }

        Statement::DoWhileStatement(ref element) => {
            let (range, mut frags) = render_do_while_statement(source, element, depth);

            frags.insert(
                0,
                format!(
                    " \u{25ba} {}",
                    format_label_kind("do_while_statement", "DoWhileStatement")
                ),
            );
            (range, frags)
        }

        Statement::ContinueStatement(ref element) => {
            let (range, mut frags) = render_continue_statement(source, element, depth);

            frags.insert(
                0,
                format!(
                    " \u{25ba} {}",
                    format_label_kind("continue_statement", "ContinueStatement")
                ),
            );
            (range, frags)
        }

        Statement::BreakStatement(ref element) => {
            let (range, mut frags) = render_break_statement(source, element, depth);

            frags.insert(
                0,
                format!(
                    " \u{25ba} {}",
                    format_label_kind("break_statement", "BreakStatement")
                ),
            );
            (range, frags)
        }

        Statement::ReturnStatement(ref element) => {
            let (range, mut frags) = render_return_statement(source, element, depth);

            frags.insert(
                0,
                format!(
                    " \u{25ba} {}",
                    format_label_kind("return_statement", "ReturnStatement")
                ),
            );
            (range, frags)
        }

        Statement::EmitStatement(ref element) => {
            let (range, mut frags) = render_emit_statement(source, element, depth);

            frags.insert(
                0,
                format!(
                    " \u{25ba} {}",
                    format_label_kind("emit_statement", "EmitStatement")
                ),
            );
            (range, frags)
        }

        Statement::TryStatement(ref element) => {
            let (range, mut frags) = render_try_statement(source, element, depth);

            frags.insert(
                0,
                format!(
                    " \u{25ba} {}",
                    format_label_kind("try_statement", "TryStatement")
                ),
            );
            (range, frags)
        }

        Statement::RevertStatement(ref element) => {
            let (range, mut frags) = render_revert_statement(source, element, depth);

            frags.insert(
                0,
                format!(
                    " \u{25ba} {}",
                    format_label_kind("revert_statement", "RevertStatement")
                ),
            );
            (range, frags)
        }

        Statement::AssemblyStatement(ref element) => {
            let (range, mut frags) = render_assembly_statement(source, element, depth);

            frags.insert(
                0,
                format!(
                    " \u{25ba} {}",
                    format_label_kind("assembly_statement", "AssemblyStatement")
                ),
            );
            (range, frags)
        }

        Statement::Block(ref element) => {
            let (range, mut frags) = render_block(source, element, depth);

            frags.insert(
                0,
                format!(" \u{25ba} {}", format_label_kind("block", "Block")),
            );
            (range, frags)
        }

        Statement::UncheckedBlock(ref element) => {
            let (range, mut frags) = render_unchecked_block(source, element, depth);

            frags.insert(
                0,
                format!(
                    " \u{25ba} {}",
                    format_label_kind("unchecked_block", "UncheckedBlock")
                ),
            );
            (range, frags)
        }

        Statement::VariableDeclarationStatement(ref element) => {
            let (range, mut frags) = render_variable_declaration_statement(source, element, depth);

            frags.insert(
                0,
                format!(
                    " \u{25ba} {}",
                    format_label_kind(
                        "variable_declaration_statement",
                        "VariableDeclarationStatement"
                    )
                ),
            );
            (range, frags)
        }

        Statement::ExpressionStatement(ref element) => {
            let (range, mut frags) = render_expression_statement(source, element, depth);

            frags.insert(
                0,
                format!(
                    " \u{25ba} {}",
                    format_label_kind("expression_statement", "ExpressionStatement")
                ),
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
                format!(
                    " \u{25ba} {}",
                    format_label_kind("memory_keyword", "MemoryKeyword")
                ),
            );
            (range, frags)
        }

        StorageLocation::StorageKeyword(ref element) => {
            let (range, mut frags) = render_terminal(source, &element.range);

            frags.insert(
                0,
                format!(
                    " \u{25ba} {}",
                    format_label_kind("storage_keyword", "StorageKeyword")
                ),
            );
            (range, frags)
        }

        StorageLocation::CallDataKeyword(ref element) => {
            let (range, mut frags) = render_terminal(source, &element.range);

            frags.insert(
                0,
                format!(
                    " \u{25ba} {}",
                    format_label_kind("call_data_keyword", "CallDataKeyword")
                ),
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
                format!(
                    " \u{25ba} {}",
                    format_label_kind("string_literals", "StringLiterals")
                ),
            );
            (range, frags)
        }

        StringExpression::HexStringLiterals(ref element) => {
            let (range, mut frags) = render_hex_string_literals(source, element, depth);

            frags.insert(
                0,
                format!(
                    " \u{25ba} {}",
                    format_label_kind("hex_string_literals", "HexStringLiterals")
                ),
            );
            (range, frags)
        }

        StringExpression::UnicodeStringLiterals(ref element) => {
            let (range, mut frags) = render_unicode_string_literals(source, element, depth);

            frags.insert(
                0,
                format!(
                    " \u{25ba} {}",
                    format_label_kind("unicode_string_literals", "UnicodeStringLiterals")
                ),
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
                format!(
                    " \u{25ba} {}",
                    format_label_kind("array_type_name", "ArrayTypeName")
                ),
            );
            (range, frags)
        }

        TypeName::FunctionType(ref element) => {
            let (range, mut frags) = render_function_type(source, element, depth);

            frags.insert(
                0,
                format!(
                    " \u{25ba} {}",
                    format_label_kind("function_type", "FunctionType")
                ),
            );
            (range, frags)
        }

        TypeName::MappingType(ref element) => {
            let (range, mut frags) = render_mapping_type(source, element, depth);

            frags.insert(
                0,
                format!(
                    " \u{25ba} {}",
                    format_label_kind("mapping_type", "MappingType")
                ),
            );
            (range, frags)
        }

        TypeName::ElementaryType(ref element) => {
            let (range, mut frags) = render_elementary_type(source, element, depth);

            frags.insert(
                0,
                format!(
                    " \u{25ba} {}",
                    format_label_kind("elementary_type", "ElementaryType")
                ),
            );
            (range, frags)
        }

        TypeName::IdentifierPath(ref element) => {
            let (range, mut frags) = render_identifier_path(source, element, depth);

            frags.insert(
                0,
                format!(
                    " \u{25ba} {}",
                    format_label_kind("identifier_path", "IdentifierPath")
                ),
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
                format!(
                    " \u{25ba} {}",
                    format_label_kind("identifier_path", "IdentifierPath")
                ),
            );
            (range, frags)
        }

        UsingClause::UsingDeconstruction(ref element) => {
            let (range, mut frags) = render_using_deconstruction(source, element, depth);

            frags.insert(
                0,
                format!(
                    " \u{25ba} {}",
                    format_label_kind("using_deconstruction", "UsingDeconstruction")
                ),
            );
            (range, frags)
        }
    }
}

pub fn render_using_operator(source: &str, node: &UsingOperator, depth: usize) -> RenderedOutput {
    match node {
        UsingOperator::Ampersand(ref element) => {
            let (range, mut frags) = render_terminal(source, &element.range);

            frags.insert(
                0,
                format!(" \u{25ba} {}", format_label_kind("ampersand", "Ampersand")),
            );
            (range, frags)
        }

        UsingOperator::Asterisk(ref element) => {
            let (range, mut frags) = render_terminal(source, &element.range);

            frags.insert(
                0,
                format!(" \u{25ba} {}", format_label_kind("asterisk", "Asterisk")),
            );
            (range, frags)
        }

        UsingOperator::BangEqual(ref element) => {
            let (range, mut frags) = render_terminal(source, &element.range);

            frags.insert(
                0,
                format!(" \u{25ba} {}", format_label_kind("bang_equal", "BangEqual")),
            );
            (range, frags)
        }

        UsingOperator::Bar(ref element) => {
            let (range, mut frags) = render_terminal(source, &element.range);

            frags.insert(0, format!(" \u{25ba} {}", format_label_kind("bar", "Bar")));
            (range, frags)
        }

        UsingOperator::Caret(ref element) => {
            let (range, mut frags) = render_terminal(source, &element.range);

            frags.insert(
                0,
                format!(" \u{25ba} {}", format_label_kind("caret", "Caret")),
            );
            (range, frags)
        }

        UsingOperator::EqualEqual(ref element) => {
            let (range, mut frags) = render_terminal(source, &element.range);

            frags.insert(
                0,
                format!(
                    " \u{25ba} {}",
                    format_label_kind("equal_equal", "EqualEqual")
                ),
            );
            (range, frags)
        }

        UsingOperator::GreaterThan(ref element) => {
            let (range, mut frags) = render_terminal(source, &element.range);

            frags.insert(
                0,
                format!(
                    " \u{25ba} {}",
                    format_label_kind("greater_than", "GreaterThan")
                ),
            );
            (range, frags)
        }

        UsingOperator::GreaterThanEqual(ref element) => {
            let (range, mut frags) = render_terminal(source, &element.range);

            frags.insert(
                0,
                format!(
                    " \u{25ba} {}",
                    format_label_kind("greater_than_equal", "GreaterThanEqual")
                ),
            );
            (range, frags)
        }

        UsingOperator::LessThan(ref element) => {
            let (range, mut frags) = render_terminal(source, &element.range);

            frags.insert(
                0,
                format!(" \u{25ba} {}", format_label_kind("less_than", "LessThan")),
            );
            (range, frags)
        }

        UsingOperator::LessThanEqual(ref element) => {
            let (range, mut frags) = render_terminal(source, &element.range);

            frags.insert(
                0,
                format!(
                    " \u{25ba} {}",
                    format_label_kind("less_than_equal", "LessThanEqual")
                ),
            );
            (range, frags)
        }

        UsingOperator::Minus(ref element) => {
            let (range, mut frags) = render_terminal(source, &element.range);

            frags.insert(
                0,
                format!(" \u{25ba} {}", format_label_kind("minus", "Minus")),
            );
            (range, frags)
        }

        UsingOperator::Percent(ref element) => {
            let (range, mut frags) = render_terminal(source, &element.range);

            frags.insert(
                0,
                format!(" \u{25ba} {}", format_label_kind("percent", "Percent")),
            );
            (range, frags)
        }

        UsingOperator::Plus(ref element) => {
            let (range, mut frags) = render_terminal(source, &element.range);

            frags.insert(
                0,
                format!(" \u{25ba} {}", format_label_kind("plus", "Plus")),
            );
            (range, frags)
        }

        UsingOperator::Slash(ref element) => {
            let (range, mut frags) = render_terminal(source, &element.range);

            frags.insert(
                0,
                format!(" \u{25ba} {}", format_label_kind("slash", "Slash")),
            );
            (range, frags)
        }

        UsingOperator::Tilde(ref element) => {
            let (range, mut frags) = render_terminal(source, &element.range);

            frags.insert(
                0,
                format!(" \u{25ba} {}", format_label_kind("tilde", "Tilde")),
            );
            (range, frags)
        }
    }
}

pub fn render_using_target(source: &str, node: &UsingTarget, depth: usize) -> RenderedOutput {
    match node {
        UsingTarget::TypeName(ref element) => {
            let (range, mut frags) = render_type_name(source, element, depth);

            frags.insert(
                0,
                format!(" \u{25ba} {}", format_label_kind("type_name", "TypeName")),
            );
            (range, frags)
        }

        UsingTarget::Asterisk(ref element) => {
            let (range, mut frags) = render_terminal(source, &element.range);

            frags.insert(
                0,
                format!(" \u{25ba} {}", format_label_kind("asterisk", "Asterisk")),
            );
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
                format!(
                    " \u{25ba} {}",
                    format_label_kind("single_typed_declaration", "SingleTypedDeclaration")
                ),
            );
            (range, frags)
        }

        VariableDeclarationTarget::MultiTypedDeclaration(ref element) => {
            let (range, mut frags) = render_multi_typed_declaration(source, element, depth);

            frags.insert(
                0,
                format!(
                    " \u{25ba} {}",
                    format_label_kind("multi_typed_declaration", "MultiTypedDeclaration")
                ),
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
                format!(
                    " \u{25ba} {}",
                    format_label_kind("version_range", "VersionRange")
                ),
            );
            (range, frags)
        }

        VersionExpression::VersionTerm(ref element) => {
            let (range, mut frags) = render_version_term(source, element, depth);

            frags.insert(
                0,
                format!(
                    " \u{25ba} {}",
                    format_label_kind("version_term", "VersionTerm")
                ),
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
                format!(
                    " \u{25ba} {}",
                    format_label_kind("simple_version_literal", "SimpleVersionLiteral")
                ),
            );
            (range, frags)
        }

        VersionLiteral::PragmaStringLiteral(ref element) => {
            let (range, mut frags) = render_terminal(source, &element.range);

            frags.insert(
                0,
                format!(
                    " \u{25ba} {}",
                    format_label_kind("pragma_string_literal", "PragmaStringLiteral")
                ),
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
                format!(
                    " \u{25ba} {}",
                    format_label_kind("pragma_caret", "PragmaCaret")
                ),
            );
            (range, frags)
        }

        VersionOperator::PragmaTilde(ref element) => {
            let (range, mut frags) = render_terminal(source, &element.range);

            frags.insert(
                0,
                format!(
                    " \u{25ba} {}",
                    format_label_kind("pragma_tilde", "PragmaTilde")
                ),
            );
            (range, frags)
        }

        VersionOperator::PragmaEqual(ref element) => {
            let (range, mut frags) = render_terminal(source, &element.range);

            frags.insert(
                0,
                format!(
                    " \u{25ba} {}",
                    format_label_kind("pragma_equal", "PragmaEqual")
                ),
            );
            (range, frags)
        }

        VersionOperator::PragmaLessThan(ref element) => {
            let (range, mut frags) = render_terminal(source, &element.range);

            frags.insert(
                0,
                format!(
                    " \u{25ba} {}",
                    format_label_kind("pragma_less_than", "PragmaLessThan")
                ),
            );
            (range, frags)
        }

        VersionOperator::PragmaGreaterThan(ref element) => {
            let (range, mut frags) = render_terminal(source, &element.range);

            frags.insert(
                0,
                format!(
                    " \u{25ba} {}",
                    format_label_kind("pragma_greater_than", "PragmaGreaterThan")
                ),
            );
            (range, frags)
        }

        VersionOperator::PragmaLessThanEqual(ref element) => {
            let (range, mut frags) = render_terminal(source, &element.range);

            frags.insert(
                0,
                format!(
                    " \u{25ba} {}",
                    format_label_kind("pragma_less_than_equal", "PragmaLessThanEqual")
                ),
            );
            (range, frags)
        }

        VersionOperator::PragmaGreaterThanEqual(ref element) => {
            let (range, mut frags) = render_terminal(source, &element.range);

            frags.insert(
                0,
                format!(
                    " \u{25ba} {}",
                    format_label_kind("pragma_greater_than_equal", "PragmaGreaterThanEqual")
                ),
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
                format!(
                    " \u{25ba} {}",
                    format_label_kind("yul_function_call_expression", "YulFunctionCallExpression")
                ),
            );
            (range, frags)
        }

        YulExpression::YulLiteral(ref element) => {
            let (range, mut frags) = render_yul_literal(source, element, depth);

            frags.insert(
                0,
                format!(
                    " \u{25ba} {}",
                    format_label_kind("yul_literal", "YulLiteral")
                ),
            );
            (range, frags)
        }

        YulExpression::YulPath(ref element) => {
            let (range, mut frags) = render_yul_path(source, element, depth);

            frags.insert(
                0,
                format!(" \u{25ba} {}", format_label_kind("yul_path", "YulPath")),
            );
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
                format!(
                    " \u{25ba} {}",
                    format_label_kind("yul_true_keyword", "YulTrueKeyword")
                ),
            );
            (range, frags)
        }

        YulLiteral::YulFalseKeyword(ref element) => {
            let (range, mut frags) = render_terminal(source, &element.range);

            frags.insert(
                0,
                format!(
                    " \u{25ba} {}",
                    format_label_kind("yul_false_keyword", "YulFalseKeyword")
                ),
            );
            (range, frags)
        }

        YulLiteral::YulDecimalLiteral(ref element) => {
            let (range, mut frags) = render_terminal(source, &element.range);

            frags.insert(
                0,
                format!(
                    " \u{25ba} {}",
                    format_label_kind("yul_decimal_literal", "YulDecimalLiteral")
                ),
            );
            (range, frags)
        }

        YulLiteral::YulHexLiteral(ref element) => {
            let (range, mut frags) = render_terminal(source, &element.range);

            frags.insert(
                0,
                format!(
                    " \u{25ba} {}",
                    format_label_kind("yul_hex_literal", "YulHexLiteral")
                ),
            );
            (range, frags)
        }

        YulLiteral::YulHexStringLiteral(ref element) => {
            let (range, mut frags) = render_terminal(source, &element.range);

            frags.insert(
                0,
                format!(
                    " \u{25ba} {}",
                    format_label_kind("yul_hex_string_literal", "YulHexStringLiteral")
                ),
            );
            (range, frags)
        }

        YulLiteral::YulStringLiteral(ref element) => {
            let (range, mut frags) = render_terminal(source, &element.range);

            frags.insert(
                0,
                format!(
                    " \u{25ba} {}",
                    format_label_kind("yul_string_literal", "YulStringLiteral")
                ),
            );
            (range, frags)
        }
    }
}

pub fn render_yul_statement(source: &str, node: &YulStatement, depth: usize) -> RenderedOutput {
    match node {
        YulStatement::YulBlock(ref element) => {
            let (range, mut frags) = render_yul_block(source, element, depth);

            frags.insert(
                0,
                format!(" \u{25ba} {}", format_label_kind("yul_block", "YulBlock")),
            );
            (range, frags)
        }

        YulStatement::YulFunctionDefinition(ref element) => {
            let (range, mut frags) = render_yul_function_definition(source, element, depth);

            frags.insert(
                0,
                format!(
                    " \u{25ba} {}",
                    format_label_kind("yul_function_definition", "YulFunctionDefinition")
                ),
            );
            (range, frags)
        }

        YulStatement::YulIfStatement(ref element) => {
            let (range, mut frags) = render_yul_if_statement(source, element, depth);

            frags.insert(
                0,
                format!(
                    " \u{25ba} {}",
                    format_label_kind("yul_if_statement", "YulIfStatement")
                ),
            );
            (range, frags)
        }

        YulStatement::YulForStatement(ref element) => {
            let (range, mut frags) = render_yul_for_statement(source, element, depth);

            frags.insert(
                0,
                format!(
                    " \u{25ba} {}",
                    format_label_kind("yul_for_statement", "YulForStatement")
                ),
            );
            (range, frags)
        }

        YulStatement::YulSwitchStatement(ref element) => {
            let (range, mut frags) = render_yul_switch_statement(source, element, depth);

            frags.insert(
                0,
                format!(
                    " \u{25ba} {}",
                    format_label_kind("yul_switch_statement", "YulSwitchStatement")
                ),
            );
            (range, frags)
        }

        YulStatement::YulLeaveStatement(ref element) => {
            let (range, mut frags) = render_yul_leave_statement(source, element, depth);

            frags.insert(
                0,
                format!(
                    " \u{25ba} {}",
                    format_label_kind("yul_leave_statement", "YulLeaveStatement")
                ),
            );
            (range, frags)
        }

        YulStatement::YulBreakStatement(ref element) => {
            let (range, mut frags) = render_yul_break_statement(source, element, depth);

            frags.insert(
                0,
                format!(
                    " \u{25ba} {}",
                    format_label_kind("yul_break_statement", "YulBreakStatement")
                ),
            );
            (range, frags)
        }

        YulStatement::YulContinueStatement(ref element) => {
            let (range, mut frags) = render_yul_continue_statement(source, element, depth);

            frags.insert(
                0,
                format!(
                    " \u{25ba} {}",
                    format_label_kind("yul_continue_statement", "YulContinueStatement")
                ),
            );
            (range, frags)
        }

        YulStatement::YulVariableAssignmentStatement(ref element) => {
            let (range, mut frags) =
                render_yul_variable_assignment_statement(source, element, depth);

            frags.insert(
                0,
                format!(
                    " \u{25ba} {}",
                    format_label_kind(
                        "yul_variable_assignment_statement",
                        "YulVariableAssignmentStatement"
                    )
                ),
            );
            (range, frags)
        }

        YulStatement::YulVariableDeclarationStatement(ref element) => {
            let (range, mut frags) =
                render_yul_variable_declaration_statement(source, element, depth);

            frags.insert(
                0,
                format!(
                    " \u{25ba} {}",
                    format_label_kind(
                        "yul_variable_declaration_statement",
                        "YulVariableDeclarationStatement"
                    )
                ),
            );
            (range, frags)
        }

        YulStatement::YulExpression(ref element) => {
            let (range, mut frags) = render_yul_expression(source, element, depth);

            frags.insert(
                0,
                format!(
                    " \u{25ba} {}",
                    format_label_kind("yul_expression", "YulExpression")
                ),
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
                format!(
                    " \u{25ba} {}",
                    format_label_kind("yul_default_case", "YulDefaultCase")
                ),
            );
            (range, frags)
        }

        YulSwitchCase::YulValueCase(ref element) => {
            let (range, mut frags) = render_yul_value_case(source, element, depth);

            frags.insert(
                0,
                format!(
                    " \u{25ba} {}",
                    format_label_kind("yul_value_case", "YulValueCase")
                ),
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

// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

#![allow(non_camel_case_types)]
#![allow(dead_code)]
#![allow(unused_variables)]

use std::marker::PhantomData;

use bumpalo::boxed::Box;
use bumpalo::collections::Vec;
use bumpalo::Bump;

// TODO(v2):
// - (perf) don't use terminals that are not needed
// - (feat) visitor/traversal/serializer
// - (feat) span information, where applicable

//
// Sequences:
//
// Note: All sequences are boxed, this keeps sizes down and avoids recursive types

pub type AbicoderPragma<'arena> = Box<'arena, AbicoderPragmaStruct<'arena>>;

#[derive(Debug)]
pub struct AbicoderPragmaStruct<'arena> {
    pub abicoder_keyword: AbicoderKeyword<'arena>,
    pub version: AbicoderVersion<'arena>,
}

pub fn new_abicoder_pragma<'arena>(
    arena: &'arena Bump,
    abicoder_keyword: AbicoderKeyword<'arena>,
    version: AbicoderVersion<'arena>,
) -> AbicoderPragma<'arena> {
    Box::new_in(
        AbicoderPragmaStruct {
            abicoder_keyword,
            version,
        },
        arena,
    )
}

pub type AdditiveExpression<'arena> = Box<'arena, AdditiveExpressionStruct<'arena>>;

#[derive(Debug)]
pub struct AdditiveExpressionStruct<'arena> {
    pub left_operand: Expression<'arena>,
    pub expression_additive_expression_operator: Expression_AdditiveExpression_Operator<'arena>,
    pub right_operand: Expression<'arena>,
}

pub fn new_additive_expression<'arena>(
    arena: &'arena Bump,
    left_operand: Expression<'arena>,
    expression_additive_expression_operator: Expression_AdditiveExpression_Operator<'arena>,
    right_operand: Expression<'arena>,
) -> AdditiveExpression<'arena> {
    Box::new_in(
        AdditiveExpressionStruct {
            left_operand,
            expression_additive_expression_operator,
            right_operand,
        },
        arena,
    )
}

pub type AddressType<'arena> = Box<'arena, AddressTypeStruct<'arena>>;

#[derive(Debug)]
pub struct AddressTypeStruct<'arena> {
    pub address_keyword: AddressKeyword<'arena>,
    pub payable_keyword: Option<PayableKeyword<'arena>>,
}

pub fn new_address_type<'arena>(
    arena: &'arena Bump,
    address_keyword: AddressKeyword<'arena>,
    payable_keyword: Option<PayableKeyword<'arena>>,
) -> AddressType<'arena> {
    Box::new_in(
        AddressTypeStruct {
            address_keyword,
            payable_keyword,
        },
        arena,
    )
}

pub type AndExpression<'arena> = Box<'arena, AndExpressionStruct<'arena>>;

#[derive(Debug)]
pub struct AndExpressionStruct<'arena> {
    pub left_operand: Expression<'arena>,
    pub operator: AmpersandAmpersand<'arena>,
    pub right_operand: Expression<'arena>,
}

pub fn new_and_expression<'arena>(
    arena: &'arena Bump,
    left_operand: Expression<'arena>,
    operator: AmpersandAmpersand<'arena>,
    right_operand: Expression<'arena>,
) -> AndExpression<'arena> {
    Box::new_in(
        AndExpressionStruct {
            left_operand,
            operator,
            right_operand,
        },
        arena,
    )
}

pub type ArrayExpression<'arena> = Box<'arena, ArrayExpressionStruct<'arena>>;

#[derive(Debug)]
pub struct ArrayExpressionStruct<'arena> {
    pub open_bracket: OpenBracket<'arena>,
    pub items: ArrayValues<'arena>,
    pub close_bracket: CloseBracket<'arena>,
}

pub fn new_array_expression<'arena>(
    arena: &'arena Bump,
    open_bracket: OpenBracket<'arena>,
    items: ArrayValues<'arena>,
    close_bracket: CloseBracket<'arena>,
) -> ArrayExpression<'arena> {
    Box::new_in(
        ArrayExpressionStruct {
            open_bracket,
            items,
            close_bracket,
        },
        arena,
    )
}

pub type ArrayTypeName<'arena> = Box<'arena, ArrayTypeNameStruct<'arena>>;

#[derive(Debug)]
pub struct ArrayTypeNameStruct<'arena> {
    pub operand: TypeName<'arena>,
    pub open_bracket: OpenBracket<'arena>,
    pub index: Option<Expression<'arena>>,
    pub close_bracket: CloseBracket<'arena>,
}

pub fn new_array_type_name<'arena>(
    arena: &'arena Bump,
    operand: TypeName<'arena>,
    open_bracket: OpenBracket<'arena>,
    index: Option<Expression<'arena>>,
    close_bracket: CloseBracket<'arena>,
) -> ArrayTypeName<'arena> {
    Box::new_in(
        ArrayTypeNameStruct {
            operand,
            open_bracket,
            index,
            close_bracket,
        },
        arena,
    )
}

pub type AssemblyFlagsDeclaration<'arena> = Box<'arena, AssemblyFlagsDeclarationStruct<'arena>>;

#[derive(Debug)]
pub struct AssemblyFlagsDeclarationStruct<'arena> {
    pub open_paren: OpenParen<'arena>,
    pub flags: AssemblyFlags<'arena>,
    pub close_paren: CloseParen<'arena>,
}

pub fn new_assembly_flags_declaration<'arena>(
    arena: &'arena Bump,
    open_paren: OpenParen<'arena>,
    flags: AssemblyFlags<'arena>,
    close_paren: CloseParen<'arena>,
) -> AssemblyFlagsDeclaration<'arena> {
    Box::new_in(
        AssemblyFlagsDeclarationStruct {
            open_paren,
            flags,
            close_paren,
        },
        arena,
    )
}

pub type AssemblyStatement<'arena> = Box<'arena, AssemblyStatementStruct<'arena>>;

#[derive(Debug)]
pub struct AssemblyStatementStruct<'arena> {
    pub assembly_keyword: AssemblyKeyword<'arena>,
    pub label: Option<StringLiteral<'arena>>,
    pub flags: Option<AssemblyFlagsDeclaration<'arena>>,
    pub body: YulBlock<'arena>,
}

pub fn new_assembly_statement<'arena>(
    arena: &'arena Bump,
    assembly_keyword: AssemblyKeyword<'arena>,
    label: Option<StringLiteral<'arena>>,
    flags: Option<AssemblyFlagsDeclaration<'arena>>,
    body: YulBlock<'arena>,
) -> AssemblyStatement<'arena> {
    Box::new_in(
        AssemblyStatementStruct {
            assembly_keyword,
            label,
            flags,
            body,
        },
        arena,
    )
}

pub type AssignmentExpression<'arena> = Box<'arena, AssignmentExpressionStruct<'arena>>;

#[derive(Debug)]
pub struct AssignmentExpressionStruct<'arena> {
    pub left_operand: Expression<'arena>,
    pub expression_assignment_expression_operator: Expression_AssignmentExpression_Operator<'arena>,
    pub right_operand: Expression<'arena>,
}

pub fn new_assignment_expression<'arena>(
    arena: &'arena Bump,
    left_operand: Expression<'arena>,
    expression_assignment_expression_operator: Expression_AssignmentExpression_Operator<'arena>,
    right_operand: Expression<'arena>,
) -> AssignmentExpression<'arena> {
    Box::new_in(
        AssignmentExpressionStruct {
            left_operand,
            expression_assignment_expression_operator,
            right_operand,
        },
        arena,
    )
}

pub type BitwiseAndExpression<'arena> = Box<'arena, BitwiseAndExpressionStruct<'arena>>;

#[derive(Debug)]
pub struct BitwiseAndExpressionStruct<'arena> {
    pub left_operand: Expression<'arena>,
    pub operator: Ampersand<'arena>,
    pub right_operand: Expression<'arena>,
}

pub fn new_bitwise_and_expression<'arena>(
    arena: &'arena Bump,
    left_operand: Expression<'arena>,
    operator: Ampersand<'arena>,
    right_operand: Expression<'arena>,
) -> BitwiseAndExpression<'arena> {
    Box::new_in(
        BitwiseAndExpressionStruct {
            left_operand,
            operator,
            right_operand,
        },
        arena,
    )
}

pub type BitwiseOrExpression<'arena> = Box<'arena, BitwiseOrExpressionStruct<'arena>>;

#[derive(Debug)]
pub struct BitwiseOrExpressionStruct<'arena> {
    pub left_operand: Expression<'arena>,
    pub operator: Bar<'arena>,
    pub right_operand: Expression<'arena>,
}

pub fn new_bitwise_or_expression<'arena>(
    arena: &'arena Bump,
    left_operand: Expression<'arena>,
    operator: Bar<'arena>,
    right_operand: Expression<'arena>,
) -> BitwiseOrExpression<'arena> {
    Box::new_in(
        BitwiseOrExpressionStruct {
            left_operand,
            operator,
            right_operand,
        },
        arena,
    )
}

pub type BitwiseXorExpression<'arena> = Box<'arena, BitwiseXorExpressionStruct<'arena>>;

#[derive(Debug)]
pub struct BitwiseXorExpressionStruct<'arena> {
    pub left_operand: Expression<'arena>,
    pub operator: Caret<'arena>,
    pub right_operand: Expression<'arena>,
}

pub fn new_bitwise_xor_expression<'arena>(
    arena: &'arena Bump,
    left_operand: Expression<'arena>,
    operator: Caret<'arena>,
    right_operand: Expression<'arena>,
) -> BitwiseXorExpression<'arena> {
    Box::new_in(
        BitwiseXorExpressionStruct {
            left_operand,
            operator,
            right_operand,
        },
        arena,
    )
}

pub type Block<'arena> = Box<'arena, BlockStruct<'arena>>;

#[derive(Debug)]
pub struct BlockStruct<'arena> {
    pub open_brace: OpenBrace<'arena>,
    pub statements: Statements<'arena>,
    pub close_brace: CloseBrace<'arena>,
}

pub fn new_block<'arena>(
    arena: &'arena Bump,
    open_brace: OpenBrace<'arena>,
    statements: Statements<'arena>,
    close_brace: CloseBrace<'arena>,
) -> Block<'arena> {
    Box::new_in(
        BlockStruct {
            open_brace,
            statements,
            close_brace,
        },
        arena,
    )
}

pub type BreakStatement<'arena> = Box<'arena, BreakStatementStruct<'arena>>;

#[derive(Debug)]
pub struct BreakStatementStruct<'arena> {
    pub break_keyword: BreakKeyword<'arena>,
    pub semicolon: Semicolon<'arena>,
}

pub fn new_break_statement<'arena>(
    arena: &'arena Bump,
    break_keyword: BreakKeyword<'arena>,
    semicolon: Semicolon<'arena>,
) -> BreakStatement<'arena> {
    Box::new_in(
        BreakStatementStruct {
            break_keyword,
            semicolon,
        },
        arena,
    )
}

pub type CallOptionsExpression<'arena> = Box<'arena, CallOptionsExpressionStruct<'arena>>;

#[derive(Debug)]
pub struct CallOptionsExpressionStruct<'arena> {
    pub operand: Expression<'arena>,
    pub open_brace: OpenBrace<'arena>,
    pub options: CallOptions<'arena>,
    pub close_brace: CloseBrace<'arena>,
}

pub fn new_call_options_expression<'arena>(
    arena: &'arena Bump,
    operand: Expression<'arena>,
    open_brace: OpenBrace<'arena>,
    options: CallOptions<'arena>,
    close_brace: CloseBrace<'arena>,
) -> CallOptionsExpression<'arena> {
    Box::new_in(
        CallOptionsExpressionStruct {
            operand,
            open_brace,
            options,
            close_brace,
        },
        arena,
    )
}

pub type CatchClause<'arena> = Box<'arena, CatchClauseStruct<'arena>>;

#[derive(Debug)]
pub struct CatchClauseStruct<'arena> {
    pub catch_keyword: CatchKeyword<'arena>,
    pub error: Option<CatchClauseError<'arena>>,
    pub body: Block<'arena>,
}

pub fn new_catch_clause<'arena>(
    arena: &'arena Bump,
    catch_keyword: CatchKeyword<'arena>,
    error: Option<CatchClauseError<'arena>>,
    body: Block<'arena>,
) -> CatchClause<'arena> {
    Box::new_in(
        CatchClauseStruct {
            catch_keyword,
            error,
            body,
        },
        arena,
    )
}

pub type CatchClauseError<'arena> = Box<'arena, CatchClauseErrorStruct<'arena>>;

#[derive(Debug)]
pub struct CatchClauseErrorStruct<'arena> {
    pub name: Option<Identifier<'arena>>,
    pub parameters: ParametersDeclaration<'arena>,
}

pub fn new_catch_clause_error<'arena>(
    arena: &'arena Bump,
    name: Option<Identifier<'arena>>,
    parameters: ParametersDeclaration<'arena>,
) -> CatchClauseError<'arena> {
    Box::new_in(CatchClauseErrorStruct { name, parameters }, arena)
}

pub type ConditionalExpression<'arena> = Box<'arena, ConditionalExpressionStruct<'arena>>;

#[derive(Debug)]
pub struct ConditionalExpressionStruct<'arena> {
    pub operand: Expression<'arena>,
    pub question_mark: QuestionMark<'arena>,
    pub true_expression: Expression<'arena>,
    pub colon: Colon<'arena>,
    pub false_expression: Expression<'arena>,
}

pub fn new_conditional_expression<'arena>(
    arena: &'arena Bump,
    operand: Expression<'arena>,
    question_mark: QuestionMark<'arena>,
    true_expression: Expression<'arena>,
    colon: Colon<'arena>,
    false_expression: Expression<'arena>,
) -> ConditionalExpression<'arena> {
    Box::new_in(
        ConditionalExpressionStruct {
            operand,
            question_mark,
            true_expression,
            colon,
            false_expression,
        },
        arena,
    )
}

pub type ConstantDefinition<'arena> = Box<'arena, ConstantDefinitionStruct<'arena>>;

#[derive(Debug)]
pub struct ConstantDefinitionStruct<'arena> {
    pub type_name: TypeName<'arena>,
    pub constant_keyword: ConstantKeyword<'arena>,
    pub name: Identifier<'arena>,
    pub equal: Equal<'arena>,
    pub value: Expression<'arena>,
    pub semicolon: Semicolon<'arena>,
}

pub fn new_constant_definition<'arena>(
    arena: &'arena Bump,
    type_name: TypeName<'arena>,
    constant_keyword: ConstantKeyword<'arena>,
    name: Identifier<'arena>,
    equal: Equal<'arena>,
    value: Expression<'arena>,
    semicolon: Semicolon<'arena>,
) -> ConstantDefinition<'arena> {
    Box::new_in(
        ConstantDefinitionStruct {
            type_name,
            constant_keyword,
            name,
            equal,
            value,
            semicolon,
        },
        arena,
    )
}

pub type ConstructorDefinition<'arena> = Box<'arena, ConstructorDefinitionStruct<'arena>>;

#[derive(Debug)]
pub struct ConstructorDefinitionStruct<'arena> {
    pub constructor_keyword: ConstructorKeyword<'arena>,
    pub parameters: ParametersDeclaration<'arena>,
    pub attributes: ConstructorAttributes<'arena>,
    pub body: Block<'arena>,
}

pub fn new_constructor_definition<'arena>(
    arena: &'arena Bump,
    constructor_keyword: ConstructorKeyword<'arena>,
    parameters: ParametersDeclaration<'arena>,
    attributes: ConstructorAttributes<'arena>,
    body: Block<'arena>,
) -> ConstructorDefinition<'arena> {
    Box::new_in(
        ConstructorDefinitionStruct {
            constructor_keyword,
            parameters,
            attributes,
            body,
        },
        arena,
    )
}

pub type ContinueStatement<'arena> = Box<'arena, ContinueStatementStruct<'arena>>;

#[derive(Debug)]
pub struct ContinueStatementStruct<'arena> {
    pub continue_keyword: ContinueKeyword<'arena>,
    pub semicolon: Semicolon<'arena>,
}

pub fn new_continue_statement<'arena>(
    arena: &'arena Bump,
    continue_keyword: ContinueKeyword<'arena>,
    semicolon: Semicolon<'arena>,
) -> ContinueStatement<'arena> {
    Box::new_in(
        ContinueStatementStruct {
            continue_keyword,
            semicolon,
        },
        arena,
    )
}

pub type ContractDefinition<'arena> = Box<'arena, ContractDefinitionStruct<'arena>>;

#[derive(Debug)]
pub struct ContractDefinitionStruct<'arena> {
    pub abstract_keyword: Option<AbstractKeyword<'arena>>,
    pub contract_keyword: ContractKeyword<'arena>,
    pub name: Identifier<'arena>,
    pub specifiers: ContractSpecifiers<'arena>,
    pub open_brace: OpenBrace<'arena>,
    pub members: ContractMembers<'arena>,
    pub close_brace: CloseBrace<'arena>,
}

pub fn new_contract_definition<'arena>(
    arena: &'arena Bump,
    abstract_keyword: Option<AbstractKeyword<'arena>>,
    contract_keyword: ContractKeyword<'arena>,
    name: Identifier<'arena>,
    specifiers: ContractSpecifiers<'arena>,
    open_brace: OpenBrace<'arena>,
    members: ContractMembers<'arena>,
    close_brace: CloseBrace<'arena>,
) -> ContractDefinition<'arena> {
    Box::new_in(
        ContractDefinitionStruct {
            abstract_keyword,
            contract_keyword,
            name,
            specifiers,
            open_brace,
            members,
            close_brace,
        },
        arena,
    )
}

pub type DecimalNumberExpression<'arena> = Box<'arena, DecimalNumberExpressionStruct<'arena>>;

#[derive(Debug)]
pub struct DecimalNumberExpressionStruct<'arena> {
    pub literal: DecimalLiteral<'arena>,
    pub unit: Option<NumberUnit<'arena>>,
}

pub fn new_decimal_number_expression<'arena>(
    arena: &'arena Bump,
    literal: DecimalLiteral<'arena>,
    unit: Option<NumberUnit<'arena>>,
) -> DecimalNumberExpression<'arena> {
    Box::new_in(DecimalNumberExpressionStruct { literal, unit }, arena)
}

pub type DoWhileStatement<'arena> = Box<'arena, DoWhileStatementStruct<'arena>>;

#[derive(Debug)]
pub struct DoWhileStatementStruct<'arena> {
    pub do_keyword: DoKeyword<'arena>,
    pub body: Statement<'arena>,
    pub while_keyword: WhileKeyword<'arena>,
    pub open_paren: OpenParen<'arena>,
    pub condition: Expression<'arena>,
    pub close_paren: CloseParen<'arena>,
    pub semicolon: Semicolon<'arena>,
}

pub fn new_do_while_statement<'arena>(
    arena: &'arena Bump,
    do_keyword: DoKeyword<'arena>,
    body: Statement<'arena>,
    while_keyword: WhileKeyword<'arena>,
    open_paren: OpenParen<'arena>,
    condition: Expression<'arena>,
    close_paren: CloseParen<'arena>,
    semicolon: Semicolon<'arena>,
) -> DoWhileStatement<'arena> {
    Box::new_in(
        DoWhileStatementStruct {
            do_keyword,
            body,
            while_keyword,
            open_paren,
            condition,
            close_paren,
            semicolon,
        },
        arena,
    )
}

pub type ElseBranch<'arena> = Box<'arena, ElseBranchStruct<'arena>>;

#[derive(Debug)]
pub struct ElseBranchStruct<'arena> {
    pub else_keyword: ElseKeyword<'arena>,
    pub body: Statement<'arena>,
}

pub fn new_else_branch<'arena>(
    arena: &'arena Bump,
    else_keyword: ElseKeyword<'arena>,
    body: Statement<'arena>,
) -> ElseBranch<'arena> {
    Box::new_in(ElseBranchStruct { else_keyword, body }, arena)
}

pub type EmitStatement<'arena> = Box<'arena, EmitStatementStruct<'arena>>;

#[derive(Debug)]
pub struct EmitStatementStruct<'arena> {
    pub emit_keyword: EmitKeyword<'arena>,
    pub event: IdentifierPath<'arena>,
    pub arguments: ArgumentsDeclaration<'arena>,
    pub semicolon: Semicolon<'arena>,
}

pub fn new_emit_statement<'arena>(
    arena: &'arena Bump,
    emit_keyword: EmitKeyword<'arena>,
    event: IdentifierPath<'arena>,
    arguments: ArgumentsDeclaration<'arena>,
    semicolon: Semicolon<'arena>,
) -> EmitStatement<'arena> {
    Box::new_in(
        EmitStatementStruct {
            emit_keyword,
            event,
            arguments,
            semicolon,
        },
        arena,
    )
}

pub type EnumDefinition<'arena> = Box<'arena, EnumDefinitionStruct<'arena>>;

#[derive(Debug)]
pub struct EnumDefinitionStruct<'arena> {
    pub enum_keyword: EnumKeyword<'arena>,
    pub name: Identifier<'arena>,
    pub open_brace: OpenBrace<'arena>,
    pub members: EnumMembers<'arena>,
    pub close_brace: CloseBrace<'arena>,
}

pub fn new_enum_definition<'arena>(
    arena: &'arena Bump,
    enum_keyword: EnumKeyword<'arena>,
    name: Identifier<'arena>,
    open_brace: OpenBrace<'arena>,
    members: EnumMembers<'arena>,
    close_brace: CloseBrace<'arena>,
) -> EnumDefinition<'arena> {
    Box::new_in(
        EnumDefinitionStruct {
            enum_keyword,
            name,
            open_brace,
            members,
            close_brace,
        },
        arena,
    )
}

pub type EqualityExpression<'arena> = Box<'arena, EqualityExpressionStruct<'arena>>;

#[derive(Debug)]
pub struct EqualityExpressionStruct<'arena> {
    pub left_operand: Expression<'arena>,
    pub expression_equality_expression_operator: Expression_EqualityExpression_Operator<'arena>,
    pub right_operand: Expression<'arena>,
}

pub fn new_equality_expression<'arena>(
    arena: &'arena Bump,
    left_operand: Expression<'arena>,
    expression_equality_expression_operator: Expression_EqualityExpression_Operator<'arena>,
    right_operand: Expression<'arena>,
) -> EqualityExpression<'arena> {
    Box::new_in(
        EqualityExpressionStruct {
            left_operand,
            expression_equality_expression_operator,
            right_operand,
        },
        arena,
    )
}

pub type ErrorDefinition<'arena> = Box<'arena, ErrorDefinitionStruct<'arena>>;

#[derive(Debug)]
pub struct ErrorDefinitionStruct<'arena> {
    pub error_keyword: ErrorKeyword<'arena>,
    pub name: Identifier<'arena>,
    pub members: ErrorParametersDeclaration<'arena>,
    pub semicolon: Semicolon<'arena>,
}

pub fn new_error_definition<'arena>(
    arena: &'arena Bump,
    error_keyword: ErrorKeyword<'arena>,
    name: Identifier<'arena>,
    members: ErrorParametersDeclaration<'arena>,
    semicolon: Semicolon<'arena>,
) -> ErrorDefinition<'arena> {
    Box::new_in(
        ErrorDefinitionStruct {
            error_keyword,
            name,
            members,
            semicolon,
        },
        arena,
    )
}

pub type ErrorParameter<'arena> = Box<'arena, ErrorParameterStruct<'arena>>;

#[derive(Debug)]
pub struct ErrorParameterStruct<'arena> {
    pub type_name: TypeName<'arena>,
    pub name: Option<Identifier<'arena>>,
}

pub fn new_error_parameter<'arena>(
    arena: &'arena Bump,
    type_name: TypeName<'arena>,
    name: Option<Identifier<'arena>>,
) -> ErrorParameter<'arena> {
    Box::new_in(ErrorParameterStruct { type_name, name }, arena)
}

pub type ErrorParametersDeclaration<'arena> = Box<'arena, ErrorParametersDeclarationStruct<'arena>>;

#[derive(Debug)]
pub struct ErrorParametersDeclarationStruct<'arena> {
    pub open_paren: OpenParen<'arena>,
    pub parameters: ErrorParameters<'arena>,
    pub close_paren: CloseParen<'arena>,
}

pub fn new_error_parameters_declaration<'arena>(
    arena: &'arena Bump,
    open_paren: OpenParen<'arena>,
    parameters: ErrorParameters<'arena>,
    close_paren: CloseParen<'arena>,
) -> ErrorParametersDeclaration<'arena> {
    Box::new_in(
        ErrorParametersDeclarationStruct {
            open_paren,
            parameters,
            close_paren,
        },
        arena,
    )
}

pub type EventDefinition<'arena> = Box<'arena, EventDefinitionStruct<'arena>>;

#[derive(Debug)]
pub struct EventDefinitionStruct<'arena> {
    pub event_keyword: EventKeyword<'arena>,
    pub name: Identifier<'arena>,
    pub parameters: EventParametersDeclaration<'arena>,
    pub anonymous_keyword: Option<AnonymousKeyword<'arena>>,
    pub semicolon: Semicolon<'arena>,
}

pub fn new_event_definition<'arena>(
    arena: &'arena Bump,
    event_keyword: EventKeyword<'arena>,
    name: Identifier<'arena>,
    parameters: EventParametersDeclaration<'arena>,
    anonymous_keyword: Option<AnonymousKeyword<'arena>>,
    semicolon: Semicolon<'arena>,
) -> EventDefinition<'arena> {
    Box::new_in(
        EventDefinitionStruct {
            event_keyword,
            name,
            parameters,
            anonymous_keyword,
            semicolon,
        },
        arena,
    )
}

pub type EventParameter<'arena> = Box<'arena, EventParameterStruct<'arena>>;

#[derive(Debug)]
pub struct EventParameterStruct<'arena> {
    pub type_name: TypeName<'arena>,
    pub indexed_keyword: Option<IndexedKeyword<'arena>>,
    pub name: Option<Identifier<'arena>>,
}

pub fn new_event_parameter<'arena>(
    arena: &'arena Bump,
    type_name: TypeName<'arena>,
    indexed_keyword: Option<IndexedKeyword<'arena>>,
    name: Option<Identifier<'arena>>,
) -> EventParameter<'arena> {
    Box::new_in(
        EventParameterStruct {
            type_name,
            indexed_keyword,
            name,
        },
        arena,
    )
}

pub type EventParametersDeclaration<'arena> = Box<'arena, EventParametersDeclarationStruct<'arena>>;

#[derive(Debug)]
pub struct EventParametersDeclarationStruct<'arena> {
    pub open_paren: OpenParen<'arena>,
    pub parameters: EventParameters<'arena>,
    pub close_paren: CloseParen<'arena>,
}

pub fn new_event_parameters_declaration<'arena>(
    arena: &'arena Bump,
    open_paren: OpenParen<'arena>,
    parameters: EventParameters<'arena>,
    close_paren: CloseParen<'arena>,
) -> EventParametersDeclaration<'arena> {
    Box::new_in(
        EventParametersDeclarationStruct {
            open_paren,
            parameters,
            close_paren,
        },
        arena,
    )
}

pub type ExperimentalPragma<'arena> = Box<'arena, ExperimentalPragmaStruct<'arena>>;

#[derive(Debug)]
pub struct ExperimentalPragmaStruct<'arena> {
    pub experimental_keyword: ExperimentalKeyword<'arena>,
    pub feature: ExperimentalFeature<'arena>,
}

pub fn new_experimental_pragma<'arena>(
    arena: &'arena Bump,
    experimental_keyword: ExperimentalKeyword<'arena>,
    feature: ExperimentalFeature<'arena>,
) -> ExperimentalPragma<'arena> {
    Box::new_in(
        ExperimentalPragmaStruct {
            experimental_keyword,
            feature,
        },
        arena,
    )
}

pub type ExponentiationExpression<'arena> = Box<'arena, ExponentiationExpressionStruct<'arena>>;

#[derive(Debug)]
pub struct ExponentiationExpressionStruct<'arena> {
    pub left_operand: Expression<'arena>,
    pub expression_exponentiation_expression_operator:
        Expression_ExponentiationExpression_Operator<'arena>,
    pub right_operand: Expression<'arena>,
}

pub fn new_exponentiation_expression<'arena>(
    arena: &'arena Bump,
    left_operand: Expression<'arena>,
    expression_exponentiation_expression_operator: Expression_ExponentiationExpression_Operator<
        'arena,
    >,
    right_operand: Expression<'arena>,
) -> ExponentiationExpression<'arena> {
    Box::new_in(
        ExponentiationExpressionStruct {
            left_operand,
            expression_exponentiation_expression_operator,
            right_operand,
        },
        arena,
    )
}

pub type ExpressionStatement<'arena> = Box<'arena, ExpressionStatementStruct<'arena>>;

#[derive(Debug)]
pub struct ExpressionStatementStruct<'arena> {
    pub expression: Expression<'arena>,
    pub semicolon: Semicolon<'arena>,
}

pub fn new_expression_statement<'arena>(
    arena: &'arena Bump,
    expression: Expression<'arena>,
    semicolon: Semicolon<'arena>,
) -> ExpressionStatement<'arena> {
    Box::new_in(
        ExpressionStatementStruct {
            expression,
            semicolon,
        },
        arena,
    )
}

pub type FallbackFunctionDefinition<'arena> = Box<'arena, FallbackFunctionDefinitionStruct<'arena>>;

#[derive(Debug)]
pub struct FallbackFunctionDefinitionStruct<'arena> {
    pub fallback_keyword: FallbackKeyword<'arena>,
    pub parameters: ParametersDeclaration<'arena>,
    pub attributes: FallbackFunctionAttributes<'arena>,
    pub returns: Option<ReturnsDeclaration<'arena>>,
    pub body: FunctionBody<'arena>,
}

pub fn new_fallback_function_definition<'arena>(
    arena: &'arena Bump,
    fallback_keyword: FallbackKeyword<'arena>,
    parameters: ParametersDeclaration<'arena>,
    attributes: FallbackFunctionAttributes<'arena>,
    returns: Option<ReturnsDeclaration<'arena>>,
    body: FunctionBody<'arena>,
) -> FallbackFunctionDefinition<'arena> {
    Box::new_in(
        FallbackFunctionDefinitionStruct {
            fallback_keyword,
            parameters,
            attributes,
            returns,
            body,
        },
        arena,
    )
}

pub type ForStatement<'arena> = Box<'arena, ForStatementStruct<'arena>>;

#[derive(Debug)]
pub struct ForStatementStruct<'arena> {
    pub for_keyword: ForKeyword<'arena>,
    pub open_paren: OpenParen<'arena>,
    pub initialization: ForStatementInitialization<'arena>,
    pub condition: ForStatementCondition<'arena>,
    pub iterator: Option<Expression<'arena>>,
    pub close_paren: CloseParen<'arena>,
    pub body: Statement<'arena>,
}

pub fn new_for_statement<'arena>(
    arena: &'arena Bump,
    for_keyword: ForKeyword<'arena>,
    open_paren: OpenParen<'arena>,
    initialization: ForStatementInitialization<'arena>,
    condition: ForStatementCondition<'arena>,
    iterator: Option<Expression<'arena>>,
    close_paren: CloseParen<'arena>,
    body: Statement<'arena>,
) -> ForStatement<'arena> {
    Box::new_in(
        ForStatementStruct {
            for_keyword,
            open_paren,
            initialization,
            condition,
            iterator,
            close_paren,
            body,
        },
        arena,
    )
}

pub type FunctionCallExpression<'arena> = Box<'arena, FunctionCallExpressionStruct<'arena>>;

#[derive(Debug)]
pub struct FunctionCallExpressionStruct<'arena> {
    pub operand: Expression<'arena>,
    pub arguments: ArgumentsDeclaration<'arena>,
}

pub fn new_function_call_expression<'arena>(
    arena: &'arena Bump,
    operand: Expression<'arena>,
    arguments: ArgumentsDeclaration<'arena>,
) -> FunctionCallExpression<'arena> {
    Box::new_in(FunctionCallExpressionStruct { operand, arguments }, arena)
}

pub type FunctionDefinition<'arena> = Box<'arena, FunctionDefinitionStruct<'arena>>;

#[derive(Debug)]
pub struct FunctionDefinitionStruct<'arena> {
    pub function_keyword: FunctionKeyword<'arena>,
    pub name: FunctionName<'arena>,
    pub parameters: ParametersDeclaration<'arena>,
    pub attributes: FunctionAttributes<'arena>,
    pub returns: Option<ReturnsDeclaration<'arena>>,
    pub body: FunctionBody<'arena>,
}

pub fn new_function_definition<'arena>(
    arena: &'arena Bump,
    function_keyword: FunctionKeyword<'arena>,
    name: FunctionName<'arena>,
    parameters: ParametersDeclaration<'arena>,
    attributes: FunctionAttributes<'arena>,
    returns: Option<ReturnsDeclaration<'arena>>,
    body: FunctionBody<'arena>,
) -> FunctionDefinition<'arena> {
    Box::new_in(
        FunctionDefinitionStruct {
            function_keyword,
            name,
            parameters,
            attributes,
            returns,
            body,
        },
        arena,
    )
}

pub type FunctionType<'arena> = Box<'arena, FunctionTypeStruct<'arena>>;

#[derive(Debug)]
pub struct FunctionTypeStruct<'arena> {
    pub function_keyword: FunctionKeyword<'arena>,
    pub parameters: ParametersDeclaration<'arena>,
    pub attributes: FunctionTypeAttributes<'arena>,
    pub returns: Option<ReturnsDeclaration<'arena>>,
}

pub fn new_function_type<'arena>(
    arena: &'arena Bump,
    function_keyword: FunctionKeyword<'arena>,
    parameters: ParametersDeclaration<'arena>,
    attributes: FunctionTypeAttributes<'arena>,
    returns: Option<ReturnsDeclaration<'arena>>,
) -> FunctionType<'arena> {
    Box::new_in(
        FunctionTypeStruct {
            function_keyword,
            parameters,
            attributes,
            returns,
        },
        arena,
    )
}

pub type HexNumberExpression<'arena> = Box<'arena, HexNumberExpressionStruct<'arena>>;

#[derive(Debug)]
pub struct HexNumberExpressionStruct<'arena> {
    pub literal: HexLiteral<'arena>,
    pub unit: Option<NumberUnit<'arena>>,
}

pub fn new_hex_number_expression<'arena>(
    arena: &'arena Bump,
    literal: HexLiteral<'arena>,
    unit: Option<NumberUnit<'arena>>,
) -> HexNumberExpression<'arena> {
    Box::new_in(HexNumberExpressionStruct { literal, unit }, arena)
}

pub type IfStatement<'arena> = Box<'arena, IfStatementStruct<'arena>>;

#[derive(Debug)]
pub struct IfStatementStruct<'arena> {
    pub if_keyword: IfKeyword<'arena>,
    pub open_paren: OpenParen<'arena>,
    pub condition: Expression<'arena>,
    pub close_paren: CloseParen<'arena>,
    pub body: Statement<'arena>,
    pub else_branch: Option<ElseBranch<'arena>>,
}

pub fn new_if_statement<'arena>(
    arena: &'arena Bump,
    if_keyword: IfKeyword<'arena>,
    open_paren: OpenParen<'arena>,
    condition: Expression<'arena>,
    close_paren: CloseParen<'arena>,
    body: Statement<'arena>,
    else_branch: Option<ElseBranch<'arena>>,
) -> IfStatement<'arena> {
    Box::new_in(
        IfStatementStruct {
            if_keyword,
            open_paren,
            condition,
            close_paren,
            body,
            else_branch,
        },
        arena,
    )
}

pub type ImportAlias<'arena> = Box<'arena, ImportAliasStruct<'arena>>;

#[derive(Debug)]
pub struct ImportAliasStruct<'arena> {
    pub as_keyword: AsKeyword<'arena>,
    pub identifier: Identifier<'arena>,
}

pub fn new_import_alias<'arena>(
    arena: &'arena Bump,
    as_keyword: AsKeyword<'arena>,
    identifier: Identifier<'arena>,
) -> ImportAlias<'arena> {
    Box::new_in(
        ImportAliasStruct {
            as_keyword,
            identifier,
        },
        arena,
    )
}

pub type ImportDeconstruction<'arena> = Box<'arena, ImportDeconstructionStruct<'arena>>;

#[derive(Debug)]
pub struct ImportDeconstructionStruct<'arena> {
    pub open_brace: OpenBrace<'arena>,
    pub symbols: ImportDeconstructionSymbols<'arena>,
    pub close_brace: CloseBrace<'arena>,
    pub from_keyword: FromKeyword<'arena>,
    pub path: StringLiteral<'arena>,
}

pub fn new_import_deconstruction<'arena>(
    arena: &'arena Bump,
    open_brace: OpenBrace<'arena>,
    symbols: ImportDeconstructionSymbols<'arena>,
    close_brace: CloseBrace<'arena>,
    from_keyword: FromKeyword<'arena>,
    path: StringLiteral<'arena>,
) -> ImportDeconstruction<'arena> {
    Box::new_in(
        ImportDeconstructionStruct {
            open_brace,
            symbols,
            close_brace,
            from_keyword,
            path,
        },
        arena,
    )
}

pub type ImportDeconstructionSymbol<'arena> = Box<'arena, ImportDeconstructionSymbolStruct<'arena>>;

#[derive(Debug)]
pub struct ImportDeconstructionSymbolStruct<'arena> {
    pub name: Identifier<'arena>,
    pub alias: Option<ImportAlias<'arena>>,
}

pub fn new_import_deconstruction_symbol<'arena>(
    arena: &'arena Bump,
    name: Identifier<'arena>,
    alias: Option<ImportAlias<'arena>>,
) -> ImportDeconstructionSymbol<'arena> {
    Box::new_in(ImportDeconstructionSymbolStruct { name, alias }, arena)
}

pub type ImportDirective<'arena> = Box<'arena, ImportDirectiveStruct<'arena>>;

#[derive(Debug)]
pub struct ImportDirectiveStruct<'arena> {
    pub import_keyword: ImportKeyword<'arena>,
    pub clause: ImportClause<'arena>,
    pub semicolon: Semicolon<'arena>,
}

pub fn new_import_directive<'arena>(
    arena: &'arena Bump,
    import_keyword: ImportKeyword<'arena>,
    clause: ImportClause<'arena>,
    semicolon: Semicolon<'arena>,
) -> ImportDirective<'arena> {
    Box::new_in(
        ImportDirectiveStruct {
            import_keyword,
            clause,
            semicolon,
        },
        arena,
    )
}

pub type IndexAccessEnd<'arena> = Box<'arena, IndexAccessEndStruct<'arena>>;

#[derive(Debug)]
pub struct IndexAccessEndStruct<'arena> {
    pub colon: Colon<'arena>,
    pub end: Option<Expression<'arena>>,
}

pub fn new_index_access_end<'arena>(
    arena: &'arena Bump,
    colon: Colon<'arena>,
    end: Option<Expression<'arena>>,
) -> IndexAccessEnd<'arena> {
    Box::new_in(IndexAccessEndStruct { colon, end }, arena)
}

pub type IndexAccessExpression<'arena> = Box<'arena, IndexAccessExpressionStruct<'arena>>;

#[derive(Debug)]
pub struct IndexAccessExpressionStruct<'arena> {
    pub operand: Expression<'arena>,
    pub open_bracket: OpenBracket<'arena>,
    pub start: Option<Expression<'arena>>,
    pub end: Option<IndexAccessEnd<'arena>>,
    pub close_bracket: CloseBracket<'arena>,
}

pub fn new_index_access_expression<'arena>(
    arena: &'arena Bump,
    operand: Expression<'arena>,
    open_bracket: OpenBracket<'arena>,
    start: Option<Expression<'arena>>,
    end: Option<IndexAccessEnd<'arena>>,
    close_bracket: CloseBracket<'arena>,
) -> IndexAccessExpression<'arena> {
    Box::new_in(
        IndexAccessExpressionStruct {
            operand,
            open_bracket,
            start,
            end,
            close_bracket,
        },
        arena,
    )
}

pub type InequalityExpression<'arena> = Box<'arena, InequalityExpressionStruct<'arena>>;

#[derive(Debug)]
pub struct InequalityExpressionStruct<'arena> {
    pub left_operand: Expression<'arena>,
    pub expression_inequality_expression_operator: Expression_InequalityExpression_Operator<'arena>,
    pub right_operand: Expression<'arena>,
}

pub fn new_inequality_expression<'arena>(
    arena: &'arena Bump,
    left_operand: Expression<'arena>,
    expression_inequality_expression_operator: Expression_InequalityExpression_Operator<'arena>,
    right_operand: Expression<'arena>,
) -> InequalityExpression<'arena> {
    Box::new_in(
        InequalityExpressionStruct {
            left_operand,
            expression_inequality_expression_operator,
            right_operand,
        },
        arena,
    )
}

pub type InheritanceSpecifier<'arena> = Box<'arena, InheritanceSpecifierStruct<'arena>>;

#[derive(Debug)]
pub struct InheritanceSpecifierStruct<'arena> {
    pub is_keyword: IsKeyword<'arena>,
    pub types: InheritanceTypes<'arena>,
}

pub fn new_inheritance_specifier<'arena>(
    arena: &'arena Bump,
    is_keyword: IsKeyword<'arena>,
    types: InheritanceTypes<'arena>,
) -> InheritanceSpecifier<'arena> {
    Box::new_in(InheritanceSpecifierStruct { is_keyword, types }, arena)
}

pub type InheritanceType<'arena> = Box<'arena, InheritanceTypeStruct<'arena>>;

#[derive(Debug)]
pub struct InheritanceTypeStruct<'arena> {
    pub type_name: IdentifierPath<'arena>,
    pub arguments: Option<ArgumentsDeclaration<'arena>>,
}

pub fn new_inheritance_type<'arena>(
    arena: &'arena Bump,
    type_name: IdentifierPath<'arena>,
    arguments: Option<ArgumentsDeclaration<'arena>>,
) -> InheritanceType<'arena> {
    Box::new_in(
        InheritanceTypeStruct {
            type_name,
            arguments,
        },
        arena,
    )
}

pub type InterfaceDefinition<'arena> = Box<'arena, InterfaceDefinitionStruct<'arena>>;

#[derive(Debug)]
pub struct InterfaceDefinitionStruct<'arena> {
    pub interface_keyword: InterfaceKeyword<'arena>,
    pub name: Identifier<'arena>,
    pub inheritance: Option<InheritanceSpecifier<'arena>>,
    pub open_brace: OpenBrace<'arena>,
    pub members: InterfaceMembers<'arena>,
    pub close_brace: CloseBrace<'arena>,
}

pub fn new_interface_definition<'arena>(
    arena: &'arena Bump,
    interface_keyword: InterfaceKeyword<'arena>,
    name: Identifier<'arena>,
    inheritance: Option<InheritanceSpecifier<'arena>>,
    open_brace: OpenBrace<'arena>,
    members: InterfaceMembers<'arena>,
    close_brace: CloseBrace<'arena>,
) -> InterfaceDefinition<'arena> {
    Box::new_in(
        InterfaceDefinitionStruct {
            interface_keyword,
            name,
            inheritance,
            open_brace,
            members,
            close_brace,
        },
        arena,
    )
}

pub type LibraryDefinition<'arena> = Box<'arena, LibraryDefinitionStruct<'arena>>;

#[derive(Debug)]
pub struct LibraryDefinitionStruct<'arena> {
    pub library_keyword: LibraryKeyword<'arena>,
    pub name: Identifier<'arena>,
    pub open_brace: OpenBrace<'arena>,
    pub members: LibraryMembers<'arena>,
    pub close_brace: CloseBrace<'arena>,
}

pub fn new_library_definition<'arena>(
    arena: &'arena Bump,
    library_keyword: LibraryKeyword<'arena>,
    name: Identifier<'arena>,
    open_brace: OpenBrace<'arena>,
    members: LibraryMembers<'arena>,
    close_brace: CloseBrace<'arena>,
) -> LibraryDefinition<'arena> {
    Box::new_in(
        LibraryDefinitionStruct {
            library_keyword,
            name,
            open_brace,
            members,
            close_brace,
        },
        arena,
    )
}

pub type MappingKey<'arena> = Box<'arena, MappingKeyStruct<'arena>>;

#[derive(Debug)]
pub struct MappingKeyStruct<'arena> {
    pub key_type: MappingKeyType<'arena>,
    pub name: Option<Identifier<'arena>>,
}

pub fn new_mapping_key<'arena>(
    arena: &'arena Bump,
    key_type: MappingKeyType<'arena>,
    name: Option<Identifier<'arena>>,
) -> MappingKey<'arena> {
    Box::new_in(MappingKeyStruct { key_type, name }, arena)
}

pub type MappingType<'arena> = Box<'arena, MappingTypeStruct<'arena>>;

#[derive(Debug)]
pub struct MappingTypeStruct<'arena> {
    pub mapping_keyword: MappingKeyword<'arena>,
    pub open_paren: OpenParen<'arena>,
    pub key_type: MappingKey<'arena>,
    pub equal_greater_than: EqualGreaterThan<'arena>,
    pub value_type: MappingValue<'arena>,
    pub close_paren: CloseParen<'arena>,
}

pub fn new_mapping_type<'arena>(
    arena: &'arena Bump,
    mapping_keyword: MappingKeyword<'arena>,
    open_paren: OpenParen<'arena>,
    key_type: MappingKey<'arena>,
    equal_greater_than: EqualGreaterThan<'arena>,
    value_type: MappingValue<'arena>,
    close_paren: CloseParen<'arena>,
) -> MappingType<'arena> {
    Box::new_in(
        MappingTypeStruct {
            mapping_keyword,
            open_paren,
            key_type,
            equal_greater_than,
            value_type,
            close_paren,
        },
        arena,
    )
}

pub type MappingValue<'arena> = Box<'arena, MappingValueStruct<'arena>>;

#[derive(Debug)]
pub struct MappingValueStruct<'arena> {
    pub type_name: TypeName<'arena>,
    pub name: Option<Identifier<'arena>>,
}

pub fn new_mapping_value<'arena>(
    arena: &'arena Bump,
    type_name: TypeName<'arena>,
    name: Option<Identifier<'arena>>,
) -> MappingValue<'arena> {
    Box::new_in(MappingValueStruct { type_name, name }, arena)
}

pub type MemberAccessExpression<'arena> = Box<'arena, MemberAccessExpressionStruct<'arena>>;

#[derive(Debug)]
pub struct MemberAccessExpressionStruct<'arena> {
    pub operand: Expression<'arena>,
    pub period: Period<'arena>,
    pub member: Identifier<'arena>,
}

pub fn new_member_access_expression<'arena>(
    arena: &'arena Bump,
    operand: Expression<'arena>,
    period: Period<'arena>,
    member: Identifier<'arena>,
) -> MemberAccessExpression<'arena> {
    Box::new_in(
        MemberAccessExpressionStruct {
            operand,
            period,
            member,
        },
        arena,
    )
}

pub type ModifierDefinition<'arena> = Box<'arena, ModifierDefinitionStruct<'arena>>;

#[derive(Debug)]
pub struct ModifierDefinitionStruct<'arena> {
    pub modifier_keyword: ModifierKeyword<'arena>,
    pub name: Identifier<'arena>,
    pub parameters: Option<ParametersDeclaration<'arena>>,
    pub attributes: ModifierAttributes<'arena>,
    pub body: FunctionBody<'arena>,
}

pub fn new_modifier_definition<'arena>(
    arena: &'arena Bump,
    modifier_keyword: ModifierKeyword<'arena>,
    name: Identifier<'arena>,
    parameters: Option<ParametersDeclaration<'arena>>,
    attributes: ModifierAttributes<'arena>,
    body: FunctionBody<'arena>,
) -> ModifierDefinition<'arena> {
    Box::new_in(
        ModifierDefinitionStruct {
            modifier_keyword,
            name,
            parameters,
            attributes,
            body,
        },
        arena,
    )
}

pub type ModifierInvocation<'arena> = Box<'arena, ModifierInvocationStruct<'arena>>;

#[derive(Debug)]
pub struct ModifierInvocationStruct<'arena> {
    pub name: IdentifierPath<'arena>,
    pub arguments: Option<ArgumentsDeclaration<'arena>>,
}

pub fn new_modifier_invocation<'arena>(
    arena: &'arena Bump,
    name: IdentifierPath<'arena>,
    arguments: Option<ArgumentsDeclaration<'arena>>,
) -> ModifierInvocation<'arena> {
    Box::new_in(ModifierInvocationStruct { name, arguments }, arena)
}

pub type MultiplicativeExpression<'arena> = Box<'arena, MultiplicativeExpressionStruct<'arena>>;

#[derive(Debug)]
pub struct MultiplicativeExpressionStruct<'arena> {
    pub left_operand: Expression<'arena>,
    pub expression_multiplicative_expression_operator:
        Expression_MultiplicativeExpression_Operator<'arena>,
    pub right_operand: Expression<'arena>,
}

pub fn new_multiplicative_expression<'arena>(
    arena: &'arena Bump,
    left_operand: Expression<'arena>,
    expression_multiplicative_expression_operator: Expression_MultiplicativeExpression_Operator<
        'arena,
    >,
    right_operand: Expression<'arena>,
) -> MultiplicativeExpression<'arena> {
    Box::new_in(
        MultiplicativeExpressionStruct {
            left_operand,
            expression_multiplicative_expression_operator,
            right_operand,
        },
        arena,
    )
}

pub type NamedArgument<'arena> = Box<'arena, NamedArgumentStruct<'arena>>;

#[derive(Debug)]
pub struct NamedArgumentStruct<'arena> {
    pub name: Identifier<'arena>,
    pub colon: Colon<'arena>,
    pub value: Expression<'arena>,
}

pub fn new_named_argument<'arena>(
    arena: &'arena Bump,
    name: Identifier<'arena>,
    colon: Colon<'arena>,
    value: Expression<'arena>,
) -> NamedArgument<'arena> {
    Box::new_in(NamedArgumentStruct { name, colon, value }, arena)
}

pub type NamedArgumentGroup<'arena> = Box<'arena, NamedArgumentGroupStruct<'arena>>;

#[derive(Debug)]
pub struct NamedArgumentGroupStruct<'arena> {
    pub open_brace: OpenBrace<'arena>,
    pub arguments: NamedArguments<'arena>,
    pub close_brace: CloseBrace<'arena>,
}

pub fn new_named_argument_group<'arena>(
    arena: &'arena Bump,
    open_brace: OpenBrace<'arena>,
    arguments: NamedArguments<'arena>,
    close_brace: CloseBrace<'arena>,
) -> NamedArgumentGroup<'arena> {
    Box::new_in(
        NamedArgumentGroupStruct {
            open_brace,
            arguments,
            close_brace,
        },
        arena,
    )
}

pub type NamedArgumentsDeclaration<'arena> = Box<'arena, NamedArgumentsDeclarationStruct<'arena>>;

#[derive(Debug)]
pub struct NamedArgumentsDeclarationStruct<'arena> {
    pub open_paren: OpenParen<'arena>,
    pub arguments: Option<NamedArgumentGroup<'arena>>,
    pub close_paren: CloseParen<'arena>,
}

pub fn new_named_arguments_declaration<'arena>(
    arena: &'arena Bump,
    open_paren: OpenParen<'arena>,
    arguments: Option<NamedArgumentGroup<'arena>>,
    close_paren: CloseParen<'arena>,
) -> NamedArgumentsDeclaration<'arena> {
    Box::new_in(
        NamedArgumentsDeclarationStruct {
            open_paren,
            arguments,
            close_paren,
        },
        arena,
    )
}

pub type NamedImport<'arena> = Box<'arena, NamedImportStruct<'arena>>;

#[derive(Debug)]
pub struct NamedImportStruct<'arena> {
    pub asterisk: Asterisk<'arena>,
    pub alias: ImportAlias<'arena>,
    pub from_keyword: FromKeyword<'arena>,
    pub path: StringLiteral<'arena>,
}

pub fn new_named_import<'arena>(
    arena: &'arena Bump,
    asterisk: Asterisk<'arena>,
    alias: ImportAlias<'arena>,
    from_keyword: FromKeyword<'arena>,
    path: StringLiteral<'arena>,
) -> NamedImport<'arena> {
    Box::new_in(
        NamedImportStruct {
            asterisk,
            alias,
            from_keyword,
            path,
        },
        arena,
    )
}

pub type NewExpression<'arena> = Box<'arena, NewExpressionStruct<'arena>>;

#[derive(Debug)]
pub struct NewExpressionStruct<'arena> {
    pub new_keyword: NewKeyword<'arena>,
    pub type_name: TypeName<'arena>,
}

pub fn new_new_expression<'arena>(
    arena: &'arena Bump,
    new_keyword: NewKeyword<'arena>,
    type_name: TypeName<'arena>,
) -> NewExpression<'arena> {
    Box::new_in(
        NewExpressionStruct {
            new_keyword,
            type_name,
        },
        arena,
    )
}

pub type OrExpression<'arena> = Box<'arena, OrExpressionStruct<'arena>>;

#[derive(Debug)]
pub struct OrExpressionStruct<'arena> {
    pub left_operand: Expression<'arena>,
    pub operator: BarBar<'arena>,
    pub right_operand: Expression<'arena>,
}

pub fn new_or_expression<'arena>(
    arena: &'arena Bump,
    left_operand: Expression<'arena>,
    operator: BarBar<'arena>,
    right_operand: Expression<'arena>,
) -> OrExpression<'arena> {
    Box::new_in(
        OrExpressionStruct {
            left_operand,
            operator,
            right_operand,
        },
        arena,
    )
}

pub type OverridePathsDeclaration<'arena> = Box<'arena, OverridePathsDeclarationStruct<'arena>>;

#[derive(Debug)]
pub struct OverridePathsDeclarationStruct<'arena> {
    pub open_paren: OpenParen<'arena>,
    pub paths: OverridePaths<'arena>,
    pub close_paren: CloseParen<'arena>,
}

pub fn new_override_paths_declaration<'arena>(
    arena: &'arena Bump,
    open_paren: OpenParen<'arena>,
    paths: OverridePaths<'arena>,
    close_paren: CloseParen<'arena>,
) -> OverridePathsDeclaration<'arena> {
    Box::new_in(
        OverridePathsDeclarationStruct {
            open_paren,
            paths,
            close_paren,
        },
        arena,
    )
}

pub type OverrideSpecifier<'arena> = Box<'arena, OverrideSpecifierStruct<'arena>>;

#[derive(Debug)]
pub struct OverrideSpecifierStruct<'arena> {
    pub override_keyword: OverrideKeyword<'arena>,
    pub overridden: Option<OverridePathsDeclaration<'arena>>,
}

pub fn new_override_specifier<'arena>(
    arena: &'arena Bump,
    override_keyword: OverrideKeyword<'arena>,
    overridden: Option<OverridePathsDeclaration<'arena>>,
) -> OverrideSpecifier<'arena> {
    Box::new_in(
        OverrideSpecifierStruct {
            override_keyword,
            overridden,
        },
        arena,
    )
}

pub type Parameter<'arena> = Box<'arena, ParameterStruct<'arena>>;

#[derive(Debug)]
pub struct ParameterStruct<'arena> {
    pub type_name: TypeName<'arena>,
    pub storage_location: Option<StorageLocation<'arena>>,
    pub name: Option<Identifier<'arena>>,
}

pub fn new_parameter<'arena>(
    arena: &'arena Bump,
    type_name: TypeName<'arena>,
    storage_location: Option<StorageLocation<'arena>>,
    name: Option<Identifier<'arena>>,
) -> Parameter<'arena> {
    Box::new_in(
        ParameterStruct {
            type_name,
            storage_location,
            name,
        },
        arena,
    )
}

pub type ParametersDeclaration<'arena> = Box<'arena, ParametersDeclarationStruct<'arena>>;

#[derive(Debug)]
pub struct ParametersDeclarationStruct<'arena> {
    pub open_paren: OpenParen<'arena>,
    pub parameters: Parameters<'arena>,
    pub close_paren: CloseParen<'arena>,
}

pub fn new_parameters_declaration<'arena>(
    arena: &'arena Bump,
    open_paren: OpenParen<'arena>,
    parameters: Parameters<'arena>,
    close_paren: CloseParen<'arena>,
) -> ParametersDeclaration<'arena> {
    Box::new_in(
        ParametersDeclarationStruct {
            open_paren,
            parameters,
            close_paren,
        },
        arena,
    )
}

pub type PathImport<'arena> = Box<'arena, PathImportStruct<'arena>>;

#[derive(Debug)]
pub struct PathImportStruct<'arena> {
    pub path: StringLiteral<'arena>,
    pub alias: Option<ImportAlias<'arena>>,
}

pub fn new_path_import<'arena>(
    arena: &'arena Bump,
    path: StringLiteral<'arena>,
    alias: Option<ImportAlias<'arena>>,
) -> PathImport<'arena> {
    Box::new_in(PathImportStruct { path, alias }, arena)
}

pub type PositionalArgumentsDeclaration<'arena> =
    Box<'arena, PositionalArgumentsDeclarationStruct<'arena>>;

#[derive(Debug)]
pub struct PositionalArgumentsDeclarationStruct<'arena> {
    pub open_paren: OpenParen<'arena>,
    pub arguments: PositionalArguments<'arena>,
    pub close_paren: CloseParen<'arena>,
}

pub fn new_positional_arguments_declaration<'arena>(
    arena: &'arena Bump,
    open_paren: OpenParen<'arena>,
    arguments: PositionalArguments<'arena>,
    close_paren: CloseParen<'arena>,
) -> PositionalArgumentsDeclaration<'arena> {
    Box::new_in(
        PositionalArgumentsDeclarationStruct {
            open_paren,
            arguments,
            close_paren,
        },
        arena,
    )
}

pub type PostfixExpression<'arena> = Box<'arena, PostfixExpressionStruct<'arena>>;

#[derive(Debug)]
pub struct PostfixExpressionStruct<'arena> {
    pub operand: Expression<'arena>,
    pub expression_postfix_expression_operator: Expression_PostfixExpression_Operator<'arena>,
}

pub fn new_postfix_expression<'arena>(
    arena: &'arena Bump,
    operand: Expression<'arena>,
    expression_postfix_expression_operator: Expression_PostfixExpression_Operator<'arena>,
) -> PostfixExpression<'arena> {
    Box::new_in(
        PostfixExpressionStruct {
            operand,
            expression_postfix_expression_operator,
        },
        arena,
    )
}

pub type PragmaDirective<'arena> = Box<'arena, PragmaDirectiveStruct<'arena>>;

#[derive(Debug)]
pub struct PragmaDirectiveStruct<'arena> {
    pub pragma_keyword: PragmaKeyword<'arena>,
    pub pragma: Pragma<'arena>,
    pub semicolon: Semicolon<'arena>,
}

pub fn new_pragma_directive<'arena>(
    arena: &'arena Bump,
    pragma_keyword: PragmaKeyword<'arena>,
    pragma: Pragma<'arena>,
    semicolon: Semicolon<'arena>,
) -> PragmaDirective<'arena> {
    Box::new_in(
        PragmaDirectiveStruct {
            pragma_keyword,
            pragma,
            semicolon,
        },
        arena,
    )
}

pub type PrefixExpression<'arena> = Box<'arena, PrefixExpressionStruct<'arena>>;

#[derive(Debug)]
pub struct PrefixExpressionStruct<'arena> {
    pub expression_prefix_expression_operator: Expression_PrefixExpression_Operator<'arena>,
    pub operand: Expression<'arena>,
}

pub fn new_prefix_expression<'arena>(
    arena: &'arena Bump,
    expression_prefix_expression_operator: Expression_PrefixExpression_Operator<'arena>,
    operand: Expression<'arena>,
) -> PrefixExpression<'arena> {
    Box::new_in(
        PrefixExpressionStruct {
            expression_prefix_expression_operator,
            operand,
        },
        arena,
    )
}

pub type ReceiveFunctionDefinition<'arena> = Box<'arena, ReceiveFunctionDefinitionStruct<'arena>>;

#[derive(Debug)]
pub struct ReceiveFunctionDefinitionStruct<'arena> {
    pub receive_keyword: ReceiveKeyword<'arena>,
    pub parameters: ParametersDeclaration<'arena>,
    pub attributes: ReceiveFunctionAttributes<'arena>,
    pub body: FunctionBody<'arena>,
}

pub fn new_receive_function_definition<'arena>(
    arena: &'arena Bump,
    receive_keyword: ReceiveKeyword<'arena>,
    parameters: ParametersDeclaration<'arena>,
    attributes: ReceiveFunctionAttributes<'arena>,
    body: FunctionBody<'arena>,
) -> ReceiveFunctionDefinition<'arena> {
    Box::new_in(
        ReceiveFunctionDefinitionStruct {
            receive_keyword,
            parameters,
            attributes,
            body,
        },
        arena,
    )
}

pub type ReturnStatement<'arena> = Box<'arena, ReturnStatementStruct<'arena>>;

#[derive(Debug)]
pub struct ReturnStatementStruct<'arena> {
    pub return_keyword: ReturnKeyword<'arena>,
    pub expression: Option<Expression<'arena>>,
    pub semicolon: Semicolon<'arena>,
}

pub fn new_return_statement<'arena>(
    arena: &'arena Bump,
    return_keyword: ReturnKeyword<'arena>,
    expression: Option<Expression<'arena>>,
    semicolon: Semicolon<'arena>,
) -> ReturnStatement<'arena> {
    Box::new_in(
        ReturnStatementStruct {
            return_keyword,
            expression,
            semicolon,
        },
        arena,
    )
}

pub type ReturnsDeclaration<'arena> = Box<'arena, ReturnsDeclarationStruct<'arena>>;

#[derive(Debug)]
pub struct ReturnsDeclarationStruct<'arena> {
    pub returns_keyword: ReturnsKeyword<'arena>,
    pub variables: ParametersDeclaration<'arena>,
}

pub fn new_returns_declaration<'arena>(
    arena: &'arena Bump,
    returns_keyword: ReturnsKeyword<'arena>,
    variables: ParametersDeclaration<'arena>,
) -> ReturnsDeclaration<'arena> {
    Box::new_in(
        ReturnsDeclarationStruct {
            returns_keyword,
            variables,
        },
        arena,
    )
}

pub type RevertStatement<'arena> = Box<'arena, RevertStatementStruct<'arena>>;

#[derive(Debug)]
pub struct RevertStatementStruct<'arena> {
    pub revert_keyword: RevertKeyword<'arena>,
    pub error: Option<IdentifierPath<'arena>>,
    pub arguments: ArgumentsDeclaration<'arena>,
    pub semicolon: Semicolon<'arena>,
}

pub fn new_revert_statement<'arena>(
    arena: &'arena Bump,
    revert_keyword: RevertKeyword<'arena>,
    error: Option<IdentifierPath<'arena>>,
    arguments: ArgumentsDeclaration<'arena>,
    semicolon: Semicolon<'arena>,
) -> RevertStatement<'arena> {
    Box::new_in(
        RevertStatementStruct {
            revert_keyword,
            error,
            arguments,
            semicolon,
        },
        arena,
    )
}

pub type ShiftExpression<'arena> = Box<'arena, ShiftExpressionStruct<'arena>>;

#[derive(Debug)]
pub struct ShiftExpressionStruct<'arena> {
    pub left_operand: Expression<'arena>,
    pub expression_shift_expression_operator: Expression_ShiftExpression_Operator<'arena>,
    pub right_operand: Expression<'arena>,
}

pub fn new_shift_expression<'arena>(
    arena: &'arena Bump,
    left_operand: Expression<'arena>,
    expression_shift_expression_operator: Expression_ShiftExpression_Operator<'arena>,
    right_operand: Expression<'arena>,
) -> ShiftExpression<'arena> {
    Box::new_in(
        ShiftExpressionStruct {
            left_operand,
            expression_shift_expression_operator,
            right_operand,
        },
        arena,
    )
}

pub type SourceUnit<'arena> = Box<'arena, SourceUnitStruct<'arena>>;

#[derive(Debug)]
pub struct SourceUnitStruct<'arena> {
    pub members: SourceUnitMembers<'arena>,
}

pub fn new_source_unit<'arena>(
    arena: &'arena Bump,
    members: SourceUnitMembers<'arena>,
) -> SourceUnit<'arena> {
    Box::new_in(SourceUnitStruct { members }, arena)
}

pub type StateVariableDefinition<'arena> = Box<'arena, StateVariableDefinitionStruct<'arena>>;

#[derive(Debug)]
pub struct StateVariableDefinitionStruct<'arena> {
    pub type_name: TypeName<'arena>,
    pub attributes: StateVariableAttributes<'arena>,
    pub name: Identifier<'arena>,
    pub value: Option<StateVariableDefinitionValue<'arena>>,
    pub semicolon: Semicolon<'arena>,
}

pub fn new_state_variable_definition<'arena>(
    arena: &'arena Bump,
    type_name: TypeName<'arena>,
    attributes: StateVariableAttributes<'arena>,
    name: Identifier<'arena>,
    value: Option<StateVariableDefinitionValue<'arena>>,
    semicolon: Semicolon<'arena>,
) -> StateVariableDefinition<'arena> {
    Box::new_in(
        StateVariableDefinitionStruct {
            type_name,
            attributes,
            name,
            value,
            semicolon,
        },
        arena,
    )
}

pub type StateVariableDefinitionValue<'arena> =
    Box<'arena, StateVariableDefinitionValueStruct<'arena>>;

#[derive(Debug)]
pub struct StateVariableDefinitionValueStruct<'arena> {
    pub equal: Equal<'arena>,
    pub value: Expression<'arena>,
}

pub fn new_state_variable_definition_value<'arena>(
    arena: &'arena Bump,
    equal: Equal<'arena>,
    value: Expression<'arena>,
) -> StateVariableDefinitionValue<'arena> {
    Box::new_in(StateVariableDefinitionValueStruct { equal, value }, arena)
}

pub type StorageLayoutSpecifier<'arena> = Box<'arena, StorageLayoutSpecifierStruct<'arena>>;

#[derive(Debug)]
pub struct StorageLayoutSpecifierStruct<'arena> {
    pub layout_keyword: LayoutKeyword<'arena>,
    pub at_keyword: AtKeyword<'arena>,
    pub expression: Expression<'arena>,
}

pub fn new_storage_layout_specifier<'arena>(
    arena: &'arena Bump,
    layout_keyword: LayoutKeyword<'arena>,
    at_keyword: AtKeyword<'arena>,
    expression: Expression<'arena>,
) -> StorageLayoutSpecifier<'arena> {
    Box::new_in(
        StorageLayoutSpecifierStruct {
            layout_keyword,
            at_keyword,
            expression,
        },
        arena,
    )
}

pub type StructDefinition<'arena> = Box<'arena, StructDefinitionStruct<'arena>>;

#[derive(Debug)]
pub struct StructDefinitionStruct<'arena> {
    pub struct_keyword: StructKeyword<'arena>,
    pub name: Identifier<'arena>,
    pub open_brace: OpenBrace<'arena>,
    pub members: StructMembers<'arena>,
    pub close_brace: CloseBrace<'arena>,
}

pub fn new_struct_definition<'arena>(
    arena: &'arena Bump,
    struct_keyword: StructKeyword<'arena>,
    name: Identifier<'arena>,
    open_brace: OpenBrace<'arena>,
    members: StructMembers<'arena>,
    close_brace: CloseBrace<'arena>,
) -> StructDefinition<'arena> {
    Box::new_in(
        StructDefinitionStruct {
            struct_keyword,
            name,
            open_brace,
            members,
            close_brace,
        },
        arena,
    )
}

pub type StructMember<'arena> = Box<'arena, StructMemberStruct<'arena>>;

#[derive(Debug)]
pub struct StructMemberStruct<'arena> {
    pub type_name: TypeName<'arena>,
    pub name: Identifier<'arena>,
    pub semicolon: Semicolon<'arena>,
}

pub fn new_struct_member<'arena>(
    arena: &'arena Bump,
    type_name: TypeName<'arena>,
    name: Identifier<'arena>,
    semicolon: Semicolon<'arena>,
) -> StructMember<'arena> {
    Box::new_in(
        StructMemberStruct {
            type_name,
            name,
            semicolon,
        },
        arena,
    )
}

pub type ThrowStatement<'arena> = Box<'arena, ThrowStatementStruct<'arena>>;

#[derive(Debug)]
pub struct ThrowStatementStruct<'arena> {
    pub throw_keyword: ThrowKeyword<'arena>,
    pub semicolon: Semicolon<'arena>,
}

pub fn new_throw_statement<'arena>(
    arena: &'arena Bump,
    throw_keyword: ThrowKeyword<'arena>,
    semicolon: Semicolon<'arena>,
) -> ThrowStatement<'arena> {
    Box::new_in(
        ThrowStatementStruct {
            throw_keyword,
            semicolon,
        },
        arena,
    )
}

pub type TryStatement<'arena> = Box<'arena, TryStatementStruct<'arena>>;

#[derive(Debug)]
pub struct TryStatementStruct<'arena> {
    pub try_keyword: TryKeyword<'arena>,
    pub expression: Expression<'arena>,
    pub returns: Option<ReturnsDeclaration<'arena>>,
    pub body: Block<'arena>,
    pub catch_clauses: CatchClauses<'arena>,
}

pub fn new_try_statement<'arena>(
    arena: &'arena Bump,
    try_keyword: TryKeyword<'arena>,
    expression: Expression<'arena>,
    returns: Option<ReturnsDeclaration<'arena>>,
    body: Block<'arena>,
    catch_clauses: CatchClauses<'arena>,
) -> TryStatement<'arena> {
    Box::new_in(
        TryStatementStruct {
            try_keyword,
            expression,
            returns,
            body,
            catch_clauses,
        },
        arena,
    )
}

pub type TupleDeconstructionElement<'arena> = Box<'arena, TupleDeconstructionElementStruct<'arena>>;

#[derive(Debug)]
pub struct TupleDeconstructionElementStruct<'arena> {
    pub member: Option<TupleMember<'arena>>,
}

pub fn new_tuple_deconstruction_element<'arena>(
    arena: &'arena Bump,
    member: Option<TupleMember<'arena>>,
) -> TupleDeconstructionElement<'arena> {
    Box::new_in(TupleDeconstructionElementStruct { member }, arena)
}

pub type TupleDeconstructionStatement<'arena> =
    Box<'arena, TupleDeconstructionStatementStruct<'arena>>;

#[derive(Debug)]
pub struct TupleDeconstructionStatementStruct<'arena> {
    pub var_keyword: Option<VarKeyword<'arena>>,
    pub open_paren: OpenParen<'arena>,
    pub elements: TupleDeconstructionElements<'arena>,
    pub close_paren: CloseParen<'arena>,
    pub equal: Equal<'arena>,
    pub expression: Expression<'arena>,
    pub semicolon: Semicolon<'arena>,
}

pub fn new_tuple_deconstruction_statement<'arena>(
    arena: &'arena Bump,
    var_keyword: Option<VarKeyword<'arena>>,
    open_paren: OpenParen<'arena>,
    elements: TupleDeconstructionElements<'arena>,
    close_paren: CloseParen<'arena>,
    equal: Equal<'arena>,
    expression: Expression<'arena>,
    semicolon: Semicolon<'arena>,
) -> TupleDeconstructionStatement<'arena> {
    Box::new_in(
        TupleDeconstructionStatementStruct {
            var_keyword,
            open_paren,
            elements,
            close_paren,
            equal,
            expression,
            semicolon,
        },
        arena,
    )
}

pub type TupleExpression<'arena> = Box<'arena, TupleExpressionStruct<'arena>>;

#[derive(Debug)]
pub struct TupleExpressionStruct<'arena> {
    pub open_paren: OpenParen<'arena>,
    pub items: TupleValues<'arena>,
    pub close_paren: CloseParen<'arena>,
}

pub fn new_tuple_expression<'arena>(
    arena: &'arena Bump,
    open_paren: OpenParen<'arena>,
    items: TupleValues<'arena>,
    close_paren: CloseParen<'arena>,
) -> TupleExpression<'arena> {
    Box::new_in(
        TupleExpressionStruct {
            open_paren,
            items,
            close_paren,
        },
        arena,
    )
}

pub type TupleValue<'arena> = Box<'arena, TupleValueStruct<'arena>>;

#[derive(Debug)]
pub struct TupleValueStruct<'arena> {
    pub expression: Option<Expression<'arena>>,
}

pub fn new_tuple_value<'arena>(
    arena: &'arena Bump,
    expression: Option<Expression<'arena>>,
) -> TupleValue<'arena> {
    Box::new_in(TupleValueStruct { expression }, arena)
}

pub type TypeExpression<'arena> = Box<'arena, TypeExpressionStruct<'arena>>;

#[derive(Debug)]
pub struct TypeExpressionStruct<'arena> {
    pub type_keyword: TypeKeyword<'arena>,
    pub open_paren: OpenParen<'arena>,
    pub type_name: TypeName<'arena>,
    pub close_paren: CloseParen<'arena>,
}

pub fn new_type_expression<'arena>(
    arena: &'arena Bump,
    type_keyword: TypeKeyword<'arena>,
    open_paren: OpenParen<'arena>,
    type_name: TypeName<'arena>,
    close_paren: CloseParen<'arena>,
) -> TypeExpression<'arena> {
    Box::new_in(
        TypeExpressionStruct {
            type_keyword,
            open_paren,
            type_name,
            close_paren,
        },
        arena,
    )
}

pub type TypedTupleMember<'arena> = Box<'arena, TypedTupleMemberStruct<'arena>>;

#[derive(Debug)]
pub struct TypedTupleMemberStruct<'arena> {
    pub type_name: TypeName<'arena>,
    pub storage_location: Option<StorageLocation<'arena>>,
    pub name: Identifier<'arena>,
}

pub fn new_typed_tuple_member<'arena>(
    arena: &'arena Bump,
    type_name: TypeName<'arena>,
    storage_location: Option<StorageLocation<'arena>>,
    name: Identifier<'arena>,
) -> TypedTupleMember<'arena> {
    Box::new_in(
        TypedTupleMemberStruct {
            type_name,
            storage_location,
            name,
        },
        arena,
    )
}

pub type UncheckedBlock<'arena> = Box<'arena, UncheckedBlockStruct<'arena>>;

#[derive(Debug)]
pub struct UncheckedBlockStruct<'arena> {
    pub unchecked_keyword: UncheckedKeyword<'arena>,
    pub block: Block<'arena>,
}

pub fn new_unchecked_block<'arena>(
    arena: &'arena Bump,
    unchecked_keyword: UncheckedKeyword<'arena>,
    block: Block<'arena>,
) -> UncheckedBlock<'arena> {
    Box::new_in(
        UncheckedBlockStruct {
            unchecked_keyword,
            block,
        },
        arena,
    )
}

pub type UnnamedFunctionDefinition<'arena> = Box<'arena, UnnamedFunctionDefinitionStruct<'arena>>;

#[derive(Debug)]
pub struct UnnamedFunctionDefinitionStruct<'arena> {
    pub function_keyword: FunctionKeyword<'arena>,
    pub parameters: ParametersDeclaration<'arena>,
    pub attributes: UnnamedFunctionAttributes<'arena>,
    pub body: FunctionBody<'arena>,
}

pub fn new_unnamed_function_definition<'arena>(
    arena: &'arena Bump,
    function_keyword: FunctionKeyword<'arena>,
    parameters: ParametersDeclaration<'arena>,
    attributes: UnnamedFunctionAttributes<'arena>,
    body: FunctionBody<'arena>,
) -> UnnamedFunctionDefinition<'arena> {
    Box::new_in(
        UnnamedFunctionDefinitionStruct {
            function_keyword,
            parameters,
            attributes,
            body,
        },
        arena,
    )
}

pub type UntypedTupleMember<'arena> = Box<'arena, UntypedTupleMemberStruct<'arena>>;

#[derive(Debug)]
pub struct UntypedTupleMemberStruct<'arena> {
    pub storage_location: Option<StorageLocation<'arena>>,
    pub name: Identifier<'arena>,
}

pub fn new_untyped_tuple_member<'arena>(
    arena: &'arena Bump,
    storage_location: Option<StorageLocation<'arena>>,
    name: Identifier<'arena>,
) -> UntypedTupleMember<'arena> {
    Box::new_in(
        UntypedTupleMemberStruct {
            storage_location,
            name,
        },
        arena,
    )
}

pub type UserDefinedValueTypeDefinition<'arena> =
    Box<'arena, UserDefinedValueTypeDefinitionStruct<'arena>>;

#[derive(Debug)]
pub struct UserDefinedValueTypeDefinitionStruct<'arena> {
    pub type_keyword: TypeKeyword<'arena>,
    pub name: Identifier<'arena>,
    pub is_keyword: IsKeyword<'arena>,
    pub value_type: ElementaryType<'arena>,
    pub semicolon: Semicolon<'arena>,
}

pub fn new_user_defined_value_type_definition<'arena>(
    arena: &'arena Bump,
    type_keyword: TypeKeyword<'arena>,
    name: Identifier<'arena>,
    is_keyword: IsKeyword<'arena>,
    value_type: ElementaryType<'arena>,
    semicolon: Semicolon<'arena>,
) -> UserDefinedValueTypeDefinition<'arena> {
    Box::new_in(
        UserDefinedValueTypeDefinitionStruct {
            type_keyword,
            name,
            is_keyword,
            value_type,
            semicolon,
        },
        arena,
    )
}

pub type UsingAlias<'arena> = Box<'arena, UsingAliasStruct<'arena>>;

#[derive(Debug)]
pub struct UsingAliasStruct<'arena> {
    pub as_keyword: AsKeyword<'arena>,
    pub operator: UsingOperator<'arena>,
}

pub fn new_using_alias<'arena>(
    arena: &'arena Bump,
    as_keyword: AsKeyword<'arena>,
    operator: UsingOperator<'arena>,
) -> UsingAlias<'arena> {
    Box::new_in(
        UsingAliasStruct {
            as_keyword,
            operator,
        },
        arena,
    )
}

pub type UsingDeconstruction<'arena> = Box<'arena, UsingDeconstructionStruct<'arena>>;

#[derive(Debug)]
pub struct UsingDeconstructionStruct<'arena> {
    pub open_brace: OpenBrace<'arena>,
    pub symbols: UsingDeconstructionSymbols<'arena>,
    pub close_brace: CloseBrace<'arena>,
}

pub fn new_using_deconstruction<'arena>(
    arena: &'arena Bump,
    open_brace: OpenBrace<'arena>,
    symbols: UsingDeconstructionSymbols<'arena>,
    close_brace: CloseBrace<'arena>,
) -> UsingDeconstruction<'arena> {
    Box::new_in(
        UsingDeconstructionStruct {
            open_brace,
            symbols,
            close_brace,
        },
        arena,
    )
}

pub type UsingDeconstructionSymbol<'arena> = Box<'arena, UsingDeconstructionSymbolStruct<'arena>>;

#[derive(Debug)]
pub struct UsingDeconstructionSymbolStruct<'arena> {
    pub name: IdentifierPath<'arena>,
    pub alias: Option<UsingAlias<'arena>>,
}

pub fn new_using_deconstruction_symbol<'arena>(
    arena: &'arena Bump,
    name: IdentifierPath<'arena>,
    alias: Option<UsingAlias<'arena>>,
) -> UsingDeconstructionSymbol<'arena> {
    Box::new_in(UsingDeconstructionSymbolStruct { name, alias }, arena)
}

pub type UsingDirective<'arena> = Box<'arena, UsingDirectiveStruct<'arena>>;

#[derive(Debug)]
pub struct UsingDirectiveStruct<'arena> {
    pub using_keyword: UsingKeyword<'arena>,
    pub clause: UsingClause<'arena>,
    pub for_keyword: ForKeyword<'arena>,
    pub target: UsingTarget<'arena>,
    pub global_keyword: Option<GlobalKeyword<'arena>>,
    pub semicolon: Semicolon<'arena>,
}

pub fn new_using_directive<'arena>(
    arena: &'arena Bump,
    using_keyword: UsingKeyword<'arena>,
    clause: UsingClause<'arena>,
    for_keyword: ForKeyword<'arena>,
    target: UsingTarget<'arena>,
    global_keyword: Option<GlobalKeyword<'arena>>,
    semicolon: Semicolon<'arena>,
) -> UsingDirective<'arena> {
    Box::new_in(
        UsingDirectiveStruct {
            using_keyword,
            clause,
            for_keyword,
            target,
            global_keyword,
            semicolon,
        },
        arena,
    )
}

pub type VariableDeclarationStatement<'arena> =
    Box<'arena, VariableDeclarationStatementStruct<'arena>>;

#[derive(Debug)]
pub struct VariableDeclarationStatementStruct<'arena> {
    pub variable_type: VariableDeclarationType<'arena>,
    pub storage_location: Option<StorageLocation<'arena>>,
    pub name: Identifier<'arena>,
    pub value: Option<VariableDeclarationValue<'arena>>,
    pub semicolon: Semicolon<'arena>,
}

pub fn new_variable_declaration_statement<'arena>(
    arena: &'arena Bump,
    variable_type: VariableDeclarationType<'arena>,
    storage_location: Option<StorageLocation<'arena>>,
    name: Identifier<'arena>,
    value: Option<VariableDeclarationValue<'arena>>,
    semicolon: Semicolon<'arena>,
) -> VariableDeclarationStatement<'arena> {
    Box::new_in(
        VariableDeclarationStatementStruct {
            variable_type,
            storage_location,
            name,
            value,
            semicolon,
        },
        arena,
    )
}

pub type VariableDeclarationValue<'arena> = Box<'arena, VariableDeclarationValueStruct<'arena>>;

#[derive(Debug)]
pub struct VariableDeclarationValueStruct<'arena> {
    pub equal: Equal<'arena>,
    pub expression: Expression<'arena>,
}

pub fn new_variable_declaration_value<'arena>(
    arena: &'arena Bump,
    equal: Equal<'arena>,
    expression: Expression<'arena>,
) -> VariableDeclarationValue<'arena> {
    Box::new_in(VariableDeclarationValueStruct { equal, expression }, arena)
}

pub type VersionPragma<'arena> = Box<'arena, VersionPragmaStruct<'arena>>;

#[derive(Debug)]
pub struct VersionPragmaStruct<'arena> {
    pub solidity_keyword: SolidityKeyword<'arena>,
    pub sets: VersionExpressionSets<'arena>,
}

pub fn new_version_pragma<'arena>(
    arena: &'arena Bump,
    solidity_keyword: SolidityKeyword<'arena>,
    sets: VersionExpressionSets<'arena>,
) -> VersionPragma<'arena> {
    Box::new_in(
        VersionPragmaStruct {
            solidity_keyword,
            sets,
        },
        arena,
    )
}

pub type VersionRange<'arena> = Box<'arena, VersionRangeStruct<'arena>>;

#[derive(Debug)]
pub struct VersionRangeStruct<'arena> {
    pub start: VersionLiteral<'arena>,
    pub minus: Minus<'arena>,
    pub end: VersionLiteral<'arena>,
}

pub fn new_version_range<'arena>(
    arena: &'arena Bump,
    start: VersionLiteral<'arena>,
    minus: Minus<'arena>,
    end: VersionLiteral<'arena>,
) -> VersionRange<'arena> {
    Box::new_in(VersionRangeStruct { start, minus, end }, arena)
}

pub type VersionTerm<'arena> = Box<'arena, VersionTermStruct<'arena>>;

#[derive(Debug)]
pub struct VersionTermStruct<'arena> {
    pub operator: Option<VersionOperator<'arena>>,
    pub literal: VersionLiteral<'arena>,
}

pub fn new_version_term<'arena>(
    arena: &'arena Bump,
    operator: Option<VersionOperator<'arena>>,
    literal: VersionLiteral<'arena>,
) -> VersionTerm<'arena> {
    Box::new_in(VersionTermStruct { operator, literal }, arena)
}

pub type WhileStatement<'arena> = Box<'arena, WhileStatementStruct<'arena>>;

#[derive(Debug)]
pub struct WhileStatementStruct<'arena> {
    pub while_keyword: WhileKeyword<'arena>,
    pub open_paren: OpenParen<'arena>,
    pub condition: Expression<'arena>,
    pub close_paren: CloseParen<'arena>,
    pub body: Statement<'arena>,
}

pub fn new_while_statement<'arena>(
    arena: &'arena Bump,
    while_keyword: WhileKeyword<'arena>,
    open_paren: OpenParen<'arena>,
    condition: Expression<'arena>,
    close_paren: CloseParen<'arena>,
    body: Statement<'arena>,
) -> WhileStatement<'arena> {
    Box::new_in(
        WhileStatementStruct {
            while_keyword,
            open_paren,
            condition,
            close_paren,
            body,
        },
        arena,
    )
}

pub type YulBlock<'arena> = Box<'arena, YulBlockStruct<'arena>>;

#[derive(Debug)]
pub struct YulBlockStruct<'arena> {
    pub open_brace: OpenBrace<'arena>,
    pub statements: YulStatements<'arena>,
    pub close_brace: CloseBrace<'arena>,
}

pub fn new_yul_block<'arena>(
    arena: &'arena Bump,
    open_brace: OpenBrace<'arena>,
    statements: YulStatements<'arena>,
    close_brace: CloseBrace<'arena>,
) -> YulBlock<'arena> {
    Box::new_in(
        YulBlockStruct {
            open_brace,
            statements,
            close_brace,
        },
        arena,
    )
}

pub type YulBreakStatement<'arena> = Box<'arena, YulBreakStatementStruct<'arena>>;

#[derive(Debug)]
pub struct YulBreakStatementStruct<'arena> {
    pub break_keyword: YulBreakKeyword<'arena>,
}

pub fn new_yul_break_statement<'arena>(
    arena: &'arena Bump,
    break_keyword: YulBreakKeyword<'arena>,
) -> YulBreakStatement<'arena> {
    Box::new_in(YulBreakStatementStruct { break_keyword }, arena)
}

pub type YulColonAndEqual<'arena> = Box<'arena, YulColonAndEqualStruct<'arena>>;

#[derive(Debug)]
pub struct YulColonAndEqualStruct<'arena> {
    pub colon: Colon<'arena>,
    pub equal: Equal<'arena>,
}

pub fn new_yul_colon_and_equal<'arena>(
    arena: &'arena Bump,
    colon: Colon<'arena>,
    equal: Equal<'arena>,
) -> YulColonAndEqual<'arena> {
    Box::new_in(YulColonAndEqualStruct { colon, equal }, arena)
}

pub type YulContinueStatement<'arena> = Box<'arena, YulContinueStatementStruct<'arena>>;

#[derive(Debug)]
pub struct YulContinueStatementStruct<'arena> {
    pub continue_keyword: YulContinueKeyword<'arena>,
}

pub fn new_yul_continue_statement<'arena>(
    arena: &'arena Bump,
    continue_keyword: YulContinueKeyword<'arena>,
) -> YulContinueStatement<'arena> {
    Box::new_in(YulContinueStatementStruct { continue_keyword }, arena)
}

pub type YulDefaultCase<'arena> = Box<'arena, YulDefaultCaseStruct<'arena>>;

#[derive(Debug)]
pub struct YulDefaultCaseStruct<'arena> {
    pub default_keyword: YulDefaultKeyword<'arena>,
    pub body: YulBlock<'arena>,
}

pub fn new_yul_default_case<'arena>(
    arena: &'arena Bump,
    default_keyword: YulDefaultKeyword<'arena>,
    body: YulBlock<'arena>,
) -> YulDefaultCase<'arena> {
    Box::new_in(
        YulDefaultCaseStruct {
            default_keyword,
            body,
        },
        arena,
    )
}

pub type YulEqualAndColon<'arena> = Box<'arena, YulEqualAndColonStruct<'arena>>;

#[derive(Debug)]
pub struct YulEqualAndColonStruct<'arena> {
    pub equal: Equal<'arena>,
    pub colon: Colon<'arena>,
}

pub fn new_yul_equal_and_colon<'arena>(
    arena: &'arena Bump,
    equal: Equal<'arena>,
    colon: Colon<'arena>,
) -> YulEqualAndColon<'arena> {
    Box::new_in(YulEqualAndColonStruct { equal, colon }, arena)
}

pub type YulForStatement<'arena> = Box<'arena, YulForStatementStruct<'arena>>;

#[derive(Debug)]
pub struct YulForStatementStruct<'arena> {
    pub for_keyword: YulForKeyword<'arena>,
    pub initialization: YulBlock<'arena>,
    pub condition: YulExpression<'arena>,
    pub iterator: YulBlock<'arena>,
    pub body: YulBlock<'arena>,
}

pub fn new_yul_for_statement<'arena>(
    arena: &'arena Bump,
    for_keyword: YulForKeyword<'arena>,
    initialization: YulBlock<'arena>,
    condition: YulExpression<'arena>,
    iterator: YulBlock<'arena>,
    body: YulBlock<'arena>,
) -> YulForStatement<'arena> {
    Box::new_in(
        YulForStatementStruct {
            for_keyword,
            initialization,
            condition,
            iterator,
            body,
        },
        arena,
    )
}

pub type YulFunctionCallExpression<'arena> = Box<'arena, YulFunctionCallExpressionStruct<'arena>>;

#[derive(Debug)]
pub struct YulFunctionCallExpressionStruct<'arena> {
    pub operand: YulExpression<'arena>,
    pub open_paren: OpenParen<'arena>,
    pub arguments: YulArguments<'arena>,
    pub close_paren: CloseParen<'arena>,
}

pub fn new_yul_function_call_expression<'arena>(
    arena: &'arena Bump,
    operand: YulExpression<'arena>,
    open_paren: OpenParen<'arena>,
    arguments: YulArguments<'arena>,
    close_paren: CloseParen<'arena>,
) -> YulFunctionCallExpression<'arena> {
    Box::new_in(
        YulFunctionCallExpressionStruct {
            operand,
            open_paren,
            arguments,
            close_paren,
        },
        arena,
    )
}

pub type YulFunctionDefinition<'arena> = Box<'arena, YulFunctionDefinitionStruct<'arena>>;

#[derive(Debug)]
pub struct YulFunctionDefinitionStruct<'arena> {
    pub function_keyword: YulFunctionKeyword<'arena>,
    pub name: YulIdentifier<'arena>,
    pub parameters: YulParametersDeclaration<'arena>,
    pub returns: Option<YulReturnsDeclaration<'arena>>,
    pub body: YulBlock<'arena>,
}

pub fn new_yul_function_definition<'arena>(
    arena: &'arena Bump,
    function_keyword: YulFunctionKeyword<'arena>,
    name: YulIdentifier<'arena>,
    parameters: YulParametersDeclaration<'arena>,
    returns: Option<YulReturnsDeclaration<'arena>>,
    body: YulBlock<'arena>,
) -> YulFunctionDefinition<'arena> {
    Box::new_in(
        YulFunctionDefinitionStruct {
            function_keyword,
            name,
            parameters,
            returns,
            body,
        },
        arena,
    )
}

pub type YulIfStatement<'arena> = Box<'arena, YulIfStatementStruct<'arena>>;

#[derive(Debug)]
pub struct YulIfStatementStruct<'arena> {
    pub if_keyword: YulIfKeyword<'arena>,
    pub condition: YulExpression<'arena>,
    pub body: YulBlock<'arena>,
}

pub fn new_yul_if_statement<'arena>(
    arena: &'arena Bump,
    if_keyword: YulIfKeyword<'arena>,
    condition: YulExpression<'arena>,
    body: YulBlock<'arena>,
) -> YulIfStatement<'arena> {
    Box::new_in(
        YulIfStatementStruct {
            if_keyword,
            condition,
            body,
        },
        arena,
    )
}

pub type YulLabel<'arena> = Box<'arena, YulLabelStruct<'arena>>;

#[derive(Debug)]
pub struct YulLabelStruct<'arena> {
    pub label: YulIdentifier<'arena>,
    pub colon: Colon<'arena>,
}

pub fn new_yul_label<'arena>(
    arena: &'arena Bump,
    label: YulIdentifier<'arena>,
    colon: Colon<'arena>,
) -> YulLabel<'arena> {
    Box::new_in(YulLabelStruct { label, colon }, arena)
}

pub type YulLeaveStatement<'arena> = Box<'arena, YulLeaveStatementStruct<'arena>>;

#[derive(Debug)]
pub struct YulLeaveStatementStruct<'arena> {
    pub leave_keyword: YulLeaveKeyword<'arena>,
}

pub fn new_yul_leave_statement<'arena>(
    arena: &'arena Bump,
    leave_keyword: YulLeaveKeyword<'arena>,
) -> YulLeaveStatement<'arena> {
    Box::new_in(YulLeaveStatementStruct { leave_keyword }, arena)
}

pub type YulParametersDeclaration<'arena> = Box<'arena, YulParametersDeclarationStruct<'arena>>;

#[derive(Debug)]
pub struct YulParametersDeclarationStruct<'arena> {
    pub open_paren: OpenParen<'arena>,
    pub parameters: YulParameters<'arena>,
    pub close_paren: CloseParen<'arena>,
}

pub fn new_yul_parameters_declaration<'arena>(
    arena: &'arena Bump,
    open_paren: OpenParen<'arena>,
    parameters: YulParameters<'arena>,
    close_paren: CloseParen<'arena>,
) -> YulParametersDeclaration<'arena> {
    Box::new_in(
        YulParametersDeclarationStruct {
            open_paren,
            parameters,
            close_paren,
        },
        arena,
    )
}

pub type YulReturnsDeclaration<'arena> = Box<'arena, YulReturnsDeclarationStruct<'arena>>;

#[derive(Debug)]
pub struct YulReturnsDeclarationStruct<'arena> {
    pub minus_greater_than: MinusGreaterThan<'arena>,
    pub variables: YulVariableNames<'arena>,
}

pub fn new_yul_returns_declaration<'arena>(
    arena: &'arena Bump,
    minus_greater_than: MinusGreaterThan<'arena>,
    variables: YulVariableNames<'arena>,
) -> YulReturnsDeclaration<'arena> {
    Box::new_in(
        YulReturnsDeclarationStruct {
            minus_greater_than,
            variables,
        },
        arena,
    )
}

pub type YulStackAssignmentStatement<'arena> =
    Box<'arena, YulStackAssignmentStatementStruct<'arena>>;

#[derive(Debug)]
pub struct YulStackAssignmentStatementStruct<'arena> {
    pub assignment: YulStackAssignmentOperator<'arena>,
    pub variable: YulIdentifier<'arena>,
}

pub fn new_yul_stack_assignment_statement<'arena>(
    arena: &'arena Bump,
    assignment: YulStackAssignmentOperator<'arena>,
    variable: YulIdentifier<'arena>,
) -> YulStackAssignmentStatement<'arena> {
    Box::new_in(
        YulStackAssignmentStatementStruct {
            assignment,
            variable,
        },
        arena,
    )
}

pub type YulSwitchStatement<'arena> = Box<'arena, YulSwitchStatementStruct<'arena>>;

#[derive(Debug)]
pub struct YulSwitchStatementStruct<'arena> {
    pub switch_keyword: YulSwitchKeyword<'arena>,
    pub expression: YulExpression<'arena>,
    pub cases: YulSwitchCases<'arena>,
}

pub fn new_yul_switch_statement<'arena>(
    arena: &'arena Bump,
    switch_keyword: YulSwitchKeyword<'arena>,
    expression: YulExpression<'arena>,
    cases: YulSwitchCases<'arena>,
) -> YulSwitchStatement<'arena> {
    Box::new_in(
        YulSwitchStatementStruct {
            switch_keyword,
            expression,
            cases,
        },
        arena,
    )
}

pub type YulValueCase<'arena> = Box<'arena, YulValueCaseStruct<'arena>>;

#[derive(Debug)]
pub struct YulValueCaseStruct<'arena> {
    pub case_keyword: YulCaseKeyword<'arena>,
    pub value: YulLiteral<'arena>,
    pub body: YulBlock<'arena>,
}

pub fn new_yul_value_case<'arena>(
    arena: &'arena Bump,
    case_keyword: YulCaseKeyword<'arena>,
    value: YulLiteral<'arena>,
    body: YulBlock<'arena>,
) -> YulValueCase<'arena> {
    Box::new_in(
        YulValueCaseStruct {
            case_keyword,
            value,
            body,
        },
        arena,
    )
}

pub type YulVariableAssignmentStatement<'arena> =
    Box<'arena, YulVariableAssignmentStatementStruct<'arena>>;

#[derive(Debug)]
pub struct YulVariableAssignmentStatementStruct<'arena> {
    pub variables: YulPaths<'arena>,
    pub assignment: YulAssignmentOperator<'arena>,
    pub expression: YulExpression<'arena>,
}

pub fn new_yul_variable_assignment_statement<'arena>(
    arena: &'arena Bump,
    variables: YulPaths<'arena>,
    assignment: YulAssignmentOperator<'arena>,
    expression: YulExpression<'arena>,
) -> YulVariableAssignmentStatement<'arena> {
    Box::new_in(
        YulVariableAssignmentStatementStruct {
            variables,
            assignment,
            expression,
        },
        arena,
    )
}

pub type YulVariableDeclarationStatement<'arena> =
    Box<'arena, YulVariableDeclarationStatementStruct<'arena>>;

#[derive(Debug)]
pub struct YulVariableDeclarationStatementStruct<'arena> {
    pub let_keyword: YulLetKeyword<'arena>,
    pub variables: YulVariableNames<'arena>,
    pub value: Option<YulVariableDeclarationValue<'arena>>,
}

pub fn new_yul_variable_declaration_statement<'arena>(
    arena: &'arena Bump,
    let_keyword: YulLetKeyword<'arena>,
    variables: YulVariableNames<'arena>,
    value: Option<YulVariableDeclarationValue<'arena>>,
) -> YulVariableDeclarationStatement<'arena> {
    Box::new_in(
        YulVariableDeclarationStatementStruct {
            let_keyword,
            variables,
            value,
        },
        arena,
    )
}

pub type YulVariableDeclarationValue<'arena> =
    Box<'arena, YulVariableDeclarationValueStruct<'arena>>;

#[derive(Debug)]
pub struct YulVariableDeclarationValueStruct<'arena> {
    pub assignment: YulAssignmentOperator<'arena>,
    pub expression: YulExpression<'arena>,
}

pub fn new_yul_variable_declaration_value<'arena>(
    arena: &'arena Bump,
    assignment: YulAssignmentOperator<'arena>,
    expression: YulExpression<'arena>,
) -> YulVariableDeclarationValue<'arena> {
    Box::new_in(
        YulVariableDeclarationValueStruct {
            assignment,
            expression,
        },
        arena,
    )
}

//
// Choices:
//
// Note: We create a constructor function for each variant

#[derive(Debug)]
pub enum AbicoderVersion<'arena> {
    AbicoderV1Keyword(AbicoderV1Keyword<'arena>),
    AbicoderV2Keyword(AbicoderV2Keyword<'arena>),
}

pub fn new_abicoder_version_abicoder_v1_keyword<'arena>(
    arena: &'arena Bump,
    element: AbicoderV1Keyword<'arena>,
) -> AbicoderVersion<'arena> {
    AbicoderVersion::AbicoderV1Keyword(element)
}

pub fn new_abicoder_version_abicoder_v2_keyword<'arena>(
    arena: &'arena Bump,
    element: AbicoderV2Keyword<'arena>,
) -> AbicoderVersion<'arena> {
    AbicoderVersion::AbicoderV2Keyword(element)
}

#[derive(Debug)]
pub enum ArgumentsDeclaration<'arena> {
    PositionalArgumentsDeclaration(PositionalArgumentsDeclaration<'arena>),
    NamedArgumentsDeclaration(NamedArgumentsDeclaration<'arena>),
}

pub fn new_arguments_declaration_positional_arguments_declaration<'arena>(
    arena: &'arena Bump,
    element: PositionalArgumentsDeclaration<'arena>,
) -> ArgumentsDeclaration<'arena> {
    ArgumentsDeclaration::PositionalArgumentsDeclaration(element)
}

pub fn new_arguments_declaration_named_arguments_declaration<'arena>(
    arena: &'arena Bump,
    element: NamedArgumentsDeclaration<'arena>,
) -> ArgumentsDeclaration<'arena> {
    ArgumentsDeclaration::NamedArgumentsDeclaration(element)
}

#[derive(Debug)]
pub enum ConstructorAttribute<'arena> {
    ModifierInvocation(ModifierInvocation<'arena>),
    InternalKeyword(InternalKeyword<'arena>),
    OverrideKeyword(OverrideKeyword<'arena>),
    PayableKeyword(PayableKeyword<'arena>),
    PublicKeyword(PublicKeyword<'arena>),
    VirtualKeyword(VirtualKeyword<'arena>),
}

pub fn new_constructor_attribute_modifier_invocation<'arena>(
    arena: &'arena Bump,
    element: ModifierInvocation<'arena>,
) -> ConstructorAttribute<'arena> {
    ConstructorAttribute::ModifierInvocation(element)
}

pub fn new_constructor_attribute_internal_keyword<'arena>(
    arena: &'arena Bump,
    element: InternalKeyword<'arena>,
) -> ConstructorAttribute<'arena> {
    ConstructorAttribute::InternalKeyword(element)
}

pub fn new_constructor_attribute_override_keyword<'arena>(
    arena: &'arena Bump,
    element: OverrideKeyword<'arena>,
) -> ConstructorAttribute<'arena> {
    ConstructorAttribute::OverrideKeyword(element)
}

pub fn new_constructor_attribute_payable_keyword<'arena>(
    arena: &'arena Bump,
    element: PayableKeyword<'arena>,
) -> ConstructorAttribute<'arena> {
    ConstructorAttribute::PayableKeyword(element)
}

pub fn new_constructor_attribute_public_keyword<'arena>(
    arena: &'arena Bump,
    element: PublicKeyword<'arena>,
) -> ConstructorAttribute<'arena> {
    ConstructorAttribute::PublicKeyword(element)
}

pub fn new_constructor_attribute_virtual_keyword<'arena>(
    arena: &'arena Bump,
    element: VirtualKeyword<'arena>,
) -> ConstructorAttribute<'arena> {
    ConstructorAttribute::VirtualKeyword(element)
}

#[derive(Debug)]
pub enum ContractMember<'arena> {
    UsingDirective(UsingDirective<'arena>),
    FunctionDefinition(FunctionDefinition<'arena>),
    ConstructorDefinition(ConstructorDefinition<'arena>),
    ReceiveFunctionDefinition(ReceiveFunctionDefinition<'arena>),
    FallbackFunctionDefinition(FallbackFunctionDefinition<'arena>),
    UnnamedFunctionDefinition(UnnamedFunctionDefinition<'arena>),
    ModifierDefinition(ModifierDefinition<'arena>),
    StructDefinition(StructDefinition<'arena>),
    EnumDefinition(EnumDefinition<'arena>),
    EventDefinition(EventDefinition<'arena>),
    ErrorDefinition(ErrorDefinition<'arena>),
    UserDefinedValueTypeDefinition(UserDefinedValueTypeDefinition<'arena>),
    StateVariableDefinition(StateVariableDefinition<'arena>),
}

pub fn new_contract_member_using_directive<'arena>(
    arena: &'arena Bump,
    element: UsingDirective<'arena>,
) -> ContractMember<'arena> {
    ContractMember::UsingDirective(element)
}

pub fn new_contract_member_function_definition<'arena>(
    arena: &'arena Bump,
    element: FunctionDefinition<'arena>,
) -> ContractMember<'arena> {
    ContractMember::FunctionDefinition(element)
}

pub fn new_contract_member_constructor_definition<'arena>(
    arena: &'arena Bump,
    element: ConstructorDefinition<'arena>,
) -> ContractMember<'arena> {
    ContractMember::ConstructorDefinition(element)
}

pub fn new_contract_member_receive_function_definition<'arena>(
    arena: &'arena Bump,
    element: ReceiveFunctionDefinition<'arena>,
) -> ContractMember<'arena> {
    ContractMember::ReceiveFunctionDefinition(element)
}

pub fn new_contract_member_fallback_function_definition<'arena>(
    arena: &'arena Bump,
    element: FallbackFunctionDefinition<'arena>,
) -> ContractMember<'arena> {
    ContractMember::FallbackFunctionDefinition(element)
}

pub fn new_contract_member_unnamed_function_definition<'arena>(
    arena: &'arena Bump,
    element: UnnamedFunctionDefinition<'arena>,
) -> ContractMember<'arena> {
    ContractMember::UnnamedFunctionDefinition(element)
}

pub fn new_contract_member_modifier_definition<'arena>(
    arena: &'arena Bump,
    element: ModifierDefinition<'arena>,
) -> ContractMember<'arena> {
    ContractMember::ModifierDefinition(element)
}

pub fn new_contract_member_struct_definition<'arena>(
    arena: &'arena Bump,
    element: StructDefinition<'arena>,
) -> ContractMember<'arena> {
    ContractMember::StructDefinition(element)
}

pub fn new_contract_member_enum_definition<'arena>(
    arena: &'arena Bump,
    element: EnumDefinition<'arena>,
) -> ContractMember<'arena> {
    ContractMember::EnumDefinition(element)
}

pub fn new_contract_member_event_definition<'arena>(
    arena: &'arena Bump,
    element: EventDefinition<'arena>,
) -> ContractMember<'arena> {
    ContractMember::EventDefinition(element)
}

pub fn new_contract_member_error_definition<'arena>(
    arena: &'arena Bump,
    element: ErrorDefinition<'arena>,
) -> ContractMember<'arena> {
    ContractMember::ErrorDefinition(element)
}

pub fn new_contract_member_user_defined_value_type_definition<'arena>(
    arena: &'arena Bump,
    element: UserDefinedValueTypeDefinition<'arena>,
) -> ContractMember<'arena> {
    ContractMember::UserDefinedValueTypeDefinition(element)
}

pub fn new_contract_member_state_variable_definition<'arena>(
    arena: &'arena Bump,
    element: StateVariableDefinition<'arena>,
) -> ContractMember<'arena> {
    ContractMember::StateVariableDefinition(element)
}

#[derive(Debug)]
pub enum ContractSpecifier<'arena> {
    InheritanceSpecifier(InheritanceSpecifier<'arena>),
    StorageLayoutSpecifier(StorageLayoutSpecifier<'arena>),
}

pub fn new_contract_specifier_inheritance_specifier<'arena>(
    arena: &'arena Bump,
    element: InheritanceSpecifier<'arena>,
) -> ContractSpecifier<'arena> {
    ContractSpecifier::InheritanceSpecifier(element)
}

pub fn new_contract_specifier_storage_layout_specifier<'arena>(
    arena: &'arena Bump,
    element: StorageLayoutSpecifier<'arena>,
) -> ContractSpecifier<'arena> {
    ContractSpecifier::StorageLayoutSpecifier(element)
}

#[derive(Debug)]
pub enum ElementaryType<'arena> {
    BoolKeyword(BoolKeyword<'arena>),
    ByteKeyword(ByteKeyword<'arena>),
    StringKeyword(StringKeyword<'arena>),
    AddressType(AddressType<'arena>),
    BytesKeyword(BytesKeyword<'arena>),
    IntKeyword(IntKeyword<'arena>),
    UintKeyword(UintKeyword<'arena>),
    FixedKeyword(FixedKeyword<'arena>),
    UfixedKeyword(UfixedKeyword<'arena>),
}

pub fn new_elementary_type_bool_keyword<'arena>(
    arena: &'arena Bump,
    element: BoolKeyword<'arena>,
) -> ElementaryType<'arena> {
    ElementaryType::BoolKeyword(element)
}

pub fn new_elementary_type_byte_keyword<'arena>(
    arena: &'arena Bump,
    element: ByteKeyword<'arena>,
) -> ElementaryType<'arena> {
    ElementaryType::ByteKeyword(element)
}

pub fn new_elementary_type_string_keyword<'arena>(
    arena: &'arena Bump,
    element: StringKeyword<'arena>,
) -> ElementaryType<'arena> {
    ElementaryType::StringKeyword(element)
}

pub fn new_elementary_type_address_type<'arena>(
    arena: &'arena Bump,
    element: AddressType<'arena>,
) -> ElementaryType<'arena> {
    ElementaryType::AddressType(element)
}

pub fn new_elementary_type_bytes_keyword<'arena>(
    arena: &'arena Bump,
    element: BytesKeyword<'arena>,
) -> ElementaryType<'arena> {
    ElementaryType::BytesKeyword(element)
}

pub fn new_elementary_type_int_keyword<'arena>(
    arena: &'arena Bump,
    element: IntKeyword<'arena>,
) -> ElementaryType<'arena> {
    ElementaryType::IntKeyword(element)
}

pub fn new_elementary_type_uint_keyword<'arena>(
    arena: &'arena Bump,
    element: UintKeyword<'arena>,
) -> ElementaryType<'arena> {
    ElementaryType::UintKeyword(element)
}

pub fn new_elementary_type_fixed_keyword<'arena>(
    arena: &'arena Bump,
    element: FixedKeyword<'arena>,
) -> ElementaryType<'arena> {
    ElementaryType::FixedKeyword(element)
}

pub fn new_elementary_type_ufixed_keyword<'arena>(
    arena: &'arena Bump,
    element: UfixedKeyword<'arena>,
) -> ElementaryType<'arena> {
    ElementaryType::UfixedKeyword(element)
}

#[derive(Debug)]
pub enum ExperimentalFeature<'arena> {
    ABIEncoderV2Keyword(ABIEncoderV2Keyword<'arena>),
    SMTCheckerKeyword(SMTCheckerKeyword<'arena>),
    StringLiteral(StringLiteral<'arena>),
}

pub fn new_experimental_feature_abi_encoder_v2_keyword<'arena>(
    arena: &'arena Bump,
    element: ABIEncoderV2Keyword<'arena>,
) -> ExperimentalFeature<'arena> {
    ExperimentalFeature::ABIEncoderV2Keyword(element)
}

pub fn new_experimental_feature_smt_checker_keyword<'arena>(
    arena: &'arena Bump,
    element: SMTCheckerKeyword<'arena>,
) -> ExperimentalFeature<'arena> {
    ExperimentalFeature::SMTCheckerKeyword(element)
}

pub fn new_experimental_feature_string_literal<'arena>(
    arena: &'arena Bump,
    element: StringLiteral<'arena>,
) -> ExperimentalFeature<'arena> {
    ExperimentalFeature::StringLiteral(element)
}

#[derive(Debug)]
pub enum Expression<'arena> {
    AssignmentExpression(AssignmentExpression<'arena>),
    ConditionalExpression(ConditionalExpression<'arena>),
    OrExpression(OrExpression<'arena>),
    AndExpression(AndExpression<'arena>),
    EqualityExpression(EqualityExpression<'arena>),
    InequalityExpression(InequalityExpression<'arena>),
    BitwiseOrExpression(BitwiseOrExpression<'arena>),
    BitwiseXorExpression(BitwiseXorExpression<'arena>),
    BitwiseAndExpression(BitwiseAndExpression<'arena>),
    ShiftExpression(ShiftExpression<'arena>),
    AdditiveExpression(AdditiveExpression<'arena>),
    MultiplicativeExpression(MultiplicativeExpression<'arena>),
    ExponentiationExpression(ExponentiationExpression<'arena>),
    PostfixExpression(PostfixExpression<'arena>),
    PrefixExpression(PrefixExpression<'arena>),
    FunctionCallExpression(FunctionCallExpression<'arena>),
    CallOptionsExpression(CallOptionsExpression<'arena>),
    MemberAccessExpression(MemberAccessExpression<'arena>),
    IndexAccessExpression(IndexAccessExpression<'arena>),
    NewExpression(NewExpression<'arena>),
    TupleExpression(TupleExpression<'arena>),
    TypeExpression(TypeExpression<'arena>),
    ArrayExpression(ArrayExpression<'arena>),
    HexNumberExpression(HexNumberExpression<'arena>),
    DecimalNumberExpression(DecimalNumberExpression<'arena>),
    StringExpression(StringExpression<'arena>),
    ElementaryType(ElementaryType<'arena>),
    PayableKeyword(PayableKeyword<'arena>),
    ThisKeyword(ThisKeyword<'arena>),
    SuperKeyword(SuperKeyword<'arena>),
    TrueKeyword(TrueKeyword<'arena>),
    FalseKeyword(FalseKeyword<'arena>),
    Identifier(Identifier<'arena>),
}

pub fn new_expression_assignment_expression<'arena>(
    arena: &'arena Bump,
    element: AssignmentExpression<'arena>,
) -> Expression<'arena> {
    Expression::AssignmentExpression(element)
}

pub fn new_expression_conditional_expression<'arena>(
    arena: &'arena Bump,
    element: ConditionalExpression<'arena>,
) -> Expression<'arena> {
    Expression::ConditionalExpression(element)
}

pub fn new_expression_or_expression<'arena>(
    arena: &'arena Bump,
    element: OrExpression<'arena>,
) -> Expression<'arena> {
    Expression::OrExpression(element)
}

pub fn new_expression_and_expression<'arena>(
    arena: &'arena Bump,
    element: AndExpression<'arena>,
) -> Expression<'arena> {
    Expression::AndExpression(element)
}

pub fn new_expression_equality_expression<'arena>(
    arena: &'arena Bump,
    element: EqualityExpression<'arena>,
) -> Expression<'arena> {
    Expression::EqualityExpression(element)
}

pub fn new_expression_inequality_expression<'arena>(
    arena: &'arena Bump,
    element: InequalityExpression<'arena>,
) -> Expression<'arena> {
    Expression::InequalityExpression(element)
}

pub fn new_expression_bitwise_or_expression<'arena>(
    arena: &'arena Bump,
    element: BitwiseOrExpression<'arena>,
) -> Expression<'arena> {
    Expression::BitwiseOrExpression(element)
}

pub fn new_expression_bitwise_xor_expression<'arena>(
    arena: &'arena Bump,
    element: BitwiseXorExpression<'arena>,
) -> Expression<'arena> {
    Expression::BitwiseXorExpression(element)
}

pub fn new_expression_bitwise_and_expression<'arena>(
    arena: &'arena Bump,
    element: BitwiseAndExpression<'arena>,
) -> Expression<'arena> {
    Expression::BitwiseAndExpression(element)
}

pub fn new_expression_shift_expression<'arena>(
    arena: &'arena Bump,
    element: ShiftExpression<'arena>,
) -> Expression<'arena> {
    Expression::ShiftExpression(element)
}

pub fn new_expression_additive_expression<'arena>(
    arena: &'arena Bump,
    element: AdditiveExpression<'arena>,
) -> Expression<'arena> {
    Expression::AdditiveExpression(element)
}

pub fn new_expression_multiplicative_expression<'arena>(
    arena: &'arena Bump,
    element: MultiplicativeExpression<'arena>,
) -> Expression<'arena> {
    Expression::MultiplicativeExpression(element)
}

pub fn new_expression_exponentiation_expression<'arena>(
    arena: &'arena Bump,
    element: ExponentiationExpression<'arena>,
) -> Expression<'arena> {
    Expression::ExponentiationExpression(element)
}

pub fn new_expression_postfix_expression<'arena>(
    arena: &'arena Bump,
    element: PostfixExpression<'arena>,
) -> Expression<'arena> {
    Expression::PostfixExpression(element)
}

pub fn new_expression_prefix_expression<'arena>(
    arena: &'arena Bump,
    element: PrefixExpression<'arena>,
) -> Expression<'arena> {
    Expression::PrefixExpression(element)
}

pub fn new_expression_function_call_expression<'arena>(
    arena: &'arena Bump,
    element: FunctionCallExpression<'arena>,
) -> Expression<'arena> {
    Expression::FunctionCallExpression(element)
}

pub fn new_expression_call_options_expression<'arena>(
    arena: &'arena Bump,
    element: CallOptionsExpression<'arena>,
) -> Expression<'arena> {
    Expression::CallOptionsExpression(element)
}

pub fn new_expression_member_access_expression<'arena>(
    arena: &'arena Bump,
    element: MemberAccessExpression<'arena>,
) -> Expression<'arena> {
    Expression::MemberAccessExpression(element)
}

pub fn new_expression_index_access_expression<'arena>(
    arena: &'arena Bump,
    element: IndexAccessExpression<'arena>,
) -> Expression<'arena> {
    Expression::IndexAccessExpression(element)
}

pub fn new_expression_new_expression<'arena>(
    arena: &'arena Bump,
    element: NewExpression<'arena>,
) -> Expression<'arena> {
    Expression::NewExpression(element)
}

pub fn new_expression_tuple_expression<'arena>(
    arena: &'arena Bump,
    element: TupleExpression<'arena>,
) -> Expression<'arena> {
    Expression::TupleExpression(element)
}

pub fn new_expression_type_expression<'arena>(
    arena: &'arena Bump,
    element: TypeExpression<'arena>,
) -> Expression<'arena> {
    Expression::TypeExpression(element)
}

pub fn new_expression_array_expression<'arena>(
    arena: &'arena Bump,
    element: ArrayExpression<'arena>,
) -> Expression<'arena> {
    Expression::ArrayExpression(element)
}

pub fn new_expression_hex_number_expression<'arena>(
    arena: &'arena Bump,
    element: HexNumberExpression<'arena>,
) -> Expression<'arena> {
    Expression::HexNumberExpression(element)
}

pub fn new_expression_decimal_number_expression<'arena>(
    arena: &'arena Bump,
    element: DecimalNumberExpression<'arena>,
) -> Expression<'arena> {
    Expression::DecimalNumberExpression(element)
}

pub fn new_expression_string_expression<'arena>(
    arena: &'arena Bump,
    element: StringExpression<'arena>,
) -> Expression<'arena> {
    Expression::StringExpression(element)
}

pub fn new_expression_elementary_type<'arena>(
    arena: &'arena Bump,
    element: ElementaryType<'arena>,
) -> Expression<'arena> {
    Expression::ElementaryType(element)
}

pub fn new_expression_payable_keyword<'arena>(
    arena: &'arena Bump,
    element: PayableKeyword<'arena>,
) -> Expression<'arena> {
    Expression::PayableKeyword(element)
}

pub fn new_expression_this_keyword<'arena>(
    arena: &'arena Bump,
    element: ThisKeyword<'arena>,
) -> Expression<'arena> {
    Expression::ThisKeyword(element)
}

pub fn new_expression_super_keyword<'arena>(
    arena: &'arena Bump,
    element: SuperKeyword<'arena>,
) -> Expression<'arena> {
    Expression::SuperKeyword(element)
}

pub fn new_expression_true_keyword<'arena>(
    arena: &'arena Bump,
    element: TrueKeyword<'arena>,
) -> Expression<'arena> {
    Expression::TrueKeyword(element)
}

pub fn new_expression_false_keyword<'arena>(
    arena: &'arena Bump,
    element: FalseKeyword<'arena>,
) -> Expression<'arena> {
    Expression::FalseKeyword(element)
}

pub fn new_expression_identifier<'arena>(
    arena: &'arena Bump,
    element: Identifier<'arena>,
) -> Expression<'arena> {
    Expression::Identifier(element)
}

#[derive(Debug)]
pub enum Expression_AdditiveExpression_Operator<'arena> {
    Minus(Minus<'arena>),
    Plus(Plus<'arena>),
}

pub fn new_expression_additive_expression_operator_minus<'arena>(
    arena: &'arena Bump,
    element: Minus<'arena>,
) -> Expression_AdditiveExpression_Operator<'arena> {
    Expression_AdditiveExpression_Operator::Minus(element)
}

pub fn new_expression_additive_expression_operator_plus<'arena>(
    arena: &'arena Bump,
    element: Plus<'arena>,
) -> Expression_AdditiveExpression_Operator<'arena> {
    Expression_AdditiveExpression_Operator::Plus(element)
}

#[derive(Debug)]
pub enum Expression_AssignmentExpression_Operator<'arena> {
    AmpersandEqual(AmpersandEqual<'arena>),
    AsteriskEqual(AsteriskEqual<'arena>),
    BarEqual(BarEqual<'arena>),
    CaretEqual(CaretEqual<'arena>),
    Equal(Equal<'arena>),
    GreaterThanGreaterThanEqual(GreaterThanGreaterThanEqual<'arena>),
    GreaterThanGreaterThanGreaterThanEqual(GreaterThanGreaterThanGreaterThanEqual<'arena>),
    LessThanLessThanEqual(LessThanLessThanEqual<'arena>),
    MinusEqual(MinusEqual<'arena>),
    PercentEqual(PercentEqual<'arena>),
    PlusEqual(PlusEqual<'arena>),
    SlashEqual(SlashEqual<'arena>),
}

pub fn new_expression_assignment_expression_operator_ampersand_equal<'arena>(
    arena: &'arena Bump,
    element: AmpersandEqual<'arena>,
) -> Expression_AssignmentExpression_Operator<'arena> {
    Expression_AssignmentExpression_Operator::AmpersandEqual(element)
}

pub fn new_expression_assignment_expression_operator_asterisk_equal<'arena>(
    arena: &'arena Bump,
    element: AsteriskEqual<'arena>,
) -> Expression_AssignmentExpression_Operator<'arena> {
    Expression_AssignmentExpression_Operator::AsteriskEqual(element)
}

pub fn new_expression_assignment_expression_operator_bar_equal<'arena>(
    arena: &'arena Bump,
    element: BarEqual<'arena>,
) -> Expression_AssignmentExpression_Operator<'arena> {
    Expression_AssignmentExpression_Operator::BarEqual(element)
}

pub fn new_expression_assignment_expression_operator_caret_equal<'arena>(
    arena: &'arena Bump,
    element: CaretEqual<'arena>,
) -> Expression_AssignmentExpression_Operator<'arena> {
    Expression_AssignmentExpression_Operator::CaretEqual(element)
}

pub fn new_expression_assignment_expression_operator_equal<'arena>(
    arena: &'arena Bump,
    element: Equal<'arena>,
) -> Expression_AssignmentExpression_Operator<'arena> {
    Expression_AssignmentExpression_Operator::Equal(element)
}

pub fn new_expression_assignment_expression_operator_greater_than_greater_than_equal<'arena>(
    arena: &'arena Bump,
    element: GreaterThanGreaterThanEqual<'arena>,
) -> Expression_AssignmentExpression_Operator<'arena> {
    Expression_AssignmentExpression_Operator::GreaterThanGreaterThanEqual(element)
}

pub fn new_expression_assignment_expression_operator_greater_than_greater_than_greater_than_equal<
    'arena,
>(
    arena: &'arena Bump,
    element: GreaterThanGreaterThanGreaterThanEqual<'arena>,
) -> Expression_AssignmentExpression_Operator<'arena> {
    Expression_AssignmentExpression_Operator::GreaterThanGreaterThanGreaterThanEqual(element)
}

pub fn new_expression_assignment_expression_operator_less_than_less_than_equal<'arena>(
    arena: &'arena Bump,
    element: LessThanLessThanEqual<'arena>,
) -> Expression_AssignmentExpression_Operator<'arena> {
    Expression_AssignmentExpression_Operator::LessThanLessThanEqual(element)
}

pub fn new_expression_assignment_expression_operator_minus_equal<'arena>(
    arena: &'arena Bump,
    element: MinusEqual<'arena>,
) -> Expression_AssignmentExpression_Operator<'arena> {
    Expression_AssignmentExpression_Operator::MinusEqual(element)
}

pub fn new_expression_assignment_expression_operator_percent_equal<'arena>(
    arena: &'arena Bump,
    element: PercentEqual<'arena>,
) -> Expression_AssignmentExpression_Operator<'arena> {
    Expression_AssignmentExpression_Operator::PercentEqual(element)
}

pub fn new_expression_assignment_expression_operator_plus_equal<'arena>(
    arena: &'arena Bump,
    element: PlusEqual<'arena>,
) -> Expression_AssignmentExpression_Operator<'arena> {
    Expression_AssignmentExpression_Operator::PlusEqual(element)
}

pub fn new_expression_assignment_expression_operator_slash_equal<'arena>(
    arena: &'arena Bump,
    element: SlashEqual<'arena>,
) -> Expression_AssignmentExpression_Operator<'arena> {
    Expression_AssignmentExpression_Operator::SlashEqual(element)
}

#[derive(Debug)]
pub enum Expression_EqualityExpression_Operator<'arena> {
    BangEqual(BangEqual<'arena>),
    EqualEqual(EqualEqual<'arena>),
}

pub fn new_expression_equality_expression_operator_bang_equal<'arena>(
    arena: &'arena Bump,
    element: BangEqual<'arena>,
) -> Expression_EqualityExpression_Operator<'arena> {
    Expression_EqualityExpression_Operator::BangEqual(element)
}

pub fn new_expression_equality_expression_operator_equal_equal<'arena>(
    arena: &'arena Bump,
    element: EqualEqual<'arena>,
) -> Expression_EqualityExpression_Operator<'arena> {
    Expression_EqualityExpression_Operator::EqualEqual(element)
}

#[derive(Debug)]
pub enum Expression_ExponentiationExpression_Operator<'arena> {
    AsteriskAsterisk(AsteriskAsterisk<'arena>),
}

pub fn new_expression_exponentiation_expression_operator_asterisk_asterisk<'arena>(
    arena: &'arena Bump,
    element: AsteriskAsterisk<'arena>,
) -> Expression_ExponentiationExpression_Operator<'arena> {
    Expression_ExponentiationExpression_Operator::AsteriskAsterisk(element)
}

#[derive(Debug)]
pub enum Expression_InequalityExpression_Operator<'arena> {
    GreaterThan(GreaterThan<'arena>),
    GreaterThanEqual(GreaterThanEqual<'arena>),
    LessThan(LessThan<'arena>),
    LessThanEqual(LessThanEqual<'arena>),
}

pub fn new_expression_inequality_expression_operator_greater_than<'arena>(
    arena: &'arena Bump,
    element: GreaterThan<'arena>,
) -> Expression_InequalityExpression_Operator<'arena> {
    Expression_InequalityExpression_Operator::GreaterThan(element)
}

pub fn new_expression_inequality_expression_operator_greater_than_equal<'arena>(
    arena: &'arena Bump,
    element: GreaterThanEqual<'arena>,
) -> Expression_InequalityExpression_Operator<'arena> {
    Expression_InequalityExpression_Operator::GreaterThanEqual(element)
}

pub fn new_expression_inequality_expression_operator_less_than<'arena>(
    arena: &'arena Bump,
    element: LessThan<'arena>,
) -> Expression_InequalityExpression_Operator<'arena> {
    Expression_InequalityExpression_Operator::LessThan(element)
}

pub fn new_expression_inequality_expression_operator_less_than_equal<'arena>(
    arena: &'arena Bump,
    element: LessThanEqual<'arena>,
) -> Expression_InequalityExpression_Operator<'arena> {
    Expression_InequalityExpression_Operator::LessThanEqual(element)
}

#[derive(Debug)]
pub enum Expression_MultiplicativeExpression_Operator<'arena> {
    Asterisk(Asterisk<'arena>),
    Percent(Percent<'arena>),
    Slash(Slash<'arena>),
}

pub fn new_expression_multiplicative_expression_operator_asterisk<'arena>(
    arena: &'arena Bump,
    element: Asterisk<'arena>,
) -> Expression_MultiplicativeExpression_Operator<'arena> {
    Expression_MultiplicativeExpression_Operator::Asterisk(element)
}

pub fn new_expression_multiplicative_expression_operator_percent<'arena>(
    arena: &'arena Bump,
    element: Percent<'arena>,
) -> Expression_MultiplicativeExpression_Operator<'arena> {
    Expression_MultiplicativeExpression_Operator::Percent(element)
}

pub fn new_expression_multiplicative_expression_operator_slash<'arena>(
    arena: &'arena Bump,
    element: Slash<'arena>,
) -> Expression_MultiplicativeExpression_Operator<'arena> {
    Expression_MultiplicativeExpression_Operator::Slash(element)
}

#[derive(Debug)]
pub enum Expression_PostfixExpression_Operator<'arena> {
    MinusMinus(MinusMinus<'arena>),
    PlusPlus(PlusPlus<'arena>),
}

pub fn new_expression_postfix_expression_operator_minus_minus<'arena>(
    arena: &'arena Bump,
    element: MinusMinus<'arena>,
) -> Expression_PostfixExpression_Operator<'arena> {
    Expression_PostfixExpression_Operator::MinusMinus(element)
}

pub fn new_expression_postfix_expression_operator_plus_plus<'arena>(
    arena: &'arena Bump,
    element: PlusPlus<'arena>,
) -> Expression_PostfixExpression_Operator<'arena> {
    Expression_PostfixExpression_Operator::PlusPlus(element)
}

#[derive(Debug)]
pub enum Expression_PrefixExpression_Operator<'arena> {
    Bang(Bang<'arena>),
    DeleteKeyword(DeleteKeyword<'arena>),
    Minus(Minus<'arena>),
    MinusMinus(MinusMinus<'arena>),
    Plus(Plus<'arena>),
    PlusPlus(PlusPlus<'arena>),
    Tilde(Tilde<'arena>),
}

pub fn new_expression_prefix_expression_operator_bang<'arena>(
    arena: &'arena Bump,
    element: Bang<'arena>,
) -> Expression_PrefixExpression_Operator<'arena> {
    Expression_PrefixExpression_Operator::Bang(element)
}

pub fn new_expression_prefix_expression_operator_delete_keyword<'arena>(
    arena: &'arena Bump,
    element: DeleteKeyword<'arena>,
) -> Expression_PrefixExpression_Operator<'arena> {
    Expression_PrefixExpression_Operator::DeleteKeyword(element)
}

pub fn new_expression_prefix_expression_operator_minus<'arena>(
    arena: &'arena Bump,
    element: Minus<'arena>,
) -> Expression_PrefixExpression_Operator<'arena> {
    Expression_PrefixExpression_Operator::Minus(element)
}

pub fn new_expression_prefix_expression_operator_minus_minus<'arena>(
    arena: &'arena Bump,
    element: MinusMinus<'arena>,
) -> Expression_PrefixExpression_Operator<'arena> {
    Expression_PrefixExpression_Operator::MinusMinus(element)
}

pub fn new_expression_prefix_expression_operator_plus<'arena>(
    arena: &'arena Bump,
    element: Plus<'arena>,
) -> Expression_PrefixExpression_Operator<'arena> {
    Expression_PrefixExpression_Operator::Plus(element)
}

pub fn new_expression_prefix_expression_operator_plus_plus<'arena>(
    arena: &'arena Bump,
    element: PlusPlus<'arena>,
) -> Expression_PrefixExpression_Operator<'arena> {
    Expression_PrefixExpression_Operator::PlusPlus(element)
}

pub fn new_expression_prefix_expression_operator_tilde<'arena>(
    arena: &'arena Bump,
    element: Tilde<'arena>,
) -> Expression_PrefixExpression_Operator<'arena> {
    Expression_PrefixExpression_Operator::Tilde(element)
}

#[derive(Debug)]
pub enum Expression_ShiftExpression_Operator<'arena> {
    GreaterThanGreaterThan(GreaterThanGreaterThan<'arena>),
    GreaterThanGreaterThanGreaterThan(GreaterThanGreaterThanGreaterThan<'arena>),
    LessThanLessThan(LessThanLessThan<'arena>),
}

pub fn new_expression_shift_expression_operator_greater_than_greater_than<'arena>(
    arena: &'arena Bump,
    element: GreaterThanGreaterThan<'arena>,
) -> Expression_ShiftExpression_Operator<'arena> {
    Expression_ShiftExpression_Operator::GreaterThanGreaterThan(element)
}

pub fn new_expression_shift_expression_operator_greater_than_greater_than_greater_than<'arena>(
    arena: &'arena Bump,
    element: GreaterThanGreaterThanGreaterThan<'arena>,
) -> Expression_ShiftExpression_Operator<'arena> {
    Expression_ShiftExpression_Operator::GreaterThanGreaterThanGreaterThan(element)
}

pub fn new_expression_shift_expression_operator_less_than_less_than<'arena>(
    arena: &'arena Bump,
    element: LessThanLessThan<'arena>,
) -> Expression_ShiftExpression_Operator<'arena> {
    Expression_ShiftExpression_Operator::LessThanLessThan(element)
}

#[derive(Debug)]
pub enum FallbackFunctionAttribute<'arena> {
    ModifierInvocation(ModifierInvocation<'arena>),
    OverrideSpecifier(OverrideSpecifier<'arena>),
    ExternalKeyword(ExternalKeyword<'arena>),
    PayableKeyword(PayableKeyword<'arena>),
    PureKeyword(PureKeyword<'arena>),
    ViewKeyword(ViewKeyword<'arena>),
    VirtualKeyword(VirtualKeyword<'arena>),
}

pub fn new_fallback_function_attribute_modifier_invocation<'arena>(
    arena: &'arena Bump,
    element: ModifierInvocation<'arena>,
) -> FallbackFunctionAttribute<'arena> {
    FallbackFunctionAttribute::ModifierInvocation(element)
}

pub fn new_fallback_function_attribute_override_specifier<'arena>(
    arena: &'arena Bump,
    element: OverrideSpecifier<'arena>,
) -> FallbackFunctionAttribute<'arena> {
    FallbackFunctionAttribute::OverrideSpecifier(element)
}

pub fn new_fallback_function_attribute_external_keyword<'arena>(
    arena: &'arena Bump,
    element: ExternalKeyword<'arena>,
) -> FallbackFunctionAttribute<'arena> {
    FallbackFunctionAttribute::ExternalKeyword(element)
}

pub fn new_fallback_function_attribute_payable_keyword<'arena>(
    arena: &'arena Bump,
    element: PayableKeyword<'arena>,
) -> FallbackFunctionAttribute<'arena> {
    FallbackFunctionAttribute::PayableKeyword(element)
}

pub fn new_fallback_function_attribute_pure_keyword<'arena>(
    arena: &'arena Bump,
    element: PureKeyword<'arena>,
) -> FallbackFunctionAttribute<'arena> {
    FallbackFunctionAttribute::PureKeyword(element)
}

pub fn new_fallback_function_attribute_view_keyword<'arena>(
    arena: &'arena Bump,
    element: ViewKeyword<'arena>,
) -> FallbackFunctionAttribute<'arena> {
    FallbackFunctionAttribute::ViewKeyword(element)
}

pub fn new_fallback_function_attribute_virtual_keyword<'arena>(
    arena: &'arena Bump,
    element: VirtualKeyword<'arena>,
) -> FallbackFunctionAttribute<'arena> {
    FallbackFunctionAttribute::VirtualKeyword(element)
}

#[derive(Debug)]
pub enum ForStatementCondition<'arena> {
    ExpressionStatement(ExpressionStatement<'arena>),
    Semicolon(Semicolon<'arena>),
}

pub fn new_for_statement_condition_expression_statement<'arena>(
    arena: &'arena Bump,
    element: ExpressionStatement<'arena>,
) -> ForStatementCondition<'arena> {
    ForStatementCondition::ExpressionStatement(element)
}

pub fn new_for_statement_condition_semicolon<'arena>(
    arena: &'arena Bump,
    element: Semicolon<'arena>,
) -> ForStatementCondition<'arena> {
    ForStatementCondition::Semicolon(element)
}

#[derive(Debug)]
pub enum ForStatementInitialization<'arena> {
    TupleDeconstructionStatement(TupleDeconstructionStatement<'arena>),
    VariableDeclarationStatement(VariableDeclarationStatement<'arena>),
    ExpressionStatement(ExpressionStatement<'arena>),
    Semicolon(Semicolon<'arena>),
}

pub fn new_for_statement_initialization_tuple_deconstruction_statement<'arena>(
    arena: &'arena Bump,
    element: TupleDeconstructionStatement<'arena>,
) -> ForStatementInitialization<'arena> {
    ForStatementInitialization::TupleDeconstructionStatement(element)
}

pub fn new_for_statement_initialization_variable_declaration_statement<'arena>(
    arena: &'arena Bump,
    element: VariableDeclarationStatement<'arena>,
) -> ForStatementInitialization<'arena> {
    ForStatementInitialization::VariableDeclarationStatement(element)
}

pub fn new_for_statement_initialization_expression_statement<'arena>(
    arena: &'arena Bump,
    element: ExpressionStatement<'arena>,
) -> ForStatementInitialization<'arena> {
    ForStatementInitialization::ExpressionStatement(element)
}

pub fn new_for_statement_initialization_semicolon<'arena>(
    arena: &'arena Bump,
    element: Semicolon<'arena>,
) -> ForStatementInitialization<'arena> {
    ForStatementInitialization::Semicolon(element)
}

#[derive(Debug)]
pub enum FunctionAttribute<'arena> {
    ModifierInvocation(ModifierInvocation<'arena>),
    OverrideSpecifier(OverrideSpecifier<'arena>),
    ConstantKeyword(ConstantKeyword<'arena>),
    ExternalKeyword(ExternalKeyword<'arena>),
    InternalKeyword(InternalKeyword<'arena>),
    PayableKeyword(PayableKeyword<'arena>),
    PrivateKeyword(PrivateKeyword<'arena>),
    PublicKeyword(PublicKeyword<'arena>),
    PureKeyword(PureKeyword<'arena>),
    ViewKeyword(ViewKeyword<'arena>),
    VirtualKeyword(VirtualKeyword<'arena>),
}

pub fn new_function_attribute_modifier_invocation<'arena>(
    arena: &'arena Bump,
    element: ModifierInvocation<'arena>,
) -> FunctionAttribute<'arena> {
    FunctionAttribute::ModifierInvocation(element)
}

pub fn new_function_attribute_override_specifier<'arena>(
    arena: &'arena Bump,
    element: OverrideSpecifier<'arena>,
) -> FunctionAttribute<'arena> {
    FunctionAttribute::OverrideSpecifier(element)
}

pub fn new_function_attribute_constant_keyword<'arena>(
    arena: &'arena Bump,
    element: ConstantKeyword<'arena>,
) -> FunctionAttribute<'arena> {
    FunctionAttribute::ConstantKeyword(element)
}

pub fn new_function_attribute_external_keyword<'arena>(
    arena: &'arena Bump,
    element: ExternalKeyword<'arena>,
) -> FunctionAttribute<'arena> {
    FunctionAttribute::ExternalKeyword(element)
}

pub fn new_function_attribute_internal_keyword<'arena>(
    arena: &'arena Bump,
    element: InternalKeyword<'arena>,
) -> FunctionAttribute<'arena> {
    FunctionAttribute::InternalKeyword(element)
}

pub fn new_function_attribute_payable_keyword<'arena>(
    arena: &'arena Bump,
    element: PayableKeyword<'arena>,
) -> FunctionAttribute<'arena> {
    FunctionAttribute::PayableKeyword(element)
}

pub fn new_function_attribute_private_keyword<'arena>(
    arena: &'arena Bump,
    element: PrivateKeyword<'arena>,
) -> FunctionAttribute<'arena> {
    FunctionAttribute::PrivateKeyword(element)
}

pub fn new_function_attribute_public_keyword<'arena>(
    arena: &'arena Bump,
    element: PublicKeyword<'arena>,
) -> FunctionAttribute<'arena> {
    FunctionAttribute::PublicKeyword(element)
}

pub fn new_function_attribute_pure_keyword<'arena>(
    arena: &'arena Bump,
    element: PureKeyword<'arena>,
) -> FunctionAttribute<'arena> {
    FunctionAttribute::PureKeyword(element)
}

pub fn new_function_attribute_view_keyword<'arena>(
    arena: &'arena Bump,
    element: ViewKeyword<'arena>,
) -> FunctionAttribute<'arena> {
    FunctionAttribute::ViewKeyword(element)
}

pub fn new_function_attribute_virtual_keyword<'arena>(
    arena: &'arena Bump,
    element: VirtualKeyword<'arena>,
) -> FunctionAttribute<'arena> {
    FunctionAttribute::VirtualKeyword(element)
}

#[derive(Debug)]
pub enum FunctionBody<'arena> {
    Block(Block<'arena>),
    Semicolon(Semicolon<'arena>),
}

pub fn new_function_body_block<'arena>(
    arena: &'arena Bump,
    element: Block<'arena>,
) -> FunctionBody<'arena> {
    FunctionBody::Block(element)
}

pub fn new_function_body_semicolon<'arena>(
    arena: &'arena Bump,
    element: Semicolon<'arena>,
) -> FunctionBody<'arena> {
    FunctionBody::Semicolon(element)
}

#[derive(Debug)]
pub enum FunctionName<'arena> {
    Identifier(Identifier<'arena>),
    FallbackKeyword(FallbackKeyword<'arena>),
    ReceiveKeyword(ReceiveKeyword<'arena>),
}

pub fn new_function_name_identifier<'arena>(
    arena: &'arena Bump,
    element: Identifier<'arena>,
) -> FunctionName<'arena> {
    FunctionName::Identifier(element)
}

pub fn new_function_name_fallback_keyword<'arena>(
    arena: &'arena Bump,
    element: FallbackKeyword<'arena>,
) -> FunctionName<'arena> {
    FunctionName::FallbackKeyword(element)
}

pub fn new_function_name_receive_keyword<'arena>(
    arena: &'arena Bump,
    element: ReceiveKeyword<'arena>,
) -> FunctionName<'arena> {
    FunctionName::ReceiveKeyword(element)
}

#[derive(Debug)]
pub enum FunctionTypeAttribute<'arena> {
    InternalKeyword(InternalKeyword<'arena>),
    ExternalKeyword(ExternalKeyword<'arena>),
    PrivateKeyword(PrivateKeyword<'arena>),
    PublicKeyword(PublicKeyword<'arena>),
    ConstantKeyword(ConstantKeyword<'arena>),
    PureKeyword(PureKeyword<'arena>),
    ViewKeyword(ViewKeyword<'arena>),
    PayableKeyword(PayableKeyword<'arena>),
}

pub fn new_function_type_attribute_internal_keyword<'arena>(
    arena: &'arena Bump,
    element: InternalKeyword<'arena>,
) -> FunctionTypeAttribute<'arena> {
    FunctionTypeAttribute::InternalKeyword(element)
}

pub fn new_function_type_attribute_external_keyword<'arena>(
    arena: &'arena Bump,
    element: ExternalKeyword<'arena>,
) -> FunctionTypeAttribute<'arena> {
    FunctionTypeAttribute::ExternalKeyword(element)
}

pub fn new_function_type_attribute_private_keyword<'arena>(
    arena: &'arena Bump,
    element: PrivateKeyword<'arena>,
) -> FunctionTypeAttribute<'arena> {
    FunctionTypeAttribute::PrivateKeyword(element)
}

pub fn new_function_type_attribute_public_keyword<'arena>(
    arena: &'arena Bump,
    element: PublicKeyword<'arena>,
) -> FunctionTypeAttribute<'arena> {
    FunctionTypeAttribute::PublicKeyword(element)
}

pub fn new_function_type_attribute_constant_keyword<'arena>(
    arena: &'arena Bump,
    element: ConstantKeyword<'arena>,
) -> FunctionTypeAttribute<'arena> {
    FunctionTypeAttribute::ConstantKeyword(element)
}

pub fn new_function_type_attribute_pure_keyword<'arena>(
    arena: &'arena Bump,
    element: PureKeyword<'arena>,
) -> FunctionTypeAttribute<'arena> {
    FunctionTypeAttribute::PureKeyword(element)
}

pub fn new_function_type_attribute_view_keyword<'arena>(
    arena: &'arena Bump,
    element: ViewKeyword<'arena>,
) -> FunctionTypeAttribute<'arena> {
    FunctionTypeAttribute::ViewKeyword(element)
}

pub fn new_function_type_attribute_payable_keyword<'arena>(
    arena: &'arena Bump,
    element: PayableKeyword<'arena>,
) -> FunctionTypeAttribute<'arena> {
    FunctionTypeAttribute::PayableKeyword(element)
}

#[derive(Debug)]
pub enum HexStringLiteral<'arena> {
    SingleQuotedHexStringLiteral(SingleQuotedHexStringLiteral<'arena>),
    DoubleQuotedHexStringLiteral(DoubleQuotedHexStringLiteral<'arena>),
}

pub fn new_hex_string_literal_single_quoted_hex_string_literal<'arena>(
    arena: &'arena Bump,
    element: SingleQuotedHexStringLiteral<'arena>,
) -> HexStringLiteral<'arena> {
    HexStringLiteral::SingleQuotedHexStringLiteral(element)
}

pub fn new_hex_string_literal_double_quoted_hex_string_literal<'arena>(
    arena: &'arena Bump,
    element: DoubleQuotedHexStringLiteral<'arena>,
) -> HexStringLiteral<'arena> {
    HexStringLiteral::DoubleQuotedHexStringLiteral(element)
}

#[derive(Debug)]
pub enum ImportClause<'arena> {
    PathImport(PathImport<'arena>),
    NamedImport(NamedImport<'arena>),
    ImportDeconstruction(ImportDeconstruction<'arena>),
}

pub fn new_import_clause_path_import<'arena>(
    arena: &'arena Bump,
    element: PathImport<'arena>,
) -> ImportClause<'arena> {
    ImportClause::PathImport(element)
}

pub fn new_import_clause_named_import<'arena>(
    arena: &'arena Bump,
    element: NamedImport<'arena>,
) -> ImportClause<'arena> {
    ImportClause::NamedImport(element)
}

pub fn new_import_clause_import_deconstruction<'arena>(
    arena: &'arena Bump,
    element: ImportDeconstruction<'arena>,
) -> ImportClause<'arena> {
    ImportClause::ImportDeconstruction(element)
}

#[derive(Debug)]
pub enum MappingKeyType<'arena> {
    ElementaryType(ElementaryType<'arena>),
    IdentifierPath(IdentifierPath<'arena>),
}

pub fn new_mapping_key_type_elementary_type<'arena>(
    arena: &'arena Bump,
    element: ElementaryType<'arena>,
) -> MappingKeyType<'arena> {
    MappingKeyType::ElementaryType(element)
}

pub fn new_mapping_key_type_identifier_path<'arena>(
    arena: &'arena Bump,
    element: IdentifierPath<'arena>,
) -> MappingKeyType<'arena> {
    MappingKeyType::IdentifierPath(element)
}

#[derive(Debug)]
pub enum ModifierAttribute<'arena> {
    OverrideSpecifier(OverrideSpecifier<'arena>),
    VirtualKeyword(VirtualKeyword<'arena>),
}

pub fn new_modifier_attribute_override_specifier<'arena>(
    arena: &'arena Bump,
    element: OverrideSpecifier<'arena>,
) -> ModifierAttribute<'arena> {
    ModifierAttribute::OverrideSpecifier(element)
}

pub fn new_modifier_attribute_virtual_keyword<'arena>(
    arena: &'arena Bump,
    element: VirtualKeyword<'arena>,
) -> ModifierAttribute<'arena> {
    ModifierAttribute::VirtualKeyword(element)
}

#[derive(Debug)]
pub enum NumberUnit<'arena> {
    WeiKeyword(WeiKeyword<'arena>),
    GweiKeyword(GweiKeyword<'arena>),
    SzaboKeyword(SzaboKeyword<'arena>),
    FinneyKeyword(FinneyKeyword<'arena>),
    EtherKeyword(EtherKeyword<'arena>),
    SecondsKeyword(SecondsKeyword<'arena>),
    MinutesKeyword(MinutesKeyword<'arena>),
    HoursKeyword(HoursKeyword<'arena>),
    DaysKeyword(DaysKeyword<'arena>),
    WeeksKeyword(WeeksKeyword<'arena>),
    YearsKeyword(YearsKeyword<'arena>),
}

pub fn new_number_unit_wei_keyword<'arena>(
    arena: &'arena Bump,
    element: WeiKeyword<'arena>,
) -> NumberUnit<'arena> {
    NumberUnit::WeiKeyword(element)
}

pub fn new_number_unit_gwei_keyword<'arena>(
    arena: &'arena Bump,
    element: GweiKeyword<'arena>,
) -> NumberUnit<'arena> {
    NumberUnit::GweiKeyword(element)
}

pub fn new_number_unit_szabo_keyword<'arena>(
    arena: &'arena Bump,
    element: SzaboKeyword<'arena>,
) -> NumberUnit<'arena> {
    NumberUnit::SzaboKeyword(element)
}

pub fn new_number_unit_finney_keyword<'arena>(
    arena: &'arena Bump,
    element: FinneyKeyword<'arena>,
) -> NumberUnit<'arena> {
    NumberUnit::FinneyKeyword(element)
}

pub fn new_number_unit_ether_keyword<'arena>(
    arena: &'arena Bump,
    element: EtherKeyword<'arena>,
) -> NumberUnit<'arena> {
    NumberUnit::EtherKeyword(element)
}

pub fn new_number_unit_seconds_keyword<'arena>(
    arena: &'arena Bump,
    element: SecondsKeyword<'arena>,
) -> NumberUnit<'arena> {
    NumberUnit::SecondsKeyword(element)
}

pub fn new_number_unit_minutes_keyword<'arena>(
    arena: &'arena Bump,
    element: MinutesKeyword<'arena>,
) -> NumberUnit<'arena> {
    NumberUnit::MinutesKeyword(element)
}

pub fn new_number_unit_hours_keyword<'arena>(
    arena: &'arena Bump,
    element: HoursKeyword<'arena>,
) -> NumberUnit<'arena> {
    NumberUnit::HoursKeyword(element)
}

pub fn new_number_unit_days_keyword<'arena>(
    arena: &'arena Bump,
    element: DaysKeyword<'arena>,
) -> NumberUnit<'arena> {
    NumberUnit::DaysKeyword(element)
}

pub fn new_number_unit_weeks_keyword<'arena>(
    arena: &'arena Bump,
    element: WeeksKeyword<'arena>,
) -> NumberUnit<'arena> {
    NumberUnit::WeeksKeyword(element)
}

pub fn new_number_unit_years_keyword<'arena>(
    arena: &'arena Bump,
    element: YearsKeyword<'arena>,
) -> NumberUnit<'arena> {
    NumberUnit::YearsKeyword(element)
}

#[derive(Debug)]
pub enum Pragma<'arena> {
    VersionPragma(VersionPragma<'arena>),
    AbicoderPragma(AbicoderPragma<'arena>),
    ExperimentalPragma(ExperimentalPragma<'arena>),
}

pub fn new_pragma_version_pragma<'arena>(
    arena: &'arena Bump,
    element: VersionPragma<'arena>,
) -> Pragma<'arena> {
    Pragma::VersionPragma(element)
}

pub fn new_pragma_abicoder_pragma<'arena>(
    arena: &'arena Bump,
    element: AbicoderPragma<'arena>,
) -> Pragma<'arena> {
    Pragma::AbicoderPragma(element)
}

pub fn new_pragma_experimental_pragma<'arena>(
    arena: &'arena Bump,
    element: ExperimentalPragma<'arena>,
) -> Pragma<'arena> {
    Pragma::ExperimentalPragma(element)
}

#[derive(Debug)]
pub enum ReceiveFunctionAttribute<'arena> {
    ModifierInvocation(ModifierInvocation<'arena>),
    OverrideSpecifier(OverrideSpecifier<'arena>),
    ExternalKeyword(ExternalKeyword<'arena>),
    PayableKeyword(PayableKeyword<'arena>),
    VirtualKeyword(VirtualKeyword<'arena>),
}

pub fn new_receive_function_attribute_modifier_invocation<'arena>(
    arena: &'arena Bump,
    element: ModifierInvocation<'arena>,
) -> ReceiveFunctionAttribute<'arena> {
    ReceiveFunctionAttribute::ModifierInvocation(element)
}

pub fn new_receive_function_attribute_override_specifier<'arena>(
    arena: &'arena Bump,
    element: OverrideSpecifier<'arena>,
) -> ReceiveFunctionAttribute<'arena> {
    ReceiveFunctionAttribute::OverrideSpecifier(element)
}

pub fn new_receive_function_attribute_external_keyword<'arena>(
    arena: &'arena Bump,
    element: ExternalKeyword<'arena>,
) -> ReceiveFunctionAttribute<'arena> {
    ReceiveFunctionAttribute::ExternalKeyword(element)
}

pub fn new_receive_function_attribute_payable_keyword<'arena>(
    arena: &'arena Bump,
    element: PayableKeyword<'arena>,
) -> ReceiveFunctionAttribute<'arena> {
    ReceiveFunctionAttribute::PayableKeyword(element)
}

pub fn new_receive_function_attribute_virtual_keyword<'arena>(
    arena: &'arena Bump,
    element: VirtualKeyword<'arena>,
) -> ReceiveFunctionAttribute<'arena> {
    ReceiveFunctionAttribute::VirtualKeyword(element)
}

#[derive(Debug)]
pub enum SourceUnitMember<'arena> {
    PragmaDirective(PragmaDirective<'arena>),
    ImportDirective(ImportDirective<'arena>),
    ContractDefinition(ContractDefinition<'arena>),
    InterfaceDefinition(InterfaceDefinition<'arena>),
    LibraryDefinition(LibraryDefinition<'arena>),
    StructDefinition(StructDefinition<'arena>),
    EnumDefinition(EnumDefinition<'arena>),
    FunctionDefinition(FunctionDefinition<'arena>),
    ErrorDefinition(ErrorDefinition<'arena>),
    UserDefinedValueTypeDefinition(UserDefinedValueTypeDefinition<'arena>),
    UsingDirective(UsingDirective<'arena>),
    EventDefinition(EventDefinition<'arena>),
    ConstantDefinition(ConstantDefinition<'arena>),
}

pub fn new_source_unit_member_pragma_directive<'arena>(
    arena: &'arena Bump,
    element: PragmaDirective<'arena>,
) -> SourceUnitMember<'arena> {
    SourceUnitMember::PragmaDirective(element)
}

pub fn new_source_unit_member_import_directive<'arena>(
    arena: &'arena Bump,
    element: ImportDirective<'arena>,
) -> SourceUnitMember<'arena> {
    SourceUnitMember::ImportDirective(element)
}

pub fn new_source_unit_member_contract_definition<'arena>(
    arena: &'arena Bump,
    element: ContractDefinition<'arena>,
) -> SourceUnitMember<'arena> {
    SourceUnitMember::ContractDefinition(element)
}

pub fn new_source_unit_member_interface_definition<'arena>(
    arena: &'arena Bump,
    element: InterfaceDefinition<'arena>,
) -> SourceUnitMember<'arena> {
    SourceUnitMember::InterfaceDefinition(element)
}

pub fn new_source_unit_member_library_definition<'arena>(
    arena: &'arena Bump,
    element: LibraryDefinition<'arena>,
) -> SourceUnitMember<'arena> {
    SourceUnitMember::LibraryDefinition(element)
}

pub fn new_source_unit_member_struct_definition<'arena>(
    arena: &'arena Bump,
    element: StructDefinition<'arena>,
) -> SourceUnitMember<'arena> {
    SourceUnitMember::StructDefinition(element)
}

pub fn new_source_unit_member_enum_definition<'arena>(
    arena: &'arena Bump,
    element: EnumDefinition<'arena>,
) -> SourceUnitMember<'arena> {
    SourceUnitMember::EnumDefinition(element)
}

pub fn new_source_unit_member_function_definition<'arena>(
    arena: &'arena Bump,
    element: FunctionDefinition<'arena>,
) -> SourceUnitMember<'arena> {
    SourceUnitMember::FunctionDefinition(element)
}

pub fn new_source_unit_member_error_definition<'arena>(
    arena: &'arena Bump,
    element: ErrorDefinition<'arena>,
) -> SourceUnitMember<'arena> {
    SourceUnitMember::ErrorDefinition(element)
}

pub fn new_source_unit_member_user_defined_value_type_definition<'arena>(
    arena: &'arena Bump,
    element: UserDefinedValueTypeDefinition<'arena>,
) -> SourceUnitMember<'arena> {
    SourceUnitMember::UserDefinedValueTypeDefinition(element)
}

pub fn new_source_unit_member_using_directive<'arena>(
    arena: &'arena Bump,
    element: UsingDirective<'arena>,
) -> SourceUnitMember<'arena> {
    SourceUnitMember::UsingDirective(element)
}

pub fn new_source_unit_member_event_definition<'arena>(
    arena: &'arena Bump,
    element: EventDefinition<'arena>,
) -> SourceUnitMember<'arena> {
    SourceUnitMember::EventDefinition(element)
}

pub fn new_source_unit_member_constant_definition<'arena>(
    arena: &'arena Bump,
    element: ConstantDefinition<'arena>,
) -> SourceUnitMember<'arena> {
    SourceUnitMember::ConstantDefinition(element)
}

#[derive(Debug)]
pub enum StateVariableAttribute<'arena> {
    OverrideSpecifier(OverrideSpecifier<'arena>),
    ConstantKeyword(ConstantKeyword<'arena>),
    InternalKeyword(InternalKeyword<'arena>),
    PrivateKeyword(PrivateKeyword<'arena>),
    PublicKeyword(PublicKeyword<'arena>),
    ImmutableKeyword(ImmutableKeyword<'arena>),
    TransientKeyword(TransientKeyword<'arena>),
}

pub fn new_state_variable_attribute_override_specifier<'arena>(
    arena: &'arena Bump,
    element: OverrideSpecifier<'arena>,
) -> StateVariableAttribute<'arena> {
    StateVariableAttribute::OverrideSpecifier(element)
}

pub fn new_state_variable_attribute_constant_keyword<'arena>(
    arena: &'arena Bump,
    element: ConstantKeyword<'arena>,
) -> StateVariableAttribute<'arena> {
    StateVariableAttribute::ConstantKeyword(element)
}

pub fn new_state_variable_attribute_internal_keyword<'arena>(
    arena: &'arena Bump,
    element: InternalKeyword<'arena>,
) -> StateVariableAttribute<'arena> {
    StateVariableAttribute::InternalKeyword(element)
}

pub fn new_state_variable_attribute_private_keyword<'arena>(
    arena: &'arena Bump,
    element: PrivateKeyword<'arena>,
) -> StateVariableAttribute<'arena> {
    StateVariableAttribute::PrivateKeyword(element)
}

pub fn new_state_variable_attribute_public_keyword<'arena>(
    arena: &'arena Bump,
    element: PublicKeyword<'arena>,
) -> StateVariableAttribute<'arena> {
    StateVariableAttribute::PublicKeyword(element)
}

pub fn new_state_variable_attribute_immutable_keyword<'arena>(
    arena: &'arena Bump,
    element: ImmutableKeyword<'arena>,
) -> StateVariableAttribute<'arena> {
    StateVariableAttribute::ImmutableKeyword(element)
}

pub fn new_state_variable_attribute_transient_keyword<'arena>(
    arena: &'arena Bump,
    element: TransientKeyword<'arena>,
) -> StateVariableAttribute<'arena> {
    StateVariableAttribute::TransientKeyword(element)
}

#[derive(Debug)]
pub enum Statement<'arena> {
    IfStatement(IfStatement<'arena>),
    ForStatement(ForStatement<'arena>),
    WhileStatement(WhileStatement<'arena>),
    DoWhileStatement(DoWhileStatement<'arena>),
    ContinueStatement(ContinueStatement<'arena>),
    BreakStatement(BreakStatement<'arena>),
    ReturnStatement(ReturnStatement<'arena>),
    ThrowStatement(ThrowStatement<'arena>),
    EmitStatement(EmitStatement<'arena>),
    TryStatement(TryStatement<'arena>),
    RevertStatement(RevertStatement<'arena>),
    AssemblyStatement(AssemblyStatement<'arena>),
    Block(Block<'arena>),
    UncheckedBlock(UncheckedBlock<'arena>),
    TupleDeconstructionStatement(TupleDeconstructionStatement<'arena>),
    VariableDeclarationStatement(VariableDeclarationStatement<'arena>),
    ExpressionStatement(ExpressionStatement<'arena>),
}

pub fn new_statement_if_statement<'arena>(
    arena: &'arena Bump,
    element: IfStatement<'arena>,
) -> Statement<'arena> {
    Statement::IfStatement(element)
}

pub fn new_statement_for_statement<'arena>(
    arena: &'arena Bump,
    element: ForStatement<'arena>,
) -> Statement<'arena> {
    Statement::ForStatement(element)
}

pub fn new_statement_while_statement<'arena>(
    arena: &'arena Bump,
    element: WhileStatement<'arena>,
) -> Statement<'arena> {
    Statement::WhileStatement(element)
}

pub fn new_statement_do_while_statement<'arena>(
    arena: &'arena Bump,
    element: DoWhileStatement<'arena>,
) -> Statement<'arena> {
    Statement::DoWhileStatement(element)
}

pub fn new_statement_continue_statement<'arena>(
    arena: &'arena Bump,
    element: ContinueStatement<'arena>,
) -> Statement<'arena> {
    Statement::ContinueStatement(element)
}

pub fn new_statement_break_statement<'arena>(
    arena: &'arena Bump,
    element: BreakStatement<'arena>,
) -> Statement<'arena> {
    Statement::BreakStatement(element)
}

pub fn new_statement_return_statement<'arena>(
    arena: &'arena Bump,
    element: ReturnStatement<'arena>,
) -> Statement<'arena> {
    Statement::ReturnStatement(element)
}

pub fn new_statement_throw_statement<'arena>(
    arena: &'arena Bump,
    element: ThrowStatement<'arena>,
) -> Statement<'arena> {
    Statement::ThrowStatement(element)
}

pub fn new_statement_emit_statement<'arena>(
    arena: &'arena Bump,
    element: EmitStatement<'arena>,
) -> Statement<'arena> {
    Statement::EmitStatement(element)
}

pub fn new_statement_try_statement<'arena>(
    arena: &'arena Bump,
    element: TryStatement<'arena>,
) -> Statement<'arena> {
    Statement::TryStatement(element)
}

pub fn new_statement_revert_statement<'arena>(
    arena: &'arena Bump,
    element: RevertStatement<'arena>,
) -> Statement<'arena> {
    Statement::RevertStatement(element)
}

pub fn new_statement_assembly_statement<'arena>(
    arena: &'arena Bump,
    element: AssemblyStatement<'arena>,
) -> Statement<'arena> {
    Statement::AssemblyStatement(element)
}

pub fn new_statement_block<'arena>(
    arena: &'arena Bump,
    element: Block<'arena>,
) -> Statement<'arena> {
    Statement::Block(element)
}

pub fn new_statement_unchecked_block<'arena>(
    arena: &'arena Bump,
    element: UncheckedBlock<'arena>,
) -> Statement<'arena> {
    Statement::UncheckedBlock(element)
}

pub fn new_statement_tuple_deconstruction_statement<'arena>(
    arena: &'arena Bump,
    element: TupleDeconstructionStatement<'arena>,
) -> Statement<'arena> {
    Statement::TupleDeconstructionStatement(element)
}

pub fn new_statement_variable_declaration_statement<'arena>(
    arena: &'arena Bump,
    element: VariableDeclarationStatement<'arena>,
) -> Statement<'arena> {
    Statement::VariableDeclarationStatement(element)
}

pub fn new_statement_expression_statement<'arena>(
    arena: &'arena Bump,
    element: ExpressionStatement<'arena>,
) -> Statement<'arena> {
    Statement::ExpressionStatement(element)
}

#[derive(Debug)]
pub enum StorageLocation<'arena> {
    MemoryKeyword(MemoryKeyword<'arena>),
    StorageKeyword(StorageKeyword<'arena>),
    CallDataKeyword(CallDataKeyword<'arena>),
}

pub fn new_storage_location_memory_keyword<'arena>(
    arena: &'arena Bump,
    element: MemoryKeyword<'arena>,
) -> StorageLocation<'arena> {
    StorageLocation::MemoryKeyword(element)
}

pub fn new_storage_location_storage_keyword<'arena>(
    arena: &'arena Bump,
    element: StorageKeyword<'arena>,
) -> StorageLocation<'arena> {
    StorageLocation::StorageKeyword(element)
}

pub fn new_storage_location_call_data_keyword<'arena>(
    arena: &'arena Bump,
    element: CallDataKeyword<'arena>,
) -> StorageLocation<'arena> {
    StorageLocation::CallDataKeyword(element)
}

#[derive(Debug)]
pub enum StringExpression<'arena> {
    StringLiteral(StringLiteral<'arena>),
    StringLiterals(StringLiterals<'arena>),
    HexStringLiteral(HexStringLiteral<'arena>),
    HexStringLiterals(HexStringLiterals<'arena>),
    UnicodeStringLiterals(UnicodeStringLiterals<'arena>),
}

pub fn new_string_expression_string_literal<'arena>(
    arena: &'arena Bump,
    element: StringLiteral<'arena>,
) -> StringExpression<'arena> {
    StringExpression::StringLiteral(element)
}

pub fn new_string_expression_string_literals<'arena>(
    arena: &'arena Bump,
    element: StringLiterals<'arena>,
) -> StringExpression<'arena> {
    StringExpression::StringLiterals(element)
}

pub fn new_string_expression_hex_string_literal<'arena>(
    arena: &'arena Bump,
    element: HexStringLiteral<'arena>,
) -> StringExpression<'arena> {
    StringExpression::HexStringLiteral(element)
}

pub fn new_string_expression_hex_string_literals<'arena>(
    arena: &'arena Bump,
    element: HexStringLiterals<'arena>,
) -> StringExpression<'arena> {
    StringExpression::HexStringLiterals(element)
}

pub fn new_string_expression_unicode_string_literals<'arena>(
    arena: &'arena Bump,
    element: UnicodeStringLiterals<'arena>,
) -> StringExpression<'arena> {
    StringExpression::UnicodeStringLiterals(element)
}

#[derive(Debug)]
pub enum StringLiteral<'arena> {
    SingleQuotedStringLiteral(SingleQuotedStringLiteral<'arena>),
    DoubleQuotedStringLiteral(DoubleQuotedStringLiteral<'arena>),
}

pub fn new_string_literal_single_quoted_string_literal<'arena>(
    arena: &'arena Bump,
    element: SingleQuotedStringLiteral<'arena>,
) -> StringLiteral<'arena> {
    StringLiteral::SingleQuotedStringLiteral(element)
}

pub fn new_string_literal_double_quoted_string_literal<'arena>(
    arena: &'arena Bump,
    element: DoubleQuotedStringLiteral<'arena>,
) -> StringLiteral<'arena> {
    StringLiteral::DoubleQuotedStringLiteral(element)
}

#[derive(Debug)]
pub enum TupleMember<'arena> {
    TypedTupleMember(TypedTupleMember<'arena>),
    UntypedTupleMember(UntypedTupleMember<'arena>),
}

pub fn new_tuple_member_typed_tuple_member<'arena>(
    arena: &'arena Bump,
    element: TypedTupleMember<'arena>,
) -> TupleMember<'arena> {
    TupleMember::TypedTupleMember(element)
}

pub fn new_tuple_member_untyped_tuple_member<'arena>(
    arena: &'arena Bump,
    element: UntypedTupleMember<'arena>,
) -> TupleMember<'arena> {
    TupleMember::UntypedTupleMember(element)
}

#[derive(Debug)]
pub enum TypeName<'arena> {
    ArrayTypeName(ArrayTypeName<'arena>),
    FunctionType(FunctionType<'arena>),
    MappingType(MappingType<'arena>),
    ElementaryType(ElementaryType<'arena>),
    IdentifierPath(IdentifierPath<'arena>),
}

pub fn new_type_name_array_type_name<'arena>(
    arena: &'arena Bump,
    element: ArrayTypeName<'arena>,
) -> TypeName<'arena> {
    TypeName::ArrayTypeName(element)
}

pub fn new_type_name_function_type<'arena>(
    arena: &'arena Bump,
    element: FunctionType<'arena>,
) -> TypeName<'arena> {
    TypeName::FunctionType(element)
}

pub fn new_type_name_mapping_type<'arena>(
    arena: &'arena Bump,
    element: MappingType<'arena>,
) -> TypeName<'arena> {
    TypeName::MappingType(element)
}

pub fn new_type_name_elementary_type<'arena>(
    arena: &'arena Bump,
    element: ElementaryType<'arena>,
) -> TypeName<'arena> {
    TypeName::ElementaryType(element)
}

pub fn new_type_name_identifier_path<'arena>(
    arena: &'arena Bump,
    element: IdentifierPath<'arena>,
) -> TypeName<'arena> {
    TypeName::IdentifierPath(element)
}

#[derive(Debug)]
pub enum UnicodeStringLiteral<'arena> {
    SingleQuotedUnicodeStringLiteral(SingleQuotedUnicodeStringLiteral<'arena>),
    DoubleQuotedUnicodeStringLiteral(DoubleQuotedUnicodeStringLiteral<'arena>),
}

pub fn new_unicode_string_literal_single_quoted_unicode_string_literal<'arena>(
    arena: &'arena Bump,
    element: SingleQuotedUnicodeStringLiteral<'arena>,
) -> UnicodeStringLiteral<'arena> {
    UnicodeStringLiteral::SingleQuotedUnicodeStringLiteral(element)
}

pub fn new_unicode_string_literal_double_quoted_unicode_string_literal<'arena>(
    arena: &'arena Bump,
    element: DoubleQuotedUnicodeStringLiteral<'arena>,
) -> UnicodeStringLiteral<'arena> {
    UnicodeStringLiteral::DoubleQuotedUnicodeStringLiteral(element)
}

#[derive(Debug)]
pub enum UnnamedFunctionAttribute<'arena> {
    ModifierInvocation(ModifierInvocation<'arena>),
    ConstantKeyword(ConstantKeyword<'arena>),
    ExternalKeyword(ExternalKeyword<'arena>),
    InternalKeyword(InternalKeyword<'arena>),
    PayableKeyword(PayableKeyword<'arena>),
    PrivateKeyword(PrivateKeyword<'arena>),
    PublicKeyword(PublicKeyword<'arena>),
    PureKeyword(PureKeyword<'arena>),
    ViewKeyword(ViewKeyword<'arena>),
}

pub fn new_unnamed_function_attribute_modifier_invocation<'arena>(
    arena: &'arena Bump,
    element: ModifierInvocation<'arena>,
) -> UnnamedFunctionAttribute<'arena> {
    UnnamedFunctionAttribute::ModifierInvocation(element)
}

pub fn new_unnamed_function_attribute_constant_keyword<'arena>(
    arena: &'arena Bump,
    element: ConstantKeyword<'arena>,
) -> UnnamedFunctionAttribute<'arena> {
    UnnamedFunctionAttribute::ConstantKeyword(element)
}

pub fn new_unnamed_function_attribute_external_keyword<'arena>(
    arena: &'arena Bump,
    element: ExternalKeyword<'arena>,
) -> UnnamedFunctionAttribute<'arena> {
    UnnamedFunctionAttribute::ExternalKeyword(element)
}

pub fn new_unnamed_function_attribute_internal_keyword<'arena>(
    arena: &'arena Bump,
    element: InternalKeyword<'arena>,
) -> UnnamedFunctionAttribute<'arena> {
    UnnamedFunctionAttribute::InternalKeyword(element)
}

pub fn new_unnamed_function_attribute_payable_keyword<'arena>(
    arena: &'arena Bump,
    element: PayableKeyword<'arena>,
) -> UnnamedFunctionAttribute<'arena> {
    UnnamedFunctionAttribute::PayableKeyword(element)
}

pub fn new_unnamed_function_attribute_private_keyword<'arena>(
    arena: &'arena Bump,
    element: PrivateKeyword<'arena>,
) -> UnnamedFunctionAttribute<'arena> {
    UnnamedFunctionAttribute::PrivateKeyword(element)
}

pub fn new_unnamed_function_attribute_public_keyword<'arena>(
    arena: &'arena Bump,
    element: PublicKeyword<'arena>,
) -> UnnamedFunctionAttribute<'arena> {
    UnnamedFunctionAttribute::PublicKeyword(element)
}

pub fn new_unnamed_function_attribute_pure_keyword<'arena>(
    arena: &'arena Bump,
    element: PureKeyword<'arena>,
) -> UnnamedFunctionAttribute<'arena> {
    UnnamedFunctionAttribute::PureKeyword(element)
}

pub fn new_unnamed_function_attribute_view_keyword<'arena>(
    arena: &'arena Bump,
    element: ViewKeyword<'arena>,
) -> UnnamedFunctionAttribute<'arena> {
    UnnamedFunctionAttribute::ViewKeyword(element)
}

#[derive(Debug)]
pub enum UsingClause<'arena> {
    IdentifierPath(IdentifierPath<'arena>),
    UsingDeconstruction(UsingDeconstruction<'arena>),
}

pub fn new_using_clause_identifier_path<'arena>(
    arena: &'arena Bump,
    element: IdentifierPath<'arena>,
) -> UsingClause<'arena> {
    UsingClause::IdentifierPath(element)
}

pub fn new_using_clause_using_deconstruction<'arena>(
    arena: &'arena Bump,
    element: UsingDeconstruction<'arena>,
) -> UsingClause<'arena> {
    UsingClause::UsingDeconstruction(element)
}

#[derive(Debug)]
pub enum UsingOperator<'arena> {
    Ampersand(Ampersand<'arena>),
    Asterisk(Asterisk<'arena>),
    BangEqual(BangEqual<'arena>),
    Bar(Bar<'arena>),
    Caret(Caret<'arena>),
    EqualEqual(EqualEqual<'arena>),
    GreaterThan(GreaterThan<'arena>),
    GreaterThanEqual(GreaterThanEqual<'arena>),
    LessThan(LessThan<'arena>),
    LessThanEqual(LessThanEqual<'arena>),
    Minus(Minus<'arena>),
    Percent(Percent<'arena>),
    Plus(Plus<'arena>),
    Slash(Slash<'arena>),
    Tilde(Tilde<'arena>),
}

pub fn new_using_operator_ampersand<'arena>(
    arena: &'arena Bump,
    element: Ampersand<'arena>,
) -> UsingOperator<'arena> {
    UsingOperator::Ampersand(element)
}

pub fn new_using_operator_asterisk<'arena>(
    arena: &'arena Bump,
    element: Asterisk<'arena>,
) -> UsingOperator<'arena> {
    UsingOperator::Asterisk(element)
}

pub fn new_using_operator_bang_equal<'arena>(
    arena: &'arena Bump,
    element: BangEqual<'arena>,
) -> UsingOperator<'arena> {
    UsingOperator::BangEqual(element)
}

pub fn new_using_operator_bar<'arena>(
    arena: &'arena Bump,
    element: Bar<'arena>,
) -> UsingOperator<'arena> {
    UsingOperator::Bar(element)
}

pub fn new_using_operator_caret<'arena>(
    arena: &'arena Bump,
    element: Caret<'arena>,
) -> UsingOperator<'arena> {
    UsingOperator::Caret(element)
}

pub fn new_using_operator_equal_equal<'arena>(
    arena: &'arena Bump,
    element: EqualEqual<'arena>,
) -> UsingOperator<'arena> {
    UsingOperator::EqualEqual(element)
}

pub fn new_using_operator_greater_than<'arena>(
    arena: &'arena Bump,
    element: GreaterThan<'arena>,
) -> UsingOperator<'arena> {
    UsingOperator::GreaterThan(element)
}

pub fn new_using_operator_greater_than_equal<'arena>(
    arena: &'arena Bump,
    element: GreaterThanEqual<'arena>,
) -> UsingOperator<'arena> {
    UsingOperator::GreaterThanEqual(element)
}

pub fn new_using_operator_less_than<'arena>(
    arena: &'arena Bump,
    element: LessThan<'arena>,
) -> UsingOperator<'arena> {
    UsingOperator::LessThan(element)
}

pub fn new_using_operator_less_than_equal<'arena>(
    arena: &'arena Bump,
    element: LessThanEqual<'arena>,
) -> UsingOperator<'arena> {
    UsingOperator::LessThanEqual(element)
}

pub fn new_using_operator_minus<'arena>(
    arena: &'arena Bump,
    element: Minus<'arena>,
) -> UsingOperator<'arena> {
    UsingOperator::Minus(element)
}

pub fn new_using_operator_percent<'arena>(
    arena: &'arena Bump,
    element: Percent<'arena>,
) -> UsingOperator<'arena> {
    UsingOperator::Percent(element)
}

pub fn new_using_operator_plus<'arena>(
    arena: &'arena Bump,
    element: Plus<'arena>,
) -> UsingOperator<'arena> {
    UsingOperator::Plus(element)
}

pub fn new_using_operator_slash<'arena>(
    arena: &'arena Bump,
    element: Slash<'arena>,
) -> UsingOperator<'arena> {
    UsingOperator::Slash(element)
}

pub fn new_using_operator_tilde<'arena>(
    arena: &'arena Bump,
    element: Tilde<'arena>,
) -> UsingOperator<'arena> {
    UsingOperator::Tilde(element)
}

#[derive(Debug)]
pub enum UsingTarget<'arena> {
    TypeName(TypeName<'arena>),
    Asterisk(Asterisk<'arena>),
}

pub fn new_using_target_type_name<'arena>(
    arena: &'arena Bump,
    element: TypeName<'arena>,
) -> UsingTarget<'arena> {
    UsingTarget::TypeName(element)
}

pub fn new_using_target_asterisk<'arena>(
    arena: &'arena Bump,
    element: Asterisk<'arena>,
) -> UsingTarget<'arena> {
    UsingTarget::Asterisk(element)
}

#[derive(Debug)]
pub enum VariableDeclarationType<'arena> {
    TypeName(TypeName<'arena>),
    VarKeyword(VarKeyword<'arena>),
}

pub fn new_variable_declaration_type_type_name<'arena>(
    arena: &'arena Bump,
    element: TypeName<'arena>,
) -> VariableDeclarationType<'arena> {
    VariableDeclarationType::TypeName(element)
}

pub fn new_variable_declaration_type_var_keyword<'arena>(
    arena: &'arena Bump,
    element: VarKeyword<'arena>,
) -> VariableDeclarationType<'arena> {
    VariableDeclarationType::VarKeyword(element)
}

#[derive(Debug)]
pub enum VersionExpression<'arena> {
    VersionRange(VersionRange<'arena>),
    VersionTerm(VersionTerm<'arena>),
}

pub fn new_version_expression_version_range<'arena>(
    arena: &'arena Bump,
    element: VersionRange<'arena>,
) -> VersionExpression<'arena> {
    VersionExpression::VersionRange(element)
}

pub fn new_version_expression_version_term<'arena>(
    arena: &'arena Bump,
    element: VersionTerm<'arena>,
) -> VersionExpression<'arena> {
    VersionExpression::VersionTerm(element)
}

#[derive(Debug)]
pub enum VersionLiteral<'arena> {
    SimpleVersionLiteral(SimpleVersionLiteral<'arena>),
    SingleQuotedVersionLiteral(SingleQuotedVersionLiteral<'arena>),
    DoubleQuotedVersionLiteral(DoubleQuotedVersionLiteral<'arena>),
}

pub fn new_version_literal_simple_version_literal<'arena>(
    arena: &'arena Bump,
    element: SimpleVersionLiteral<'arena>,
) -> VersionLiteral<'arena> {
    VersionLiteral::SimpleVersionLiteral(element)
}

pub fn new_version_literal_single_quoted_version_literal<'arena>(
    arena: &'arena Bump,
    element: SingleQuotedVersionLiteral<'arena>,
) -> VersionLiteral<'arena> {
    VersionLiteral::SingleQuotedVersionLiteral(element)
}

pub fn new_version_literal_double_quoted_version_literal<'arena>(
    arena: &'arena Bump,
    element: DoubleQuotedVersionLiteral<'arena>,
) -> VersionLiteral<'arena> {
    VersionLiteral::DoubleQuotedVersionLiteral(element)
}

#[derive(Debug)]
pub enum VersionOperator<'arena> {
    Caret(Caret<'arena>),
    Tilde(Tilde<'arena>),
    Equal(Equal<'arena>),
    LessThan(LessThan<'arena>),
    GreaterThan(GreaterThan<'arena>),
    LessThanEqual(LessThanEqual<'arena>),
    GreaterThanEqual(GreaterThanEqual<'arena>),
}

pub fn new_version_operator_caret<'arena>(
    arena: &'arena Bump,
    element: Caret<'arena>,
) -> VersionOperator<'arena> {
    VersionOperator::Caret(element)
}

pub fn new_version_operator_tilde<'arena>(
    arena: &'arena Bump,
    element: Tilde<'arena>,
) -> VersionOperator<'arena> {
    VersionOperator::Tilde(element)
}

pub fn new_version_operator_equal<'arena>(
    arena: &'arena Bump,
    element: Equal<'arena>,
) -> VersionOperator<'arena> {
    VersionOperator::Equal(element)
}

pub fn new_version_operator_less_than<'arena>(
    arena: &'arena Bump,
    element: LessThan<'arena>,
) -> VersionOperator<'arena> {
    VersionOperator::LessThan(element)
}

pub fn new_version_operator_greater_than<'arena>(
    arena: &'arena Bump,
    element: GreaterThan<'arena>,
) -> VersionOperator<'arena> {
    VersionOperator::GreaterThan(element)
}

pub fn new_version_operator_less_than_equal<'arena>(
    arena: &'arena Bump,
    element: LessThanEqual<'arena>,
) -> VersionOperator<'arena> {
    VersionOperator::LessThanEqual(element)
}

pub fn new_version_operator_greater_than_equal<'arena>(
    arena: &'arena Bump,
    element: GreaterThanEqual<'arena>,
) -> VersionOperator<'arena> {
    VersionOperator::GreaterThanEqual(element)
}

#[derive(Debug)]
pub enum YulAssignmentOperator<'arena> {
    ColonEqual(ColonEqual<'arena>),
    YulColonAndEqual(YulColonAndEqual<'arena>),
}

pub fn new_yul_assignment_operator_colon_equal<'arena>(
    arena: &'arena Bump,
    element: ColonEqual<'arena>,
) -> YulAssignmentOperator<'arena> {
    YulAssignmentOperator::ColonEqual(element)
}

pub fn new_yul_assignment_operator_yul_colon_and_equal<'arena>(
    arena: &'arena Bump,
    element: YulColonAndEqual<'arena>,
) -> YulAssignmentOperator<'arena> {
    YulAssignmentOperator::YulColonAndEqual(element)
}

#[derive(Debug)]
pub enum YulExpression<'arena> {
    YulFunctionCallExpression(YulFunctionCallExpression<'arena>),
    YulLiteral(YulLiteral<'arena>),
    YulPath(YulPath<'arena>),
}

pub fn new_yul_expression_yul_function_call_expression<'arena>(
    arena: &'arena Bump,
    element: YulFunctionCallExpression<'arena>,
) -> YulExpression<'arena> {
    YulExpression::YulFunctionCallExpression(element)
}

pub fn new_yul_expression_yul_literal<'arena>(
    arena: &'arena Bump,
    element: YulLiteral<'arena>,
) -> YulExpression<'arena> {
    YulExpression::YulLiteral(element)
}

pub fn new_yul_expression_yul_path<'arena>(
    arena: &'arena Bump,
    element: YulPath<'arena>,
) -> YulExpression<'arena> {
    YulExpression::YulPath(element)
}

#[derive(Debug)]
pub enum YulLiteral<'arena> {
    YulTrueKeyword(YulTrueKeyword<'arena>),
    YulFalseKeyword(YulFalseKeyword<'arena>),
    YulDecimalLiteral(YulDecimalLiteral<'arena>),
    YulHexLiteral(YulHexLiteral<'arena>),
    HexStringLiteral(HexStringLiteral<'arena>),
    StringLiteral(StringLiteral<'arena>),
}

pub fn new_yul_literal_yul_true_keyword<'arena>(
    arena: &'arena Bump,
    element: YulTrueKeyword<'arena>,
) -> YulLiteral<'arena> {
    YulLiteral::YulTrueKeyword(element)
}

pub fn new_yul_literal_yul_false_keyword<'arena>(
    arena: &'arena Bump,
    element: YulFalseKeyword<'arena>,
) -> YulLiteral<'arena> {
    YulLiteral::YulFalseKeyword(element)
}

pub fn new_yul_literal_yul_decimal_literal<'arena>(
    arena: &'arena Bump,
    element: YulDecimalLiteral<'arena>,
) -> YulLiteral<'arena> {
    YulLiteral::YulDecimalLiteral(element)
}

pub fn new_yul_literal_yul_hex_literal<'arena>(
    arena: &'arena Bump,
    element: YulHexLiteral<'arena>,
) -> YulLiteral<'arena> {
    YulLiteral::YulHexLiteral(element)
}

pub fn new_yul_literal_hex_string_literal<'arena>(
    arena: &'arena Bump,
    element: HexStringLiteral<'arena>,
) -> YulLiteral<'arena> {
    YulLiteral::HexStringLiteral(element)
}

pub fn new_yul_literal_string_literal<'arena>(
    arena: &'arena Bump,
    element: StringLiteral<'arena>,
) -> YulLiteral<'arena> {
    YulLiteral::StringLiteral(element)
}

#[derive(Debug)]
pub enum YulStackAssignmentOperator<'arena> {
    EqualColon(EqualColon<'arena>),
    YulEqualAndColon(YulEqualAndColon<'arena>),
}

pub fn new_yul_stack_assignment_operator_equal_colon<'arena>(
    arena: &'arena Bump,
    element: EqualColon<'arena>,
) -> YulStackAssignmentOperator<'arena> {
    YulStackAssignmentOperator::EqualColon(element)
}

pub fn new_yul_stack_assignment_operator_yul_equal_and_colon<'arena>(
    arena: &'arena Bump,
    element: YulEqualAndColon<'arena>,
) -> YulStackAssignmentOperator<'arena> {
    YulStackAssignmentOperator::YulEqualAndColon(element)
}

#[derive(Debug)]
pub enum YulStatement<'arena> {
    YulBlock(YulBlock<'arena>),
    YulFunctionDefinition(YulFunctionDefinition<'arena>),
    YulStackAssignmentStatement(YulStackAssignmentStatement<'arena>),
    YulIfStatement(YulIfStatement<'arena>),
    YulForStatement(YulForStatement<'arena>),
    YulSwitchStatement(YulSwitchStatement<'arena>),
    YulLeaveStatement(YulLeaveStatement<'arena>),
    YulBreakStatement(YulBreakStatement<'arena>),
    YulContinueStatement(YulContinueStatement<'arena>),
    YulVariableAssignmentStatement(YulVariableAssignmentStatement<'arena>),
    YulLabel(YulLabel<'arena>),
    YulVariableDeclarationStatement(YulVariableDeclarationStatement<'arena>),
    YulExpression(YulExpression<'arena>),
}

pub fn new_yul_statement_yul_block<'arena>(
    arena: &'arena Bump,
    element: YulBlock<'arena>,
) -> YulStatement<'arena> {
    YulStatement::YulBlock(element)
}

pub fn new_yul_statement_yul_function_definition<'arena>(
    arena: &'arena Bump,
    element: YulFunctionDefinition<'arena>,
) -> YulStatement<'arena> {
    YulStatement::YulFunctionDefinition(element)
}

pub fn new_yul_statement_yul_stack_assignment_statement<'arena>(
    arena: &'arena Bump,
    element: YulStackAssignmentStatement<'arena>,
) -> YulStatement<'arena> {
    YulStatement::YulStackAssignmentStatement(element)
}

pub fn new_yul_statement_yul_if_statement<'arena>(
    arena: &'arena Bump,
    element: YulIfStatement<'arena>,
) -> YulStatement<'arena> {
    YulStatement::YulIfStatement(element)
}

pub fn new_yul_statement_yul_for_statement<'arena>(
    arena: &'arena Bump,
    element: YulForStatement<'arena>,
) -> YulStatement<'arena> {
    YulStatement::YulForStatement(element)
}

pub fn new_yul_statement_yul_switch_statement<'arena>(
    arena: &'arena Bump,
    element: YulSwitchStatement<'arena>,
) -> YulStatement<'arena> {
    YulStatement::YulSwitchStatement(element)
}

pub fn new_yul_statement_yul_leave_statement<'arena>(
    arena: &'arena Bump,
    element: YulLeaveStatement<'arena>,
) -> YulStatement<'arena> {
    YulStatement::YulLeaveStatement(element)
}

pub fn new_yul_statement_yul_break_statement<'arena>(
    arena: &'arena Bump,
    element: YulBreakStatement<'arena>,
) -> YulStatement<'arena> {
    YulStatement::YulBreakStatement(element)
}

pub fn new_yul_statement_yul_continue_statement<'arena>(
    arena: &'arena Bump,
    element: YulContinueStatement<'arena>,
) -> YulStatement<'arena> {
    YulStatement::YulContinueStatement(element)
}

pub fn new_yul_statement_yul_variable_assignment_statement<'arena>(
    arena: &'arena Bump,
    element: YulVariableAssignmentStatement<'arena>,
) -> YulStatement<'arena> {
    YulStatement::YulVariableAssignmentStatement(element)
}

pub fn new_yul_statement_yul_label<'arena>(
    arena: &'arena Bump,
    element: YulLabel<'arena>,
) -> YulStatement<'arena> {
    YulStatement::YulLabel(element)
}

pub fn new_yul_statement_yul_variable_declaration_statement<'arena>(
    arena: &'arena Bump,
    element: YulVariableDeclarationStatement<'arena>,
) -> YulStatement<'arena> {
    YulStatement::YulVariableDeclarationStatement(element)
}

pub fn new_yul_statement_yul_expression<'arena>(
    arena: &'arena Bump,
    element: YulExpression<'arena>,
) -> YulStatement<'arena> {
    YulStatement::YulExpression(element)
}

#[derive(Debug)]
pub enum YulSwitchCase<'arena> {
    YulDefaultCase(YulDefaultCase<'arena>),
    YulValueCase(YulValueCase<'arena>),
}

pub fn new_yul_switch_case_yul_default_case<'arena>(
    arena: &'arena Bump,
    element: YulDefaultCase<'arena>,
) -> YulSwitchCase<'arena> {
    YulSwitchCase::YulDefaultCase(element)
}

pub fn new_yul_switch_case_yul_value_case<'arena>(
    arena: &'arena Bump,
    element: YulValueCase<'arena>,
) -> YulSwitchCase<'arena> {
    YulSwitchCase::YulValueCase(element)
}

//
// Repeated & Separated
//
// TODO(v2): consider using a transparent representation

#[derive(Debug)]
pub struct ArrayValues<'arena> {
    pub elements: Vec<'arena, Expression<'arena>>,
}

pub fn new_array_values<'arena>(
    arena: &'arena Bump,
    elements: Vec<'arena, Expression<'arena>>,
) -> ArrayValues<'arena> {
    ArrayValues { elements }
}

#[derive(Debug)]
pub struct AssemblyFlags<'arena> {
    pub elements: Vec<'arena, StringLiteral<'arena>>,
}

pub fn new_assembly_flags<'arena>(
    arena: &'arena Bump,
    elements: Vec<'arena, StringLiteral<'arena>>,
) -> AssemblyFlags<'arena> {
    AssemblyFlags { elements }
}

#[derive(Debug)]
pub struct CallOptions<'arena> {
    pub elements: Vec<'arena, NamedArgument<'arena>>,
}

pub fn new_call_options<'arena>(
    arena: &'arena Bump,
    elements: Vec<'arena, NamedArgument<'arena>>,
) -> CallOptions<'arena> {
    CallOptions { elements }
}

#[derive(Debug)]
pub struct CatchClauses<'arena> {
    pub elements: Vec<'arena, CatchClause<'arena>>,
}

pub fn new_catch_clauses<'arena>(
    arena: &'arena Bump,
    elements: Vec<'arena, CatchClause<'arena>>,
) -> CatchClauses<'arena> {
    CatchClauses { elements }
}

#[derive(Debug)]
pub struct ConstructorAttributes<'arena> {
    pub elements: Vec<'arena, ConstructorAttribute<'arena>>,
}

pub fn new_constructor_attributes<'arena>(
    arena: &'arena Bump,
    elements: Vec<'arena, ConstructorAttribute<'arena>>,
) -> ConstructorAttributes<'arena> {
    ConstructorAttributes { elements }
}

#[derive(Debug)]
pub struct ContractMembers<'arena> {
    pub elements: Vec<'arena, ContractMember<'arena>>,
}

pub fn new_contract_members<'arena>(
    arena: &'arena Bump,
    elements: Vec<'arena, ContractMember<'arena>>,
) -> ContractMembers<'arena> {
    ContractMembers { elements }
}

#[derive(Debug)]
pub struct ContractSpecifiers<'arena> {
    pub elements: Vec<'arena, ContractSpecifier<'arena>>,
}

pub fn new_contract_specifiers<'arena>(
    arena: &'arena Bump,
    elements: Vec<'arena, ContractSpecifier<'arena>>,
) -> ContractSpecifiers<'arena> {
    ContractSpecifiers { elements }
}

#[derive(Debug)]
pub struct EnumMembers<'arena> {
    pub elements: Vec<'arena, Identifier<'arena>>,
}

pub fn new_enum_members<'arena>(
    arena: &'arena Bump,
    elements: Vec<'arena, Identifier<'arena>>,
) -> EnumMembers<'arena> {
    EnumMembers { elements }
}

#[derive(Debug)]
pub struct ErrorParameters<'arena> {
    pub elements: Vec<'arena, ErrorParameter<'arena>>,
}

pub fn new_error_parameters<'arena>(
    arena: &'arena Bump,
    elements: Vec<'arena, ErrorParameter<'arena>>,
) -> ErrorParameters<'arena> {
    ErrorParameters { elements }
}

#[derive(Debug)]
pub struct EventParameters<'arena> {
    pub elements: Vec<'arena, EventParameter<'arena>>,
}

pub fn new_event_parameters<'arena>(
    arena: &'arena Bump,
    elements: Vec<'arena, EventParameter<'arena>>,
) -> EventParameters<'arena> {
    EventParameters { elements }
}

#[derive(Debug)]
pub struct FallbackFunctionAttributes<'arena> {
    pub elements: Vec<'arena, FallbackFunctionAttribute<'arena>>,
}

pub fn new_fallback_function_attributes<'arena>(
    arena: &'arena Bump,
    elements: Vec<'arena, FallbackFunctionAttribute<'arena>>,
) -> FallbackFunctionAttributes<'arena> {
    FallbackFunctionAttributes { elements }
}

#[derive(Debug)]
pub struct FunctionAttributes<'arena> {
    pub elements: Vec<'arena, FunctionAttribute<'arena>>,
}

pub fn new_function_attributes<'arena>(
    arena: &'arena Bump,
    elements: Vec<'arena, FunctionAttribute<'arena>>,
) -> FunctionAttributes<'arena> {
    FunctionAttributes { elements }
}

#[derive(Debug)]
pub struct FunctionTypeAttributes<'arena> {
    pub elements: Vec<'arena, FunctionTypeAttribute<'arena>>,
}

pub fn new_function_type_attributes<'arena>(
    arena: &'arena Bump,
    elements: Vec<'arena, FunctionTypeAttribute<'arena>>,
) -> FunctionTypeAttributes<'arena> {
    FunctionTypeAttributes { elements }
}

#[derive(Debug)]
pub struct HexStringLiterals<'arena> {
    pub elements: Vec<'arena, HexStringLiteral<'arena>>,
}

pub fn new_hex_string_literals<'arena>(
    arena: &'arena Bump,
    elements: Vec<'arena, HexStringLiteral<'arena>>,
) -> HexStringLiterals<'arena> {
    HexStringLiterals { elements }
}

#[derive(Debug)]
pub struct IdentifierPath<'arena> {
    pub elements: Vec<'arena, Identifier<'arena>>,
}

pub fn new_identifier_path<'arena>(
    arena: &'arena Bump,
    elements: Vec<'arena, Identifier<'arena>>,
) -> IdentifierPath<'arena> {
    IdentifierPath { elements }
}

#[derive(Debug)]
pub struct ImportDeconstructionSymbols<'arena> {
    pub elements: Vec<'arena, ImportDeconstructionSymbol<'arena>>,
}

pub fn new_import_deconstruction_symbols<'arena>(
    arena: &'arena Bump,
    elements: Vec<'arena, ImportDeconstructionSymbol<'arena>>,
) -> ImportDeconstructionSymbols<'arena> {
    ImportDeconstructionSymbols { elements }
}

#[derive(Debug)]
pub struct InheritanceTypes<'arena> {
    pub elements: Vec<'arena, InheritanceType<'arena>>,
}

pub fn new_inheritance_types<'arena>(
    arena: &'arena Bump,
    elements: Vec<'arena, InheritanceType<'arena>>,
) -> InheritanceTypes<'arena> {
    InheritanceTypes { elements }
}

#[derive(Debug)]
pub struct InterfaceMembers<'arena> {
    pub elements: Vec<'arena, ContractMember<'arena>>,
}

pub fn new_interface_members<'arena>(
    arena: &'arena Bump,
    elements: Vec<'arena, ContractMember<'arena>>,
) -> InterfaceMembers<'arena> {
    InterfaceMembers { elements }
}

#[derive(Debug)]
pub struct LibraryMembers<'arena> {
    pub elements: Vec<'arena, ContractMember<'arena>>,
}

pub fn new_library_members<'arena>(
    arena: &'arena Bump,
    elements: Vec<'arena, ContractMember<'arena>>,
) -> LibraryMembers<'arena> {
    LibraryMembers { elements }
}

#[derive(Debug)]
pub struct ModifierAttributes<'arena> {
    pub elements: Vec<'arena, ModifierAttribute<'arena>>,
}

pub fn new_modifier_attributes<'arena>(
    arena: &'arena Bump,
    elements: Vec<'arena, ModifierAttribute<'arena>>,
) -> ModifierAttributes<'arena> {
    ModifierAttributes { elements }
}

#[derive(Debug)]
pub struct NamedArguments<'arena> {
    pub elements: Vec<'arena, NamedArgument<'arena>>,
}

pub fn new_named_arguments<'arena>(
    arena: &'arena Bump,
    elements: Vec<'arena, NamedArgument<'arena>>,
) -> NamedArguments<'arena> {
    NamedArguments { elements }
}

#[derive(Debug)]
pub struct OverridePaths<'arena> {
    pub elements: Vec<'arena, IdentifierPath<'arena>>,
}

pub fn new_override_paths<'arena>(
    arena: &'arena Bump,
    elements: Vec<'arena, IdentifierPath<'arena>>,
) -> OverridePaths<'arena> {
    OverridePaths { elements }
}

#[derive(Debug)]
pub struct Parameters<'arena> {
    pub elements: Vec<'arena, Parameter<'arena>>,
}

pub fn new_parameters<'arena>(
    arena: &'arena Bump,
    elements: Vec<'arena, Parameter<'arena>>,
) -> Parameters<'arena> {
    Parameters { elements }
}

#[derive(Debug)]
pub struct PositionalArguments<'arena> {
    pub elements: Vec<'arena, Expression<'arena>>,
}

pub fn new_positional_arguments<'arena>(
    arena: &'arena Bump,
    elements: Vec<'arena, Expression<'arena>>,
) -> PositionalArguments<'arena> {
    PositionalArguments { elements }
}

#[derive(Debug)]
pub struct ReceiveFunctionAttributes<'arena> {
    pub elements: Vec<'arena, ReceiveFunctionAttribute<'arena>>,
}

pub fn new_receive_function_attributes<'arena>(
    arena: &'arena Bump,
    elements: Vec<'arena, ReceiveFunctionAttribute<'arena>>,
) -> ReceiveFunctionAttributes<'arena> {
    ReceiveFunctionAttributes { elements }
}

#[derive(Debug)]
pub struct SimpleVersionLiteral<'arena> {
    pub elements: Vec<'arena, VersionSpecifier<'arena>>,
}

pub fn new_simple_version_literal<'arena>(
    arena: &'arena Bump,
    elements: Vec<'arena, VersionSpecifier<'arena>>,
) -> SimpleVersionLiteral<'arena> {
    SimpleVersionLiteral { elements }
}

#[derive(Debug)]
pub struct SourceUnitMembers<'arena> {
    pub elements: Vec<'arena, SourceUnitMember<'arena>>,
}

pub fn new_source_unit_members<'arena>(
    arena: &'arena Bump,
    elements: Vec<'arena, SourceUnitMember<'arena>>,
) -> SourceUnitMembers<'arena> {
    SourceUnitMembers { elements }
}

#[derive(Debug)]
pub struct StateVariableAttributes<'arena> {
    pub elements: Vec<'arena, StateVariableAttribute<'arena>>,
}

pub fn new_state_variable_attributes<'arena>(
    arena: &'arena Bump,
    elements: Vec<'arena, StateVariableAttribute<'arena>>,
) -> StateVariableAttributes<'arena> {
    StateVariableAttributes { elements }
}

#[derive(Debug)]
pub struct Statements<'arena> {
    pub elements: Vec<'arena, Statement<'arena>>,
}

pub fn new_statements<'arena>(
    arena: &'arena Bump,
    elements: Vec<'arena, Statement<'arena>>,
) -> Statements<'arena> {
    Statements { elements }
}

#[derive(Debug)]
pub struct StringLiterals<'arena> {
    pub elements: Vec<'arena, StringLiteral<'arena>>,
}

pub fn new_string_literals<'arena>(
    arena: &'arena Bump,
    elements: Vec<'arena, StringLiteral<'arena>>,
) -> StringLiterals<'arena> {
    StringLiterals { elements }
}

#[derive(Debug)]
pub struct StructMembers<'arena> {
    pub elements: Vec<'arena, StructMember<'arena>>,
}

pub fn new_struct_members<'arena>(
    arena: &'arena Bump,
    elements: Vec<'arena, StructMember<'arena>>,
) -> StructMembers<'arena> {
    StructMembers { elements }
}

#[derive(Debug)]
pub struct TupleDeconstructionElements<'arena> {
    pub elements: Vec<'arena, TupleDeconstructionElement<'arena>>,
}

pub fn new_tuple_deconstruction_elements<'arena>(
    arena: &'arena Bump,
    elements: Vec<'arena, TupleDeconstructionElement<'arena>>,
) -> TupleDeconstructionElements<'arena> {
    TupleDeconstructionElements { elements }
}

#[derive(Debug)]
pub struct TupleValues<'arena> {
    pub elements: Vec<'arena, TupleValue<'arena>>,
}

pub fn new_tuple_values<'arena>(
    arena: &'arena Bump,
    elements: Vec<'arena, TupleValue<'arena>>,
) -> TupleValues<'arena> {
    TupleValues { elements }
}

#[derive(Debug)]
pub struct UnicodeStringLiterals<'arena> {
    pub elements: Vec<'arena, UnicodeStringLiteral<'arena>>,
}

pub fn new_unicode_string_literals<'arena>(
    arena: &'arena Bump,
    elements: Vec<'arena, UnicodeStringLiteral<'arena>>,
) -> UnicodeStringLiterals<'arena> {
    UnicodeStringLiterals { elements }
}

#[derive(Debug)]
pub struct UnnamedFunctionAttributes<'arena> {
    pub elements: Vec<'arena, UnnamedFunctionAttribute<'arena>>,
}

pub fn new_unnamed_function_attributes<'arena>(
    arena: &'arena Bump,
    elements: Vec<'arena, UnnamedFunctionAttribute<'arena>>,
) -> UnnamedFunctionAttributes<'arena> {
    UnnamedFunctionAttributes { elements }
}

#[derive(Debug)]
pub struct UsingDeconstructionSymbols<'arena> {
    pub elements: Vec<'arena, UsingDeconstructionSymbol<'arena>>,
}

pub fn new_using_deconstruction_symbols<'arena>(
    arena: &'arena Bump,
    elements: Vec<'arena, UsingDeconstructionSymbol<'arena>>,
) -> UsingDeconstructionSymbols<'arena> {
    UsingDeconstructionSymbols { elements }
}

#[derive(Debug)]
pub struct VersionExpressionSet<'arena> {
    pub elements: Vec<'arena, VersionExpression<'arena>>,
}

pub fn new_version_expression_set<'arena>(
    arena: &'arena Bump,
    elements: Vec<'arena, VersionExpression<'arena>>,
) -> VersionExpressionSet<'arena> {
    VersionExpressionSet { elements }
}

#[derive(Debug)]
pub struct VersionExpressionSets<'arena> {
    pub elements: Vec<'arena, VersionExpressionSet<'arena>>,
}

pub fn new_version_expression_sets<'arena>(
    arena: &'arena Bump,
    elements: Vec<'arena, VersionExpressionSet<'arena>>,
) -> VersionExpressionSets<'arena> {
    VersionExpressionSets { elements }
}

#[derive(Debug)]
pub struct YulArguments<'arena> {
    pub elements: Vec<'arena, YulExpression<'arena>>,
}

pub fn new_yul_arguments<'arena>(
    arena: &'arena Bump,
    elements: Vec<'arena, YulExpression<'arena>>,
) -> YulArguments<'arena> {
    YulArguments { elements }
}

#[derive(Debug)]
pub struct YulParameters<'arena> {
    pub elements: Vec<'arena, YulIdentifier<'arena>>,
}

pub fn new_yul_parameters<'arena>(
    arena: &'arena Bump,
    elements: Vec<'arena, YulIdentifier<'arena>>,
) -> YulParameters<'arena> {
    YulParameters { elements }
}

#[derive(Debug)]
pub struct YulPath<'arena> {
    pub elements: Vec<'arena, YulIdentifier<'arena>>,
}

pub fn new_yul_path<'arena>(
    arena: &'arena Bump,
    elements: Vec<'arena, YulIdentifier<'arena>>,
) -> YulPath<'arena> {
    YulPath { elements }
}

#[derive(Debug)]
pub struct YulPaths<'arena> {
    pub elements: Vec<'arena, YulPath<'arena>>,
}

pub fn new_yul_paths<'arena>(
    arena: &'arena Bump,
    elements: Vec<'arena, YulPath<'arena>>,
) -> YulPaths<'arena> {
    YulPaths { elements }
}

#[derive(Debug)]
pub struct YulStatements<'arena> {
    pub elements: Vec<'arena, YulStatement<'arena>>,
}

pub fn new_yul_statements<'arena>(
    arena: &'arena Bump,
    elements: Vec<'arena, YulStatement<'arena>>,
) -> YulStatements<'arena> {
    YulStatements { elements }
}

#[derive(Debug)]
pub struct YulSwitchCases<'arena> {
    pub elements: Vec<'arena, YulSwitchCase<'arena>>,
}

pub fn new_yul_switch_cases<'arena>(
    arena: &'arena Bump,
    elements: Vec<'arena, YulSwitchCase<'arena>>,
) -> YulSwitchCases<'arena> {
    YulSwitchCases { elements }
}

#[derive(Debug)]
pub struct YulVariableNames<'arena> {
    pub elements: Vec<'arena, YulIdentifier<'arena>>,
}

pub fn new_yul_variable_names<'arena>(
    arena: &'arena Bump,
    elements: Vec<'arena, YulIdentifier<'arena>>,
) -> YulVariableNames<'arena> {
    YulVariableNames { elements }
}

//
// Terminals
//
// Note: _arena and _source are unused on the constructor methods, but kept for uniformity with other constructors
// and because they may be needed in the future
// Note: Similarly we keep the 'arena lifetime parameter even if unused

#[derive(Debug)]
pub struct ABIEncoderV2Keyword<'arena> {
    pub l: usize,
    pub r: usize,
    pub phantom: PhantomData<&'arena ()>,
}

pub fn new_abi_encoder_v2_keyword<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    _source: &str,
) -> ABIEncoderV2Keyword<'arena> {
    ABIEncoderV2Keyword {
        l,
        r,
        phantom: PhantomData,
    }
}

#[derive(Debug)]
pub struct AbicoderKeyword<'arena> {
    pub l: usize,
    pub r: usize,
    pub phantom: PhantomData<&'arena ()>,
}

pub fn new_abicoder_keyword<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    _source: &str,
) -> AbicoderKeyword<'arena> {
    AbicoderKeyword {
        l,
        r,
        phantom: PhantomData,
    }
}

#[derive(Debug)]
pub struct AbicoderV1Keyword<'arena> {
    pub l: usize,
    pub r: usize,
    pub phantom: PhantomData<&'arena ()>,
}

pub fn new_abicoder_v1_keyword<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    _source: &str,
) -> AbicoderV1Keyword<'arena> {
    AbicoderV1Keyword {
        l,
        r,
        phantom: PhantomData,
    }
}

#[derive(Debug)]
pub struct AbicoderV2Keyword<'arena> {
    pub l: usize,
    pub r: usize,
    pub phantom: PhantomData<&'arena ()>,
}

pub fn new_abicoder_v2_keyword<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    _source: &str,
) -> AbicoderV2Keyword<'arena> {
    AbicoderV2Keyword {
        l,
        r,
        phantom: PhantomData,
    }
}

#[derive(Debug)]
pub struct AbstractKeyword<'arena> {
    pub l: usize,
    pub r: usize,
    pub phantom: PhantomData<&'arena ()>,
}

pub fn new_abstract_keyword<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    _source: &str,
) -> AbstractKeyword<'arena> {
    AbstractKeyword {
        l,
        r,
        phantom: PhantomData,
    }
}

#[derive(Debug)]
pub struct AddressKeyword<'arena> {
    pub l: usize,
    pub r: usize,
    pub phantom: PhantomData<&'arena ()>,
}

pub fn new_address_keyword<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    _source: &str,
) -> AddressKeyword<'arena> {
    AddressKeyword {
        l,
        r,
        phantom: PhantomData,
    }
}

#[derive(Debug)]
pub struct AfterKeyword<'arena> {
    pub l: usize,
    pub r: usize,
    pub phantom: PhantomData<&'arena ()>,
}

pub fn new_after_keyword<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    _source: &str,
) -> AfterKeyword<'arena> {
    AfterKeyword {
        l,
        r,
        phantom: PhantomData,
    }
}

#[derive(Debug)]
pub struct AliasKeyword<'arena> {
    pub l: usize,
    pub r: usize,
    pub phantom: PhantomData<&'arena ()>,
}

pub fn new_alias_keyword<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    _source: &str,
) -> AliasKeyword<'arena> {
    AliasKeyword {
        l,
        r,
        phantom: PhantomData,
    }
}

#[derive(Debug)]
pub struct Ampersand<'arena> {
    pub l: usize,
    pub r: usize,
    pub phantom: PhantomData<&'arena ()>,
}

pub fn new_ampersand<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    _source: &str,
) -> Ampersand<'arena> {
    Ampersand {
        l,
        r,
        phantom: PhantomData,
    }
}

#[derive(Debug)]
pub struct AmpersandAmpersand<'arena> {
    pub l: usize,
    pub r: usize,
    pub phantom: PhantomData<&'arena ()>,
}

pub fn new_ampersand_ampersand<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    _source: &str,
) -> AmpersandAmpersand<'arena> {
    AmpersandAmpersand {
        l,
        r,
        phantom: PhantomData,
    }
}

#[derive(Debug)]
pub struct AmpersandEqual<'arena> {
    pub l: usize,
    pub r: usize,
    pub phantom: PhantomData<&'arena ()>,
}

pub fn new_ampersand_equal<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    _source: &str,
) -> AmpersandEqual<'arena> {
    AmpersandEqual {
        l,
        r,
        phantom: PhantomData,
    }
}

#[derive(Debug)]
pub struct AnonymousKeyword<'arena> {
    pub l: usize,
    pub r: usize,
    pub phantom: PhantomData<&'arena ()>,
}

pub fn new_anonymous_keyword<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    _source: &str,
) -> AnonymousKeyword<'arena> {
    AnonymousKeyword {
        l,
        r,
        phantom: PhantomData,
    }
}

#[derive(Debug)]
pub struct ApplyKeyword<'arena> {
    pub l: usize,
    pub r: usize,
    pub phantom: PhantomData<&'arena ()>,
}

pub fn new_apply_keyword<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    _source: &str,
) -> ApplyKeyword<'arena> {
    ApplyKeyword {
        l,
        r,
        phantom: PhantomData,
    }
}

#[derive(Debug)]
pub struct AsKeyword<'arena> {
    pub l: usize,
    pub r: usize,
    pub phantom: PhantomData<&'arena ()>,
}

pub fn new_as_keyword<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    _source: &str,
) -> AsKeyword<'arena> {
    AsKeyword {
        l,
        r,
        phantom: PhantomData,
    }
}

#[derive(Debug)]
pub struct AssemblyKeyword<'arena> {
    pub l: usize,
    pub r: usize,
    pub phantom: PhantomData<&'arena ()>,
}

pub fn new_assembly_keyword<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    _source: &str,
) -> AssemblyKeyword<'arena> {
    AssemblyKeyword {
        l,
        r,
        phantom: PhantomData,
    }
}

#[derive(Debug)]
pub struct Asterisk<'arena> {
    pub l: usize,
    pub r: usize,
    pub phantom: PhantomData<&'arena ()>,
}

pub fn new_asterisk<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    _source: &str,
) -> Asterisk<'arena> {
    Asterisk {
        l,
        r,
        phantom: PhantomData,
    }
}

#[derive(Debug)]
pub struct AsteriskAsterisk<'arena> {
    pub l: usize,
    pub r: usize,
    pub phantom: PhantomData<&'arena ()>,
}

pub fn new_asterisk_asterisk<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    _source: &str,
) -> AsteriskAsterisk<'arena> {
    AsteriskAsterisk {
        l,
        r,
        phantom: PhantomData,
    }
}

#[derive(Debug)]
pub struct AsteriskEqual<'arena> {
    pub l: usize,
    pub r: usize,
    pub phantom: PhantomData<&'arena ()>,
}

pub fn new_asterisk_equal<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    _source: &str,
) -> AsteriskEqual<'arena> {
    AsteriskEqual {
        l,
        r,
        phantom: PhantomData,
    }
}

#[derive(Debug)]
pub struct AtKeyword<'arena> {
    pub l: usize,
    pub r: usize,
    pub phantom: PhantomData<&'arena ()>,
}

pub fn new_at_keyword<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    _source: &str,
) -> AtKeyword<'arena> {
    AtKeyword {
        l,
        r,
        phantom: PhantomData,
    }
}

#[derive(Debug)]
pub struct AutoKeyword<'arena> {
    pub l: usize,
    pub r: usize,
    pub phantom: PhantomData<&'arena ()>,
}

pub fn new_auto_keyword<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    _source: &str,
) -> AutoKeyword<'arena> {
    AutoKeyword {
        l,
        r,
        phantom: PhantomData,
    }
}

#[derive(Debug)]
pub struct Bang<'arena> {
    pub l: usize,
    pub r: usize,
    pub phantom: PhantomData<&'arena ()>,
}

pub fn new_bang<'arena>(_arena: &'arena Bump, l: usize, r: usize, _source: &str) -> Bang<'arena> {
    Bang {
        l,
        r,
        phantom: PhantomData,
    }
}

#[derive(Debug)]
pub struct BangEqual<'arena> {
    pub l: usize,
    pub r: usize,
    pub phantom: PhantomData<&'arena ()>,
}

pub fn new_bang_equal<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    _source: &str,
) -> BangEqual<'arena> {
    BangEqual {
        l,
        r,
        phantom: PhantomData,
    }
}

#[derive(Debug)]
pub struct Bar<'arena> {
    pub l: usize,
    pub r: usize,
    pub phantom: PhantomData<&'arena ()>,
}

pub fn new_bar<'arena>(_arena: &'arena Bump, l: usize, r: usize, _source: &str) -> Bar<'arena> {
    Bar {
        l,
        r,
        phantom: PhantomData,
    }
}

#[derive(Debug)]
pub struct BarBar<'arena> {
    pub l: usize,
    pub r: usize,
    pub phantom: PhantomData<&'arena ()>,
}

pub fn new_bar_bar<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    _source: &str,
) -> BarBar<'arena> {
    BarBar {
        l,
        r,
        phantom: PhantomData,
    }
}

#[derive(Debug)]
pub struct BarEqual<'arena> {
    pub l: usize,
    pub r: usize,
    pub phantom: PhantomData<&'arena ()>,
}

pub fn new_bar_equal<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    _source: &str,
) -> BarEqual<'arena> {
    BarEqual {
        l,
        r,
        phantom: PhantomData,
    }
}

#[derive(Debug)]
pub struct BoolKeyword<'arena> {
    pub l: usize,
    pub r: usize,
    pub phantom: PhantomData<&'arena ()>,
}

pub fn new_bool_keyword<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    _source: &str,
) -> BoolKeyword<'arena> {
    BoolKeyword {
        l,
        r,
        phantom: PhantomData,
    }
}

#[derive(Debug)]
pub struct BreakKeyword<'arena> {
    pub l: usize,
    pub r: usize,
    pub phantom: PhantomData<&'arena ()>,
}

pub fn new_break_keyword<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    _source: &str,
) -> BreakKeyword<'arena> {
    BreakKeyword {
        l,
        r,
        phantom: PhantomData,
    }
}

#[derive(Debug)]
pub struct ByteKeyword<'arena> {
    pub l: usize,
    pub r: usize,
    pub phantom: PhantomData<&'arena ()>,
}

pub fn new_byte_keyword<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    _source: &str,
) -> ByteKeyword<'arena> {
    ByteKeyword {
        l,
        r,
        phantom: PhantomData,
    }
}

#[derive(Debug)]
pub struct BytesKeyword<'arena> {
    pub l: usize,
    pub r: usize,
    pub phantom: PhantomData<&'arena ()>,
}

pub fn new_bytes_keyword<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    _source: &str,
) -> BytesKeyword<'arena> {
    BytesKeyword {
        l,
        r,
        phantom: PhantomData,
    }
}

#[derive(Debug)]
pub struct CallDataKeyword<'arena> {
    pub l: usize,
    pub r: usize,
    pub phantom: PhantomData<&'arena ()>,
}

pub fn new_call_data_keyword<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    _source: &str,
) -> CallDataKeyword<'arena> {
    CallDataKeyword {
        l,
        r,
        phantom: PhantomData,
    }
}

#[derive(Debug)]
pub struct Caret<'arena> {
    pub l: usize,
    pub r: usize,
    pub phantom: PhantomData<&'arena ()>,
}

pub fn new_caret<'arena>(_arena: &'arena Bump, l: usize, r: usize, _source: &str) -> Caret<'arena> {
    Caret {
        l,
        r,
        phantom: PhantomData,
    }
}

#[derive(Debug)]
pub struct CaretEqual<'arena> {
    pub l: usize,
    pub r: usize,
    pub phantom: PhantomData<&'arena ()>,
}

pub fn new_caret_equal<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    _source: &str,
) -> CaretEqual<'arena> {
    CaretEqual {
        l,
        r,
        phantom: PhantomData,
    }
}

#[derive(Debug)]
pub struct CaseKeyword<'arena> {
    pub l: usize,
    pub r: usize,
    pub phantom: PhantomData<&'arena ()>,
}

pub fn new_case_keyword<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    _source: &str,
) -> CaseKeyword<'arena> {
    CaseKeyword {
        l,
        r,
        phantom: PhantomData,
    }
}

#[derive(Debug)]
pub struct CatchKeyword<'arena> {
    pub l: usize,
    pub r: usize,
    pub phantom: PhantomData<&'arena ()>,
}

pub fn new_catch_keyword<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    _source: &str,
) -> CatchKeyword<'arena> {
    CatchKeyword {
        l,
        r,
        phantom: PhantomData,
    }
}

#[derive(Debug)]
pub struct CloseBrace<'arena> {
    pub l: usize,
    pub r: usize,
    pub phantom: PhantomData<&'arena ()>,
}

pub fn new_close_brace<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    _source: &str,
) -> CloseBrace<'arena> {
    CloseBrace {
        l,
        r,
        phantom: PhantomData,
    }
}

#[derive(Debug)]
pub struct CloseBracket<'arena> {
    pub l: usize,
    pub r: usize,
    pub phantom: PhantomData<&'arena ()>,
}

pub fn new_close_bracket<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    _source: &str,
) -> CloseBracket<'arena> {
    CloseBracket {
        l,
        r,
        phantom: PhantomData,
    }
}

#[derive(Debug)]
pub struct CloseParen<'arena> {
    pub l: usize,
    pub r: usize,
    pub phantom: PhantomData<&'arena ()>,
}

pub fn new_close_paren<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    _source: &str,
) -> CloseParen<'arena> {
    CloseParen {
        l,
        r,
        phantom: PhantomData,
    }
}

#[derive(Debug)]
pub struct Colon<'arena> {
    pub l: usize,
    pub r: usize,
    pub phantom: PhantomData<&'arena ()>,
}

pub fn new_colon<'arena>(_arena: &'arena Bump, l: usize, r: usize, _source: &str) -> Colon<'arena> {
    Colon {
        l,
        r,
        phantom: PhantomData,
    }
}

#[derive(Debug)]
pub struct ColonEqual<'arena> {
    pub l: usize,
    pub r: usize,
    pub phantom: PhantomData<&'arena ()>,
}

pub fn new_colon_equal<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    _source: &str,
) -> ColonEqual<'arena> {
    ColonEqual {
        l,
        r,
        phantom: PhantomData,
    }
}

#[derive(Debug)]
pub struct Comma<'arena> {
    pub l: usize,
    pub r: usize,
    pub phantom: PhantomData<&'arena ()>,
}

pub fn new_comma<'arena>(_arena: &'arena Bump, l: usize, r: usize, _source: &str) -> Comma<'arena> {
    Comma {
        l,
        r,
        phantom: PhantomData,
    }
}

#[derive(Debug)]
pub struct ConstantKeyword<'arena> {
    pub l: usize,
    pub r: usize,
    pub phantom: PhantomData<&'arena ()>,
}

pub fn new_constant_keyword<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    _source: &str,
) -> ConstantKeyword<'arena> {
    ConstantKeyword {
        l,
        r,
        phantom: PhantomData,
    }
}

#[derive(Debug)]
pub struct ConstructorKeyword<'arena> {
    pub l: usize,
    pub r: usize,
    pub phantom: PhantomData<&'arena ()>,
}

pub fn new_constructor_keyword<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    _source: &str,
) -> ConstructorKeyword<'arena> {
    ConstructorKeyword {
        l,
        r,
        phantom: PhantomData,
    }
}

#[derive(Debug)]
pub struct ContinueKeyword<'arena> {
    pub l: usize,
    pub r: usize,
    pub phantom: PhantomData<&'arena ()>,
}

pub fn new_continue_keyword<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    _source: &str,
) -> ContinueKeyword<'arena> {
    ContinueKeyword {
        l,
        r,
        phantom: PhantomData,
    }
}

#[derive(Debug)]
pub struct ContractKeyword<'arena> {
    pub l: usize,
    pub r: usize,
    pub phantom: PhantomData<&'arena ()>,
}

pub fn new_contract_keyword<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    _source: &str,
) -> ContractKeyword<'arena> {
    ContractKeyword {
        l,
        r,
        phantom: PhantomData,
    }
}

#[derive(Debug)]
pub struct CopyOfKeyword<'arena> {
    pub l: usize,
    pub r: usize,
    pub phantom: PhantomData<&'arena ()>,
}

pub fn new_copy_of_keyword<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    _source: &str,
) -> CopyOfKeyword<'arena> {
    CopyOfKeyword {
        l,
        r,
        phantom: PhantomData,
    }
}

#[derive(Debug)]
pub struct DaysKeyword<'arena> {
    pub l: usize,
    pub r: usize,
    pub phantom: PhantomData<&'arena ()>,
}

pub fn new_days_keyword<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    _source: &str,
) -> DaysKeyword<'arena> {
    DaysKeyword {
        l,
        r,
        phantom: PhantomData,
    }
}

#[derive(Debug)]
pub struct DecimalLiteral<'arena> {
    pub l: usize,
    pub r: usize,
    pub phantom: PhantomData<&'arena ()>,
}

pub fn new_decimal_literal<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    _source: &str,
) -> DecimalLiteral<'arena> {
    DecimalLiteral {
        l,
        r,
        phantom: PhantomData,
    }
}

#[derive(Debug)]
pub struct DefaultKeyword<'arena> {
    pub l: usize,
    pub r: usize,
    pub phantom: PhantomData<&'arena ()>,
}

pub fn new_default_keyword<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    _source: &str,
) -> DefaultKeyword<'arena> {
    DefaultKeyword {
        l,
        r,
        phantom: PhantomData,
    }
}

#[derive(Debug)]
pub struct DefineKeyword<'arena> {
    pub l: usize,
    pub r: usize,
    pub phantom: PhantomData<&'arena ()>,
}

pub fn new_define_keyword<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    _source: &str,
) -> DefineKeyword<'arena> {
    DefineKeyword {
        l,
        r,
        phantom: PhantomData,
    }
}

#[derive(Debug)]
pub struct DeleteKeyword<'arena> {
    pub l: usize,
    pub r: usize,
    pub phantom: PhantomData<&'arena ()>,
}

pub fn new_delete_keyword<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    _source: &str,
) -> DeleteKeyword<'arena> {
    DeleteKeyword {
        l,
        r,
        phantom: PhantomData,
    }
}

#[derive(Debug)]
pub struct DoKeyword<'arena> {
    pub l: usize,
    pub r: usize,
    pub phantom: PhantomData<&'arena ()>,
}

pub fn new_do_keyword<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    _source: &str,
) -> DoKeyword<'arena> {
    DoKeyword {
        l,
        r,
        phantom: PhantomData,
    }
}

#[derive(Debug)]
pub struct DoubleQuotedHexStringLiteral<'arena> {
    pub l: usize,
    pub r: usize,
    pub phantom: PhantomData<&'arena ()>,
}

pub fn new_double_quoted_hex_string_literal<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    _source: &str,
) -> DoubleQuotedHexStringLiteral<'arena> {
    DoubleQuotedHexStringLiteral {
        l,
        r,
        phantom: PhantomData,
    }
}

#[derive(Debug)]
pub struct DoubleQuotedStringLiteral<'arena> {
    pub l: usize,
    pub r: usize,
    pub phantom: PhantomData<&'arena ()>,
}

pub fn new_double_quoted_string_literal<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    _source: &str,
) -> DoubleQuotedStringLiteral<'arena> {
    DoubleQuotedStringLiteral {
        l,
        r,
        phantom: PhantomData,
    }
}

#[derive(Debug)]
pub struct DoubleQuotedUnicodeStringLiteral<'arena> {
    pub l: usize,
    pub r: usize,
    pub phantom: PhantomData<&'arena ()>,
}

pub fn new_double_quoted_unicode_string_literal<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    _source: &str,
) -> DoubleQuotedUnicodeStringLiteral<'arena> {
    DoubleQuotedUnicodeStringLiteral {
        l,
        r,
        phantom: PhantomData,
    }
}

#[derive(Debug)]
pub struct DoubleQuotedVersionLiteral<'arena> {
    pub l: usize,
    pub r: usize,
    pub phantom: PhantomData<&'arena ()>,
}

pub fn new_double_quoted_version_literal<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    _source: &str,
) -> DoubleQuotedVersionLiteral<'arena> {
    DoubleQuotedVersionLiteral {
        l,
        r,
        phantom: PhantomData,
    }
}

#[derive(Debug)]
pub struct ElseKeyword<'arena> {
    pub l: usize,
    pub r: usize,
    pub phantom: PhantomData<&'arena ()>,
}

pub fn new_else_keyword<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    _source: &str,
) -> ElseKeyword<'arena> {
    ElseKeyword {
        l,
        r,
        phantom: PhantomData,
    }
}

#[derive(Debug)]
pub struct EmitKeyword<'arena> {
    pub l: usize,
    pub r: usize,
    pub phantom: PhantomData<&'arena ()>,
}

pub fn new_emit_keyword<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    _source: &str,
) -> EmitKeyword<'arena> {
    EmitKeyword {
        l,
        r,
        phantom: PhantomData,
    }
}

#[derive(Debug)]
pub struct EndOfLine<'arena> {
    pub l: usize,
    pub r: usize,
    pub phantom: PhantomData<&'arena ()>,
}

pub fn new_end_of_line<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    _source: &str,
) -> EndOfLine<'arena> {
    EndOfLine {
        l,
        r,
        phantom: PhantomData,
    }
}

#[derive(Debug)]
pub struct EnumKeyword<'arena> {
    pub l: usize,
    pub r: usize,
    pub phantom: PhantomData<&'arena ()>,
}

pub fn new_enum_keyword<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    _source: &str,
) -> EnumKeyword<'arena> {
    EnumKeyword {
        l,
        r,
        phantom: PhantomData,
    }
}

#[derive(Debug)]
pub struct Equal<'arena> {
    pub l: usize,
    pub r: usize,
    pub phantom: PhantomData<&'arena ()>,
}

pub fn new_equal<'arena>(_arena: &'arena Bump, l: usize, r: usize, _source: &str) -> Equal<'arena> {
    Equal {
        l,
        r,
        phantom: PhantomData,
    }
}

#[derive(Debug)]
pub struct EqualColon<'arena> {
    pub l: usize,
    pub r: usize,
    pub phantom: PhantomData<&'arena ()>,
}

pub fn new_equal_colon<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    _source: &str,
) -> EqualColon<'arena> {
    EqualColon {
        l,
        r,
        phantom: PhantomData,
    }
}

#[derive(Debug)]
pub struct EqualEqual<'arena> {
    pub l: usize,
    pub r: usize,
    pub phantom: PhantomData<&'arena ()>,
}

pub fn new_equal_equal<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    _source: &str,
) -> EqualEqual<'arena> {
    EqualEqual {
        l,
        r,
        phantom: PhantomData,
    }
}

#[derive(Debug)]
pub struct EqualGreaterThan<'arena> {
    pub l: usize,
    pub r: usize,
    pub phantom: PhantomData<&'arena ()>,
}

pub fn new_equal_greater_than<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    _source: &str,
) -> EqualGreaterThan<'arena> {
    EqualGreaterThan {
        l,
        r,
        phantom: PhantomData,
    }
}

#[derive(Debug)]
pub struct ErrorKeyword<'arena> {
    pub l: usize,
    pub r: usize,
    pub phantom: PhantomData<&'arena ()>,
}

pub fn new_error_keyword<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    _source: &str,
) -> ErrorKeyword<'arena> {
    ErrorKeyword {
        l,
        r,
        phantom: PhantomData,
    }
}

#[derive(Debug)]
pub struct EtherKeyword<'arena> {
    pub l: usize,
    pub r: usize,
    pub phantom: PhantomData<&'arena ()>,
}

pub fn new_ether_keyword<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    _source: &str,
) -> EtherKeyword<'arena> {
    EtherKeyword {
        l,
        r,
        phantom: PhantomData,
    }
}

#[derive(Debug)]
pub struct EventKeyword<'arena> {
    pub l: usize,
    pub r: usize,
    pub phantom: PhantomData<&'arena ()>,
}

pub fn new_event_keyword<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    _source: &str,
) -> EventKeyword<'arena> {
    EventKeyword {
        l,
        r,
        phantom: PhantomData,
    }
}

#[derive(Debug)]
pub struct ExperimentalKeyword<'arena> {
    pub l: usize,
    pub r: usize,
    pub phantom: PhantomData<&'arena ()>,
}

pub fn new_experimental_keyword<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    _source: &str,
) -> ExperimentalKeyword<'arena> {
    ExperimentalKeyword {
        l,
        r,
        phantom: PhantomData,
    }
}

#[derive(Debug)]
pub struct ExternalKeyword<'arena> {
    pub l: usize,
    pub r: usize,
    pub phantom: PhantomData<&'arena ()>,
}

pub fn new_external_keyword<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    _source: &str,
) -> ExternalKeyword<'arena> {
    ExternalKeyword {
        l,
        r,
        phantom: PhantomData,
    }
}

#[derive(Debug)]
pub struct FallbackKeyword<'arena> {
    pub l: usize,
    pub r: usize,
    pub phantom: PhantomData<&'arena ()>,
}

pub fn new_fallback_keyword<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    _source: &str,
) -> FallbackKeyword<'arena> {
    FallbackKeyword {
        l,
        r,
        phantom: PhantomData,
    }
}

#[derive(Debug)]
pub struct FalseKeyword<'arena> {
    pub l: usize,
    pub r: usize,
    pub phantom: PhantomData<&'arena ()>,
}

pub fn new_false_keyword<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    _source: &str,
) -> FalseKeyword<'arena> {
    FalseKeyword {
        l,
        r,
        phantom: PhantomData,
    }
}

#[derive(Debug)]
pub struct FinalKeyword<'arena> {
    pub l: usize,
    pub r: usize,
    pub phantom: PhantomData<&'arena ()>,
}

pub fn new_final_keyword<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    _source: &str,
) -> FinalKeyword<'arena> {
    FinalKeyword {
        l,
        r,
        phantom: PhantomData,
    }
}

#[derive(Debug)]
pub struct FinneyKeyword<'arena> {
    pub l: usize,
    pub r: usize,
    pub phantom: PhantomData<&'arena ()>,
}

pub fn new_finney_keyword<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    _source: &str,
) -> FinneyKeyword<'arena> {
    FinneyKeyword {
        l,
        r,
        phantom: PhantomData,
    }
}

#[derive(Debug)]
pub struct FixedKeyword<'arena> {
    pub l: usize,
    pub r: usize,
    pub phantom: PhantomData<&'arena ()>,
}

pub fn new_fixed_keyword<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    _source: &str,
) -> FixedKeyword<'arena> {
    FixedKeyword {
        l,
        r,
        phantom: PhantomData,
    }
}

#[derive(Debug)]
pub struct ForKeyword<'arena> {
    pub l: usize,
    pub r: usize,
    pub phantom: PhantomData<&'arena ()>,
}

pub fn new_for_keyword<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    _source: &str,
) -> ForKeyword<'arena> {
    ForKeyword {
        l,
        r,
        phantom: PhantomData,
    }
}

#[derive(Debug)]
pub struct FromKeyword<'arena> {
    pub l: usize,
    pub r: usize,
    pub phantom: PhantomData<&'arena ()>,
}

pub fn new_from_keyword<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    _source: &str,
) -> FromKeyword<'arena> {
    FromKeyword {
        l,
        r,
        phantom: PhantomData,
    }
}

#[derive(Debug)]
pub struct FunctionKeyword<'arena> {
    pub l: usize,
    pub r: usize,
    pub phantom: PhantomData<&'arena ()>,
}

pub fn new_function_keyword<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    _source: &str,
) -> FunctionKeyword<'arena> {
    FunctionKeyword {
        l,
        r,
        phantom: PhantomData,
    }
}

#[derive(Debug)]
pub struct GlobalKeyword<'arena> {
    pub l: usize,
    pub r: usize,
    pub phantom: PhantomData<&'arena ()>,
}

pub fn new_global_keyword<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    _source: &str,
) -> GlobalKeyword<'arena> {
    GlobalKeyword {
        l,
        r,
        phantom: PhantomData,
    }
}

#[derive(Debug)]
pub struct GreaterThan<'arena> {
    pub l: usize,
    pub r: usize,
    pub phantom: PhantomData<&'arena ()>,
}

pub fn new_greater_than<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    _source: &str,
) -> GreaterThan<'arena> {
    GreaterThan {
        l,
        r,
        phantom: PhantomData,
    }
}

#[derive(Debug)]
pub struct GreaterThanEqual<'arena> {
    pub l: usize,
    pub r: usize,
    pub phantom: PhantomData<&'arena ()>,
}

pub fn new_greater_than_equal<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    _source: &str,
) -> GreaterThanEqual<'arena> {
    GreaterThanEqual {
        l,
        r,
        phantom: PhantomData,
    }
}

#[derive(Debug)]
pub struct GreaterThanGreaterThan<'arena> {
    pub l: usize,
    pub r: usize,
    pub phantom: PhantomData<&'arena ()>,
}

pub fn new_greater_than_greater_than<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    _source: &str,
) -> GreaterThanGreaterThan<'arena> {
    GreaterThanGreaterThan {
        l,
        r,
        phantom: PhantomData,
    }
}

#[derive(Debug)]
pub struct GreaterThanGreaterThanEqual<'arena> {
    pub l: usize,
    pub r: usize,
    pub phantom: PhantomData<&'arena ()>,
}

pub fn new_greater_than_greater_than_equal<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    _source: &str,
) -> GreaterThanGreaterThanEqual<'arena> {
    GreaterThanGreaterThanEqual {
        l,
        r,
        phantom: PhantomData,
    }
}

#[derive(Debug)]
pub struct GreaterThanGreaterThanGreaterThan<'arena> {
    pub l: usize,
    pub r: usize,
    pub phantom: PhantomData<&'arena ()>,
}

pub fn new_greater_than_greater_than_greater_than<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    _source: &str,
) -> GreaterThanGreaterThanGreaterThan<'arena> {
    GreaterThanGreaterThanGreaterThan {
        l,
        r,
        phantom: PhantomData,
    }
}

#[derive(Debug)]
pub struct GreaterThanGreaterThanGreaterThanEqual<'arena> {
    pub l: usize,
    pub r: usize,
    pub phantom: PhantomData<&'arena ()>,
}

pub fn new_greater_than_greater_than_greater_than_equal<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    _source: &str,
) -> GreaterThanGreaterThanGreaterThanEqual<'arena> {
    GreaterThanGreaterThanGreaterThanEqual {
        l,
        r,
        phantom: PhantomData,
    }
}

#[derive(Debug)]
pub struct GweiKeyword<'arena> {
    pub l: usize,
    pub r: usize,
    pub phantom: PhantomData<&'arena ()>,
}

pub fn new_gwei_keyword<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    _source: &str,
) -> GweiKeyword<'arena> {
    GweiKeyword {
        l,
        r,
        phantom: PhantomData,
    }
}

#[derive(Debug)]
pub struct HexKeyword<'arena> {
    pub l: usize,
    pub r: usize,
    pub phantom: PhantomData<&'arena ()>,
}

pub fn new_hex_keyword<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    _source: &str,
) -> HexKeyword<'arena> {
    HexKeyword {
        l,
        r,
        phantom: PhantomData,
    }
}

#[derive(Debug)]
pub struct HexLiteral<'arena> {
    pub l: usize,
    pub r: usize,
    pub phantom: PhantomData<&'arena ()>,
}

pub fn new_hex_literal<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    _source: &str,
) -> HexLiteral<'arena> {
    HexLiteral {
        l,
        r,
        phantom: PhantomData,
    }
}

#[derive(Debug)]
pub struct HoursKeyword<'arena> {
    pub l: usize,
    pub r: usize,
    pub phantom: PhantomData<&'arena ()>,
}

pub fn new_hours_keyword<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    _source: &str,
) -> HoursKeyword<'arena> {
    HoursKeyword {
        l,
        r,
        phantom: PhantomData,
    }
}

#[derive(Debug)]
pub struct Identifier<'arena> {
    pub l: usize,
    pub r: usize,
    pub phantom: PhantomData<&'arena ()>,
}

pub fn new_identifier<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    _source: &str,
) -> Identifier<'arena> {
    Identifier {
        l,
        r,
        phantom: PhantomData,
    }
}

#[derive(Debug)]
pub struct IfKeyword<'arena> {
    pub l: usize,
    pub r: usize,
    pub phantom: PhantomData<&'arena ()>,
}

pub fn new_if_keyword<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    _source: &str,
) -> IfKeyword<'arena> {
    IfKeyword {
        l,
        r,
        phantom: PhantomData,
    }
}

#[derive(Debug)]
pub struct ImmutableKeyword<'arena> {
    pub l: usize,
    pub r: usize,
    pub phantom: PhantomData<&'arena ()>,
}

pub fn new_immutable_keyword<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    _source: &str,
) -> ImmutableKeyword<'arena> {
    ImmutableKeyword {
        l,
        r,
        phantom: PhantomData,
    }
}

#[derive(Debug)]
pub struct ImplementsKeyword<'arena> {
    pub l: usize,
    pub r: usize,
    pub phantom: PhantomData<&'arena ()>,
}

pub fn new_implements_keyword<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    _source: &str,
) -> ImplementsKeyword<'arena> {
    ImplementsKeyword {
        l,
        r,
        phantom: PhantomData,
    }
}

#[derive(Debug)]
pub struct ImportKeyword<'arena> {
    pub l: usize,
    pub r: usize,
    pub phantom: PhantomData<&'arena ()>,
}

pub fn new_import_keyword<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    _source: &str,
) -> ImportKeyword<'arena> {
    ImportKeyword {
        l,
        r,
        phantom: PhantomData,
    }
}

#[derive(Debug)]
pub struct InKeyword<'arena> {
    pub l: usize,
    pub r: usize,
    pub phantom: PhantomData<&'arena ()>,
}

pub fn new_in_keyword<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    _source: &str,
) -> InKeyword<'arena> {
    InKeyword {
        l,
        r,
        phantom: PhantomData,
    }
}

#[derive(Debug)]
pub struct IndexedKeyword<'arena> {
    pub l: usize,
    pub r: usize,
    pub phantom: PhantomData<&'arena ()>,
}

pub fn new_indexed_keyword<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    _source: &str,
) -> IndexedKeyword<'arena> {
    IndexedKeyword {
        l,
        r,
        phantom: PhantomData,
    }
}

#[derive(Debug)]
pub struct InlineKeyword<'arena> {
    pub l: usize,
    pub r: usize,
    pub phantom: PhantomData<&'arena ()>,
}

pub fn new_inline_keyword<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    _source: &str,
) -> InlineKeyword<'arena> {
    InlineKeyword {
        l,
        r,
        phantom: PhantomData,
    }
}

#[derive(Debug)]
pub struct IntKeyword<'arena> {
    pub l: usize,
    pub r: usize,
    pub phantom: PhantomData<&'arena ()>,
}

pub fn new_int_keyword<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    _source: &str,
) -> IntKeyword<'arena> {
    IntKeyword {
        l,
        r,
        phantom: PhantomData,
    }
}

#[derive(Debug)]
pub struct InterfaceKeyword<'arena> {
    pub l: usize,
    pub r: usize,
    pub phantom: PhantomData<&'arena ()>,
}

pub fn new_interface_keyword<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    _source: &str,
) -> InterfaceKeyword<'arena> {
    InterfaceKeyword {
        l,
        r,
        phantom: PhantomData,
    }
}

#[derive(Debug)]
pub struct InternalKeyword<'arena> {
    pub l: usize,
    pub r: usize,
    pub phantom: PhantomData<&'arena ()>,
}

pub fn new_internal_keyword<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    _source: &str,
) -> InternalKeyword<'arena> {
    InternalKeyword {
        l,
        r,
        phantom: PhantomData,
    }
}

#[derive(Debug)]
pub struct IsKeyword<'arena> {
    pub l: usize,
    pub r: usize,
    pub phantom: PhantomData<&'arena ()>,
}

pub fn new_is_keyword<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    _source: &str,
) -> IsKeyword<'arena> {
    IsKeyword {
        l,
        r,
        phantom: PhantomData,
    }
}

#[derive(Debug)]
pub struct LayoutKeyword<'arena> {
    pub l: usize,
    pub r: usize,
    pub phantom: PhantomData<&'arena ()>,
}

pub fn new_layout_keyword<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    _source: &str,
) -> LayoutKeyword<'arena> {
    LayoutKeyword {
        l,
        r,
        phantom: PhantomData,
    }
}

#[derive(Debug)]
pub struct LessThan<'arena> {
    pub l: usize,
    pub r: usize,
    pub phantom: PhantomData<&'arena ()>,
}

pub fn new_less_than<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    _source: &str,
) -> LessThan<'arena> {
    LessThan {
        l,
        r,
        phantom: PhantomData,
    }
}

#[derive(Debug)]
pub struct LessThanEqual<'arena> {
    pub l: usize,
    pub r: usize,
    pub phantom: PhantomData<&'arena ()>,
}

pub fn new_less_than_equal<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    _source: &str,
) -> LessThanEqual<'arena> {
    LessThanEqual {
        l,
        r,
        phantom: PhantomData,
    }
}

#[derive(Debug)]
pub struct LessThanLessThan<'arena> {
    pub l: usize,
    pub r: usize,
    pub phantom: PhantomData<&'arena ()>,
}

pub fn new_less_than_less_than<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    _source: &str,
) -> LessThanLessThan<'arena> {
    LessThanLessThan {
        l,
        r,
        phantom: PhantomData,
    }
}

#[derive(Debug)]
pub struct LessThanLessThanEqual<'arena> {
    pub l: usize,
    pub r: usize,
    pub phantom: PhantomData<&'arena ()>,
}

pub fn new_less_than_less_than_equal<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    _source: &str,
) -> LessThanLessThanEqual<'arena> {
    LessThanLessThanEqual {
        l,
        r,
        phantom: PhantomData,
    }
}

#[derive(Debug)]
pub struct LetKeyword<'arena> {
    pub l: usize,
    pub r: usize,
    pub phantom: PhantomData<&'arena ()>,
}

pub fn new_let_keyword<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    _source: &str,
) -> LetKeyword<'arena> {
    LetKeyword {
        l,
        r,
        phantom: PhantomData,
    }
}

#[derive(Debug)]
pub struct LibraryKeyword<'arena> {
    pub l: usize,
    pub r: usize,
    pub phantom: PhantomData<&'arena ()>,
}

pub fn new_library_keyword<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    _source: &str,
) -> LibraryKeyword<'arena> {
    LibraryKeyword {
        l,
        r,
        phantom: PhantomData,
    }
}

#[derive(Debug)]
pub struct MacroKeyword<'arena> {
    pub l: usize,
    pub r: usize,
    pub phantom: PhantomData<&'arena ()>,
}

pub fn new_macro_keyword<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    _source: &str,
) -> MacroKeyword<'arena> {
    MacroKeyword {
        l,
        r,
        phantom: PhantomData,
    }
}

#[derive(Debug)]
pub struct MappingKeyword<'arena> {
    pub l: usize,
    pub r: usize,
    pub phantom: PhantomData<&'arena ()>,
}

pub fn new_mapping_keyword<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    _source: &str,
) -> MappingKeyword<'arena> {
    MappingKeyword {
        l,
        r,
        phantom: PhantomData,
    }
}

#[derive(Debug)]
pub struct MatchKeyword<'arena> {
    pub l: usize,
    pub r: usize,
    pub phantom: PhantomData<&'arena ()>,
}

pub fn new_match_keyword<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    _source: &str,
) -> MatchKeyword<'arena> {
    MatchKeyword {
        l,
        r,
        phantom: PhantomData,
    }
}

#[derive(Debug)]
pub struct MemoryKeyword<'arena> {
    pub l: usize,
    pub r: usize,
    pub phantom: PhantomData<&'arena ()>,
}

pub fn new_memory_keyword<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    _source: &str,
) -> MemoryKeyword<'arena> {
    MemoryKeyword {
        l,
        r,
        phantom: PhantomData,
    }
}

#[derive(Debug)]
pub struct Minus<'arena> {
    pub l: usize,
    pub r: usize,
    pub phantom: PhantomData<&'arena ()>,
}

pub fn new_minus<'arena>(_arena: &'arena Bump, l: usize, r: usize, _source: &str) -> Minus<'arena> {
    Minus {
        l,
        r,
        phantom: PhantomData,
    }
}

#[derive(Debug)]
pub struct MinusEqual<'arena> {
    pub l: usize,
    pub r: usize,
    pub phantom: PhantomData<&'arena ()>,
}

pub fn new_minus_equal<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    _source: &str,
) -> MinusEqual<'arena> {
    MinusEqual {
        l,
        r,
        phantom: PhantomData,
    }
}

#[derive(Debug)]
pub struct MinusGreaterThan<'arena> {
    pub l: usize,
    pub r: usize,
    pub phantom: PhantomData<&'arena ()>,
}

pub fn new_minus_greater_than<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    _source: &str,
) -> MinusGreaterThan<'arena> {
    MinusGreaterThan {
        l,
        r,
        phantom: PhantomData,
    }
}

#[derive(Debug)]
pub struct MinusMinus<'arena> {
    pub l: usize,
    pub r: usize,
    pub phantom: PhantomData<&'arena ()>,
}

pub fn new_minus_minus<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    _source: &str,
) -> MinusMinus<'arena> {
    MinusMinus {
        l,
        r,
        phantom: PhantomData,
    }
}

#[derive(Debug)]
pub struct MinutesKeyword<'arena> {
    pub l: usize,
    pub r: usize,
    pub phantom: PhantomData<&'arena ()>,
}

pub fn new_minutes_keyword<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    _source: &str,
) -> MinutesKeyword<'arena> {
    MinutesKeyword {
        l,
        r,
        phantom: PhantomData,
    }
}

#[derive(Debug)]
pub struct ModifierKeyword<'arena> {
    pub l: usize,
    pub r: usize,
    pub phantom: PhantomData<&'arena ()>,
}

pub fn new_modifier_keyword<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    _source: &str,
) -> ModifierKeyword<'arena> {
    ModifierKeyword {
        l,
        r,
        phantom: PhantomData,
    }
}

#[derive(Debug)]
pub struct MultiLineComment<'arena> {
    pub l: usize,
    pub r: usize,
    pub phantom: PhantomData<&'arena ()>,
}

pub fn new_multi_line_comment<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    _source: &str,
) -> MultiLineComment<'arena> {
    MultiLineComment {
        l,
        r,
        phantom: PhantomData,
    }
}

#[derive(Debug)]
pub struct MultiLineNatSpecComment<'arena> {
    pub l: usize,
    pub r: usize,
    pub phantom: PhantomData<&'arena ()>,
}

pub fn new_multi_line_nat_spec_comment<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    _source: &str,
) -> MultiLineNatSpecComment<'arena> {
    MultiLineNatSpecComment {
        l,
        r,
        phantom: PhantomData,
    }
}

#[derive(Debug)]
pub struct MutableKeyword<'arena> {
    pub l: usize,
    pub r: usize,
    pub phantom: PhantomData<&'arena ()>,
}

pub fn new_mutable_keyword<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    _source: &str,
) -> MutableKeyword<'arena> {
    MutableKeyword {
        l,
        r,
        phantom: PhantomData,
    }
}

#[derive(Debug)]
pub struct NewKeyword<'arena> {
    pub l: usize,
    pub r: usize,
    pub phantom: PhantomData<&'arena ()>,
}

pub fn new_new_keyword<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    _source: &str,
) -> NewKeyword<'arena> {
    NewKeyword {
        l,
        r,
        phantom: PhantomData,
    }
}

#[derive(Debug)]
pub struct NullKeyword<'arena> {
    pub l: usize,
    pub r: usize,
    pub phantom: PhantomData<&'arena ()>,
}

pub fn new_null_keyword<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    _source: &str,
) -> NullKeyword<'arena> {
    NullKeyword {
        l,
        r,
        phantom: PhantomData,
    }
}

#[derive(Debug)]
pub struct OfKeyword<'arena> {
    pub l: usize,
    pub r: usize,
    pub phantom: PhantomData<&'arena ()>,
}

pub fn new_of_keyword<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    _source: &str,
) -> OfKeyword<'arena> {
    OfKeyword {
        l,
        r,
        phantom: PhantomData,
    }
}

#[derive(Debug)]
pub struct OpenBrace<'arena> {
    pub l: usize,
    pub r: usize,
    pub phantom: PhantomData<&'arena ()>,
}

pub fn new_open_brace<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    _source: &str,
) -> OpenBrace<'arena> {
    OpenBrace {
        l,
        r,
        phantom: PhantomData,
    }
}

#[derive(Debug)]
pub struct OpenBracket<'arena> {
    pub l: usize,
    pub r: usize,
    pub phantom: PhantomData<&'arena ()>,
}

pub fn new_open_bracket<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    _source: &str,
) -> OpenBracket<'arena> {
    OpenBracket {
        l,
        r,
        phantom: PhantomData,
    }
}

#[derive(Debug)]
pub struct OpenParen<'arena> {
    pub l: usize,
    pub r: usize,
    pub phantom: PhantomData<&'arena ()>,
}

pub fn new_open_paren<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    _source: &str,
) -> OpenParen<'arena> {
    OpenParen {
        l,
        r,
        phantom: PhantomData,
    }
}

#[derive(Debug)]
pub struct OverrideKeyword<'arena> {
    pub l: usize,
    pub r: usize,
    pub phantom: PhantomData<&'arena ()>,
}

pub fn new_override_keyword<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    _source: &str,
) -> OverrideKeyword<'arena> {
    OverrideKeyword {
        l,
        r,
        phantom: PhantomData,
    }
}

#[derive(Debug)]
pub struct PartialKeyword<'arena> {
    pub l: usize,
    pub r: usize,
    pub phantom: PhantomData<&'arena ()>,
}

pub fn new_partial_keyword<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    _source: &str,
) -> PartialKeyword<'arena> {
    PartialKeyword {
        l,
        r,
        phantom: PhantomData,
    }
}

#[derive(Debug)]
pub struct PayableKeyword<'arena> {
    pub l: usize,
    pub r: usize,
    pub phantom: PhantomData<&'arena ()>,
}

pub fn new_payable_keyword<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    _source: &str,
) -> PayableKeyword<'arena> {
    PayableKeyword {
        l,
        r,
        phantom: PhantomData,
    }
}

#[derive(Debug)]
pub struct Percent<'arena> {
    pub l: usize,
    pub r: usize,
    pub phantom: PhantomData<&'arena ()>,
}

pub fn new_percent<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    _source: &str,
) -> Percent<'arena> {
    Percent {
        l,
        r,
        phantom: PhantomData,
    }
}

#[derive(Debug)]
pub struct PercentEqual<'arena> {
    pub l: usize,
    pub r: usize,
    pub phantom: PhantomData<&'arena ()>,
}

pub fn new_percent_equal<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    _source: &str,
) -> PercentEqual<'arena> {
    PercentEqual {
        l,
        r,
        phantom: PhantomData,
    }
}

#[derive(Debug)]
pub struct Period<'arena> {
    pub l: usize,
    pub r: usize,
    pub phantom: PhantomData<&'arena ()>,
}

pub fn new_period<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    _source: &str,
) -> Period<'arena> {
    Period {
        l,
        r,
        phantom: PhantomData,
    }
}

#[derive(Debug)]
pub struct Plus<'arena> {
    pub l: usize,
    pub r: usize,
    pub phantom: PhantomData<&'arena ()>,
}

pub fn new_plus<'arena>(_arena: &'arena Bump, l: usize, r: usize, _source: &str) -> Plus<'arena> {
    Plus {
        l,
        r,
        phantom: PhantomData,
    }
}

#[derive(Debug)]
pub struct PlusEqual<'arena> {
    pub l: usize,
    pub r: usize,
    pub phantom: PhantomData<&'arena ()>,
}

pub fn new_plus_equal<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    _source: &str,
) -> PlusEqual<'arena> {
    PlusEqual {
        l,
        r,
        phantom: PhantomData,
    }
}

#[derive(Debug)]
pub struct PlusPlus<'arena> {
    pub l: usize,
    pub r: usize,
    pub phantom: PhantomData<&'arena ()>,
}

pub fn new_plus_plus<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    _source: &str,
) -> PlusPlus<'arena> {
    PlusPlus {
        l,
        r,
        phantom: PhantomData,
    }
}

#[derive(Debug)]
pub struct PragmaKeyword<'arena> {
    pub l: usize,
    pub r: usize,
    pub phantom: PhantomData<&'arena ()>,
}

pub fn new_pragma_keyword<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    _source: &str,
) -> PragmaKeyword<'arena> {
    PragmaKeyword {
        l,
        r,
        phantom: PhantomData,
    }
}

#[derive(Debug)]
pub struct PrivateKeyword<'arena> {
    pub l: usize,
    pub r: usize,
    pub phantom: PhantomData<&'arena ()>,
}

pub fn new_private_keyword<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    _source: &str,
) -> PrivateKeyword<'arena> {
    PrivateKeyword {
        l,
        r,
        phantom: PhantomData,
    }
}

#[derive(Debug)]
pub struct PromiseKeyword<'arena> {
    pub l: usize,
    pub r: usize,
    pub phantom: PhantomData<&'arena ()>,
}

pub fn new_promise_keyword<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    _source: &str,
) -> PromiseKeyword<'arena> {
    PromiseKeyword {
        l,
        r,
        phantom: PhantomData,
    }
}

#[derive(Debug)]
pub struct PublicKeyword<'arena> {
    pub l: usize,
    pub r: usize,
    pub phantom: PhantomData<&'arena ()>,
}

pub fn new_public_keyword<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    _source: &str,
) -> PublicKeyword<'arena> {
    PublicKeyword {
        l,
        r,
        phantom: PhantomData,
    }
}

#[derive(Debug)]
pub struct PureKeyword<'arena> {
    pub l: usize,
    pub r: usize,
    pub phantom: PhantomData<&'arena ()>,
}

pub fn new_pure_keyword<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    _source: &str,
) -> PureKeyword<'arena> {
    PureKeyword {
        l,
        r,
        phantom: PhantomData,
    }
}

#[derive(Debug)]
pub struct QuestionMark<'arena> {
    pub l: usize,
    pub r: usize,
    pub phantom: PhantomData<&'arena ()>,
}

pub fn new_question_mark<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    _source: &str,
) -> QuestionMark<'arena> {
    QuestionMark {
        l,
        r,
        phantom: PhantomData,
    }
}

#[derive(Debug)]
pub struct ReceiveKeyword<'arena> {
    pub l: usize,
    pub r: usize,
    pub phantom: PhantomData<&'arena ()>,
}

pub fn new_receive_keyword<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    _source: &str,
) -> ReceiveKeyword<'arena> {
    ReceiveKeyword {
        l,
        r,
        phantom: PhantomData,
    }
}

#[derive(Debug)]
pub struct ReferenceKeyword<'arena> {
    pub l: usize,
    pub r: usize,
    pub phantom: PhantomData<&'arena ()>,
}

pub fn new_reference_keyword<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    _source: &str,
) -> ReferenceKeyword<'arena> {
    ReferenceKeyword {
        l,
        r,
        phantom: PhantomData,
    }
}

#[derive(Debug)]
pub struct RelocatableKeyword<'arena> {
    pub l: usize,
    pub r: usize,
    pub phantom: PhantomData<&'arena ()>,
}

pub fn new_relocatable_keyword<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    _source: &str,
) -> RelocatableKeyword<'arena> {
    RelocatableKeyword {
        l,
        r,
        phantom: PhantomData,
    }
}

#[derive(Debug)]
pub struct ReturnKeyword<'arena> {
    pub l: usize,
    pub r: usize,
    pub phantom: PhantomData<&'arena ()>,
}

pub fn new_return_keyword<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    _source: &str,
) -> ReturnKeyword<'arena> {
    ReturnKeyword {
        l,
        r,
        phantom: PhantomData,
    }
}

#[derive(Debug)]
pub struct ReturnsKeyword<'arena> {
    pub l: usize,
    pub r: usize,
    pub phantom: PhantomData<&'arena ()>,
}

pub fn new_returns_keyword<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    _source: &str,
) -> ReturnsKeyword<'arena> {
    ReturnsKeyword {
        l,
        r,
        phantom: PhantomData,
    }
}

#[derive(Debug)]
pub struct RevertKeyword<'arena> {
    pub l: usize,
    pub r: usize,
    pub phantom: PhantomData<&'arena ()>,
}

pub fn new_revert_keyword<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    _source: &str,
) -> RevertKeyword<'arena> {
    RevertKeyword {
        l,
        r,
        phantom: PhantomData,
    }
}

#[derive(Debug)]
pub struct SMTCheckerKeyword<'arena> {
    pub l: usize,
    pub r: usize,
    pub phantom: PhantomData<&'arena ()>,
}

pub fn new_smt_checker_keyword<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    _source: &str,
) -> SMTCheckerKeyword<'arena> {
    SMTCheckerKeyword {
        l,
        r,
        phantom: PhantomData,
    }
}

#[derive(Debug)]
pub struct SealedKeyword<'arena> {
    pub l: usize,
    pub r: usize,
    pub phantom: PhantomData<&'arena ()>,
}

pub fn new_sealed_keyword<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    _source: &str,
) -> SealedKeyword<'arena> {
    SealedKeyword {
        l,
        r,
        phantom: PhantomData,
    }
}

#[derive(Debug)]
pub struct SecondsKeyword<'arena> {
    pub l: usize,
    pub r: usize,
    pub phantom: PhantomData<&'arena ()>,
}

pub fn new_seconds_keyword<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    _source: &str,
) -> SecondsKeyword<'arena> {
    SecondsKeyword {
        l,
        r,
        phantom: PhantomData,
    }
}

#[derive(Debug)]
pub struct Semicolon<'arena> {
    pub l: usize,
    pub r: usize,
    pub phantom: PhantomData<&'arena ()>,
}

pub fn new_semicolon<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    _source: &str,
) -> Semicolon<'arena> {
    Semicolon {
        l,
        r,
        phantom: PhantomData,
    }
}

#[derive(Debug)]
pub struct SingleLineComment<'arena> {
    pub l: usize,
    pub r: usize,
    pub phantom: PhantomData<&'arena ()>,
}

pub fn new_single_line_comment<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    _source: &str,
) -> SingleLineComment<'arena> {
    SingleLineComment {
        l,
        r,
        phantom: PhantomData,
    }
}

#[derive(Debug)]
pub struct SingleLineNatSpecComment<'arena> {
    pub l: usize,
    pub r: usize,
    pub phantom: PhantomData<&'arena ()>,
}

pub fn new_single_line_nat_spec_comment<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    _source: &str,
) -> SingleLineNatSpecComment<'arena> {
    SingleLineNatSpecComment {
        l,
        r,
        phantom: PhantomData,
    }
}

#[derive(Debug)]
pub struct SingleQuotedHexStringLiteral<'arena> {
    pub l: usize,
    pub r: usize,
    pub phantom: PhantomData<&'arena ()>,
}

pub fn new_single_quoted_hex_string_literal<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    _source: &str,
) -> SingleQuotedHexStringLiteral<'arena> {
    SingleQuotedHexStringLiteral {
        l,
        r,
        phantom: PhantomData,
    }
}

#[derive(Debug)]
pub struct SingleQuotedStringLiteral<'arena> {
    pub l: usize,
    pub r: usize,
    pub phantom: PhantomData<&'arena ()>,
}

pub fn new_single_quoted_string_literal<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    _source: &str,
) -> SingleQuotedStringLiteral<'arena> {
    SingleQuotedStringLiteral {
        l,
        r,
        phantom: PhantomData,
    }
}

#[derive(Debug)]
pub struct SingleQuotedUnicodeStringLiteral<'arena> {
    pub l: usize,
    pub r: usize,
    pub phantom: PhantomData<&'arena ()>,
}

pub fn new_single_quoted_unicode_string_literal<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    _source: &str,
) -> SingleQuotedUnicodeStringLiteral<'arena> {
    SingleQuotedUnicodeStringLiteral {
        l,
        r,
        phantom: PhantomData,
    }
}

#[derive(Debug)]
pub struct SingleQuotedVersionLiteral<'arena> {
    pub l: usize,
    pub r: usize,
    pub phantom: PhantomData<&'arena ()>,
}

pub fn new_single_quoted_version_literal<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    _source: &str,
) -> SingleQuotedVersionLiteral<'arena> {
    SingleQuotedVersionLiteral {
        l,
        r,
        phantom: PhantomData,
    }
}

#[derive(Debug)]
pub struct SizeOfKeyword<'arena> {
    pub l: usize,
    pub r: usize,
    pub phantom: PhantomData<&'arena ()>,
}

pub fn new_size_of_keyword<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    _source: &str,
) -> SizeOfKeyword<'arena> {
    SizeOfKeyword {
        l,
        r,
        phantom: PhantomData,
    }
}

#[derive(Debug)]
pub struct Slash<'arena> {
    pub l: usize,
    pub r: usize,
    pub phantom: PhantomData<&'arena ()>,
}

pub fn new_slash<'arena>(_arena: &'arena Bump, l: usize, r: usize, _source: &str) -> Slash<'arena> {
    Slash {
        l,
        r,
        phantom: PhantomData,
    }
}

#[derive(Debug)]
pub struct SlashEqual<'arena> {
    pub l: usize,
    pub r: usize,
    pub phantom: PhantomData<&'arena ()>,
}

pub fn new_slash_equal<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    _source: &str,
) -> SlashEqual<'arena> {
    SlashEqual {
        l,
        r,
        phantom: PhantomData,
    }
}

#[derive(Debug)]
pub struct SolidityKeyword<'arena> {
    pub l: usize,
    pub r: usize,
    pub phantom: PhantomData<&'arena ()>,
}

pub fn new_solidity_keyword<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    _source: &str,
) -> SolidityKeyword<'arena> {
    SolidityKeyword {
        l,
        r,
        phantom: PhantomData,
    }
}

#[derive(Debug)]
pub struct StaticKeyword<'arena> {
    pub l: usize,
    pub r: usize,
    pub phantom: PhantomData<&'arena ()>,
}

pub fn new_static_keyword<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    _source: &str,
) -> StaticKeyword<'arena> {
    StaticKeyword {
        l,
        r,
        phantom: PhantomData,
    }
}

#[derive(Debug)]
pub struct StorageKeyword<'arena> {
    pub l: usize,
    pub r: usize,
    pub phantom: PhantomData<&'arena ()>,
}

pub fn new_storage_keyword<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    _source: &str,
) -> StorageKeyword<'arena> {
    StorageKeyword {
        l,
        r,
        phantom: PhantomData,
    }
}

#[derive(Debug)]
pub struct StringKeyword<'arena> {
    pub l: usize,
    pub r: usize,
    pub phantom: PhantomData<&'arena ()>,
}

pub fn new_string_keyword<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    _source: &str,
) -> StringKeyword<'arena> {
    StringKeyword {
        l,
        r,
        phantom: PhantomData,
    }
}

#[derive(Debug)]
pub struct StructKeyword<'arena> {
    pub l: usize,
    pub r: usize,
    pub phantom: PhantomData<&'arena ()>,
}

pub fn new_struct_keyword<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    _source: &str,
) -> StructKeyword<'arena> {
    StructKeyword {
        l,
        r,
        phantom: PhantomData,
    }
}

#[derive(Debug)]
pub struct SuperKeyword<'arena> {
    pub l: usize,
    pub r: usize,
    pub phantom: PhantomData<&'arena ()>,
}

pub fn new_super_keyword<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    _source: &str,
) -> SuperKeyword<'arena> {
    SuperKeyword {
        l,
        r,
        phantom: PhantomData,
    }
}

#[derive(Debug)]
pub struct SupportsKeyword<'arena> {
    pub l: usize,
    pub r: usize,
    pub phantom: PhantomData<&'arena ()>,
}

pub fn new_supports_keyword<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    _source: &str,
) -> SupportsKeyword<'arena> {
    SupportsKeyword {
        l,
        r,
        phantom: PhantomData,
    }
}

#[derive(Debug)]
pub struct SwitchKeyword<'arena> {
    pub l: usize,
    pub r: usize,
    pub phantom: PhantomData<&'arena ()>,
}

pub fn new_switch_keyword<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    _source: &str,
) -> SwitchKeyword<'arena> {
    SwitchKeyword {
        l,
        r,
        phantom: PhantomData,
    }
}

#[derive(Debug)]
pub struct SzaboKeyword<'arena> {
    pub l: usize,
    pub r: usize,
    pub phantom: PhantomData<&'arena ()>,
}

pub fn new_szabo_keyword<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    _source: &str,
) -> SzaboKeyword<'arena> {
    SzaboKeyword {
        l,
        r,
        phantom: PhantomData,
    }
}

#[derive(Debug)]
pub struct ThisKeyword<'arena> {
    pub l: usize,
    pub r: usize,
    pub phantom: PhantomData<&'arena ()>,
}

pub fn new_this_keyword<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    _source: &str,
) -> ThisKeyword<'arena> {
    ThisKeyword {
        l,
        r,
        phantom: PhantomData,
    }
}

#[derive(Debug)]
pub struct ThrowKeyword<'arena> {
    pub l: usize,
    pub r: usize,
    pub phantom: PhantomData<&'arena ()>,
}

pub fn new_throw_keyword<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    _source: &str,
) -> ThrowKeyword<'arena> {
    ThrowKeyword {
        l,
        r,
        phantom: PhantomData,
    }
}

#[derive(Debug)]
pub struct Tilde<'arena> {
    pub l: usize,
    pub r: usize,
    pub phantom: PhantomData<&'arena ()>,
}

pub fn new_tilde<'arena>(_arena: &'arena Bump, l: usize, r: usize, _source: &str) -> Tilde<'arena> {
    Tilde {
        l,
        r,
        phantom: PhantomData,
    }
}

#[derive(Debug)]
pub struct TransientKeyword<'arena> {
    pub l: usize,
    pub r: usize,
    pub phantom: PhantomData<&'arena ()>,
}

pub fn new_transient_keyword<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    _source: &str,
) -> TransientKeyword<'arena> {
    TransientKeyword {
        l,
        r,
        phantom: PhantomData,
    }
}

#[derive(Debug)]
pub struct TrueKeyword<'arena> {
    pub l: usize,
    pub r: usize,
    pub phantom: PhantomData<&'arena ()>,
}

pub fn new_true_keyword<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    _source: &str,
) -> TrueKeyword<'arena> {
    TrueKeyword {
        l,
        r,
        phantom: PhantomData,
    }
}

#[derive(Debug)]
pub struct TryKeyword<'arena> {
    pub l: usize,
    pub r: usize,
    pub phantom: PhantomData<&'arena ()>,
}

pub fn new_try_keyword<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    _source: &str,
) -> TryKeyword<'arena> {
    TryKeyword {
        l,
        r,
        phantom: PhantomData,
    }
}

#[derive(Debug)]
pub struct TypeDefKeyword<'arena> {
    pub l: usize,
    pub r: usize,
    pub phantom: PhantomData<&'arena ()>,
}

pub fn new_type_def_keyword<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    _source: &str,
) -> TypeDefKeyword<'arena> {
    TypeDefKeyword {
        l,
        r,
        phantom: PhantomData,
    }
}

#[derive(Debug)]
pub struct TypeKeyword<'arena> {
    pub l: usize,
    pub r: usize,
    pub phantom: PhantomData<&'arena ()>,
}

pub fn new_type_keyword<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    _source: &str,
) -> TypeKeyword<'arena> {
    TypeKeyword {
        l,
        r,
        phantom: PhantomData,
    }
}

#[derive(Debug)]
pub struct TypeOfKeyword<'arena> {
    pub l: usize,
    pub r: usize,
    pub phantom: PhantomData<&'arena ()>,
}

pub fn new_type_of_keyword<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    _source: &str,
) -> TypeOfKeyword<'arena> {
    TypeOfKeyword {
        l,
        r,
        phantom: PhantomData,
    }
}

#[derive(Debug)]
pub struct UfixedKeyword<'arena> {
    pub l: usize,
    pub r: usize,
    pub phantom: PhantomData<&'arena ()>,
}

pub fn new_ufixed_keyword<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    _source: &str,
) -> UfixedKeyword<'arena> {
    UfixedKeyword {
        l,
        r,
        phantom: PhantomData,
    }
}

#[derive(Debug)]
pub struct UintKeyword<'arena> {
    pub l: usize,
    pub r: usize,
    pub phantom: PhantomData<&'arena ()>,
}

pub fn new_uint_keyword<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    _source: &str,
) -> UintKeyword<'arena> {
    UintKeyword {
        l,
        r,
        phantom: PhantomData,
    }
}

#[derive(Debug)]
pub struct UncheckedKeyword<'arena> {
    pub l: usize,
    pub r: usize,
    pub phantom: PhantomData<&'arena ()>,
}

pub fn new_unchecked_keyword<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    _source: &str,
) -> UncheckedKeyword<'arena> {
    UncheckedKeyword {
        l,
        r,
        phantom: PhantomData,
    }
}

#[derive(Debug)]
pub struct UsingKeyword<'arena> {
    pub l: usize,
    pub r: usize,
    pub phantom: PhantomData<&'arena ()>,
}

pub fn new_using_keyword<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    _source: &str,
) -> UsingKeyword<'arena> {
    UsingKeyword {
        l,
        r,
        phantom: PhantomData,
    }
}

#[derive(Debug)]
pub struct VarKeyword<'arena> {
    pub l: usize,
    pub r: usize,
    pub phantom: PhantomData<&'arena ()>,
}

pub fn new_var_keyword<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    _source: &str,
) -> VarKeyword<'arena> {
    VarKeyword {
        l,
        r,
        phantom: PhantomData,
    }
}

#[derive(Debug)]
pub struct VersionSpecifier<'arena> {
    pub l: usize,
    pub r: usize,
    pub phantom: PhantomData<&'arena ()>,
}

pub fn new_version_specifier<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    _source: &str,
) -> VersionSpecifier<'arena> {
    VersionSpecifier {
        l,
        r,
        phantom: PhantomData,
    }
}

#[derive(Debug)]
pub struct ViewKeyword<'arena> {
    pub l: usize,
    pub r: usize,
    pub phantom: PhantomData<&'arena ()>,
}

pub fn new_view_keyword<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    _source: &str,
) -> ViewKeyword<'arena> {
    ViewKeyword {
        l,
        r,
        phantom: PhantomData,
    }
}

#[derive(Debug)]
pub struct VirtualKeyword<'arena> {
    pub l: usize,
    pub r: usize,
    pub phantom: PhantomData<&'arena ()>,
}

pub fn new_virtual_keyword<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    _source: &str,
) -> VirtualKeyword<'arena> {
    VirtualKeyword {
        l,
        r,
        phantom: PhantomData,
    }
}

#[derive(Debug)]
pub struct WeeksKeyword<'arena> {
    pub l: usize,
    pub r: usize,
    pub phantom: PhantomData<&'arena ()>,
}

pub fn new_weeks_keyword<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    _source: &str,
) -> WeeksKeyword<'arena> {
    WeeksKeyword {
        l,
        r,
        phantom: PhantomData,
    }
}

#[derive(Debug)]
pub struct WeiKeyword<'arena> {
    pub l: usize,
    pub r: usize,
    pub phantom: PhantomData<&'arena ()>,
}

pub fn new_wei_keyword<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    _source: &str,
) -> WeiKeyword<'arena> {
    WeiKeyword {
        l,
        r,
        phantom: PhantomData,
    }
}

#[derive(Debug)]
pub struct WhileKeyword<'arena> {
    pub l: usize,
    pub r: usize,
    pub phantom: PhantomData<&'arena ()>,
}

pub fn new_while_keyword<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    _source: &str,
) -> WhileKeyword<'arena> {
    WhileKeyword {
        l,
        r,
        phantom: PhantomData,
    }
}

#[derive(Debug)]
pub struct Whitespace<'arena> {
    pub l: usize,
    pub r: usize,
    pub phantom: PhantomData<&'arena ()>,
}

pub fn new_whitespace<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    _source: &str,
) -> Whitespace<'arena> {
    Whitespace {
        l,
        r,
        phantom: PhantomData,
    }
}

#[derive(Debug)]
pub struct YearsKeyword<'arena> {
    pub l: usize,
    pub r: usize,
    pub phantom: PhantomData<&'arena ()>,
}

pub fn new_years_keyword<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    _source: &str,
) -> YearsKeyword<'arena> {
    YearsKeyword {
        l,
        r,
        phantom: PhantomData,
    }
}

#[derive(Debug)]
pub struct YulAbstractKeyword<'arena> {
    pub l: usize,
    pub r: usize,
    pub phantom: PhantomData<&'arena ()>,
}

pub fn new_yul_abstract_keyword<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    _source: &str,
) -> YulAbstractKeyword<'arena> {
    YulAbstractKeyword {
        l,
        r,
        phantom: PhantomData,
    }
}

#[derive(Debug)]
pub struct YulAfterKeyword<'arena> {
    pub l: usize,
    pub r: usize,
    pub phantom: PhantomData<&'arena ()>,
}

pub fn new_yul_after_keyword<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    _source: &str,
) -> YulAfterKeyword<'arena> {
    YulAfterKeyword {
        l,
        r,
        phantom: PhantomData,
    }
}

#[derive(Debug)]
pub struct YulAliasKeyword<'arena> {
    pub l: usize,
    pub r: usize,
    pub phantom: PhantomData<&'arena ()>,
}

pub fn new_yul_alias_keyword<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    _source: &str,
) -> YulAliasKeyword<'arena> {
    YulAliasKeyword {
        l,
        r,
        phantom: PhantomData,
    }
}

#[derive(Debug)]
pub struct YulAnonymousKeyword<'arena> {
    pub l: usize,
    pub r: usize,
    pub phantom: PhantomData<&'arena ()>,
}

pub fn new_yul_anonymous_keyword<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    _source: &str,
) -> YulAnonymousKeyword<'arena> {
    YulAnonymousKeyword {
        l,
        r,
        phantom: PhantomData,
    }
}

#[derive(Debug)]
pub struct YulApplyKeyword<'arena> {
    pub l: usize,
    pub r: usize,
    pub phantom: PhantomData<&'arena ()>,
}

pub fn new_yul_apply_keyword<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    _source: &str,
) -> YulApplyKeyword<'arena> {
    YulApplyKeyword {
        l,
        r,
        phantom: PhantomData,
    }
}

#[derive(Debug)]
pub struct YulAsKeyword<'arena> {
    pub l: usize,
    pub r: usize,
    pub phantom: PhantomData<&'arena ()>,
}

pub fn new_yul_as_keyword<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    _source: &str,
) -> YulAsKeyword<'arena> {
    YulAsKeyword {
        l,
        r,
        phantom: PhantomData,
    }
}

#[derive(Debug)]
pub struct YulAssemblyKeyword<'arena> {
    pub l: usize,
    pub r: usize,
    pub phantom: PhantomData<&'arena ()>,
}

pub fn new_yul_assembly_keyword<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    _source: &str,
) -> YulAssemblyKeyword<'arena> {
    YulAssemblyKeyword {
        l,
        r,
        phantom: PhantomData,
    }
}

#[derive(Debug)]
pub struct YulAutoKeyword<'arena> {
    pub l: usize,
    pub r: usize,
    pub phantom: PhantomData<&'arena ()>,
}

pub fn new_yul_auto_keyword<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    _source: &str,
) -> YulAutoKeyword<'arena> {
    YulAutoKeyword {
        l,
        r,
        phantom: PhantomData,
    }
}

#[derive(Debug)]
pub struct YulBoolKeyword<'arena> {
    pub l: usize,
    pub r: usize,
    pub phantom: PhantomData<&'arena ()>,
}

pub fn new_yul_bool_keyword<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    _source: &str,
) -> YulBoolKeyword<'arena> {
    YulBoolKeyword {
        l,
        r,
        phantom: PhantomData,
    }
}

#[derive(Debug)]
pub struct YulBreakKeyword<'arena> {
    pub l: usize,
    pub r: usize,
    pub phantom: PhantomData<&'arena ()>,
}

pub fn new_yul_break_keyword<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    _source: &str,
) -> YulBreakKeyword<'arena> {
    YulBreakKeyword {
        l,
        r,
        phantom: PhantomData,
    }
}

#[derive(Debug)]
pub struct YulBytesKeyword<'arena> {
    pub l: usize,
    pub r: usize,
    pub phantom: PhantomData<&'arena ()>,
}

pub fn new_yul_bytes_keyword<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    _source: &str,
) -> YulBytesKeyword<'arena> {
    YulBytesKeyword {
        l,
        r,
        phantom: PhantomData,
    }
}

#[derive(Debug)]
pub struct YulCallDataKeyword<'arena> {
    pub l: usize,
    pub r: usize,
    pub phantom: PhantomData<&'arena ()>,
}

pub fn new_yul_call_data_keyword<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    _source: &str,
) -> YulCallDataKeyword<'arena> {
    YulCallDataKeyword {
        l,
        r,
        phantom: PhantomData,
    }
}

#[derive(Debug)]
pub struct YulCaseKeyword<'arena> {
    pub l: usize,
    pub r: usize,
    pub phantom: PhantomData<&'arena ()>,
}

pub fn new_yul_case_keyword<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    _source: &str,
) -> YulCaseKeyword<'arena> {
    YulCaseKeyword {
        l,
        r,
        phantom: PhantomData,
    }
}

#[derive(Debug)]
pub struct YulCatchKeyword<'arena> {
    pub l: usize,
    pub r: usize,
    pub phantom: PhantomData<&'arena ()>,
}

pub fn new_yul_catch_keyword<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    _source: &str,
) -> YulCatchKeyword<'arena> {
    YulCatchKeyword {
        l,
        r,
        phantom: PhantomData,
    }
}

#[derive(Debug)]
pub struct YulConstantKeyword<'arena> {
    pub l: usize,
    pub r: usize,
    pub phantom: PhantomData<&'arena ()>,
}

pub fn new_yul_constant_keyword<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    _source: &str,
) -> YulConstantKeyword<'arena> {
    YulConstantKeyword {
        l,
        r,
        phantom: PhantomData,
    }
}

#[derive(Debug)]
pub struct YulConstructorKeyword<'arena> {
    pub l: usize,
    pub r: usize,
    pub phantom: PhantomData<&'arena ()>,
}

pub fn new_yul_constructor_keyword<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    _source: &str,
) -> YulConstructorKeyword<'arena> {
    YulConstructorKeyword {
        l,
        r,
        phantom: PhantomData,
    }
}

#[derive(Debug)]
pub struct YulContinueKeyword<'arena> {
    pub l: usize,
    pub r: usize,
    pub phantom: PhantomData<&'arena ()>,
}

pub fn new_yul_continue_keyword<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    _source: &str,
) -> YulContinueKeyword<'arena> {
    YulContinueKeyword {
        l,
        r,
        phantom: PhantomData,
    }
}

#[derive(Debug)]
pub struct YulContractKeyword<'arena> {
    pub l: usize,
    pub r: usize,
    pub phantom: PhantomData<&'arena ()>,
}

pub fn new_yul_contract_keyword<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    _source: &str,
) -> YulContractKeyword<'arena> {
    YulContractKeyword {
        l,
        r,
        phantom: PhantomData,
    }
}

#[derive(Debug)]
pub struct YulCopyOfKeyword<'arena> {
    pub l: usize,
    pub r: usize,
    pub phantom: PhantomData<&'arena ()>,
}

pub fn new_yul_copy_of_keyword<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    _source: &str,
) -> YulCopyOfKeyword<'arena> {
    YulCopyOfKeyword {
        l,
        r,
        phantom: PhantomData,
    }
}

#[derive(Debug)]
pub struct YulDaysKeyword<'arena> {
    pub l: usize,
    pub r: usize,
    pub phantom: PhantomData<&'arena ()>,
}

pub fn new_yul_days_keyword<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    _source: &str,
) -> YulDaysKeyword<'arena> {
    YulDaysKeyword {
        l,
        r,
        phantom: PhantomData,
    }
}

#[derive(Debug)]
pub struct YulDecimalLiteral<'arena> {
    pub l: usize,
    pub r: usize,
    pub phantom: PhantomData<&'arena ()>,
}

pub fn new_yul_decimal_literal<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    _source: &str,
) -> YulDecimalLiteral<'arena> {
    YulDecimalLiteral {
        l,
        r,
        phantom: PhantomData,
    }
}

#[derive(Debug)]
pub struct YulDefaultKeyword<'arena> {
    pub l: usize,
    pub r: usize,
    pub phantom: PhantomData<&'arena ()>,
}

pub fn new_yul_default_keyword<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    _source: &str,
) -> YulDefaultKeyword<'arena> {
    YulDefaultKeyword {
        l,
        r,
        phantom: PhantomData,
    }
}

#[derive(Debug)]
pub struct YulDefineKeyword<'arena> {
    pub l: usize,
    pub r: usize,
    pub phantom: PhantomData<&'arena ()>,
}

pub fn new_yul_define_keyword<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    _source: &str,
) -> YulDefineKeyword<'arena> {
    YulDefineKeyword {
        l,
        r,
        phantom: PhantomData,
    }
}

#[derive(Debug)]
pub struct YulDeleteKeyword<'arena> {
    pub l: usize,
    pub r: usize,
    pub phantom: PhantomData<&'arena ()>,
}

pub fn new_yul_delete_keyword<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    _source: &str,
) -> YulDeleteKeyword<'arena> {
    YulDeleteKeyword {
        l,
        r,
        phantom: PhantomData,
    }
}

#[derive(Debug)]
pub struct YulDoKeyword<'arena> {
    pub l: usize,
    pub r: usize,
    pub phantom: PhantomData<&'arena ()>,
}

pub fn new_yul_do_keyword<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    _source: &str,
) -> YulDoKeyword<'arena> {
    YulDoKeyword {
        l,
        r,
        phantom: PhantomData,
    }
}

#[derive(Debug)]
pub struct YulElseKeyword<'arena> {
    pub l: usize,
    pub r: usize,
    pub phantom: PhantomData<&'arena ()>,
}

pub fn new_yul_else_keyword<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    _source: &str,
) -> YulElseKeyword<'arena> {
    YulElseKeyword {
        l,
        r,
        phantom: PhantomData,
    }
}

#[derive(Debug)]
pub struct YulEmitKeyword<'arena> {
    pub l: usize,
    pub r: usize,
    pub phantom: PhantomData<&'arena ()>,
}

pub fn new_yul_emit_keyword<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    _source: &str,
) -> YulEmitKeyword<'arena> {
    YulEmitKeyword {
        l,
        r,
        phantom: PhantomData,
    }
}

#[derive(Debug)]
pub struct YulEnumKeyword<'arena> {
    pub l: usize,
    pub r: usize,
    pub phantom: PhantomData<&'arena ()>,
}

pub fn new_yul_enum_keyword<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    _source: &str,
) -> YulEnumKeyword<'arena> {
    YulEnumKeyword {
        l,
        r,
        phantom: PhantomData,
    }
}

#[derive(Debug)]
pub struct YulEtherKeyword<'arena> {
    pub l: usize,
    pub r: usize,
    pub phantom: PhantomData<&'arena ()>,
}

pub fn new_yul_ether_keyword<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    _source: &str,
) -> YulEtherKeyword<'arena> {
    YulEtherKeyword {
        l,
        r,
        phantom: PhantomData,
    }
}

#[derive(Debug)]
pub struct YulEventKeyword<'arena> {
    pub l: usize,
    pub r: usize,
    pub phantom: PhantomData<&'arena ()>,
}

pub fn new_yul_event_keyword<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    _source: &str,
) -> YulEventKeyword<'arena> {
    YulEventKeyword {
        l,
        r,
        phantom: PhantomData,
    }
}

#[derive(Debug)]
pub struct YulExternalKeyword<'arena> {
    pub l: usize,
    pub r: usize,
    pub phantom: PhantomData<&'arena ()>,
}

pub fn new_yul_external_keyword<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    _source: &str,
) -> YulExternalKeyword<'arena> {
    YulExternalKeyword {
        l,
        r,
        phantom: PhantomData,
    }
}

#[derive(Debug)]
pub struct YulFallbackKeyword<'arena> {
    pub l: usize,
    pub r: usize,
    pub phantom: PhantomData<&'arena ()>,
}

pub fn new_yul_fallback_keyword<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    _source: &str,
) -> YulFallbackKeyword<'arena> {
    YulFallbackKeyword {
        l,
        r,
        phantom: PhantomData,
    }
}

#[derive(Debug)]
pub struct YulFalseKeyword<'arena> {
    pub l: usize,
    pub r: usize,
    pub phantom: PhantomData<&'arena ()>,
}

pub fn new_yul_false_keyword<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    _source: &str,
) -> YulFalseKeyword<'arena> {
    YulFalseKeyword {
        l,
        r,
        phantom: PhantomData,
    }
}

#[derive(Debug)]
pub struct YulFinalKeyword<'arena> {
    pub l: usize,
    pub r: usize,
    pub phantom: PhantomData<&'arena ()>,
}

pub fn new_yul_final_keyword<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    _source: &str,
) -> YulFinalKeyword<'arena> {
    YulFinalKeyword {
        l,
        r,
        phantom: PhantomData,
    }
}

#[derive(Debug)]
pub struct YulFinneyKeyword<'arena> {
    pub l: usize,
    pub r: usize,
    pub phantom: PhantomData<&'arena ()>,
}

pub fn new_yul_finney_keyword<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    _source: &str,
) -> YulFinneyKeyword<'arena> {
    YulFinneyKeyword {
        l,
        r,
        phantom: PhantomData,
    }
}

#[derive(Debug)]
pub struct YulFixedKeyword<'arena> {
    pub l: usize,
    pub r: usize,
    pub phantom: PhantomData<&'arena ()>,
}

pub fn new_yul_fixed_keyword<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    _source: &str,
) -> YulFixedKeyword<'arena> {
    YulFixedKeyword {
        l,
        r,
        phantom: PhantomData,
    }
}

#[derive(Debug)]
pub struct YulForKeyword<'arena> {
    pub l: usize,
    pub r: usize,
    pub phantom: PhantomData<&'arena ()>,
}

pub fn new_yul_for_keyword<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    _source: &str,
) -> YulForKeyword<'arena> {
    YulForKeyword {
        l,
        r,
        phantom: PhantomData,
    }
}

#[derive(Debug)]
pub struct YulFunctionKeyword<'arena> {
    pub l: usize,
    pub r: usize,
    pub phantom: PhantomData<&'arena ()>,
}

pub fn new_yul_function_keyword<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    _source: &str,
) -> YulFunctionKeyword<'arena> {
    YulFunctionKeyword {
        l,
        r,
        phantom: PhantomData,
    }
}

#[derive(Debug)]
pub struct YulGweiKeyword<'arena> {
    pub l: usize,
    pub r: usize,
    pub phantom: PhantomData<&'arena ()>,
}

pub fn new_yul_gwei_keyword<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    _source: &str,
) -> YulGweiKeyword<'arena> {
    YulGweiKeyword {
        l,
        r,
        phantom: PhantomData,
    }
}

#[derive(Debug)]
pub struct YulHexKeyword<'arena> {
    pub l: usize,
    pub r: usize,
    pub phantom: PhantomData<&'arena ()>,
}

pub fn new_yul_hex_keyword<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    _source: &str,
) -> YulHexKeyword<'arena> {
    YulHexKeyword {
        l,
        r,
        phantom: PhantomData,
    }
}

#[derive(Debug)]
pub struct YulHexLiteral<'arena> {
    pub l: usize,
    pub r: usize,
    pub phantom: PhantomData<&'arena ()>,
}

pub fn new_yul_hex_literal<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    _source: &str,
) -> YulHexLiteral<'arena> {
    YulHexLiteral {
        l,
        r,
        phantom: PhantomData,
    }
}

#[derive(Debug)]
pub struct YulHoursKeyword<'arena> {
    pub l: usize,
    pub r: usize,
    pub phantom: PhantomData<&'arena ()>,
}

pub fn new_yul_hours_keyword<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    _source: &str,
) -> YulHoursKeyword<'arena> {
    YulHoursKeyword {
        l,
        r,
        phantom: PhantomData,
    }
}

#[derive(Debug)]
pub struct YulIdentifier<'arena> {
    pub l: usize,
    pub r: usize,
    pub phantom: PhantomData<&'arena ()>,
}

pub fn new_yul_identifier<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    _source: &str,
) -> YulIdentifier<'arena> {
    YulIdentifier {
        l,
        r,
        phantom: PhantomData,
    }
}

#[derive(Debug)]
pub struct YulIfKeyword<'arena> {
    pub l: usize,
    pub r: usize,
    pub phantom: PhantomData<&'arena ()>,
}

pub fn new_yul_if_keyword<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    _source: &str,
) -> YulIfKeyword<'arena> {
    YulIfKeyword {
        l,
        r,
        phantom: PhantomData,
    }
}

#[derive(Debug)]
pub struct YulImmutableKeyword<'arena> {
    pub l: usize,
    pub r: usize,
    pub phantom: PhantomData<&'arena ()>,
}

pub fn new_yul_immutable_keyword<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    _source: &str,
) -> YulImmutableKeyword<'arena> {
    YulImmutableKeyword {
        l,
        r,
        phantom: PhantomData,
    }
}

#[derive(Debug)]
pub struct YulImplementsKeyword<'arena> {
    pub l: usize,
    pub r: usize,
    pub phantom: PhantomData<&'arena ()>,
}

pub fn new_yul_implements_keyword<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    _source: &str,
) -> YulImplementsKeyword<'arena> {
    YulImplementsKeyword {
        l,
        r,
        phantom: PhantomData,
    }
}

#[derive(Debug)]
pub struct YulImportKeyword<'arena> {
    pub l: usize,
    pub r: usize,
    pub phantom: PhantomData<&'arena ()>,
}

pub fn new_yul_import_keyword<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    _source: &str,
) -> YulImportKeyword<'arena> {
    YulImportKeyword {
        l,
        r,
        phantom: PhantomData,
    }
}

#[derive(Debug)]
pub struct YulInKeyword<'arena> {
    pub l: usize,
    pub r: usize,
    pub phantom: PhantomData<&'arena ()>,
}

pub fn new_yul_in_keyword<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    _source: &str,
) -> YulInKeyword<'arena> {
    YulInKeyword {
        l,
        r,
        phantom: PhantomData,
    }
}

#[derive(Debug)]
pub struct YulIndexedKeyword<'arena> {
    pub l: usize,
    pub r: usize,
    pub phantom: PhantomData<&'arena ()>,
}

pub fn new_yul_indexed_keyword<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    _source: &str,
) -> YulIndexedKeyword<'arena> {
    YulIndexedKeyword {
        l,
        r,
        phantom: PhantomData,
    }
}

#[derive(Debug)]
pub struct YulInlineKeyword<'arena> {
    pub l: usize,
    pub r: usize,
    pub phantom: PhantomData<&'arena ()>,
}

pub fn new_yul_inline_keyword<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    _source: &str,
) -> YulInlineKeyword<'arena> {
    YulInlineKeyword {
        l,
        r,
        phantom: PhantomData,
    }
}

#[derive(Debug)]
pub struct YulIntKeyword<'arena> {
    pub l: usize,
    pub r: usize,
    pub phantom: PhantomData<&'arena ()>,
}

pub fn new_yul_int_keyword<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    _source: &str,
) -> YulIntKeyword<'arena> {
    YulIntKeyword {
        l,
        r,
        phantom: PhantomData,
    }
}

#[derive(Debug)]
pub struct YulInterfaceKeyword<'arena> {
    pub l: usize,
    pub r: usize,
    pub phantom: PhantomData<&'arena ()>,
}

pub fn new_yul_interface_keyword<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    _source: &str,
) -> YulInterfaceKeyword<'arena> {
    YulInterfaceKeyword {
        l,
        r,
        phantom: PhantomData,
    }
}

#[derive(Debug)]
pub struct YulInternalKeyword<'arena> {
    pub l: usize,
    pub r: usize,
    pub phantom: PhantomData<&'arena ()>,
}

pub fn new_yul_internal_keyword<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    _source: &str,
) -> YulInternalKeyword<'arena> {
    YulInternalKeyword {
        l,
        r,
        phantom: PhantomData,
    }
}

#[derive(Debug)]
pub struct YulIsKeyword<'arena> {
    pub l: usize,
    pub r: usize,
    pub phantom: PhantomData<&'arena ()>,
}

pub fn new_yul_is_keyword<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    _source: &str,
) -> YulIsKeyword<'arena> {
    YulIsKeyword {
        l,
        r,
        phantom: PhantomData,
    }
}

#[derive(Debug)]
pub struct YulLeaveKeyword<'arena> {
    pub l: usize,
    pub r: usize,
    pub phantom: PhantomData<&'arena ()>,
}

pub fn new_yul_leave_keyword<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    _source: &str,
) -> YulLeaveKeyword<'arena> {
    YulLeaveKeyword {
        l,
        r,
        phantom: PhantomData,
    }
}

#[derive(Debug)]
pub struct YulLetKeyword<'arena> {
    pub l: usize,
    pub r: usize,
    pub phantom: PhantomData<&'arena ()>,
}

pub fn new_yul_let_keyword<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    _source: &str,
) -> YulLetKeyword<'arena> {
    YulLetKeyword {
        l,
        r,
        phantom: PhantomData,
    }
}

#[derive(Debug)]
pub struct YulLibraryKeyword<'arena> {
    pub l: usize,
    pub r: usize,
    pub phantom: PhantomData<&'arena ()>,
}

pub fn new_yul_library_keyword<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    _source: &str,
) -> YulLibraryKeyword<'arena> {
    YulLibraryKeyword {
        l,
        r,
        phantom: PhantomData,
    }
}

#[derive(Debug)]
pub struct YulMacroKeyword<'arena> {
    pub l: usize,
    pub r: usize,
    pub phantom: PhantomData<&'arena ()>,
}

pub fn new_yul_macro_keyword<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    _source: &str,
) -> YulMacroKeyword<'arena> {
    YulMacroKeyword {
        l,
        r,
        phantom: PhantomData,
    }
}

#[derive(Debug)]
pub struct YulMappingKeyword<'arena> {
    pub l: usize,
    pub r: usize,
    pub phantom: PhantomData<&'arena ()>,
}

pub fn new_yul_mapping_keyword<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    _source: &str,
) -> YulMappingKeyword<'arena> {
    YulMappingKeyword {
        l,
        r,
        phantom: PhantomData,
    }
}

#[derive(Debug)]
pub struct YulMatchKeyword<'arena> {
    pub l: usize,
    pub r: usize,
    pub phantom: PhantomData<&'arena ()>,
}

pub fn new_yul_match_keyword<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    _source: &str,
) -> YulMatchKeyword<'arena> {
    YulMatchKeyword {
        l,
        r,
        phantom: PhantomData,
    }
}

#[derive(Debug)]
pub struct YulMemoryKeyword<'arena> {
    pub l: usize,
    pub r: usize,
    pub phantom: PhantomData<&'arena ()>,
}

pub fn new_yul_memory_keyword<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    _source: &str,
) -> YulMemoryKeyword<'arena> {
    YulMemoryKeyword {
        l,
        r,
        phantom: PhantomData,
    }
}

#[derive(Debug)]
pub struct YulMinutesKeyword<'arena> {
    pub l: usize,
    pub r: usize,
    pub phantom: PhantomData<&'arena ()>,
}

pub fn new_yul_minutes_keyword<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    _source: &str,
) -> YulMinutesKeyword<'arena> {
    YulMinutesKeyword {
        l,
        r,
        phantom: PhantomData,
    }
}

#[derive(Debug)]
pub struct YulModifierKeyword<'arena> {
    pub l: usize,
    pub r: usize,
    pub phantom: PhantomData<&'arena ()>,
}

pub fn new_yul_modifier_keyword<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    _source: &str,
) -> YulModifierKeyword<'arena> {
    YulModifierKeyword {
        l,
        r,
        phantom: PhantomData,
    }
}

#[derive(Debug)]
pub struct YulMutableKeyword<'arena> {
    pub l: usize,
    pub r: usize,
    pub phantom: PhantomData<&'arena ()>,
}

pub fn new_yul_mutable_keyword<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    _source: &str,
) -> YulMutableKeyword<'arena> {
    YulMutableKeyword {
        l,
        r,
        phantom: PhantomData,
    }
}

#[derive(Debug)]
pub struct YulNewKeyword<'arena> {
    pub l: usize,
    pub r: usize,
    pub phantom: PhantomData<&'arena ()>,
}

pub fn new_yul_new_keyword<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    _source: &str,
) -> YulNewKeyword<'arena> {
    YulNewKeyword {
        l,
        r,
        phantom: PhantomData,
    }
}

#[derive(Debug)]
pub struct YulNullKeyword<'arena> {
    pub l: usize,
    pub r: usize,
    pub phantom: PhantomData<&'arena ()>,
}

pub fn new_yul_null_keyword<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    _source: &str,
) -> YulNullKeyword<'arena> {
    YulNullKeyword {
        l,
        r,
        phantom: PhantomData,
    }
}

#[derive(Debug)]
pub struct YulOfKeyword<'arena> {
    pub l: usize,
    pub r: usize,
    pub phantom: PhantomData<&'arena ()>,
}

pub fn new_yul_of_keyword<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    _source: &str,
) -> YulOfKeyword<'arena> {
    YulOfKeyword {
        l,
        r,
        phantom: PhantomData,
    }
}

#[derive(Debug)]
pub struct YulOverrideKeyword<'arena> {
    pub l: usize,
    pub r: usize,
    pub phantom: PhantomData<&'arena ()>,
}

pub fn new_yul_override_keyword<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    _source: &str,
) -> YulOverrideKeyword<'arena> {
    YulOverrideKeyword {
        l,
        r,
        phantom: PhantomData,
    }
}

#[derive(Debug)]
pub struct YulPartialKeyword<'arena> {
    pub l: usize,
    pub r: usize,
    pub phantom: PhantomData<&'arena ()>,
}

pub fn new_yul_partial_keyword<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    _source: &str,
) -> YulPartialKeyword<'arena> {
    YulPartialKeyword {
        l,
        r,
        phantom: PhantomData,
    }
}

#[derive(Debug)]
pub struct YulPayableKeyword<'arena> {
    pub l: usize,
    pub r: usize,
    pub phantom: PhantomData<&'arena ()>,
}

pub fn new_yul_payable_keyword<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    _source: &str,
) -> YulPayableKeyword<'arena> {
    YulPayableKeyword {
        l,
        r,
        phantom: PhantomData,
    }
}

#[derive(Debug)]
pub struct YulPragmaKeyword<'arena> {
    pub l: usize,
    pub r: usize,
    pub phantom: PhantomData<&'arena ()>,
}

pub fn new_yul_pragma_keyword<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    _source: &str,
) -> YulPragmaKeyword<'arena> {
    YulPragmaKeyword {
        l,
        r,
        phantom: PhantomData,
    }
}

#[derive(Debug)]
pub struct YulPrivateKeyword<'arena> {
    pub l: usize,
    pub r: usize,
    pub phantom: PhantomData<&'arena ()>,
}

pub fn new_yul_private_keyword<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    _source: &str,
) -> YulPrivateKeyword<'arena> {
    YulPrivateKeyword {
        l,
        r,
        phantom: PhantomData,
    }
}

#[derive(Debug)]
pub struct YulPromiseKeyword<'arena> {
    pub l: usize,
    pub r: usize,
    pub phantom: PhantomData<&'arena ()>,
}

pub fn new_yul_promise_keyword<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    _source: &str,
) -> YulPromiseKeyword<'arena> {
    YulPromiseKeyword {
        l,
        r,
        phantom: PhantomData,
    }
}

#[derive(Debug)]
pub struct YulPublicKeyword<'arena> {
    pub l: usize,
    pub r: usize,
    pub phantom: PhantomData<&'arena ()>,
}

pub fn new_yul_public_keyword<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    _source: &str,
) -> YulPublicKeyword<'arena> {
    YulPublicKeyword {
        l,
        r,
        phantom: PhantomData,
    }
}

#[derive(Debug)]
pub struct YulPureKeyword<'arena> {
    pub l: usize,
    pub r: usize,
    pub phantom: PhantomData<&'arena ()>,
}

pub fn new_yul_pure_keyword<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    _source: &str,
) -> YulPureKeyword<'arena> {
    YulPureKeyword {
        l,
        r,
        phantom: PhantomData,
    }
}

#[derive(Debug)]
pub struct YulReceiveKeyword<'arena> {
    pub l: usize,
    pub r: usize,
    pub phantom: PhantomData<&'arena ()>,
}

pub fn new_yul_receive_keyword<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    _source: &str,
) -> YulReceiveKeyword<'arena> {
    YulReceiveKeyword {
        l,
        r,
        phantom: PhantomData,
    }
}

#[derive(Debug)]
pub struct YulReferenceKeyword<'arena> {
    pub l: usize,
    pub r: usize,
    pub phantom: PhantomData<&'arena ()>,
}

pub fn new_yul_reference_keyword<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    _source: &str,
) -> YulReferenceKeyword<'arena> {
    YulReferenceKeyword {
        l,
        r,
        phantom: PhantomData,
    }
}

#[derive(Debug)]
pub struct YulRelocatableKeyword<'arena> {
    pub l: usize,
    pub r: usize,
    pub phantom: PhantomData<&'arena ()>,
}

pub fn new_yul_relocatable_keyword<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    _source: &str,
) -> YulRelocatableKeyword<'arena> {
    YulRelocatableKeyword {
        l,
        r,
        phantom: PhantomData,
    }
}

#[derive(Debug)]
pub struct YulReturnsKeyword<'arena> {
    pub l: usize,
    pub r: usize,
    pub phantom: PhantomData<&'arena ()>,
}

pub fn new_yul_returns_keyword<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    _source: &str,
) -> YulReturnsKeyword<'arena> {
    YulReturnsKeyword {
        l,
        r,
        phantom: PhantomData,
    }
}

#[derive(Debug)]
pub struct YulSealedKeyword<'arena> {
    pub l: usize,
    pub r: usize,
    pub phantom: PhantomData<&'arena ()>,
}

pub fn new_yul_sealed_keyword<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    _source: &str,
) -> YulSealedKeyword<'arena> {
    YulSealedKeyword {
        l,
        r,
        phantom: PhantomData,
    }
}

#[derive(Debug)]
pub struct YulSecondsKeyword<'arena> {
    pub l: usize,
    pub r: usize,
    pub phantom: PhantomData<&'arena ()>,
}

pub fn new_yul_seconds_keyword<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    _source: &str,
) -> YulSecondsKeyword<'arena> {
    YulSecondsKeyword {
        l,
        r,
        phantom: PhantomData,
    }
}

#[derive(Debug)]
pub struct YulSizeOfKeyword<'arena> {
    pub l: usize,
    pub r: usize,
    pub phantom: PhantomData<&'arena ()>,
}

pub fn new_yul_size_of_keyword<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    _source: &str,
) -> YulSizeOfKeyword<'arena> {
    YulSizeOfKeyword {
        l,
        r,
        phantom: PhantomData,
    }
}

#[derive(Debug)]
pub struct YulStaticKeyword<'arena> {
    pub l: usize,
    pub r: usize,
    pub phantom: PhantomData<&'arena ()>,
}

pub fn new_yul_static_keyword<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    _source: &str,
) -> YulStaticKeyword<'arena> {
    YulStaticKeyword {
        l,
        r,
        phantom: PhantomData,
    }
}

#[derive(Debug)]
pub struct YulStorageKeyword<'arena> {
    pub l: usize,
    pub r: usize,
    pub phantom: PhantomData<&'arena ()>,
}

pub fn new_yul_storage_keyword<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    _source: &str,
) -> YulStorageKeyword<'arena> {
    YulStorageKeyword {
        l,
        r,
        phantom: PhantomData,
    }
}

#[derive(Debug)]
pub struct YulStringKeyword<'arena> {
    pub l: usize,
    pub r: usize,
    pub phantom: PhantomData<&'arena ()>,
}

pub fn new_yul_string_keyword<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    _source: &str,
) -> YulStringKeyword<'arena> {
    YulStringKeyword {
        l,
        r,
        phantom: PhantomData,
    }
}

#[derive(Debug)]
pub struct YulStructKeyword<'arena> {
    pub l: usize,
    pub r: usize,
    pub phantom: PhantomData<&'arena ()>,
}

pub fn new_yul_struct_keyword<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    _source: &str,
) -> YulStructKeyword<'arena> {
    YulStructKeyword {
        l,
        r,
        phantom: PhantomData,
    }
}

#[derive(Debug)]
pub struct YulSuperKeyword<'arena> {
    pub l: usize,
    pub r: usize,
    pub phantom: PhantomData<&'arena ()>,
}

pub fn new_yul_super_keyword<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    _source: &str,
) -> YulSuperKeyword<'arena> {
    YulSuperKeyword {
        l,
        r,
        phantom: PhantomData,
    }
}

#[derive(Debug)]
pub struct YulSupportsKeyword<'arena> {
    pub l: usize,
    pub r: usize,
    pub phantom: PhantomData<&'arena ()>,
}

pub fn new_yul_supports_keyword<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    _source: &str,
) -> YulSupportsKeyword<'arena> {
    YulSupportsKeyword {
        l,
        r,
        phantom: PhantomData,
    }
}

#[derive(Debug)]
pub struct YulSwitchKeyword<'arena> {
    pub l: usize,
    pub r: usize,
    pub phantom: PhantomData<&'arena ()>,
}

pub fn new_yul_switch_keyword<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    _source: &str,
) -> YulSwitchKeyword<'arena> {
    YulSwitchKeyword {
        l,
        r,
        phantom: PhantomData,
    }
}

#[derive(Debug)]
pub struct YulSzaboKeyword<'arena> {
    pub l: usize,
    pub r: usize,
    pub phantom: PhantomData<&'arena ()>,
}

pub fn new_yul_szabo_keyword<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    _source: &str,
) -> YulSzaboKeyword<'arena> {
    YulSzaboKeyword {
        l,
        r,
        phantom: PhantomData,
    }
}

#[derive(Debug)]
pub struct YulThisKeyword<'arena> {
    pub l: usize,
    pub r: usize,
    pub phantom: PhantomData<&'arena ()>,
}

pub fn new_yul_this_keyword<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    _source: &str,
) -> YulThisKeyword<'arena> {
    YulThisKeyword {
        l,
        r,
        phantom: PhantomData,
    }
}

#[derive(Debug)]
pub struct YulThrowKeyword<'arena> {
    pub l: usize,
    pub r: usize,
    pub phantom: PhantomData<&'arena ()>,
}

pub fn new_yul_throw_keyword<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    _source: &str,
) -> YulThrowKeyword<'arena> {
    YulThrowKeyword {
        l,
        r,
        phantom: PhantomData,
    }
}

#[derive(Debug)]
pub struct YulTrueKeyword<'arena> {
    pub l: usize,
    pub r: usize,
    pub phantom: PhantomData<&'arena ()>,
}

pub fn new_yul_true_keyword<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    _source: &str,
) -> YulTrueKeyword<'arena> {
    YulTrueKeyword {
        l,
        r,
        phantom: PhantomData,
    }
}

#[derive(Debug)]
pub struct YulTryKeyword<'arena> {
    pub l: usize,
    pub r: usize,
    pub phantom: PhantomData<&'arena ()>,
}

pub fn new_yul_try_keyword<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    _source: &str,
) -> YulTryKeyword<'arena> {
    YulTryKeyword {
        l,
        r,
        phantom: PhantomData,
    }
}

#[derive(Debug)]
pub struct YulTypeDefKeyword<'arena> {
    pub l: usize,
    pub r: usize,
    pub phantom: PhantomData<&'arena ()>,
}

pub fn new_yul_type_def_keyword<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    _source: &str,
) -> YulTypeDefKeyword<'arena> {
    YulTypeDefKeyword {
        l,
        r,
        phantom: PhantomData,
    }
}

#[derive(Debug)]
pub struct YulTypeKeyword<'arena> {
    pub l: usize,
    pub r: usize,
    pub phantom: PhantomData<&'arena ()>,
}

pub fn new_yul_type_keyword<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    _source: &str,
) -> YulTypeKeyword<'arena> {
    YulTypeKeyword {
        l,
        r,
        phantom: PhantomData,
    }
}

#[derive(Debug)]
pub struct YulTypeOfKeyword<'arena> {
    pub l: usize,
    pub r: usize,
    pub phantom: PhantomData<&'arena ()>,
}

pub fn new_yul_type_of_keyword<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    _source: &str,
) -> YulTypeOfKeyword<'arena> {
    YulTypeOfKeyword {
        l,
        r,
        phantom: PhantomData,
    }
}

#[derive(Debug)]
pub struct YulUfixedKeyword<'arena> {
    pub l: usize,
    pub r: usize,
    pub phantom: PhantomData<&'arena ()>,
}

pub fn new_yul_ufixed_keyword<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    _source: &str,
) -> YulUfixedKeyword<'arena> {
    YulUfixedKeyword {
        l,
        r,
        phantom: PhantomData,
    }
}

#[derive(Debug)]
pub struct YulUintKeyword<'arena> {
    pub l: usize,
    pub r: usize,
    pub phantom: PhantomData<&'arena ()>,
}

pub fn new_yul_uint_keyword<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    _source: &str,
) -> YulUintKeyword<'arena> {
    YulUintKeyword {
        l,
        r,
        phantom: PhantomData,
    }
}

#[derive(Debug)]
pub struct YulUncheckedKeyword<'arena> {
    pub l: usize,
    pub r: usize,
    pub phantom: PhantomData<&'arena ()>,
}

pub fn new_yul_unchecked_keyword<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    _source: &str,
) -> YulUncheckedKeyword<'arena> {
    YulUncheckedKeyword {
        l,
        r,
        phantom: PhantomData,
    }
}

#[derive(Debug)]
pub struct YulUsingKeyword<'arena> {
    pub l: usize,
    pub r: usize,
    pub phantom: PhantomData<&'arena ()>,
}

pub fn new_yul_using_keyword<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    _source: &str,
) -> YulUsingKeyword<'arena> {
    YulUsingKeyword {
        l,
        r,
        phantom: PhantomData,
    }
}

#[derive(Debug)]
pub struct YulVarKeyword<'arena> {
    pub l: usize,
    pub r: usize,
    pub phantom: PhantomData<&'arena ()>,
}

pub fn new_yul_var_keyword<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    _source: &str,
) -> YulVarKeyword<'arena> {
    YulVarKeyword {
        l,
        r,
        phantom: PhantomData,
    }
}

#[derive(Debug)]
pub struct YulViewKeyword<'arena> {
    pub l: usize,
    pub r: usize,
    pub phantom: PhantomData<&'arena ()>,
}

pub fn new_yul_view_keyword<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    _source: &str,
) -> YulViewKeyword<'arena> {
    YulViewKeyword {
        l,
        r,
        phantom: PhantomData,
    }
}

#[derive(Debug)]
pub struct YulVirtualKeyword<'arena> {
    pub l: usize,
    pub r: usize,
    pub phantom: PhantomData<&'arena ()>,
}

pub fn new_yul_virtual_keyword<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    _source: &str,
) -> YulVirtualKeyword<'arena> {
    YulVirtualKeyword {
        l,
        r,
        phantom: PhantomData,
    }
}

#[derive(Debug)]
pub struct YulWeeksKeyword<'arena> {
    pub l: usize,
    pub r: usize,
    pub phantom: PhantomData<&'arena ()>,
}

pub fn new_yul_weeks_keyword<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    _source: &str,
) -> YulWeeksKeyword<'arena> {
    YulWeeksKeyword {
        l,
        r,
        phantom: PhantomData,
    }
}

#[derive(Debug)]
pub struct YulWeiKeyword<'arena> {
    pub l: usize,
    pub r: usize,
    pub phantom: PhantomData<&'arena ()>,
}

pub fn new_yul_wei_keyword<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    _source: &str,
) -> YulWeiKeyword<'arena> {
    YulWeiKeyword {
        l,
        r,
        phantom: PhantomData,
    }
}

#[derive(Debug)]
pub struct YulWhileKeyword<'arena> {
    pub l: usize,
    pub r: usize,
    pub phantom: PhantomData<&'arena ()>,
}

pub fn new_yul_while_keyword<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    _source: &str,
) -> YulWhileKeyword<'arena> {
    YulWhileKeyword {
        l,
        r,
        phantom: PhantomData,
    }
}

#[derive(Debug)]
pub struct YulYearsKeyword<'arena> {
    pub l: usize,
    pub r: usize,
    pub phantom: PhantomData<&'arena ()>,
}

pub fn new_yul_years_keyword<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    _source: &str,
) -> YulYearsKeyword<'arena> {
    YulYearsKeyword {
        l,
        r,
        phantom: PhantomData,
    }
}

// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

#![allow(non_camel_case_types)]
#![allow(dead_code)]
#![allow(unused_variables)]

use std::marker::PhantomData;

use bumpalo::boxed::Box;
use bumpalo::collections::Vec;
use bumpalo::Bump;

// TODO:
// - (perf) don't use terminals that are not needed

//
// Sequences:
//

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
    pub operator: Plus<'arena>,
    pub right_operand: Expression<'arena>,
}

pub fn new_additive_expression<'arena>(
    arena: &'arena Bump,
    left_operand: Expression<'arena>,
    operator: Plus<'arena>,
    right_operand: Expression<'arena>,
) -> AdditiveExpression<'arena> {
    Box::new_in(
        AdditiveExpressionStruct {
            left_operand,
            operator,
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
    pub operator: Equal<'arena>,
    pub right_operand: Expression<'arena>,
}

pub fn new_assignment_expression<'arena>(
    arena: &'arena Bump,
    left_operand: Expression<'arena>,
    operator: Equal<'arena>,
    right_operand: Expression<'arena>,
) -> AssignmentExpression<'arena> {
    Box::new_in(
        AssignmentExpressionStruct {
            left_operand,
            operator,
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

pub type CallOptionsNew<'arena> = Box<'arena, CallOptionsNewStruct<'arena>>;

#[derive(Debug)]
pub struct CallOptionsNewStruct<'arena> {
    pub open_brace: OpenBrace<'arena>,
    pub options: CallOptions<'arena>,
    pub close_brace: CloseBrace<'arena>,
}

pub fn new_call_options_new<'arena>(
    arena: &'arena Bump,
    open_brace: OpenBrace<'arena>,
    options: CallOptions<'arena>,
    close_brace: CloseBrace<'arena>,
) -> CallOptionsNew<'arena> {
    Box::new_in(
        CallOptionsNewStruct {
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
    pub operator: EqualEqual<'arena>,
    pub right_operand: Expression<'arena>,
}

pub fn new_equality_expression<'arena>(
    arena: &'arena Bump,
    left_operand: Expression<'arena>,
    operator: EqualEqual<'arena>,
    right_operand: Expression<'arena>,
) -> EqualityExpression<'arena> {
    Box::new_in(
        EqualityExpressionStruct {
            left_operand,
            operator,
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
    pub operator: AsteriskAsterisk<'arena>,
    pub right_operand: Expression<'arena>,
}

pub fn new_exponentiation_expression<'arena>(
    arena: &'arena Bump,
    left_operand: Expression<'arena>,
    operator: AsteriskAsterisk<'arena>,
    right_operand: Expression<'arena>,
) -> ExponentiationExpression<'arena> {
    Box::new_in(
        ExponentiationExpressionStruct {
            left_operand,
            operator,
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
    pub operator: LessThan<'arena>,
    pub right_operand: Expression<'arena>,
}

pub fn new_inequality_expression<'arena>(
    arena: &'arena Bump,
    left_operand: Expression<'arena>,
    operator: LessThan<'arena>,
    right_operand: Expression<'arena>,
) -> InequalityExpression<'arena> {
    Box::new_in(
        InequalityExpressionStruct {
            left_operand,
            operator,
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
    pub member: MemberAccessIdentifier<'arena>,
}

pub fn new_member_access_expression<'arena>(
    arena: &'arena Bump,
    operand: Expression<'arena>,
    period: Period<'arena>,
    member: MemberAccessIdentifier<'arena>,
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
    pub operator: Asterisk<'arena>,
    pub right_operand: Expression<'arena>,
}

pub fn new_multiplicative_expression<'arena>(
    arena: &'arena Bump,
    left_operand: Expression<'arena>,
    operator: Asterisk<'arena>,
    right_operand: Expression<'arena>,
) -> MultiplicativeExpression<'arena> {
    Box::new_in(
        MultiplicativeExpressionStruct {
            left_operand,
            operator,
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
    pub arguments: NamedArgumentGroup<'arena>,
    pub close_paren: CloseParen<'arena>,
}

pub fn new_named_arguments_declaration<'arena>(
    arena: &'arena Bump,
    open_paren: OpenParen<'arena>,
    arguments: NamedArgumentGroup<'arena>,
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
    pub options: Option<CallOptionsNew<'arena>>,
    pub arguments: ArgumentsDeclaration<'arena>,
}

pub fn new_new_expression<'arena>(
    arena: &'arena Bump,
    new_keyword: NewKeyword<'arena>,
    type_name: TypeName<'arena>,
    options: Option<CallOptionsNew<'arena>>,
    arguments: ArgumentsDeclaration<'arena>,
) -> NewExpression<'arena> {
    Box::new_in(
        NewExpressionStruct {
            new_keyword,
            type_name,
            options,
            arguments,
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
    pub operator: PlusPlus<'arena>,
}

pub fn new_postfix_expression<'arena>(
    arena: &'arena Bump,
    operand: Expression<'arena>,
    operator: PlusPlus<'arena>,
) -> PostfixExpression<'arena> {
    Box::new_in(PostfixExpressionStruct { operand, operator }, arena)
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
    pub operator: PlusPlus<'arena>,
    pub operand: Expression<'arena>,
}

pub fn new_prefix_expression<'arena>(
    arena: &'arena Bump,
    operator: PlusPlus<'arena>,
    operand: Expression<'arena>,
) -> PrefixExpression<'arena> {
    Box::new_in(PrefixExpressionStruct { operator, operand }, arena)
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
    pub operator: LessThanLessThan<'arena>,
    pub right_operand: Expression<'arena>,
}

pub fn new_shift_expression<'arena>(
    arena: &'arena Bump,
    left_operand: Expression<'arena>,
    operator: LessThanLessThan<'arena>,
    right_operand: Expression<'arena>,
) -> ShiftExpression<'arena> {
    Box::new_in(
        ShiftExpressionStruct {
            left_operand,
            operator,
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

pub type VariableDeclaration<'arena> = Box<'arena, VariableDeclarationStruct<'arena>>;

#[derive(Debug)]
pub struct VariableDeclarationStruct<'arena> {
    pub variable_type: VariableDeclarationType<'arena>,
    pub storage_location: Option<StorageLocation<'arena>>,
    pub name: Identifier<'arena>,
}

pub fn new_variable_declaration<'arena>(
    arena: &'arena Bump,
    variable_type: VariableDeclarationType<'arena>,
    storage_location: Option<StorageLocation<'arena>>,
    name: Identifier<'arena>,
) -> VariableDeclaration<'arena> {
    Box::new_in(
        VariableDeclarationStruct {
            variable_type,
            storage_location,
            name,
        },
        arena,
    )
}

pub type VariableDeclarationStatement<'arena> =
    Box<'arena, VariableDeclarationStatementStruct<'arena>>;

#[derive(Debug)]
pub struct VariableDeclarationStatementStruct<'arena> {
    pub variable_declaration: VariableDeclaration<'arena>,
    pub value: Option<VariableDeclarationValue<'arena>>,
    pub semicolon: Semicolon<'arena>,
}

pub fn new_variable_declaration_statement<'arena>(
    arena: &'arena Bump,
    variable_declaration: VariableDeclaration<'arena>,
    value: Option<VariableDeclarationValue<'arena>>,
    semicolon: Semicolon<'arena>,
) -> VariableDeclarationStatement<'arena> {
    Box::new_in(
        VariableDeclarationStatementStruct {
            variable_declaration,
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
pub enum MemberAccessIdentifier<'arena> {
    Identifier(Identifier<'arena>),
    AddressKeyword(AddressKeyword<'arena>),
}

pub fn new_member_access_identifier_identifier<'arena>(
    arena: &'arena Bump,
    element: Identifier<'arena>,
) -> MemberAccessIdentifier<'arena> {
    MemberAccessIdentifier::Identifier(element)
}

pub fn new_member_access_identifier_address_keyword<'arena>(
    arena: &'arena Bump,
    element: AddressKeyword<'arena>,
) -> MemberAccessIdentifier<'arena> {
    MemberAccessIdentifier::AddressKeyword(element)
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
#[derive(Debug)]
pub struct TerminalType<'arena> {
    pub value: String,
    pub l: usize,
    pub r: usize,
    pub kind: LexemeKind,
    pub phantom: PhantomData<&'arena ()>,
}

pub fn new_empty_terminal(kind: LexemeKind) -> TerminalType<'static> {
    TerminalType {
        value: String::new(),
        l: 0,
        r: 0,
        kind,
        phantom: PhantomData,
    }
}

pub type ABIEncoderV2Keyword<'arena> = TerminalType<'arena>;

pub type AbicoderKeyword<'arena> = TerminalType<'arena>;

pub type AbicoderV1Keyword<'arena> = TerminalType<'arena>;

pub type AbicoderV2Keyword<'arena> = TerminalType<'arena>;

pub type AbstractKeyword<'arena> = TerminalType<'arena>;

pub type AddressKeyword<'arena> = TerminalType<'arena>;

pub type AfterKeyword<'arena> = TerminalType<'arena>;

pub type AliasKeyword<'arena> = TerminalType<'arena>;

pub type AnonymousKeyword<'arena> = TerminalType<'arena>;

pub type ApplyKeyword<'arena> = TerminalType<'arena>;

pub type AsKeyword<'arena> = TerminalType<'arena>;

pub type AssemblyKeyword<'arena> = TerminalType<'arena>;

pub type AtKeyword<'arena> = TerminalType<'arena>;

pub type AutoKeyword<'arena> = TerminalType<'arena>;

pub type BoolKeyword<'arena> = TerminalType<'arena>;

pub type BreakKeyword<'arena> = TerminalType<'arena>;

pub type ByteKeyword<'arena> = TerminalType<'arena>;

pub type BytesKeyword<'arena> = TerminalType<'arena>;

pub type CallDataKeyword<'arena> = TerminalType<'arena>;

pub type CaseKeyword<'arena> = TerminalType<'arena>;

pub type CatchKeyword<'arena> = TerminalType<'arena>;

pub type ConstantKeyword<'arena> = TerminalType<'arena>;

pub type ConstructorKeyword<'arena> = TerminalType<'arena>;

pub type ContinueKeyword<'arena> = TerminalType<'arena>;

pub type ContractKeyword<'arena> = TerminalType<'arena>;

pub type CopyOfKeyword<'arena> = TerminalType<'arena>;

pub type DaysKeyword<'arena> = TerminalType<'arena>;

pub type DefaultKeyword<'arena> = TerminalType<'arena>;

pub type DefineKeyword<'arena> = TerminalType<'arena>;

pub type DeleteKeyword<'arena> = TerminalType<'arena>;

pub type DoKeyword<'arena> = TerminalType<'arena>;

pub type ElseKeyword<'arena> = TerminalType<'arena>;

pub type EmitKeyword<'arena> = TerminalType<'arena>;

pub type EnumKeyword<'arena> = TerminalType<'arena>;

pub type ErrorKeyword<'arena> = TerminalType<'arena>;

pub type EtherKeyword<'arena> = TerminalType<'arena>;

pub type EventKeyword<'arena> = TerminalType<'arena>;

pub type ExperimentalKeyword<'arena> = TerminalType<'arena>;

pub type ExternalKeyword<'arena> = TerminalType<'arena>;

pub type FallbackKeyword<'arena> = TerminalType<'arena>;

pub type FalseKeyword<'arena> = TerminalType<'arena>;

pub type FinalKeyword<'arena> = TerminalType<'arena>;

pub type FinneyKeyword<'arena> = TerminalType<'arena>;

pub type FixedKeyword<'arena> = TerminalType<'arena>;

pub type ForKeyword<'arena> = TerminalType<'arena>;

pub type FromKeyword<'arena> = TerminalType<'arena>;

pub type FunctionKeyword<'arena> = TerminalType<'arena>;

pub type GlobalKeyword<'arena> = TerminalType<'arena>;

pub type GweiKeyword<'arena> = TerminalType<'arena>;

pub type HexKeyword<'arena> = TerminalType<'arena>;

pub type HoursKeyword<'arena> = TerminalType<'arena>;

pub type IfKeyword<'arena> = TerminalType<'arena>;

pub type ImmutableKeyword<'arena> = TerminalType<'arena>;

pub type ImplementsKeyword<'arena> = TerminalType<'arena>;

pub type ImportKeyword<'arena> = TerminalType<'arena>;

pub type InKeyword<'arena> = TerminalType<'arena>;

pub type IndexedKeyword<'arena> = TerminalType<'arena>;

pub type InlineKeyword<'arena> = TerminalType<'arena>;

pub type IntKeyword<'arena> = TerminalType<'arena>;

pub type InterfaceKeyword<'arena> = TerminalType<'arena>;

pub type InternalKeyword<'arena> = TerminalType<'arena>;

pub type IsKeyword<'arena> = TerminalType<'arena>;

pub type LayoutKeyword<'arena> = TerminalType<'arena>;

pub type LetKeyword<'arena> = TerminalType<'arena>;

pub type LibraryKeyword<'arena> = TerminalType<'arena>;

pub type MacroKeyword<'arena> = TerminalType<'arena>;

pub type MappingKeyword<'arena> = TerminalType<'arena>;

pub type MatchKeyword<'arena> = TerminalType<'arena>;

pub type MemoryKeyword<'arena> = TerminalType<'arena>;

pub type MinutesKeyword<'arena> = TerminalType<'arena>;

pub type ModifierKeyword<'arena> = TerminalType<'arena>;

pub type MutableKeyword<'arena> = TerminalType<'arena>;

pub type NewKeyword<'arena> = TerminalType<'arena>;

pub type NullKeyword<'arena> = TerminalType<'arena>;

pub type OfKeyword<'arena> = TerminalType<'arena>;

pub type OverrideKeyword<'arena> = TerminalType<'arena>;

pub type PartialKeyword<'arena> = TerminalType<'arena>;

pub type PayableKeyword<'arena> = TerminalType<'arena>;

pub type PragmaKeyword<'arena> = TerminalType<'arena>;

pub type PrivateKeyword<'arena> = TerminalType<'arena>;

pub type PromiseKeyword<'arena> = TerminalType<'arena>;

pub type PublicKeyword<'arena> = TerminalType<'arena>;

pub type PureKeyword<'arena> = TerminalType<'arena>;

pub type ReceiveKeyword<'arena> = TerminalType<'arena>;

pub type ReferenceKeyword<'arena> = TerminalType<'arena>;

pub type RelocatableKeyword<'arena> = TerminalType<'arena>;

pub type ReturnKeyword<'arena> = TerminalType<'arena>;

pub type ReturnsKeyword<'arena> = TerminalType<'arena>;

pub type RevertKeyword<'arena> = TerminalType<'arena>;

pub type SMTCheckerKeyword<'arena> = TerminalType<'arena>;

pub type SealedKeyword<'arena> = TerminalType<'arena>;

pub type SecondsKeyword<'arena> = TerminalType<'arena>;

pub type SizeOfKeyword<'arena> = TerminalType<'arena>;

pub type SolidityKeyword<'arena> = TerminalType<'arena>;

pub type StaticKeyword<'arena> = TerminalType<'arena>;

pub type StorageKeyword<'arena> = TerminalType<'arena>;

pub type StringKeyword<'arena> = TerminalType<'arena>;

pub type StructKeyword<'arena> = TerminalType<'arena>;

pub type SuperKeyword<'arena> = TerminalType<'arena>;

pub type SupportsKeyword<'arena> = TerminalType<'arena>;

pub type SwitchKeyword<'arena> = TerminalType<'arena>;

pub type SzaboKeyword<'arena> = TerminalType<'arena>;

pub type ThisKeyword<'arena> = TerminalType<'arena>;

pub type ThrowKeyword<'arena> = TerminalType<'arena>;

pub type TransientKeyword<'arena> = TerminalType<'arena>;

pub type TrueKeyword<'arena> = TerminalType<'arena>;

pub type TryKeyword<'arena> = TerminalType<'arena>;

pub type TypeDefKeyword<'arena> = TerminalType<'arena>;

pub type TypeKeyword<'arena> = TerminalType<'arena>;

pub type TypeOfKeyword<'arena> = TerminalType<'arena>;

pub type UfixedKeyword<'arena> = TerminalType<'arena>;

pub type UintKeyword<'arena> = TerminalType<'arena>;

pub type UncheckedKeyword<'arena> = TerminalType<'arena>;

pub type UsingKeyword<'arena> = TerminalType<'arena>;

pub type VarKeyword<'arena> = TerminalType<'arena>;

pub type ViewKeyword<'arena> = TerminalType<'arena>;

pub type VirtualKeyword<'arena> = TerminalType<'arena>;

pub type WeeksKeyword<'arena> = TerminalType<'arena>;

pub type WeiKeyword<'arena> = TerminalType<'arena>;

pub type WhileKeyword<'arena> = TerminalType<'arena>;

pub type YearsKeyword<'arena> = TerminalType<'arena>;

pub type YulAbstractKeyword<'arena> = TerminalType<'arena>;

pub type YulAfterKeyword<'arena> = TerminalType<'arena>;

pub type YulAliasKeyword<'arena> = TerminalType<'arena>;

pub type YulAnonymousKeyword<'arena> = TerminalType<'arena>;

pub type YulApplyKeyword<'arena> = TerminalType<'arena>;

pub type YulAsKeyword<'arena> = TerminalType<'arena>;

pub type YulAssemblyKeyword<'arena> = TerminalType<'arena>;

pub type YulAutoKeyword<'arena> = TerminalType<'arena>;

pub type YulBoolKeyword<'arena> = TerminalType<'arena>;

pub type YulBreakKeyword<'arena> = TerminalType<'arena>;

pub type YulBytesKeyword<'arena> = TerminalType<'arena>;

pub type YulCallDataKeyword<'arena> = TerminalType<'arena>;

pub type YulCaseKeyword<'arena> = TerminalType<'arena>;

pub type YulCatchKeyword<'arena> = TerminalType<'arena>;

pub type YulConstantKeyword<'arena> = TerminalType<'arena>;

pub type YulConstructorKeyword<'arena> = TerminalType<'arena>;

pub type YulContinueKeyword<'arena> = TerminalType<'arena>;

pub type YulContractKeyword<'arena> = TerminalType<'arena>;

pub type YulCopyOfKeyword<'arena> = TerminalType<'arena>;

pub type YulDaysKeyword<'arena> = TerminalType<'arena>;

pub type YulDefaultKeyword<'arena> = TerminalType<'arena>;

pub type YulDefineKeyword<'arena> = TerminalType<'arena>;

pub type YulDeleteKeyword<'arena> = TerminalType<'arena>;

pub type YulDoKeyword<'arena> = TerminalType<'arena>;

pub type YulElseKeyword<'arena> = TerminalType<'arena>;

pub type YulEmitKeyword<'arena> = TerminalType<'arena>;

pub type YulEnumKeyword<'arena> = TerminalType<'arena>;

pub type YulEtherKeyword<'arena> = TerminalType<'arena>;

pub type YulEventKeyword<'arena> = TerminalType<'arena>;

pub type YulExternalKeyword<'arena> = TerminalType<'arena>;

pub type YulFallbackKeyword<'arena> = TerminalType<'arena>;

pub type YulFalseKeyword<'arena> = TerminalType<'arena>;

pub type YulFinalKeyword<'arena> = TerminalType<'arena>;

pub type YulFinneyKeyword<'arena> = TerminalType<'arena>;

pub type YulFixedKeyword<'arena> = TerminalType<'arena>;

pub type YulForKeyword<'arena> = TerminalType<'arena>;

pub type YulFunctionKeyword<'arena> = TerminalType<'arena>;

pub type YulGweiKeyword<'arena> = TerminalType<'arena>;

pub type YulHexKeyword<'arena> = TerminalType<'arena>;

pub type YulHoursKeyword<'arena> = TerminalType<'arena>;

pub type YulIfKeyword<'arena> = TerminalType<'arena>;

pub type YulImmutableKeyword<'arena> = TerminalType<'arena>;

pub type YulImplementsKeyword<'arena> = TerminalType<'arena>;

pub type YulImportKeyword<'arena> = TerminalType<'arena>;

pub type YulInKeyword<'arena> = TerminalType<'arena>;

pub type YulIndexedKeyword<'arena> = TerminalType<'arena>;

pub type YulInlineKeyword<'arena> = TerminalType<'arena>;

pub type YulIntKeyword<'arena> = TerminalType<'arena>;

pub type YulInterfaceKeyword<'arena> = TerminalType<'arena>;

pub type YulInternalKeyword<'arena> = TerminalType<'arena>;

pub type YulIsKeyword<'arena> = TerminalType<'arena>;

pub type YulLeaveKeyword<'arena> = TerminalType<'arena>;

pub type YulLetKeyword<'arena> = TerminalType<'arena>;

pub type YulLibraryKeyword<'arena> = TerminalType<'arena>;

pub type YulMacroKeyword<'arena> = TerminalType<'arena>;

pub type YulMappingKeyword<'arena> = TerminalType<'arena>;

pub type YulMatchKeyword<'arena> = TerminalType<'arena>;

pub type YulMemoryKeyword<'arena> = TerminalType<'arena>;

pub type YulMinutesKeyword<'arena> = TerminalType<'arena>;

pub type YulModifierKeyword<'arena> = TerminalType<'arena>;

pub type YulMutableKeyword<'arena> = TerminalType<'arena>;

pub type YulNewKeyword<'arena> = TerminalType<'arena>;

pub type YulNullKeyword<'arena> = TerminalType<'arena>;

pub type YulOfKeyword<'arena> = TerminalType<'arena>;

pub type YulOverrideKeyword<'arena> = TerminalType<'arena>;

pub type YulPartialKeyword<'arena> = TerminalType<'arena>;

pub type YulPayableKeyword<'arena> = TerminalType<'arena>;

pub type YulPragmaKeyword<'arena> = TerminalType<'arena>;

pub type YulPrivateKeyword<'arena> = TerminalType<'arena>;

pub type YulPromiseKeyword<'arena> = TerminalType<'arena>;

pub type YulPublicKeyword<'arena> = TerminalType<'arena>;

pub type YulPureKeyword<'arena> = TerminalType<'arena>;

pub type YulReceiveKeyword<'arena> = TerminalType<'arena>;

pub type YulReferenceKeyword<'arena> = TerminalType<'arena>;

pub type YulRelocatableKeyword<'arena> = TerminalType<'arena>;

pub type YulReturnsKeyword<'arena> = TerminalType<'arena>;

pub type YulSealedKeyword<'arena> = TerminalType<'arena>;

pub type YulSecondsKeyword<'arena> = TerminalType<'arena>;

pub type YulSizeOfKeyword<'arena> = TerminalType<'arena>;

pub type YulStaticKeyword<'arena> = TerminalType<'arena>;

pub type YulStorageKeyword<'arena> = TerminalType<'arena>;

pub type YulStringKeyword<'arena> = TerminalType<'arena>;

pub type YulStructKeyword<'arena> = TerminalType<'arena>;

pub type YulSuperKeyword<'arena> = TerminalType<'arena>;

pub type YulSupportsKeyword<'arena> = TerminalType<'arena>;

pub type YulSwitchKeyword<'arena> = TerminalType<'arena>;

pub type YulSzaboKeyword<'arena> = TerminalType<'arena>;

pub type YulThisKeyword<'arena> = TerminalType<'arena>;

pub type YulThrowKeyword<'arena> = TerminalType<'arena>;

pub type YulTrueKeyword<'arena> = TerminalType<'arena>;

pub type YulTryKeyword<'arena> = TerminalType<'arena>;

pub type YulTypeDefKeyword<'arena> = TerminalType<'arena>;

pub type YulTypeKeyword<'arena> = TerminalType<'arena>;

pub type YulTypeOfKeyword<'arena> = TerminalType<'arena>;

pub type YulUfixedKeyword<'arena> = TerminalType<'arena>;

pub type YulUintKeyword<'arena> = TerminalType<'arena>;

pub type YulUncheckedKeyword<'arena> = TerminalType<'arena>;

pub type YulUsingKeyword<'arena> = TerminalType<'arena>;

pub type YulVarKeyword<'arena> = TerminalType<'arena>;

pub type YulViewKeyword<'arena> = TerminalType<'arena>;

pub type YulVirtualKeyword<'arena> = TerminalType<'arena>;

pub type YulWeeksKeyword<'arena> = TerminalType<'arena>;

pub type YulWeiKeyword<'arena> = TerminalType<'arena>;

pub type YulWhileKeyword<'arena> = TerminalType<'arena>;

pub type YulYearsKeyword<'arena> = TerminalType<'arena>;

pub type ABIEncoderV2Keyword_Unreserved<'arena> = TerminalType<'arena>;
pub fn new_abi_encoder_v2_keyword_unreserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> ABIEncoderV2Keyword_Unreserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::ABIEncoderV2Keyword_Unreserved,
        phantom: PhantomData,
    }
}

pub type AbicoderKeyword_Unreserved<'arena> = TerminalType<'arena>;
pub fn new_abicoder_keyword_unreserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> AbicoderKeyword_Unreserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::AbicoderKeyword_Unreserved,
        phantom: PhantomData,
    }
}

pub type AbicoderV1Keyword_Unreserved<'arena> = TerminalType<'arena>;
pub fn new_abicoder_v1_keyword_unreserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> AbicoderV1Keyword_Unreserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::AbicoderV1Keyword_Unreserved,
        phantom: PhantomData,
    }
}

pub type AbicoderV2Keyword_Unreserved<'arena> = TerminalType<'arena>;
pub fn new_abicoder_v2_keyword_unreserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> AbicoderV2Keyword_Unreserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::AbicoderV2Keyword_Unreserved,
        phantom: PhantomData,
    }
}

pub type AbstractKeyword_Reserved<'arena> = TerminalType<'arena>;
pub fn new_abstract_keyword_reserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> AbstractKeyword_Reserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::AbstractKeyword_Reserved,
        phantom: PhantomData,
    }
}

pub type AddressKeyword_Reserved<'arena> = TerminalType<'arena>;
pub fn new_address_keyword_reserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> AddressKeyword_Reserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::AddressKeyword_Reserved,
        phantom: PhantomData,
    }
}

pub type AfterKeyword_Reserved<'arena> = TerminalType<'arena>;
pub fn new_after_keyword_reserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> AfterKeyword_Reserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::AfterKeyword_Reserved,
        phantom: PhantomData,
    }
}

pub type AliasKeyword_Reserved<'arena> = TerminalType<'arena>;
pub fn new_alias_keyword_reserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> AliasKeyword_Reserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::AliasKeyword_Reserved,
        phantom: PhantomData,
    }
}

pub type AliasKeyword_Unreserved<'arena> = TerminalType<'arena>;
pub fn new_alias_keyword_unreserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> AliasKeyword_Unreserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::AliasKeyword_Unreserved,
        phantom: PhantomData,
    }
}

pub type Ampersand<'arena> = TerminalType<'arena>;
pub fn new_ampersand<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> Ampersand<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::Ampersand,
        phantom: PhantomData,
    }
}

pub type AmpersandAmpersand<'arena> = TerminalType<'arena>;
pub fn new_ampersand_ampersand<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> AmpersandAmpersand<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::AmpersandAmpersand,
        phantom: PhantomData,
    }
}

pub type AmpersandEqual<'arena> = TerminalType<'arena>;
pub fn new_ampersand_equal<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> AmpersandEqual<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::AmpersandEqual,
        phantom: PhantomData,
    }
}

pub type AnonymousKeyword_Reserved<'arena> = TerminalType<'arena>;
pub fn new_anonymous_keyword_reserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> AnonymousKeyword_Reserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::AnonymousKeyword_Reserved,
        phantom: PhantomData,
    }
}

pub type ApplyKeyword_Reserved<'arena> = TerminalType<'arena>;
pub fn new_apply_keyword_reserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> ApplyKeyword_Reserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::ApplyKeyword_Reserved,
        phantom: PhantomData,
    }
}

pub type ApplyKeyword_Unreserved<'arena> = TerminalType<'arena>;
pub fn new_apply_keyword_unreserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> ApplyKeyword_Unreserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::ApplyKeyword_Unreserved,
        phantom: PhantomData,
    }
}

pub type AsKeyword_Reserved<'arena> = TerminalType<'arena>;
pub fn new_as_keyword_reserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> AsKeyword_Reserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::AsKeyword_Reserved,
        phantom: PhantomData,
    }
}

pub type AssemblyKeyword_Reserved<'arena> = TerminalType<'arena>;
pub fn new_assembly_keyword_reserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> AssemblyKeyword_Reserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::AssemblyKeyword_Reserved,
        phantom: PhantomData,
    }
}

pub type Asterisk<'arena> = TerminalType<'arena>;
pub fn new_asterisk<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> Asterisk<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::Asterisk,
        phantom: PhantomData,
    }
}

pub type AsteriskAsterisk<'arena> = TerminalType<'arena>;
pub fn new_asterisk_asterisk<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> AsteriskAsterisk<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::AsteriskAsterisk,
        phantom: PhantomData,
    }
}

pub type AsteriskEqual<'arena> = TerminalType<'arena>;
pub fn new_asterisk_equal<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> AsteriskEqual<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::AsteriskEqual,
        phantom: PhantomData,
    }
}

pub type AtKeyword_Unreserved<'arena> = TerminalType<'arena>;
pub fn new_at_keyword_unreserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> AtKeyword_Unreserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::AtKeyword_Unreserved,
        phantom: PhantomData,
    }
}

pub type AutoKeyword_Reserved<'arena> = TerminalType<'arena>;
pub fn new_auto_keyword_reserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> AutoKeyword_Reserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::AutoKeyword_Reserved,
        phantom: PhantomData,
    }
}

pub type AutoKeyword_Unreserved<'arena> = TerminalType<'arena>;
pub fn new_auto_keyword_unreserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> AutoKeyword_Unreserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::AutoKeyword_Unreserved,
        phantom: PhantomData,
    }
}

pub type Bang<'arena> = TerminalType<'arena>;
pub fn new_bang<'arena>(_arena: &'arena Bump, l: usize, r: usize, source: &str) -> Bang<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::Bang,
        phantom: PhantomData,
    }
}

pub type BangEqual<'arena> = TerminalType<'arena>;
pub fn new_bang_equal<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> BangEqual<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::BangEqual,
        phantom: PhantomData,
    }
}

pub type Bar<'arena> = TerminalType<'arena>;
pub fn new_bar<'arena>(_arena: &'arena Bump, l: usize, r: usize, source: &str) -> Bar<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::Bar,
        phantom: PhantomData,
    }
}

pub type BarBar<'arena> = TerminalType<'arena>;
pub fn new_bar_bar<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> BarBar<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::BarBar,
        phantom: PhantomData,
    }
}

pub type BarEqual<'arena> = TerminalType<'arena>;
pub fn new_bar_equal<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> BarEqual<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::BarEqual,
        phantom: PhantomData,
    }
}

pub type BoolKeyword_Reserved<'arena> = TerminalType<'arena>;
pub fn new_bool_keyword_reserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> BoolKeyword_Reserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::BoolKeyword_Reserved,
        phantom: PhantomData,
    }
}

pub type BreakKeyword_Reserved<'arena> = TerminalType<'arena>;
pub fn new_break_keyword_reserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> BreakKeyword_Reserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::BreakKeyword_Reserved,
        phantom: PhantomData,
    }
}

pub type ByteKeyword_Reserved<'arena> = TerminalType<'arena>;
pub fn new_byte_keyword_reserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> ByteKeyword_Reserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::ByteKeyword_Reserved,
        phantom: PhantomData,
    }
}

pub type BytesKeyword_Reserved<'arena> = TerminalType<'arena>;
pub fn new_bytes_keyword_reserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> BytesKeyword_Reserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::BytesKeyword_Reserved,
        phantom: PhantomData,
    }
}

pub type CallDataKeyword_Reserved<'arena> = TerminalType<'arena>;
pub fn new_call_data_keyword_reserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> CallDataKeyword_Reserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::CallDataKeyword_Reserved,
        phantom: PhantomData,
    }
}

pub type CallDataKeyword_Unreserved<'arena> = TerminalType<'arena>;
pub fn new_call_data_keyword_unreserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> CallDataKeyword_Unreserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::CallDataKeyword_Unreserved,
        phantom: PhantomData,
    }
}

pub type Caret<'arena> = TerminalType<'arena>;
pub fn new_caret<'arena>(_arena: &'arena Bump, l: usize, r: usize, source: &str) -> Caret<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::Caret,
        phantom: PhantomData,
    }
}

pub type CaretEqual<'arena> = TerminalType<'arena>;
pub fn new_caret_equal<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> CaretEqual<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::CaretEqual,
        phantom: PhantomData,
    }
}

pub type CaseKeyword_Reserved<'arena> = TerminalType<'arena>;
pub fn new_case_keyword_reserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> CaseKeyword_Reserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::CaseKeyword_Reserved,
        phantom: PhantomData,
    }
}

pub type CatchKeyword_Reserved<'arena> = TerminalType<'arena>;
pub fn new_catch_keyword_reserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> CatchKeyword_Reserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::CatchKeyword_Reserved,
        phantom: PhantomData,
    }
}

pub type CloseBrace<'arena> = TerminalType<'arena>;
pub fn new_close_brace<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> CloseBrace<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::CloseBrace,
        phantom: PhantomData,
    }
}

pub type CloseBracket<'arena> = TerminalType<'arena>;
pub fn new_close_bracket<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> CloseBracket<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::CloseBracket,
        phantom: PhantomData,
    }
}

pub type CloseParen<'arena> = TerminalType<'arena>;
pub fn new_close_paren<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> CloseParen<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::CloseParen,
        phantom: PhantomData,
    }
}

pub type Colon<'arena> = TerminalType<'arena>;
pub fn new_colon<'arena>(_arena: &'arena Bump, l: usize, r: usize, source: &str) -> Colon<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::Colon,
        phantom: PhantomData,
    }
}

pub type ColonEqual<'arena> = TerminalType<'arena>;
pub fn new_colon_equal<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> ColonEqual<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::ColonEqual,
        phantom: PhantomData,
    }
}

pub type Comma<'arena> = TerminalType<'arena>;
pub fn new_comma<'arena>(_arena: &'arena Bump, l: usize, r: usize, source: &str) -> Comma<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::Comma,
        phantom: PhantomData,
    }
}

pub type ConstantKeyword_Reserved<'arena> = TerminalType<'arena>;
pub fn new_constant_keyword_reserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> ConstantKeyword_Reserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::ConstantKeyword_Reserved,
        phantom: PhantomData,
    }
}

pub type ConstructorKeyword_Reserved<'arena> = TerminalType<'arena>;
pub fn new_constructor_keyword_reserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> ConstructorKeyword_Reserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::ConstructorKeyword_Reserved,
        phantom: PhantomData,
    }
}

pub type ConstructorKeyword_Unreserved<'arena> = TerminalType<'arena>;
pub fn new_constructor_keyword_unreserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> ConstructorKeyword_Unreserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::ConstructorKeyword_Unreserved,
        phantom: PhantomData,
    }
}

pub type ContinueKeyword_Reserved<'arena> = TerminalType<'arena>;
pub fn new_continue_keyword_reserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> ContinueKeyword_Reserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::ContinueKeyword_Reserved,
        phantom: PhantomData,
    }
}

pub type ContractKeyword_Reserved<'arena> = TerminalType<'arena>;
pub fn new_contract_keyword_reserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> ContractKeyword_Reserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::ContractKeyword_Reserved,
        phantom: PhantomData,
    }
}

pub type CopyOfKeyword_Reserved<'arena> = TerminalType<'arena>;
pub fn new_copy_of_keyword_reserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> CopyOfKeyword_Reserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::CopyOfKeyword_Reserved,
        phantom: PhantomData,
    }
}

pub type CopyOfKeyword_Unreserved<'arena> = TerminalType<'arena>;
pub fn new_copy_of_keyword_unreserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> CopyOfKeyword_Unreserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::CopyOfKeyword_Unreserved,
        phantom: PhantomData,
    }
}

pub type DaysKeyword_Reserved<'arena> = TerminalType<'arena>;
pub fn new_days_keyword_reserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> DaysKeyword_Reserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::DaysKeyword_Reserved,
        phantom: PhantomData,
    }
}

pub type DecimalLiteral<'arena> = TerminalType<'arena>;
pub fn new_decimal_literal<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> DecimalLiteral<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::DecimalLiteral,
        phantom: PhantomData,
    }
}

pub type DefaultKeyword_Reserved<'arena> = TerminalType<'arena>;
pub fn new_default_keyword_reserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> DefaultKeyword_Reserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::DefaultKeyword_Reserved,
        phantom: PhantomData,
    }
}

pub type DefineKeyword_Reserved<'arena> = TerminalType<'arena>;
pub fn new_define_keyword_reserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> DefineKeyword_Reserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::DefineKeyword_Reserved,
        phantom: PhantomData,
    }
}

pub type DefineKeyword_Unreserved<'arena> = TerminalType<'arena>;
pub fn new_define_keyword_unreserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> DefineKeyword_Unreserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::DefineKeyword_Unreserved,
        phantom: PhantomData,
    }
}

pub type DeleteKeyword_Reserved<'arena> = TerminalType<'arena>;
pub fn new_delete_keyword_reserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> DeleteKeyword_Reserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::DeleteKeyword_Reserved,
        phantom: PhantomData,
    }
}

pub type DoKeyword_Reserved<'arena> = TerminalType<'arena>;
pub fn new_do_keyword_reserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> DoKeyword_Reserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::DoKeyword_Reserved,
        phantom: PhantomData,
    }
}

pub type DoubleQuotedHexStringLiteral<'arena> = TerminalType<'arena>;
pub fn new_double_quoted_hex_string_literal<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> DoubleQuotedHexStringLiteral<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::DoubleQuotedHexStringLiteral,
        phantom: PhantomData,
    }
}

pub type DoubleQuotedStringLiteral<'arena> = TerminalType<'arena>;
pub fn new_double_quoted_string_literal<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> DoubleQuotedStringLiteral<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::DoubleQuotedStringLiteral,
        phantom: PhantomData,
    }
}

pub type DoubleQuotedUnicodeStringLiteral<'arena> = TerminalType<'arena>;
pub fn new_double_quoted_unicode_string_literal<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> DoubleQuotedUnicodeStringLiteral<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::DoubleQuotedUnicodeStringLiteral,
        phantom: PhantomData,
    }
}

pub type DoubleQuotedVersionLiteral<'arena> = TerminalType<'arena>;
pub fn new_double_quoted_version_literal<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> DoubleQuotedVersionLiteral<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::DoubleQuotedVersionLiteral,
        phantom: PhantomData,
    }
}

pub type ElseKeyword_Reserved<'arena> = TerminalType<'arena>;
pub fn new_else_keyword_reserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> ElseKeyword_Reserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::ElseKeyword_Reserved,
        phantom: PhantomData,
    }
}

pub type EmitKeyword_Reserved<'arena> = TerminalType<'arena>;
pub fn new_emit_keyword_reserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> EmitKeyword_Reserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::EmitKeyword_Reserved,
        phantom: PhantomData,
    }
}

pub type EmitKeyword_Unreserved<'arena> = TerminalType<'arena>;
pub fn new_emit_keyword_unreserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> EmitKeyword_Unreserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::EmitKeyword_Unreserved,
        phantom: PhantomData,
    }
}

pub type EndOfLine<'arena> = TerminalType<'arena>;
pub fn new_end_of_line<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> EndOfLine<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::EndOfLine,
        phantom: PhantomData,
    }
}

pub type EnumKeyword_Reserved<'arena> = TerminalType<'arena>;
pub fn new_enum_keyword_reserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> EnumKeyword_Reserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::EnumKeyword_Reserved,
        phantom: PhantomData,
    }
}

pub type Equal<'arena> = TerminalType<'arena>;
pub fn new_equal<'arena>(_arena: &'arena Bump, l: usize, r: usize, source: &str) -> Equal<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::Equal,
        phantom: PhantomData,
    }
}

pub type EqualColon<'arena> = TerminalType<'arena>;
pub fn new_equal_colon<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> EqualColon<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::EqualColon,
        phantom: PhantomData,
    }
}

pub type EqualEqual<'arena> = TerminalType<'arena>;
pub fn new_equal_equal<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> EqualEqual<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::EqualEqual,
        phantom: PhantomData,
    }
}

pub type EqualGreaterThan<'arena> = TerminalType<'arena>;
pub fn new_equal_greater_than<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> EqualGreaterThan<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::EqualGreaterThan,
        phantom: PhantomData,
    }
}

pub type ErrorKeyword_Reserved<'arena> = TerminalType<'arena>;
pub fn new_error_keyword_reserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> ErrorKeyword_Reserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::ErrorKeyword_Reserved,
        phantom: PhantomData,
    }
}

pub type EtherKeyword_Reserved<'arena> = TerminalType<'arena>;
pub fn new_ether_keyword_reserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> EtherKeyword_Reserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::EtherKeyword_Reserved,
        phantom: PhantomData,
    }
}

pub type EventKeyword_Reserved<'arena> = TerminalType<'arena>;
pub fn new_event_keyword_reserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> EventKeyword_Reserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::EventKeyword_Reserved,
        phantom: PhantomData,
    }
}

pub type ExperimentalKeyword_Unreserved<'arena> = TerminalType<'arena>;
pub fn new_experimental_keyword_unreserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> ExperimentalKeyword_Unreserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::ExperimentalKeyword_Unreserved,
        phantom: PhantomData,
    }
}

pub type ExternalKeyword_Reserved<'arena> = TerminalType<'arena>;
pub fn new_external_keyword_reserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> ExternalKeyword_Reserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::ExternalKeyword_Reserved,
        phantom: PhantomData,
    }
}

pub type FallbackKeyword_Reserved<'arena> = TerminalType<'arena>;
pub fn new_fallback_keyword_reserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> FallbackKeyword_Reserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::FallbackKeyword_Reserved,
        phantom: PhantomData,
    }
}

pub type FallbackKeyword_Unreserved<'arena> = TerminalType<'arena>;
pub fn new_fallback_keyword_unreserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> FallbackKeyword_Unreserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::FallbackKeyword_Unreserved,
        phantom: PhantomData,
    }
}

pub type FalseKeyword_Reserved<'arena> = TerminalType<'arena>;
pub fn new_false_keyword_reserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> FalseKeyword_Reserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::FalseKeyword_Reserved,
        phantom: PhantomData,
    }
}

pub type FinalKeyword_Reserved<'arena> = TerminalType<'arena>;
pub fn new_final_keyword_reserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> FinalKeyword_Reserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::FinalKeyword_Reserved,
        phantom: PhantomData,
    }
}

pub type FinneyKeyword_Reserved<'arena> = TerminalType<'arena>;
pub fn new_finney_keyword_reserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> FinneyKeyword_Reserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::FinneyKeyword_Reserved,
        phantom: PhantomData,
    }
}

pub type FinneyKeyword_Unreserved<'arena> = TerminalType<'arena>;
pub fn new_finney_keyword_unreserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> FinneyKeyword_Unreserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::FinneyKeyword_Unreserved,
        phantom: PhantomData,
    }
}

pub type FixedKeyword_Reserved<'arena> = TerminalType<'arena>;
pub fn new_fixed_keyword_reserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> FixedKeyword_Reserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::FixedKeyword_Reserved,
        phantom: PhantomData,
    }
}

pub type FixedKeyword_Unreserved<'arena> = TerminalType<'arena>;
pub fn new_fixed_keyword_unreserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> FixedKeyword_Unreserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::FixedKeyword_Unreserved,
        phantom: PhantomData,
    }
}

pub type ForKeyword_Reserved<'arena> = TerminalType<'arena>;
pub fn new_for_keyword_reserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> ForKeyword_Reserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::ForKeyword_Reserved,
        phantom: PhantomData,
    }
}

pub type FromKeyword_Unreserved<'arena> = TerminalType<'arena>;
pub fn new_from_keyword_unreserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> FromKeyword_Unreserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::FromKeyword_Unreserved,
        phantom: PhantomData,
    }
}

pub type FunctionKeyword_Reserved<'arena> = TerminalType<'arena>;
pub fn new_function_keyword_reserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> FunctionKeyword_Reserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::FunctionKeyword_Reserved,
        phantom: PhantomData,
    }
}

pub type GlobalKeyword_Unreserved<'arena> = TerminalType<'arena>;
pub fn new_global_keyword_unreserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> GlobalKeyword_Unreserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::GlobalKeyword_Unreserved,
        phantom: PhantomData,
    }
}

pub type GreaterThan<'arena> = TerminalType<'arena>;
pub fn new_greater_than<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> GreaterThan<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::GreaterThan,
        phantom: PhantomData,
    }
}

pub type GreaterThanEqual<'arena> = TerminalType<'arena>;
pub fn new_greater_than_equal<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> GreaterThanEqual<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::GreaterThanEqual,
        phantom: PhantomData,
    }
}

pub type GreaterThanGreaterThan<'arena> = TerminalType<'arena>;
pub fn new_greater_than_greater_than<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> GreaterThanGreaterThan<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::GreaterThanGreaterThan,
        phantom: PhantomData,
    }
}

pub type GreaterThanGreaterThanEqual<'arena> = TerminalType<'arena>;
pub fn new_greater_than_greater_than_equal<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> GreaterThanGreaterThanEqual<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::GreaterThanGreaterThanEqual,
        phantom: PhantomData,
    }
}

pub type GreaterThanGreaterThanGreaterThan<'arena> = TerminalType<'arena>;
pub fn new_greater_than_greater_than_greater_than<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> GreaterThanGreaterThanGreaterThan<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::GreaterThanGreaterThanGreaterThan,
        phantom: PhantomData,
    }
}

pub type GreaterThanGreaterThanGreaterThanEqual<'arena> = TerminalType<'arena>;
pub fn new_greater_than_greater_than_greater_than_equal<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> GreaterThanGreaterThanGreaterThanEqual<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::GreaterThanGreaterThanGreaterThanEqual,
        phantom: PhantomData,
    }
}

pub type GweiKeyword_Reserved<'arena> = TerminalType<'arena>;
pub fn new_gwei_keyword_reserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> GweiKeyword_Reserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::GweiKeyword_Reserved,
        phantom: PhantomData,
    }
}

pub type GweiKeyword_Unreserved<'arena> = TerminalType<'arena>;
pub fn new_gwei_keyword_unreserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> GweiKeyword_Unreserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::GweiKeyword_Unreserved,
        phantom: PhantomData,
    }
}

pub type HexKeyword_Reserved<'arena> = TerminalType<'arena>;
pub fn new_hex_keyword_reserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> HexKeyword_Reserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::HexKeyword_Reserved,
        phantom: PhantomData,
    }
}

pub type HexLiteral<'arena> = TerminalType<'arena>;
pub fn new_hex_literal<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> HexLiteral<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::HexLiteral,
        phantom: PhantomData,
    }
}

pub type HoursKeyword_Reserved<'arena> = TerminalType<'arena>;
pub fn new_hours_keyword_reserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> HoursKeyword_Reserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::HoursKeyword_Reserved,
        phantom: PhantomData,
    }
}

pub type Identifier<'arena> = TerminalType<'arena>;
pub fn new_identifier<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> Identifier<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::Identifier,
        phantom: PhantomData,
    }
}

pub type IfKeyword_Reserved<'arena> = TerminalType<'arena>;
pub fn new_if_keyword_reserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> IfKeyword_Reserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::IfKeyword_Reserved,
        phantom: PhantomData,
    }
}

pub type ImmutableKeyword_Reserved<'arena> = TerminalType<'arena>;
pub fn new_immutable_keyword_reserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> ImmutableKeyword_Reserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::ImmutableKeyword_Reserved,
        phantom: PhantomData,
    }
}

pub type ImmutableKeyword_Unreserved<'arena> = TerminalType<'arena>;
pub fn new_immutable_keyword_unreserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> ImmutableKeyword_Unreserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::ImmutableKeyword_Unreserved,
        phantom: PhantomData,
    }
}

pub type ImplementsKeyword_Reserved<'arena> = TerminalType<'arena>;
pub fn new_implements_keyword_reserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> ImplementsKeyword_Reserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::ImplementsKeyword_Reserved,
        phantom: PhantomData,
    }
}

pub type ImplementsKeyword_Unreserved<'arena> = TerminalType<'arena>;
pub fn new_implements_keyword_unreserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> ImplementsKeyword_Unreserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::ImplementsKeyword_Unreserved,
        phantom: PhantomData,
    }
}

pub type ImportKeyword_Reserved<'arena> = TerminalType<'arena>;
pub fn new_import_keyword_reserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> ImportKeyword_Reserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::ImportKeyword_Reserved,
        phantom: PhantomData,
    }
}

pub type InKeyword_Reserved<'arena> = TerminalType<'arena>;
pub fn new_in_keyword_reserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> InKeyword_Reserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::InKeyword_Reserved,
        phantom: PhantomData,
    }
}

pub type IndexedKeyword_Reserved<'arena> = TerminalType<'arena>;
pub fn new_indexed_keyword_reserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> IndexedKeyword_Reserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::IndexedKeyword_Reserved,
        phantom: PhantomData,
    }
}

pub type InlineKeyword_Reserved<'arena> = TerminalType<'arena>;
pub fn new_inline_keyword_reserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> InlineKeyword_Reserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::InlineKeyword_Reserved,
        phantom: PhantomData,
    }
}

pub type IntKeyword_Reserved<'arena> = TerminalType<'arena>;
pub fn new_int_keyword_reserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> IntKeyword_Reserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::IntKeyword_Reserved,
        phantom: PhantomData,
    }
}

pub type InterfaceKeyword_Reserved<'arena> = TerminalType<'arena>;
pub fn new_interface_keyword_reserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> InterfaceKeyword_Reserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::InterfaceKeyword_Reserved,
        phantom: PhantomData,
    }
}

pub type InternalKeyword_Reserved<'arena> = TerminalType<'arena>;
pub fn new_internal_keyword_reserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> InternalKeyword_Reserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::InternalKeyword_Reserved,
        phantom: PhantomData,
    }
}

pub type IsKeyword_Reserved<'arena> = TerminalType<'arena>;
pub fn new_is_keyword_reserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> IsKeyword_Reserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::IsKeyword_Reserved,
        phantom: PhantomData,
    }
}

pub type LayoutKeyword_Unreserved<'arena> = TerminalType<'arena>;
pub fn new_layout_keyword_unreserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> LayoutKeyword_Unreserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::LayoutKeyword_Unreserved,
        phantom: PhantomData,
    }
}

pub type LessThan<'arena> = TerminalType<'arena>;
pub fn new_less_than<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> LessThan<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::LessThan,
        phantom: PhantomData,
    }
}

pub type LessThanEqual<'arena> = TerminalType<'arena>;
pub fn new_less_than_equal<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> LessThanEqual<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::LessThanEqual,
        phantom: PhantomData,
    }
}

pub type LessThanLessThan<'arena> = TerminalType<'arena>;
pub fn new_less_than_less_than<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> LessThanLessThan<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::LessThanLessThan,
        phantom: PhantomData,
    }
}

pub type LessThanLessThanEqual<'arena> = TerminalType<'arena>;
pub fn new_less_than_less_than_equal<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> LessThanLessThanEqual<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::LessThanLessThanEqual,
        phantom: PhantomData,
    }
}

pub type LetKeyword_Reserved<'arena> = TerminalType<'arena>;
pub fn new_let_keyword_reserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> LetKeyword_Reserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::LetKeyword_Reserved,
        phantom: PhantomData,
    }
}

pub type LibraryKeyword_Reserved<'arena> = TerminalType<'arena>;
pub fn new_library_keyword_reserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> LibraryKeyword_Reserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::LibraryKeyword_Reserved,
        phantom: PhantomData,
    }
}

pub type MacroKeyword_Reserved<'arena> = TerminalType<'arena>;
pub fn new_macro_keyword_reserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> MacroKeyword_Reserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::MacroKeyword_Reserved,
        phantom: PhantomData,
    }
}

pub type MacroKeyword_Unreserved<'arena> = TerminalType<'arena>;
pub fn new_macro_keyword_unreserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> MacroKeyword_Unreserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::MacroKeyword_Unreserved,
        phantom: PhantomData,
    }
}

pub type MappingKeyword_Reserved<'arena> = TerminalType<'arena>;
pub fn new_mapping_keyword_reserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> MappingKeyword_Reserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::MappingKeyword_Reserved,
        phantom: PhantomData,
    }
}

pub type MatchKeyword_Reserved<'arena> = TerminalType<'arena>;
pub fn new_match_keyword_reserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> MatchKeyword_Reserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::MatchKeyword_Reserved,
        phantom: PhantomData,
    }
}

pub type MemoryKeyword_Reserved<'arena> = TerminalType<'arena>;
pub fn new_memory_keyword_reserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> MemoryKeyword_Reserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::MemoryKeyword_Reserved,
        phantom: PhantomData,
    }
}

pub type Minus<'arena> = TerminalType<'arena>;
pub fn new_minus<'arena>(_arena: &'arena Bump, l: usize, r: usize, source: &str) -> Minus<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::Minus,
        phantom: PhantomData,
    }
}

pub type MinusEqual<'arena> = TerminalType<'arena>;
pub fn new_minus_equal<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> MinusEqual<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::MinusEqual,
        phantom: PhantomData,
    }
}

pub type MinusGreaterThan<'arena> = TerminalType<'arena>;
pub fn new_minus_greater_than<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> MinusGreaterThan<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::MinusGreaterThan,
        phantom: PhantomData,
    }
}

pub type MinusMinus<'arena> = TerminalType<'arena>;
pub fn new_minus_minus<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> MinusMinus<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::MinusMinus,
        phantom: PhantomData,
    }
}

pub type MinutesKeyword_Reserved<'arena> = TerminalType<'arena>;
pub fn new_minutes_keyword_reserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> MinutesKeyword_Reserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::MinutesKeyword_Reserved,
        phantom: PhantomData,
    }
}

pub type ModifierKeyword_Reserved<'arena> = TerminalType<'arena>;
pub fn new_modifier_keyword_reserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> ModifierKeyword_Reserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::ModifierKeyword_Reserved,
        phantom: PhantomData,
    }
}

pub type MultiLineComment<'arena> = TerminalType<'arena>;
pub fn new_multi_line_comment<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> MultiLineComment<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::MultiLineComment,
        phantom: PhantomData,
    }
}

pub type MultiLineNatSpecComment<'arena> = TerminalType<'arena>;
pub fn new_multi_line_nat_spec_comment<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> MultiLineNatSpecComment<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::MultiLineNatSpecComment,
        phantom: PhantomData,
    }
}

pub type MutableKeyword_Reserved<'arena> = TerminalType<'arena>;
pub fn new_mutable_keyword_reserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> MutableKeyword_Reserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::MutableKeyword_Reserved,
        phantom: PhantomData,
    }
}

pub type MutableKeyword_Unreserved<'arena> = TerminalType<'arena>;
pub fn new_mutable_keyword_unreserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> MutableKeyword_Unreserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::MutableKeyword_Unreserved,
        phantom: PhantomData,
    }
}

pub type NewKeyword_Reserved<'arena> = TerminalType<'arena>;
pub fn new_new_keyword_reserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> NewKeyword_Reserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::NewKeyword_Reserved,
        phantom: PhantomData,
    }
}

pub type NullKeyword_Reserved<'arena> = TerminalType<'arena>;
pub fn new_null_keyword_reserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> NullKeyword_Reserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::NullKeyword_Reserved,
        phantom: PhantomData,
    }
}

pub type OfKeyword_Reserved<'arena> = TerminalType<'arena>;
pub fn new_of_keyword_reserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> OfKeyword_Reserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::OfKeyword_Reserved,
        phantom: PhantomData,
    }
}

pub type OpenBrace<'arena> = TerminalType<'arena>;
pub fn new_open_brace<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> OpenBrace<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::OpenBrace,
        phantom: PhantomData,
    }
}

pub type OpenBracket<'arena> = TerminalType<'arena>;
pub fn new_open_bracket<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> OpenBracket<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::OpenBracket,
        phantom: PhantomData,
    }
}

pub type OpenParen<'arena> = TerminalType<'arena>;
pub fn new_open_paren<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> OpenParen<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::OpenParen,
        phantom: PhantomData,
    }
}

pub type OverrideKeyword_Reserved<'arena> = TerminalType<'arena>;
pub fn new_override_keyword_reserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> OverrideKeyword_Reserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::OverrideKeyword_Reserved,
        phantom: PhantomData,
    }
}

pub type OverrideKeyword_Unreserved<'arena> = TerminalType<'arena>;
pub fn new_override_keyword_unreserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> OverrideKeyword_Unreserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::OverrideKeyword_Unreserved,
        phantom: PhantomData,
    }
}

pub type PartialKeyword_Reserved<'arena> = TerminalType<'arena>;
pub fn new_partial_keyword_reserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> PartialKeyword_Reserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::PartialKeyword_Reserved,
        phantom: PhantomData,
    }
}

pub type PartialKeyword_Unreserved<'arena> = TerminalType<'arena>;
pub fn new_partial_keyword_unreserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> PartialKeyword_Unreserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::PartialKeyword_Unreserved,
        phantom: PhantomData,
    }
}

pub type PayableKeyword_Reserved<'arena> = TerminalType<'arena>;
pub fn new_payable_keyword_reserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> PayableKeyword_Reserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::PayableKeyword_Reserved,
        phantom: PhantomData,
    }
}

pub type Percent<'arena> = TerminalType<'arena>;
pub fn new_percent<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> Percent<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::Percent,
        phantom: PhantomData,
    }
}

pub type PercentEqual<'arena> = TerminalType<'arena>;
pub fn new_percent_equal<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> PercentEqual<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::PercentEqual,
        phantom: PhantomData,
    }
}

pub type Period<'arena> = TerminalType<'arena>;
pub fn new_period<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> Period<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::Period,
        phantom: PhantomData,
    }
}

pub type Plus<'arena> = TerminalType<'arena>;
pub fn new_plus<'arena>(_arena: &'arena Bump, l: usize, r: usize, source: &str) -> Plus<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::Plus,
        phantom: PhantomData,
    }
}

pub type PlusEqual<'arena> = TerminalType<'arena>;
pub fn new_plus_equal<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> PlusEqual<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::PlusEqual,
        phantom: PhantomData,
    }
}

pub type PlusPlus<'arena> = TerminalType<'arena>;
pub fn new_plus_plus<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> PlusPlus<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::PlusPlus,
        phantom: PhantomData,
    }
}

pub type PragmaKeyword_Reserved<'arena> = TerminalType<'arena>;
pub fn new_pragma_keyword_reserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> PragmaKeyword_Reserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::PragmaKeyword_Reserved,
        phantom: PhantomData,
    }
}

pub type PrivateKeyword_Reserved<'arena> = TerminalType<'arena>;
pub fn new_private_keyword_reserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> PrivateKeyword_Reserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::PrivateKeyword_Reserved,
        phantom: PhantomData,
    }
}

pub type PromiseKeyword_Reserved<'arena> = TerminalType<'arena>;
pub fn new_promise_keyword_reserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> PromiseKeyword_Reserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::PromiseKeyword_Reserved,
        phantom: PhantomData,
    }
}

pub type PromiseKeyword_Unreserved<'arena> = TerminalType<'arena>;
pub fn new_promise_keyword_unreserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> PromiseKeyword_Unreserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::PromiseKeyword_Unreserved,
        phantom: PhantomData,
    }
}

pub type PublicKeyword_Reserved<'arena> = TerminalType<'arena>;
pub fn new_public_keyword_reserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> PublicKeyword_Reserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::PublicKeyword_Reserved,
        phantom: PhantomData,
    }
}

pub type PureKeyword_Reserved<'arena> = TerminalType<'arena>;
pub fn new_pure_keyword_reserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> PureKeyword_Reserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::PureKeyword_Reserved,
        phantom: PhantomData,
    }
}

pub type QuestionMark<'arena> = TerminalType<'arena>;
pub fn new_question_mark<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> QuestionMark<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::QuestionMark,
        phantom: PhantomData,
    }
}

pub type ReceiveKeyword_Reserved<'arena> = TerminalType<'arena>;
pub fn new_receive_keyword_reserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> ReceiveKeyword_Reserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::ReceiveKeyword_Reserved,
        phantom: PhantomData,
    }
}

pub type ReceiveKeyword_Unreserved<'arena> = TerminalType<'arena>;
pub fn new_receive_keyword_unreserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> ReceiveKeyword_Unreserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::ReceiveKeyword_Unreserved,
        phantom: PhantomData,
    }
}

pub type ReferenceKeyword_Reserved<'arena> = TerminalType<'arena>;
pub fn new_reference_keyword_reserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> ReferenceKeyword_Reserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::ReferenceKeyword_Reserved,
        phantom: PhantomData,
    }
}

pub type ReferenceKeyword_Unreserved<'arena> = TerminalType<'arena>;
pub fn new_reference_keyword_unreserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> ReferenceKeyword_Unreserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::ReferenceKeyword_Unreserved,
        phantom: PhantomData,
    }
}

pub type RelocatableKeyword_Reserved<'arena> = TerminalType<'arena>;
pub fn new_relocatable_keyword_reserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> RelocatableKeyword_Reserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::RelocatableKeyword_Reserved,
        phantom: PhantomData,
    }
}

pub type ReturnKeyword_Reserved<'arena> = TerminalType<'arena>;
pub fn new_return_keyword_reserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> ReturnKeyword_Reserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::ReturnKeyword_Reserved,
        phantom: PhantomData,
    }
}

pub type ReturnsKeyword_Reserved<'arena> = TerminalType<'arena>;
pub fn new_returns_keyword_reserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> ReturnsKeyword_Reserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::ReturnsKeyword_Reserved,
        phantom: PhantomData,
    }
}

pub type RevertKeyword_Reserved<'arena> = TerminalType<'arena>;
pub fn new_revert_keyword_reserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> RevertKeyword_Reserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::RevertKeyword_Reserved,
        phantom: PhantomData,
    }
}

pub type SMTCheckerKeyword_Unreserved<'arena> = TerminalType<'arena>;
pub fn new_smt_checker_keyword_unreserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> SMTCheckerKeyword_Unreserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::SMTCheckerKeyword_Unreserved,
        phantom: PhantomData,
    }
}

pub type SealedKeyword_Reserved<'arena> = TerminalType<'arena>;
pub fn new_sealed_keyword_reserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> SealedKeyword_Reserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::SealedKeyword_Reserved,
        phantom: PhantomData,
    }
}

pub type SealedKeyword_Unreserved<'arena> = TerminalType<'arena>;
pub fn new_sealed_keyword_unreserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> SealedKeyword_Unreserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::SealedKeyword_Unreserved,
        phantom: PhantomData,
    }
}

pub type SecondsKeyword_Reserved<'arena> = TerminalType<'arena>;
pub fn new_seconds_keyword_reserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> SecondsKeyword_Reserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::SecondsKeyword_Reserved,
        phantom: PhantomData,
    }
}

pub type Semicolon<'arena> = TerminalType<'arena>;
pub fn new_semicolon<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> Semicolon<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::Semicolon,
        phantom: PhantomData,
    }
}

pub type SingleLineComment<'arena> = TerminalType<'arena>;
pub fn new_single_line_comment<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> SingleLineComment<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::SingleLineComment,
        phantom: PhantomData,
    }
}

pub type SingleLineNatSpecComment<'arena> = TerminalType<'arena>;
pub fn new_single_line_nat_spec_comment<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> SingleLineNatSpecComment<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::SingleLineNatSpecComment,
        phantom: PhantomData,
    }
}

pub type SingleQuotedHexStringLiteral<'arena> = TerminalType<'arena>;
pub fn new_single_quoted_hex_string_literal<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> SingleQuotedHexStringLiteral<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::SingleQuotedHexStringLiteral,
        phantom: PhantomData,
    }
}

pub type SingleQuotedStringLiteral<'arena> = TerminalType<'arena>;
pub fn new_single_quoted_string_literal<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> SingleQuotedStringLiteral<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::SingleQuotedStringLiteral,
        phantom: PhantomData,
    }
}

pub type SingleQuotedUnicodeStringLiteral<'arena> = TerminalType<'arena>;
pub fn new_single_quoted_unicode_string_literal<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> SingleQuotedUnicodeStringLiteral<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::SingleQuotedUnicodeStringLiteral,
        phantom: PhantomData,
    }
}

pub type SingleQuotedVersionLiteral<'arena> = TerminalType<'arena>;
pub fn new_single_quoted_version_literal<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> SingleQuotedVersionLiteral<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::SingleQuotedVersionLiteral,
        phantom: PhantomData,
    }
}

pub type SizeOfKeyword_Reserved<'arena> = TerminalType<'arena>;
pub fn new_size_of_keyword_reserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> SizeOfKeyword_Reserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::SizeOfKeyword_Reserved,
        phantom: PhantomData,
    }
}

pub type SizeOfKeyword_Unreserved<'arena> = TerminalType<'arena>;
pub fn new_size_of_keyword_unreserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> SizeOfKeyword_Unreserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::SizeOfKeyword_Unreserved,
        phantom: PhantomData,
    }
}

pub type Slash<'arena> = TerminalType<'arena>;
pub fn new_slash<'arena>(_arena: &'arena Bump, l: usize, r: usize, source: &str) -> Slash<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::Slash,
        phantom: PhantomData,
    }
}

pub type SlashEqual<'arena> = TerminalType<'arena>;
pub fn new_slash_equal<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> SlashEqual<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::SlashEqual,
        phantom: PhantomData,
    }
}

pub type SolidityKeyword_Unreserved<'arena> = TerminalType<'arena>;
pub fn new_solidity_keyword_unreserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> SolidityKeyword_Unreserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::SolidityKeyword_Unreserved,
        phantom: PhantomData,
    }
}

pub type StaticKeyword_Reserved<'arena> = TerminalType<'arena>;
pub fn new_static_keyword_reserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> StaticKeyword_Reserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::StaticKeyword_Reserved,
        phantom: PhantomData,
    }
}

pub type StorageKeyword_Reserved<'arena> = TerminalType<'arena>;
pub fn new_storage_keyword_reserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> StorageKeyword_Reserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::StorageKeyword_Reserved,
        phantom: PhantomData,
    }
}

pub type StringKeyword_Reserved<'arena> = TerminalType<'arena>;
pub fn new_string_keyword_reserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> StringKeyword_Reserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::StringKeyword_Reserved,
        phantom: PhantomData,
    }
}

pub type StructKeyword_Reserved<'arena> = TerminalType<'arena>;
pub fn new_struct_keyword_reserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> StructKeyword_Reserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::StructKeyword_Reserved,
        phantom: PhantomData,
    }
}

pub type SuperKeyword_Reserved<'arena> = TerminalType<'arena>;
pub fn new_super_keyword_reserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> SuperKeyword_Reserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::SuperKeyword_Reserved,
        phantom: PhantomData,
    }
}

pub type SuperKeyword_Unreserved<'arena> = TerminalType<'arena>;
pub fn new_super_keyword_unreserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> SuperKeyword_Unreserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::SuperKeyword_Unreserved,
        phantom: PhantomData,
    }
}

pub type SupportsKeyword_Reserved<'arena> = TerminalType<'arena>;
pub fn new_supports_keyword_reserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> SupportsKeyword_Reserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::SupportsKeyword_Reserved,
        phantom: PhantomData,
    }
}

pub type SupportsKeyword_Unreserved<'arena> = TerminalType<'arena>;
pub fn new_supports_keyword_unreserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> SupportsKeyword_Unreserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::SupportsKeyword_Unreserved,
        phantom: PhantomData,
    }
}

pub type SwitchKeyword_Reserved<'arena> = TerminalType<'arena>;
pub fn new_switch_keyword_reserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> SwitchKeyword_Reserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::SwitchKeyword_Reserved,
        phantom: PhantomData,
    }
}

pub type SzaboKeyword_Reserved<'arena> = TerminalType<'arena>;
pub fn new_szabo_keyword_reserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> SzaboKeyword_Reserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::SzaboKeyword_Reserved,
        phantom: PhantomData,
    }
}

pub type SzaboKeyword_Unreserved<'arena> = TerminalType<'arena>;
pub fn new_szabo_keyword_unreserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> SzaboKeyword_Unreserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::SzaboKeyword_Unreserved,
        phantom: PhantomData,
    }
}

pub type ThisKeyword_Reserved<'arena> = TerminalType<'arena>;
pub fn new_this_keyword_reserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> ThisKeyword_Reserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::ThisKeyword_Reserved,
        phantom: PhantomData,
    }
}

pub type ThisKeyword_Unreserved<'arena> = TerminalType<'arena>;
pub fn new_this_keyword_unreserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> ThisKeyword_Unreserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::ThisKeyword_Unreserved,
        phantom: PhantomData,
    }
}

pub type ThrowKeyword_Reserved<'arena> = TerminalType<'arena>;
pub fn new_throw_keyword_reserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> ThrowKeyword_Reserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::ThrowKeyword_Reserved,
        phantom: PhantomData,
    }
}

pub type Tilde<'arena> = TerminalType<'arena>;
pub fn new_tilde<'arena>(_arena: &'arena Bump, l: usize, r: usize, source: &str) -> Tilde<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::Tilde,
        phantom: PhantomData,
    }
}

pub type TransientKeyword_Unreserved<'arena> = TerminalType<'arena>;
pub fn new_transient_keyword_unreserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> TransientKeyword_Unreserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::TransientKeyword_Unreserved,
        phantom: PhantomData,
    }
}

pub type TrueKeyword_Reserved<'arena> = TerminalType<'arena>;
pub fn new_true_keyword_reserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> TrueKeyword_Reserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::TrueKeyword_Reserved,
        phantom: PhantomData,
    }
}

pub type TryKeyword_Reserved<'arena> = TerminalType<'arena>;
pub fn new_try_keyword_reserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> TryKeyword_Reserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::TryKeyword_Reserved,
        phantom: PhantomData,
    }
}

pub type TypeDefKeyword_Reserved<'arena> = TerminalType<'arena>;
pub fn new_type_def_keyword_reserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> TypeDefKeyword_Reserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::TypeDefKeyword_Reserved,
        phantom: PhantomData,
    }
}

pub type TypeDefKeyword_Unreserved<'arena> = TerminalType<'arena>;
pub fn new_type_def_keyword_unreserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> TypeDefKeyword_Unreserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::TypeDefKeyword_Unreserved,
        phantom: PhantomData,
    }
}

pub type TypeKeyword_Reserved<'arena> = TerminalType<'arena>;
pub fn new_type_keyword_reserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> TypeKeyword_Reserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::TypeKeyword_Reserved,
        phantom: PhantomData,
    }
}

pub type TypeOfKeyword_Reserved<'arena> = TerminalType<'arena>;
pub fn new_type_of_keyword_reserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> TypeOfKeyword_Reserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::TypeOfKeyword_Reserved,
        phantom: PhantomData,
    }
}

pub type UfixedKeyword_Reserved<'arena> = TerminalType<'arena>;
pub fn new_ufixed_keyword_reserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> UfixedKeyword_Reserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::UfixedKeyword_Reserved,
        phantom: PhantomData,
    }
}

pub type UfixedKeyword_Unreserved<'arena> = TerminalType<'arena>;
pub fn new_ufixed_keyword_unreserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> UfixedKeyword_Unreserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::UfixedKeyword_Unreserved,
        phantom: PhantomData,
    }
}

pub type UintKeyword_Reserved<'arena> = TerminalType<'arena>;
pub fn new_uint_keyword_reserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> UintKeyword_Reserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::UintKeyword_Reserved,
        phantom: PhantomData,
    }
}

pub type UncheckedKeyword_Reserved<'arena> = TerminalType<'arena>;
pub fn new_unchecked_keyword_reserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> UncheckedKeyword_Reserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::UncheckedKeyword_Reserved,
        phantom: PhantomData,
    }
}

pub type UncheckedKeyword_Unreserved<'arena> = TerminalType<'arena>;
pub fn new_unchecked_keyword_unreserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> UncheckedKeyword_Unreserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::UncheckedKeyword_Unreserved,
        phantom: PhantomData,
    }
}

pub type UsingKeyword_Reserved<'arena> = TerminalType<'arena>;
pub fn new_using_keyword_reserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> UsingKeyword_Reserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::UsingKeyword_Reserved,
        phantom: PhantomData,
    }
}

pub type VarKeyword_Reserved<'arena> = TerminalType<'arena>;
pub fn new_var_keyword_reserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> VarKeyword_Reserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::VarKeyword_Reserved,
        phantom: PhantomData,
    }
}

pub type VersionSpecifier<'arena> = TerminalType<'arena>;
pub fn new_version_specifier<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> VersionSpecifier<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::VersionSpecifier,
        phantom: PhantomData,
    }
}

pub type ViewKeyword_Reserved<'arena> = TerminalType<'arena>;
pub fn new_view_keyword_reserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> ViewKeyword_Reserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::ViewKeyword_Reserved,
        phantom: PhantomData,
    }
}

pub type VirtualKeyword_Reserved<'arena> = TerminalType<'arena>;
pub fn new_virtual_keyword_reserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> VirtualKeyword_Reserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::VirtualKeyword_Reserved,
        phantom: PhantomData,
    }
}

pub type VirtualKeyword_Unreserved<'arena> = TerminalType<'arena>;
pub fn new_virtual_keyword_unreserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> VirtualKeyword_Unreserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::VirtualKeyword_Unreserved,
        phantom: PhantomData,
    }
}

pub type WeeksKeyword_Reserved<'arena> = TerminalType<'arena>;
pub fn new_weeks_keyword_reserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> WeeksKeyword_Reserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::WeeksKeyword_Reserved,
        phantom: PhantomData,
    }
}

pub type WeiKeyword_Reserved<'arena> = TerminalType<'arena>;
pub fn new_wei_keyword_reserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> WeiKeyword_Reserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::WeiKeyword_Reserved,
        phantom: PhantomData,
    }
}

pub type WhileKeyword_Reserved<'arena> = TerminalType<'arena>;
pub fn new_while_keyword_reserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> WhileKeyword_Reserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::WhileKeyword_Reserved,
        phantom: PhantomData,
    }
}

pub type Whitespace<'arena> = TerminalType<'arena>;
pub fn new_whitespace<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> Whitespace<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::Whitespace,
        phantom: PhantomData,
    }
}

pub type YearsKeyword_Reserved<'arena> = TerminalType<'arena>;
pub fn new_years_keyword_reserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> YearsKeyword_Reserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::YearsKeyword_Reserved,
        phantom: PhantomData,
    }
}

pub type YulAbstractKeyword_Reserved<'arena> = TerminalType<'arena>;
pub fn new_yul_abstract_keyword_reserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> YulAbstractKeyword_Reserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::YulAbstractKeyword_Reserved,
        phantom: PhantomData,
    }
}

pub type YulAbstractKeyword_Unreserved<'arena> = TerminalType<'arena>;
pub fn new_yul_abstract_keyword_unreserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> YulAbstractKeyword_Unreserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::YulAbstractKeyword_Unreserved,
        phantom: PhantomData,
    }
}

pub type YulAfterKeyword_Reserved<'arena> = TerminalType<'arena>;
pub fn new_yul_after_keyword_reserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> YulAfterKeyword_Reserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::YulAfterKeyword_Reserved,
        phantom: PhantomData,
    }
}

pub type YulAfterKeyword_Unreserved<'arena> = TerminalType<'arena>;
pub fn new_yul_after_keyword_unreserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> YulAfterKeyword_Unreserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::YulAfterKeyword_Unreserved,
        phantom: PhantomData,
    }
}

pub type YulAliasKeyword_Reserved<'arena> = TerminalType<'arena>;
pub fn new_yul_alias_keyword_reserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> YulAliasKeyword_Reserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::YulAliasKeyword_Reserved,
        phantom: PhantomData,
    }
}

pub type YulAliasKeyword_Unreserved<'arena> = TerminalType<'arena>;
pub fn new_yul_alias_keyword_unreserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> YulAliasKeyword_Unreserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::YulAliasKeyword_Unreserved,
        phantom: PhantomData,
    }
}

pub type YulAnonymousKeyword_Reserved<'arena> = TerminalType<'arena>;
pub fn new_yul_anonymous_keyword_reserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> YulAnonymousKeyword_Reserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::YulAnonymousKeyword_Reserved,
        phantom: PhantomData,
    }
}

pub type YulAnonymousKeyword_Unreserved<'arena> = TerminalType<'arena>;
pub fn new_yul_anonymous_keyword_unreserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> YulAnonymousKeyword_Unreserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::YulAnonymousKeyword_Unreserved,
        phantom: PhantomData,
    }
}

pub type YulApplyKeyword_Reserved<'arena> = TerminalType<'arena>;
pub fn new_yul_apply_keyword_reserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> YulApplyKeyword_Reserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::YulApplyKeyword_Reserved,
        phantom: PhantomData,
    }
}

pub type YulApplyKeyword_Unreserved<'arena> = TerminalType<'arena>;
pub fn new_yul_apply_keyword_unreserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> YulApplyKeyword_Unreserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::YulApplyKeyword_Unreserved,
        phantom: PhantomData,
    }
}

pub type YulAsKeyword_Reserved<'arena> = TerminalType<'arena>;
pub fn new_yul_as_keyword_reserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> YulAsKeyword_Reserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::YulAsKeyword_Reserved,
        phantom: PhantomData,
    }
}

pub type YulAsKeyword_Unreserved<'arena> = TerminalType<'arena>;
pub fn new_yul_as_keyword_unreserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> YulAsKeyword_Unreserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::YulAsKeyword_Unreserved,
        phantom: PhantomData,
    }
}

pub type YulAssemblyKeyword_Reserved<'arena> = TerminalType<'arena>;
pub fn new_yul_assembly_keyword_reserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> YulAssemblyKeyword_Reserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::YulAssemblyKeyword_Reserved,
        phantom: PhantomData,
    }
}

pub type YulAssemblyKeyword_Unreserved<'arena> = TerminalType<'arena>;
pub fn new_yul_assembly_keyword_unreserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> YulAssemblyKeyword_Unreserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::YulAssemblyKeyword_Unreserved,
        phantom: PhantomData,
    }
}

pub type YulAutoKeyword_Reserved<'arena> = TerminalType<'arena>;
pub fn new_yul_auto_keyword_reserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> YulAutoKeyword_Reserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::YulAutoKeyword_Reserved,
        phantom: PhantomData,
    }
}

pub type YulAutoKeyword_Unreserved<'arena> = TerminalType<'arena>;
pub fn new_yul_auto_keyword_unreserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> YulAutoKeyword_Unreserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::YulAutoKeyword_Unreserved,
        phantom: PhantomData,
    }
}

pub type YulBoolKeyword_Reserved<'arena> = TerminalType<'arena>;
pub fn new_yul_bool_keyword_reserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> YulBoolKeyword_Reserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::YulBoolKeyword_Reserved,
        phantom: PhantomData,
    }
}

pub type YulBoolKeyword_Unreserved<'arena> = TerminalType<'arena>;
pub fn new_yul_bool_keyword_unreserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> YulBoolKeyword_Unreserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::YulBoolKeyword_Unreserved,
        phantom: PhantomData,
    }
}

pub type YulBreakKeyword_Reserved<'arena> = TerminalType<'arena>;
pub fn new_yul_break_keyword_reserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> YulBreakKeyword_Reserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::YulBreakKeyword_Reserved,
        phantom: PhantomData,
    }
}

pub type YulBytesKeyword_Reserved<'arena> = TerminalType<'arena>;
pub fn new_yul_bytes_keyword_reserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> YulBytesKeyword_Reserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::YulBytesKeyword_Reserved,
        phantom: PhantomData,
    }
}

pub type YulBytesKeyword_Unreserved<'arena> = TerminalType<'arena>;
pub fn new_yul_bytes_keyword_unreserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> YulBytesKeyword_Unreserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::YulBytesKeyword_Unreserved,
        phantom: PhantomData,
    }
}

pub type YulCallDataKeyword_Reserved<'arena> = TerminalType<'arena>;
pub fn new_yul_call_data_keyword_reserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> YulCallDataKeyword_Reserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::YulCallDataKeyword_Reserved,
        phantom: PhantomData,
    }
}

pub type YulCallDataKeyword_Unreserved<'arena> = TerminalType<'arena>;
pub fn new_yul_call_data_keyword_unreserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> YulCallDataKeyword_Unreserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::YulCallDataKeyword_Unreserved,
        phantom: PhantomData,
    }
}

pub type YulCaseKeyword_Reserved<'arena> = TerminalType<'arena>;
pub fn new_yul_case_keyword_reserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> YulCaseKeyword_Reserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::YulCaseKeyword_Reserved,
        phantom: PhantomData,
    }
}

pub type YulCatchKeyword_Reserved<'arena> = TerminalType<'arena>;
pub fn new_yul_catch_keyword_reserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> YulCatchKeyword_Reserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::YulCatchKeyword_Reserved,
        phantom: PhantomData,
    }
}

pub type YulCatchKeyword_Unreserved<'arena> = TerminalType<'arena>;
pub fn new_yul_catch_keyword_unreserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> YulCatchKeyword_Unreserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::YulCatchKeyword_Unreserved,
        phantom: PhantomData,
    }
}

pub type YulConstantKeyword_Reserved<'arena> = TerminalType<'arena>;
pub fn new_yul_constant_keyword_reserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> YulConstantKeyword_Reserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::YulConstantKeyword_Reserved,
        phantom: PhantomData,
    }
}

pub type YulConstantKeyword_Unreserved<'arena> = TerminalType<'arena>;
pub fn new_yul_constant_keyword_unreserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> YulConstantKeyword_Unreserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::YulConstantKeyword_Unreserved,
        phantom: PhantomData,
    }
}

pub type YulConstructorKeyword_Reserved<'arena> = TerminalType<'arena>;
pub fn new_yul_constructor_keyword_reserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> YulConstructorKeyword_Reserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::YulConstructorKeyword_Reserved,
        phantom: PhantomData,
    }
}

pub type YulConstructorKeyword_Unreserved<'arena> = TerminalType<'arena>;
pub fn new_yul_constructor_keyword_unreserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> YulConstructorKeyword_Unreserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::YulConstructorKeyword_Unreserved,
        phantom: PhantomData,
    }
}

pub type YulContinueKeyword_Reserved<'arena> = TerminalType<'arena>;
pub fn new_yul_continue_keyword_reserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> YulContinueKeyword_Reserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::YulContinueKeyword_Reserved,
        phantom: PhantomData,
    }
}

pub type YulContractKeyword_Reserved<'arena> = TerminalType<'arena>;
pub fn new_yul_contract_keyword_reserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> YulContractKeyword_Reserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::YulContractKeyword_Reserved,
        phantom: PhantomData,
    }
}

pub type YulContractKeyword_Unreserved<'arena> = TerminalType<'arena>;
pub fn new_yul_contract_keyword_unreserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> YulContractKeyword_Unreserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::YulContractKeyword_Unreserved,
        phantom: PhantomData,
    }
}

pub type YulCopyOfKeyword_Reserved<'arena> = TerminalType<'arena>;
pub fn new_yul_copy_of_keyword_reserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> YulCopyOfKeyword_Reserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::YulCopyOfKeyword_Reserved,
        phantom: PhantomData,
    }
}

pub type YulCopyOfKeyword_Unreserved<'arena> = TerminalType<'arena>;
pub fn new_yul_copy_of_keyword_unreserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> YulCopyOfKeyword_Unreserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::YulCopyOfKeyword_Unreserved,
        phantom: PhantomData,
    }
}

pub type YulDaysKeyword_Reserved<'arena> = TerminalType<'arena>;
pub fn new_yul_days_keyword_reserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> YulDaysKeyword_Reserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::YulDaysKeyword_Reserved,
        phantom: PhantomData,
    }
}

pub type YulDaysKeyword_Unreserved<'arena> = TerminalType<'arena>;
pub fn new_yul_days_keyword_unreserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> YulDaysKeyword_Unreserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::YulDaysKeyword_Unreserved,
        phantom: PhantomData,
    }
}

pub type YulDecimalLiteral<'arena> = TerminalType<'arena>;
pub fn new_yul_decimal_literal<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> YulDecimalLiteral<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::YulDecimalLiteral,
        phantom: PhantomData,
    }
}

pub type YulDefaultKeyword_Reserved<'arena> = TerminalType<'arena>;
pub fn new_yul_default_keyword_reserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> YulDefaultKeyword_Reserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::YulDefaultKeyword_Reserved,
        phantom: PhantomData,
    }
}

pub type YulDefineKeyword_Reserved<'arena> = TerminalType<'arena>;
pub fn new_yul_define_keyword_reserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> YulDefineKeyword_Reserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::YulDefineKeyword_Reserved,
        phantom: PhantomData,
    }
}

pub type YulDefineKeyword_Unreserved<'arena> = TerminalType<'arena>;
pub fn new_yul_define_keyword_unreserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> YulDefineKeyword_Unreserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::YulDefineKeyword_Unreserved,
        phantom: PhantomData,
    }
}

pub type YulDeleteKeyword_Reserved<'arena> = TerminalType<'arena>;
pub fn new_yul_delete_keyword_reserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> YulDeleteKeyword_Reserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::YulDeleteKeyword_Reserved,
        phantom: PhantomData,
    }
}

pub type YulDeleteKeyword_Unreserved<'arena> = TerminalType<'arena>;
pub fn new_yul_delete_keyword_unreserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> YulDeleteKeyword_Unreserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::YulDeleteKeyword_Unreserved,
        phantom: PhantomData,
    }
}

pub type YulDoKeyword_Reserved<'arena> = TerminalType<'arena>;
pub fn new_yul_do_keyword_reserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> YulDoKeyword_Reserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::YulDoKeyword_Reserved,
        phantom: PhantomData,
    }
}

pub type YulDoKeyword_Unreserved<'arena> = TerminalType<'arena>;
pub fn new_yul_do_keyword_unreserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> YulDoKeyword_Unreserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::YulDoKeyword_Unreserved,
        phantom: PhantomData,
    }
}

pub type YulElseKeyword_Reserved<'arena> = TerminalType<'arena>;
pub fn new_yul_else_keyword_reserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> YulElseKeyword_Reserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::YulElseKeyword_Reserved,
        phantom: PhantomData,
    }
}

pub type YulElseKeyword_Unreserved<'arena> = TerminalType<'arena>;
pub fn new_yul_else_keyword_unreserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> YulElseKeyword_Unreserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::YulElseKeyword_Unreserved,
        phantom: PhantomData,
    }
}

pub type YulEmitKeyword_Reserved<'arena> = TerminalType<'arena>;
pub fn new_yul_emit_keyword_reserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> YulEmitKeyword_Reserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::YulEmitKeyword_Reserved,
        phantom: PhantomData,
    }
}

pub type YulEmitKeyword_Unreserved<'arena> = TerminalType<'arena>;
pub fn new_yul_emit_keyword_unreserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> YulEmitKeyword_Unreserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::YulEmitKeyword_Unreserved,
        phantom: PhantomData,
    }
}

pub type YulEnumKeyword_Reserved<'arena> = TerminalType<'arena>;
pub fn new_yul_enum_keyword_reserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> YulEnumKeyword_Reserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::YulEnumKeyword_Reserved,
        phantom: PhantomData,
    }
}

pub type YulEnumKeyword_Unreserved<'arena> = TerminalType<'arena>;
pub fn new_yul_enum_keyword_unreserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> YulEnumKeyword_Unreserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::YulEnumKeyword_Unreserved,
        phantom: PhantomData,
    }
}

pub type YulEtherKeyword_Reserved<'arena> = TerminalType<'arena>;
pub fn new_yul_ether_keyword_reserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> YulEtherKeyword_Reserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::YulEtherKeyword_Reserved,
        phantom: PhantomData,
    }
}

pub type YulEtherKeyword_Unreserved<'arena> = TerminalType<'arena>;
pub fn new_yul_ether_keyword_unreserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> YulEtherKeyword_Unreserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::YulEtherKeyword_Unreserved,
        phantom: PhantomData,
    }
}

pub type YulEventKeyword_Reserved<'arena> = TerminalType<'arena>;
pub fn new_yul_event_keyword_reserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> YulEventKeyword_Reserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::YulEventKeyword_Reserved,
        phantom: PhantomData,
    }
}

pub type YulEventKeyword_Unreserved<'arena> = TerminalType<'arena>;
pub fn new_yul_event_keyword_unreserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> YulEventKeyword_Unreserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::YulEventKeyword_Unreserved,
        phantom: PhantomData,
    }
}

pub type YulExternalKeyword_Reserved<'arena> = TerminalType<'arena>;
pub fn new_yul_external_keyword_reserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> YulExternalKeyword_Reserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::YulExternalKeyword_Reserved,
        phantom: PhantomData,
    }
}

pub type YulExternalKeyword_Unreserved<'arena> = TerminalType<'arena>;
pub fn new_yul_external_keyword_unreserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> YulExternalKeyword_Unreserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::YulExternalKeyword_Unreserved,
        phantom: PhantomData,
    }
}

pub type YulFallbackKeyword_Reserved<'arena> = TerminalType<'arena>;
pub fn new_yul_fallback_keyword_reserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> YulFallbackKeyword_Reserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::YulFallbackKeyword_Reserved,
        phantom: PhantomData,
    }
}

pub type YulFallbackKeyword_Unreserved<'arena> = TerminalType<'arena>;
pub fn new_yul_fallback_keyword_unreserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> YulFallbackKeyword_Unreserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::YulFallbackKeyword_Unreserved,
        phantom: PhantomData,
    }
}

pub type YulFalseKeyword_Reserved<'arena> = TerminalType<'arena>;
pub fn new_yul_false_keyword_reserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> YulFalseKeyword_Reserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::YulFalseKeyword_Reserved,
        phantom: PhantomData,
    }
}

pub type YulFinalKeyword_Reserved<'arena> = TerminalType<'arena>;
pub fn new_yul_final_keyword_reserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> YulFinalKeyword_Reserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::YulFinalKeyword_Reserved,
        phantom: PhantomData,
    }
}

pub type YulFinalKeyword_Unreserved<'arena> = TerminalType<'arena>;
pub fn new_yul_final_keyword_unreserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> YulFinalKeyword_Unreserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::YulFinalKeyword_Unreserved,
        phantom: PhantomData,
    }
}

pub type YulFinneyKeyword_Reserved<'arena> = TerminalType<'arena>;
pub fn new_yul_finney_keyword_reserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> YulFinneyKeyword_Reserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::YulFinneyKeyword_Reserved,
        phantom: PhantomData,
    }
}

pub type YulFinneyKeyword_Unreserved<'arena> = TerminalType<'arena>;
pub fn new_yul_finney_keyword_unreserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> YulFinneyKeyword_Unreserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::YulFinneyKeyword_Unreserved,
        phantom: PhantomData,
    }
}

pub type YulFixedKeyword_Reserved<'arena> = TerminalType<'arena>;
pub fn new_yul_fixed_keyword_reserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> YulFixedKeyword_Reserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::YulFixedKeyword_Reserved,
        phantom: PhantomData,
    }
}

pub type YulFixedKeyword_Unreserved<'arena> = TerminalType<'arena>;
pub fn new_yul_fixed_keyword_unreserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> YulFixedKeyword_Unreserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::YulFixedKeyword_Unreserved,
        phantom: PhantomData,
    }
}

pub type YulForKeyword_Reserved<'arena> = TerminalType<'arena>;
pub fn new_yul_for_keyword_reserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> YulForKeyword_Reserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::YulForKeyword_Reserved,
        phantom: PhantomData,
    }
}

pub type YulFunctionKeyword_Reserved<'arena> = TerminalType<'arena>;
pub fn new_yul_function_keyword_reserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> YulFunctionKeyword_Reserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::YulFunctionKeyword_Reserved,
        phantom: PhantomData,
    }
}

pub type YulGweiKeyword_Reserved<'arena> = TerminalType<'arena>;
pub fn new_yul_gwei_keyword_reserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> YulGweiKeyword_Reserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::YulGweiKeyword_Reserved,
        phantom: PhantomData,
    }
}

pub type YulGweiKeyword_Unreserved<'arena> = TerminalType<'arena>;
pub fn new_yul_gwei_keyword_unreserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> YulGweiKeyword_Unreserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::YulGweiKeyword_Unreserved,
        phantom: PhantomData,
    }
}

pub type YulHexKeyword_Reserved<'arena> = TerminalType<'arena>;
pub fn new_yul_hex_keyword_reserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> YulHexKeyword_Reserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::YulHexKeyword_Reserved,
        phantom: PhantomData,
    }
}

pub type YulHexLiteral<'arena> = TerminalType<'arena>;
pub fn new_yul_hex_literal<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> YulHexLiteral<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::YulHexLiteral,
        phantom: PhantomData,
    }
}

pub type YulHoursKeyword_Reserved<'arena> = TerminalType<'arena>;
pub fn new_yul_hours_keyword_reserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> YulHoursKeyword_Reserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::YulHoursKeyword_Reserved,
        phantom: PhantomData,
    }
}

pub type YulHoursKeyword_Unreserved<'arena> = TerminalType<'arena>;
pub fn new_yul_hours_keyword_unreserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> YulHoursKeyword_Unreserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::YulHoursKeyword_Unreserved,
        phantom: PhantomData,
    }
}

pub type YulIdentifier<'arena> = TerminalType<'arena>;
pub fn new_yul_identifier<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> YulIdentifier<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::YulIdentifier,
        phantom: PhantomData,
    }
}

pub type YulIfKeyword_Reserved<'arena> = TerminalType<'arena>;
pub fn new_yul_if_keyword_reserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> YulIfKeyword_Reserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::YulIfKeyword_Reserved,
        phantom: PhantomData,
    }
}

pub type YulImmutableKeyword_Reserved<'arena> = TerminalType<'arena>;
pub fn new_yul_immutable_keyword_reserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> YulImmutableKeyword_Reserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::YulImmutableKeyword_Reserved,
        phantom: PhantomData,
    }
}

pub type YulImmutableKeyword_Unreserved<'arena> = TerminalType<'arena>;
pub fn new_yul_immutable_keyword_unreserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> YulImmutableKeyword_Unreserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::YulImmutableKeyword_Unreserved,
        phantom: PhantomData,
    }
}

pub type YulImplementsKeyword_Reserved<'arena> = TerminalType<'arena>;
pub fn new_yul_implements_keyword_reserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> YulImplementsKeyword_Reserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::YulImplementsKeyword_Reserved,
        phantom: PhantomData,
    }
}

pub type YulImplementsKeyword_Unreserved<'arena> = TerminalType<'arena>;
pub fn new_yul_implements_keyword_unreserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> YulImplementsKeyword_Unreserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::YulImplementsKeyword_Unreserved,
        phantom: PhantomData,
    }
}

pub type YulImportKeyword_Reserved<'arena> = TerminalType<'arena>;
pub fn new_yul_import_keyword_reserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> YulImportKeyword_Reserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::YulImportKeyword_Reserved,
        phantom: PhantomData,
    }
}

pub type YulImportKeyword_Unreserved<'arena> = TerminalType<'arena>;
pub fn new_yul_import_keyword_unreserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> YulImportKeyword_Unreserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::YulImportKeyword_Unreserved,
        phantom: PhantomData,
    }
}

pub type YulInKeyword_Reserved<'arena> = TerminalType<'arena>;
pub fn new_yul_in_keyword_reserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> YulInKeyword_Reserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::YulInKeyword_Reserved,
        phantom: PhantomData,
    }
}

pub type YulInKeyword_Unreserved<'arena> = TerminalType<'arena>;
pub fn new_yul_in_keyword_unreserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> YulInKeyword_Unreserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::YulInKeyword_Unreserved,
        phantom: PhantomData,
    }
}

pub type YulIndexedKeyword_Reserved<'arena> = TerminalType<'arena>;
pub fn new_yul_indexed_keyword_reserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> YulIndexedKeyword_Reserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::YulIndexedKeyword_Reserved,
        phantom: PhantomData,
    }
}

pub type YulIndexedKeyword_Unreserved<'arena> = TerminalType<'arena>;
pub fn new_yul_indexed_keyword_unreserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> YulIndexedKeyword_Unreserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::YulIndexedKeyword_Unreserved,
        phantom: PhantomData,
    }
}

pub type YulInlineKeyword_Reserved<'arena> = TerminalType<'arena>;
pub fn new_yul_inline_keyword_reserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> YulInlineKeyword_Reserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::YulInlineKeyword_Reserved,
        phantom: PhantomData,
    }
}

pub type YulInlineKeyword_Unreserved<'arena> = TerminalType<'arena>;
pub fn new_yul_inline_keyword_unreserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> YulInlineKeyword_Unreserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::YulInlineKeyword_Unreserved,
        phantom: PhantomData,
    }
}

pub type YulIntKeyword_Reserved<'arena> = TerminalType<'arena>;
pub fn new_yul_int_keyword_reserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> YulIntKeyword_Reserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::YulIntKeyword_Reserved,
        phantom: PhantomData,
    }
}

pub type YulIntKeyword_Unreserved<'arena> = TerminalType<'arena>;
pub fn new_yul_int_keyword_unreserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> YulIntKeyword_Unreserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::YulIntKeyword_Unreserved,
        phantom: PhantomData,
    }
}

pub type YulInterfaceKeyword_Reserved<'arena> = TerminalType<'arena>;
pub fn new_yul_interface_keyword_reserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> YulInterfaceKeyword_Reserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::YulInterfaceKeyword_Reserved,
        phantom: PhantomData,
    }
}

pub type YulInterfaceKeyword_Unreserved<'arena> = TerminalType<'arena>;
pub fn new_yul_interface_keyword_unreserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> YulInterfaceKeyword_Unreserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::YulInterfaceKeyword_Unreserved,
        phantom: PhantomData,
    }
}

pub type YulInternalKeyword_Reserved<'arena> = TerminalType<'arena>;
pub fn new_yul_internal_keyword_reserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> YulInternalKeyword_Reserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::YulInternalKeyword_Reserved,
        phantom: PhantomData,
    }
}

pub type YulInternalKeyword_Unreserved<'arena> = TerminalType<'arena>;
pub fn new_yul_internal_keyword_unreserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> YulInternalKeyword_Unreserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::YulInternalKeyword_Unreserved,
        phantom: PhantomData,
    }
}

pub type YulIsKeyword_Reserved<'arena> = TerminalType<'arena>;
pub fn new_yul_is_keyword_reserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> YulIsKeyword_Reserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::YulIsKeyword_Reserved,
        phantom: PhantomData,
    }
}

pub type YulIsKeyword_Unreserved<'arena> = TerminalType<'arena>;
pub fn new_yul_is_keyword_unreserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> YulIsKeyword_Unreserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::YulIsKeyword_Unreserved,
        phantom: PhantomData,
    }
}

pub type YulLeaveKeyword_Reserved<'arena> = TerminalType<'arena>;
pub fn new_yul_leave_keyword_reserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> YulLeaveKeyword_Reserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::YulLeaveKeyword_Reserved,
        phantom: PhantomData,
    }
}

pub type YulLeaveKeyword_Unreserved<'arena> = TerminalType<'arena>;
pub fn new_yul_leave_keyword_unreserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> YulLeaveKeyword_Unreserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::YulLeaveKeyword_Unreserved,
        phantom: PhantomData,
    }
}

pub type YulLetKeyword_Reserved<'arena> = TerminalType<'arena>;
pub fn new_yul_let_keyword_reserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> YulLetKeyword_Reserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::YulLetKeyword_Reserved,
        phantom: PhantomData,
    }
}

pub type YulLibraryKeyword_Reserved<'arena> = TerminalType<'arena>;
pub fn new_yul_library_keyword_reserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> YulLibraryKeyword_Reserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::YulLibraryKeyword_Reserved,
        phantom: PhantomData,
    }
}

pub type YulLibraryKeyword_Unreserved<'arena> = TerminalType<'arena>;
pub fn new_yul_library_keyword_unreserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> YulLibraryKeyword_Unreserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::YulLibraryKeyword_Unreserved,
        phantom: PhantomData,
    }
}

pub type YulMacroKeyword_Reserved<'arena> = TerminalType<'arena>;
pub fn new_yul_macro_keyword_reserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> YulMacroKeyword_Reserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::YulMacroKeyword_Reserved,
        phantom: PhantomData,
    }
}

pub type YulMacroKeyword_Unreserved<'arena> = TerminalType<'arena>;
pub fn new_yul_macro_keyword_unreserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> YulMacroKeyword_Unreserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::YulMacroKeyword_Unreserved,
        phantom: PhantomData,
    }
}

pub type YulMappingKeyword_Reserved<'arena> = TerminalType<'arena>;
pub fn new_yul_mapping_keyword_reserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> YulMappingKeyword_Reserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::YulMappingKeyword_Reserved,
        phantom: PhantomData,
    }
}

pub type YulMappingKeyword_Unreserved<'arena> = TerminalType<'arena>;
pub fn new_yul_mapping_keyword_unreserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> YulMappingKeyword_Unreserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::YulMappingKeyword_Unreserved,
        phantom: PhantomData,
    }
}

pub type YulMatchKeyword_Reserved<'arena> = TerminalType<'arena>;
pub fn new_yul_match_keyword_reserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> YulMatchKeyword_Reserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::YulMatchKeyword_Reserved,
        phantom: PhantomData,
    }
}

pub type YulMatchKeyword_Unreserved<'arena> = TerminalType<'arena>;
pub fn new_yul_match_keyword_unreserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> YulMatchKeyword_Unreserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::YulMatchKeyword_Unreserved,
        phantom: PhantomData,
    }
}

pub type YulMemoryKeyword_Reserved<'arena> = TerminalType<'arena>;
pub fn new_yul_memory_keyword_reserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> YulMemoryKeyword_Reserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::YulMemoryKeyword_Reserved,
        phantom: PhantomData,
    }
}

pub type YulMemoryKeyword_Unreserved<'arena> = TerminalType<'arena>;
pub fn new_yul_memory_keyword_unreserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> YulMemoryKeyword_Unreserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::YulMemoryKeyword_Unreserved,
        phantom: PhantomData,
    }
}

pub type YulMinutesKeyword_Reserved<'arena> = TerminalType<'arena>;
pub fn new_yul_minutes_keyword_reserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> YulMinutesKeyword_Reserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::YulMinutesKeyword_Reserved,
        phantom: PhantomData,
    }
}

pub type YulMinutesKeyword_Unreserved<'arena> = TerminalType<'arena>;
pub fn new_yul_minutes_keyword_unreserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> YulMinutesKeyword_Unreserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::YulMinutesKeyword_Unreserved,
        phantom: PhantomData,
    }
}

pub type YulModifierKeyword_Reserved<'arena> = TerminalType<'arena>;
pub fn new_yul_modifier_keyword_reserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> YulModifierKeyword_Reserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::YulModifierKeyword_Reserved,
        phantom: PhantomData,
    }
}

pub type YulModifierKeyword_Unreserved<'arena> = TerminalType<'arena>;
pub fn new_yul_modifier_keyword_unreserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> YulModifierKeyword_Unreserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::YulModifierKeyword_Unreserved,
        phantom: PhantomData,
    }
}

pub type YulMutableKeyword_Reserved<'arena> = TerminalType<'arena>;
pub fn new_yul_mutable_keyword_reserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> YulMutableKeyword_Reserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::YulMutableKeyword_Reserved,
        phantom: PhantomData,
    }
}

pub type YulMutableKeyword_Unreserved<'arena> = TerminalType<'arena>;
pub fn new_yul_mutable_keyword_unreserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> YulMutableKeyword_Unreserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::YulMutableKeyword_Unreserved,
        phantom: PhantomData,
    }
}

pub type YulNewKeyword_Reserved<'arena> = TerminalType<'arena>;
pub fn new_yul_new_keyword_reserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> YulNewKeyword_Reserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::YulNewKeyword_Reserved,
        phantom: PhantomData,
    }
}

pub type YulNewKeyword_Unreserved<'arena> = TerminalType<'arena>;
pub fn new_yul_new_keyword_unreserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> YulNewKeyword_Unreserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::YulNewKeyword_Unreserved,
        phantom: PhantomData,
    }
}

pub type YulNullKeyword_Reserved<'arena> = TerminalType<'arena>;
pub fn new_yul_null_keyword_reserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> YulNullKeyword_Reserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::YulNullKeyword_Reserved,
        phantom: PhantomData,
    }
}

pub type YulNullKeyword_Unreserved<'arena> = TerminalType<'arena>;
pub fn new_yul_null_keyword_unreserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> YulNullKeyword_Unreserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::YulNullKeyword_Unreserved,
        phantom: PhantomData,
    }
}

pub type YulOfKeyword_Reserved<'arena> = TerminalType<'arena>;
pub fn new_yul_of_keyword_reserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> YulOfKeyword_Reserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::YulOfKeyword_Reserved,
        phantom: PhantomData,
    }
}

pub type YulOfKeyword_Unreserved<'arena> = TerminalType<'arena>;
pub fn new_yul_of_keyword_unreserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> YulOfKeyword_Unreserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::YulOfKeyword_Unreserved,
        phantom: PhantomData,
    }
}

pub type YulOverrideKeyword_Reserved<'arena> = TerminalType<'arena>;
pub fn new_yul_override_keyword_reserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> YulOverrideKeyword_Reserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::YulOverrideKeyword_Reserved,
        phantom: PhantomData,
    }
}

pub type YulOverrideKeyword_Unreserved<'arena> = TerminalType<'arena>;
pub fn new_yul_override_keyword_unreserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> YulOverrideKeyword_Unreserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::YulOverrideKeyword_Unreserved,
        phantom: PhantomData,
    }
}

pub type YulPartialKeyword_Reserved<'arena> = TerminalType<'arena>;
pub fn new_yul_partial_keyword_reserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> YulPartialKeyword_Reserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::YulPartialKeyword_Reserved,
        phantom: PhantomData,
    }
}

pub type YulPartialKeyword_Unreserved<'arena> = TerminalType<'arena>;
pub fn new_yul_partial_keyword_unreserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> YulPartialKeyword_Unreserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::YulPartialKeyword_Unreserved,
        phantom: PhantomData,
    }
}

pub type YulPayableKeyword_Reserved<'arena> = TerminalType<'arena>;
pub fn new_yul_payable_keyword_reserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> YulPayableKeyword_Reserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::YulPayableKeyword_Reserved,
        phantom: PhantomData,
    }
}

pub type YulPayableKeyword_Unreserved<'arena> = TerminalType<'arena>;
pub fn new_yul_payable_keyword_unreserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> YulPayableKeyword_Unreserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::YulPayableKeyword_Unreserved,
        phantom: PhantomData,
    }
}

pub type YulPragmaKeyword_Reserved<'arena> = TerminalType<'arena>;
pub fn new_yul_pragma_keyword_reserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> YulPragmaKeyword_Reserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::YulPragmaKeyword_Reserved,
        phantom: PhantomData,
    }
}

pub type YulPragmaKeyword_Unreserved<'arena> = TerminalType<'arena>;
pub fn new_yul_pragma_keyword_unreserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> YulPragmaKeyword_Unreserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::YulPragmaKeyword_Unreserved,
        phantom: PhantomData,
    }
}

pub type YulPrivateKeyword_Reserved<'arena> = TerminalType<'arena>;
pub fn new_yul_private_keyword_reserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> YulPrivateKeyword_Reserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::YulPrivateKeyword_Reserved,
        phantom: PhantomData,
    }
}

pub type YulPrivateKeyword_Unreserved<'arena> = TerminalType<'arena>;
pub fn new_yul_private_keyword_unreserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> YulPrivateKeyword_Unreserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::YulPrivateKeyword_Unreserved,
        phantom: PhantomData,
    }
}

pub type YulPromiseKeyword_Reserved<'arena> = TerminalType<'arena>;
pub fn new_yul_promise_keyword_reserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> YulPromiseKeyword_Reserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::YulPromiseKeyword_Reserved,
        phantom: PhantomData,
    }
}

pub type YulPromiseKeyword_Unreserved<'arena> = TerminalType<'arena>;
pub fn new_yul_promise_keyword_unreserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> YulPromiseKeyword_Unreserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::YulPromiseKeyword_Unreserved,
        phantom: PhantomData,
    }
}

pub type YulPublicKeyword_Reserved<'arena> = TerminalType<'arena>;
pub fn new_yul_public_keyword_reserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> YulPublicKeyword_Reserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::YulPublicKeyword_Reserved,
        phantom: PhantomData,
    }
}

pub type YulPublicKeyword_Unreserved<'arena> = TerminalType<'arena>;
pub fn new_yul_public_keyword_unreserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> YulPublicKeyword_Unreserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::YulPublicKeyword_Unreserved,
        phantom: PhantomData,
    }
}

pub type YulPureKeyword_Reserved<'arena> = TerminalType<'arena>;
pub fn new_yul_pure_keyword_reserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> YulPureKeyword_Reserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::YulPureKeyword_Reserved,
        phantom: PhantomData,
    }
}

pub type YulPureKeyword_Unreserved<'arena> = TerminalType<'arena>;
pub fn new_yul_pure_keyword_unreserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> YulPureKeyword_Unreserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::YulPureKeyword_Unreserved,
        phantom: PhantomData,
    }
}

pub type YulReceiveKeyword_Reserved<'arena> = TerminalType<'arena>;
pub fn new_yul_receive_keyword_reserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> YulReceiveKeyword_Reserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::YulReceiveKeyword_Reserved,
        phantom: PhantomData,
    }
}

pub type YulReceiveKeyword_Unreserved<'arena> = TerminalType<'arena>;
pub fn new_yul_receive_keyword_unreserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> YulReceiveKeyword_Unreserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::YulReceiveKeyword_Unreserved,
        phantom: PhantomData,
    }
}

pub type YulReferenceKeyword_Reserved<'arena> = TerminalType<'arena>;
pub fn new_yul_reference_keyword_reserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> YulReferenceKeyword_Reserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::YulReferenceKeyword_Reserved,
        phantom: PhantomData,
    }
}

pub type YulReferenceKeyword_Unreserved<'arena> = TerminalType<'arena>;
pub fn new_yul_reference_keyword_unreserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> YulReferenceKeyword_Unreserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::YulReferenceKeyword_Unreserved,
        phantom: PhantomData,
    }
}

pub type YulRelocatableKeyword_Reserved<'arena> = TerminalType<'arena>;
pub fn new_yul_relocatable_keyword_reserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> YulRelocatableKeyword_Reserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::YulRelocatableKeyword_Reserved,
        phantom: PhantomData,
    }
}

pub type YulRelocatableKeyword_Unreserved<'arena> = TerminalType<'arena>;
pub fn new_yul_relocatable_keyword_unreserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> YulRelocatableKeyword_Unreserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::YulRelocatableKeyword_Unreserved,
        phantom: PhantomData,
    }
}

pub type YulReturnsKeyword_Reserved<'arena> = TerminalType<'arena>;
pub fn new_yul_returns_keyword_reserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> YulReturnsKeyword_Reserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::YulReturnsKeyword_Reserved,
        phantom: PhantomData,
    }
}

pub type YulReturnsKeyword_Unreserved<'arena> = TerminalType<'arena>;
pub fn new_yul_returns_keyword_unreserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> YulReturnsKeyword_Unreserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::YulReturnsKeyword_Unreserved,
        phantom: PhantomData,
    }
}

pub type YulSealedKeyword_Reserved<'arena> = TerminalType<'arena>;
pub fn new_yul_sealed_keyword_reserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> YulSealedKeyword_Reserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::YulSealedKeyword_Reserved,
        phantom: PhantomData,
    }
}

pub type YulSealedKeyword_Unreserved<'arena> = TerminalType<'arena>;
pub fn new_yul_sealed_keyword_unreserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> YulSealedKeyword_Unreserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::YulSealedKeyword_Unreserved,
        phantom: PhantomData,
    }
}

pub type YulSecondsKeyword_Reserved<'arena> = TerminalType<'arena>;
pub fn new_yul_seconds_keyword_reserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> YulSecondsKeyword_Reserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::YulSecondsKeyword_Reserved,
        phantom: PhantomData,
    }
}

pub type YulSecondsKeyword_Unreserved<'arena> = TerminalType<'arena>;
pub fn new_yul_seconds_keyword_unreserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> YulSecondsKeyword_Unreserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::YulSecondsKeyword_Unreserved,
        phantom: PhantomData,
    }
}

pub type YulSizeOfKeyword_Reserved<'arena> = TerminalType<'arena>;
pub fn new_yul_size_of_keyword_reserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> YulSizeOfKeyword_Reserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::YulSizeOfKeyword_Reserved,
        phantom: PhantomData,
    }
}

pub type YulSizeOfKeyword_Unreserved<'arena> = TerminalType<'arena>;
pub fn new_yul_size_of_keyword_unreserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> YulSizeOfKeyword_Unreserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::YulSizeOfKeyword_Unreserved,
        phantom: PhantomData,
    }
}

pub type YulStaticKeyword_Reserved<'arena> = TerminalType<'arena>;
pub fn new_yul_static_keyword_reserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> YulStaticKeyword_Reserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::YulStaticKeyword_Reserved,
        phantom: PhantomData,
    }
}

pub type YulStaticKeyword_Unreserved<'arena> = TerminalType<'arena>;
pub fn new_yul_static_keyword_unreserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> YulStaticKeyword_Unreserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::YulStaticKeyword_Unreserved,
        phantom: PhantomData,
    }
}

pub type YulStorageKeyword_Reserved<'arena> = TerminalType<'arena>;
pub fn new_yul_storage_keyword_reserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> YulStorageKeyword_Reserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::YulStorageKeyword_Reserved,
        phantom: PhantomData,
    }
}

pub type YulStorageKeyword_Unreserved<'arena> = TerminalType<'arena>;
pub fn new_yul_storage_keyword_unreserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> YulStorageKeyword_Unreserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::YulStorageKeyword_Unreserved,
        phantom: PhantomData,
    }
}

pub type YulStringKeyword_Reserved<'arena> = TerminalType<'arena>;
pub fn new_yul_string_keyword_reserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> YulStringKeyword_Reserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::YulStringKeyword_Reserved,
        phantom: PhantomData,
    }
}

pub type YulStringKeyword_Unreserved<'arena> = TerminalType<'arena>;
pub fn new_yul_string_keyword_unreserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> YulStringKeyword_Unreserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::YulStringKeyword_Unreserved,
        phantom: PhantomData,
    }
}

pub type YulStructKeyword_Reserved<'arena> = TerminalType<'arena>;
pub fn new_yul_struct_keyword_reserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> YulStructKeyword_Reserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::YulStructKeyword_Reserved,
        phantom: PhantomData,
    }
}

pub type YulStructKeyword_Unreserved<'arena> = TerminalType<'arena>;
pub fn new_yul_struct_keyword_unreserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> YulStructKeyword_Unreserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::YulStructKeyword_Unreserved,
        phantom: PhantomData,
    }
}

pub type YulSuperKeyword_Reserved<'arena> = TerminalType<'arena>;
pub fn new_yul_super_keyword_reserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> YulSuperKeyword_Reserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::YulSuperKeyword_Reserved,
        phantom: PhantomData,
    }
}

pub type YulSuperKeyword_Unreserved<'arena> = TerminalType<'arena>;
pub fn new_yul_super_keyword_unreserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> YulSuperKeyword_Unreserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::YulSuperKeyword_Unreserved,
        phantom: PhantomData,
    }
}

pub type YulSupportsKeyword_Reserved<'arena> = TerminalType<'arena>;
pub fn new_yul_supports_keyword_reserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> YulSupportsKeyword_Reserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::YulSupportsKeyword_Reserved,
        phantom: PhantomData,
    }
}

pub type YulSupportsKeyword_Unreserved<'arena> = TerminalType<'arena>;
pub fn new_yul_supports_keyword_unreserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> YulSupportsKeyword_Unreserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::YulSupportsKeyword_Unreserved,
        phantom: PhantomData,
    }
}

pub type YulSwitchKeyword_Reserved<'arena> = TerminalType<'arena>;
pub fn new_yul_switch_keyword_reserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> YulSwitchKeyword_Reserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::YulSwitchKeyword_Reserved,
        phantom: PhantomData,
    }
}

pub type YulSzaboKeyword_Reserved<'arena> = TerminalType<'arena>;
pub fn new_yul_szabo_keyword_reserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> YulSzaboKeyword_Reserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::YulSzaboKeyword_Reserved,
        phantom: PhantomData,
    }
}

pub type YulSzaboKeyword_Unreserved<'arena> = TerminalType<'arena>;
pub fn new_yul_szabo_keyword_unreserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> YulSzaboKeyword_Unreserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::YulSzaboKeyword_Unreserved,
        phantom: PhantomData,
    }
}

pub type YulThisKeyword_Reserved<'arena> = TerminalType<'arena>;
pub fn new_yul_this_keyword_reserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> YulThisKeyword_Reserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::YulThisKeyword_Reserved,
        phantom: PhantomData,
    }
}

pub type YulThisKeyword_Unreserved<'arena> = TerminalType<'arena>;
pub fn new_yul_this_keyword_unreserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> YulThisKeyword_Unreserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::YulThisKeyword_Unreserved,
        phantom: PhantomData,
    }
}

pub type YulThrowKeyword_Reserved<'arena> = TerminalType<'arena>;
pub fn new_yul_throw_keyword_reserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> YulThrowKeyword_Reserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::YulThrowKeyword_Reserved,
        phantom: PhantomData,
    }
}

pub type YulThrowKeyword_Unreserved<'arena> = TerminalType<'arena>;
pub fn new_yul_throw_keyword_unreserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> YulThrowKeyword_Unreserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::YulThrowKeyword_Unreserved,
        phantom: PhantomData,
    }
}

pub type YulTrueKeyword_Reserved<'arena> = TerminalType<'arena>;
pub fn new_yul_true_keyword_reserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> YulTrueKeyword_Reserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::YulTrueKeyword_Reserved,
        phantom: PhantomData,
    }
}

pub type YulTryKeyword_Reserved<'arena> = TerminalType<'arena>;
pub fn new_yul_try_keyword_reserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> YulTryKeyword_Reserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::YulTryKeyword_Reserved,
        phantom: PhantomData,
    }
}

pub type YulTryKeyword_Unreserved<'arena> = TerminalType<'arena>;
pub fn new_yul_try_keyword_unreserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> YulTryKeyword_Unreserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::YulTryKeyword_Unreserved,
        phantom: PhantomData,
    }
}

pub type YulTypeDefKeyword_Reserved<'arena> = TerminalType<'arena>;
pub fn new_yul_type_def_keyword_reserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> YulTypeDefKeyword_Reserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::YulTypeDefKeyword_Reserved,
        phantom: PhantomData,
    }
}

pub type YulTypeDefKeyword_Unreserved<'arena> = TerminalType<'arena>;
pub fn new_yul_type_def_keyword_unreserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> YulTypeDefKeyword_Unreserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::YulTypeDefKeyword_Unreserved,
        phantom: PhantomData,
    }
}

pub type YulTypeKeyword_Reserved<'arena> = TerminalType<'arena>;
pub fn new_yul_type_keyword_reserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> YulTypeKeyword_Reserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::YulTypeKeyword_Reserved,
        phantom: PhantomData,
    }
}

pub type YulTypeKeyword_Unreserved<'arena> = TerminalType<'arena>;
pub fn new_yul_type_keyword_unreserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> YulTypeKeyword_Unreserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::YulTypeKeyword_Unreserved,
        phantom: PhantomData,
    }
}

pub type YulTypeOfKeyword_Reserved<'arena> = TerminalType<'arena>;
pub fn new_yul_type_of_keyword_reserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> YulTypeOfKeyword_Reserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::YulTypeOfKeyword_Reserved,
        phantom: PhantomData,
    }
}

pub type YulTypeOfKeyword_Unreserved<'arena> = TerminalType<'arena>;
pub fn new_yul_type_of_keyword_unreserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> YulTypeOfKeyword_Unreserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::YulTypeOfKeyword_Unreserved,
        phantom: PhantomData,
    }
}

pub type YulUfixedKeyword_Reserved<'arena> = TerminalType<'arena>;
pub fn new_yul_ufixed_keyword_reserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> YulUfixedKeyword_Reserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::YulUfixedKeyword_Reserved,
        phantom: PhantomData,
    }
}

pub type YulUfixedKeyword_Unreserved<'arena> = TerminalType<'arena>;
pub fn new_yul_ufixed_keyword_unreserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> YulUfixedKeyword_Unreserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::YulUfixedKeyword_Unreserved,
        phantom: PhantomData,
    }
}

pub type YulUintKeyword_Reserved<'arena> = TerminalType<'arena>;
pub fn new_yul_uint_keyword_reserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> YulUintKeyword_Reserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::YulUintKeyword_Reserved,
        phantom: PhantomData,
    }
}

pub type YulUintKeyword_Unreserved<'arena> = TerminalType<'arena>;
pub fn new_yul_uint_keyword_unreserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> YulUintKeyword_Unreserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::YulUintKeyword_Unreserved,
        phantom: PhantomData,
    }
}

pub type YulUncheckedKeyword_Reserved<'arena> = TerminalType<'arena>;
pub fn new_yul_unchecked_keyword_reserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> YulUncheckedKeyword_Reserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::YulUncheckedKeyword_Reserved,
        phantom: PhantomData,
    }
}

pub type YulUncheckedKeyword_Unreserved<'arena> = TerminalType<'arena>;
pub fn new_yul_unchecked_keyword_unreserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> YulUncheckedKeyword_Unreserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::YulUncheckedKeyword_Unreserved,
        phantom: PhantomData,
    }
}

pub type YulUsingKeyword_Reserved<'arena> = TerminalType<'arena>;
pub fn new_yul_using_keyword_reserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> YulUsingKeyword_Reserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::YulUsingKeyword_Reserved,
        phantom: PhantomData,
    }
}

pub type YulUsingKeyword_Unreserved<'arena> = TerminalType<'arena>;
pub fn new_yul_using_keyword_unreserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> YulUsingKeyword_Unreserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::YulUsingKeyword_Unreserved,
        phantom: PhantomData,
    }
}

pub type YulVarKeyword_Reserved<'arena> = TerminalType<'arena>;
pub fn new_yul_var_keyword_reserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> YulVarKeyword_Reserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::YulVarKeyword_Reserved,
        phantom: PhantomData,
    }
}

pub type YulVarKeyword_Unreserved<'arena> = TerminalType<'arena>;
pub fn new_yul_var_keyword_unreserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> YulVarKeyword_Unreserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::YulVarKeyword_Unreserved,
        phantom: PhantomData,
    }
}

pub type YulViewKeyword_Reserved<'arena> = TerminalType<'arena>;
pub fn new_yul_view_keyword_reserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> YulViewKeyword_Reserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::YulViewKeyword_Reserved,
        phantom: PhantomData,
    }
}

pub type YulViewKeyword_Unreserved<'arena> = TerminalType<'arena>;
pub fn new_yul_view_keyword_unreserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> YulViewKeyword_Unreserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::YulViewKeyword_Unreserved,
        phantom: PhantomData,
    }
}

pub type YulVirtualKeyword_Reserved<'arena> = TerminalType<'arena>;
pub fn new_yul_virtual_keyword_reserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> YulVirtualKeyword_Reserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::YulVirtualKeyword_Reserved,
        phantom: PhantomData,
    }
}

pub type YulVirtualKeyword_Unreserved<'arena> = TerminalType<'arena>;
pub fn new_yul_virtual_keyword_unreserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> YulVirtualKeyword_Unreserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::YulVirtualKeyword_Unreserved,
        phantom: PhantomData,
    }
}

pub type YulWeeksKeyword_Reserved<'arena> = TerminalType<'arena>;
pub fn new_yul_weeks_keyword_reserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> YulWeeksKeyword_Reserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::YulWeeksKeyword_Reserved,
        phantom: PhantomData,
    }
}

pub type YulWeeksKeyword_Unreserved<'arena> = TerminalType<'arena>;
pub fn new_yul_weeks_keyword_unreserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> YulWeeksKeyword_Unreserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::YulWeeksKeyword_Unreserved,
        phantom: PhantomData,
    }
}

pub type YulWeiKeyword_Reserved<'arena> = TerminalType<'arena>;
pub fn new_yul_wei_keyword_reserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> YulWeiKeyword_Reserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::YulWeiKeyword_Reserved,
        phantom: PhantomData,
    }
}

pub type YulWeiKeyword_Unreserved<'arena> = TerminalType<'arena>;
pub fn new_yul_wei_keyword_unreserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> YulWeiKeyword_Unreserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::YulWeiKeyword_Unreserved,
        phantom: PhantomData,
    }
}

pub type YulWhileKeyword_Reserved<'arena> = TerminalType<'arena>;
pub fn new_yul_while_keyword_reserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> YulWhileKeyword_Reserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::YulWhileKeyword_Reserved,
        phantom: PhantomData,
    }
}

pub type YulWhileKeyword_Unreserved<'arena> = TerminalType<'arena>;
pub fn new_yul_while_keyword_unreserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> YulWhileKeyword_Unreserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::YulWhileKeyword_Unreserved,
        phantom: PhantomData,
    }
}

pub type YulYearsKeyword_Reserved<'arena> = TerminalType<'arena>;
pub fn new_yul_years_keyword_reserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> YulYearsKeyword_Reserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::YulYearsKeyword_Reserved,
        phantom: PhantomData,
    }
}

pub type YulYearsKeyword_Unreserved<'arena> = TerminalType<'arena>;
pub fn new_yul_years_keyword_unreserved<'arena>(
    _arena: &'arena Bump,
    l: usize,
    r: usize,
    source: &str,
) -> YulYearsKeyword_Unreserved<'arena> {
    TerminalType {
        // TODO(v2): avoid the allocation here, it's not needed
        value: source[l..r].to_owned(),
        l,
        r,
        kind: LexemeKind::YulYearsKeyword_Unreserved,
        phantom: PhantomData,
    }
}

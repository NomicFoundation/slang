// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

#![allow(non_camel_case_types)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(clippy::too_many_arguments)]

use std::ops::Range;
use std::rc::Rc;

// TODO(v2):
// - (perf) don't use terminals that are not needed
// - (feat) visitor/traversal/serializer
// - (feat) span information, where applicable
// - (feat) Use `new` impl methods instead of free functions:
// ```
// impl YulUncheckedKeyword {
//   pub fn new(...) {
//     ...
//   }
// }
// ```

//
// Sequences:
//
// Note: All sequences are wrapped in Rc, this keeps sizes down and avoids recursive types

pub type AbicoderPragma = Rc<AbicoderPragmaStruct>;

#[derive(Clone, Debug)]
pub struct AbicoderPragmaStruct {
    pub abicoder_keyword: AbicoderKeyword,
    pub version: AbicoderVersion,
}

pub fn new_abicoder_pragma(
    abicoder_keyword: AbicoderKeyword,
    version: AbicoderVersion,
) -> AbicoderPragma {
    Rc::new(AbicoderPragmaStruct {
        abicoder_keyword,
        version,
    })
}

pub type AdditiveExpression = Rc<AdditiveExpressionStruct>;

#[derive(Clone, Debug)]
pub struct AdditiveExpressionStruct {
    pub left_operand: Expression,
    pub expression_additive_expression_operator: Expression_AdditiveExpression_Operator,
    pub right_operand: Expression,
}

pub fn new_additive_expression(
    left_operand: Expression,
    expression_additive_expression_operator: Expression_AdditiveExpression_Operator,
    right_operand: Expression,
) -> AdditiveExpression {
    Rc::new(AdditiveExpressionStruct {
        left_operand,
        expression_additive_expression_operator,
        right_operand,
    })
}

pub type AddressType = Rc<AddressTypeStruct>;

#[derive(Clone, Debug)]
pub struct AddressTypeStruct {
    pub address_keyword: AddressKeyword,
    pub payable_keyword: Option<PayableKeyword>,
}

pub fn new_address_type(
    address_keyword: AddressKeyword,
    payable_keyword: Option<PayableKeyword>,
) -> AddressType {
    Rc::new(AddressTypeStruct {
        address_keyword,
        payable_keyword,
    })
}

pub type AndExpression = Rc<AndExpressionStruct>;

#[derive(Clone, Debug)]
pub struct AndExpressionStruct {
    pub left_operand: Expression,
    pub operator: AmpersandAmpersand,
    pub right_operand: Expression,
}

pub fn new_and_expression(
    left_operand: Expression,
    operator: AmpersandAmpersand,
    right_operand: Expression,
) -> AndExpression {
    Rc::new(AndExpressionStruct {
        left_operand,
        operator,
        right_operand,
    })
}

pub type ArrayExpression = Rc<ArrayExpressionStruct>;

#[derive(Clone, Debug)]
pub struct ArrayExpressionStruct {
    pub open_bracket: OpenBracket,
    pub items: ArrayValues,
    pub close_bracket: CloseBracket,
}

pub fn new_array_expression(
    open_bracket: OpenBracket,
    items: ArrayValues,
    close_bracket: CloseBracket,
) -> ArrayExpression {
    Rc::new(ArrayExpressionStruct {
        open_bracket,
        items,
        close_bracket,
    })
}

pub type ArrayTypeName = Rc<ArrayTypeNameStruct>;

#[derive(Clone, Debug)]
pub struct ArrayTypeNameStruct {
    pub operand: TypeName,
    pub open_bracket: OpenBracket,
    pub index: Option<Expression>,
    pub close_bracket: CloseBracket,
}

pub fn new_array_type_name(
    operand: TypeName,
    open_bracket: OpenBracket,
    index: Option<Expression>,
    close_bracket: CloseBracket,
) -> ArrayTypeName {
    Rc::new(ArrayTypeNameStruct {
        operand,
        open_bracket,
        index,
        close_bracket,
    })
}

pub type AssemblyFlagsDeclaration = Rc<AssemblyFlagsDeclarationStruct>;

#[derive(Clone, Debug)]
pub struct AssemblyFlagsDeclarationStruct {
    pub open_paren: OpenParen,
    pub flags: AssemblyFlags,
    pub close_paren: CloseParen,
}

pub fn new_assembly_flags_declaration(
    open_paren: OpenParen,
    flags: AssemblyFlags,
    close_paren: CloseParen,
) -> AssemblyFlagsDeclaration {
    Rc::new(AssemblyFlagsDeclarationStruct {
        open_paren,
        flags,
        close_paren,
    })
}

pub type AssemblyStatement = Rc<AssemblyStatementStruct>;

#[derive(Clone, Debug)]
pub struct AssemblyStatementStruct {
    pub assembly_keyword: AssemblyKeyword,
    pub label: Option<StringLiteral>,
    pub flags: Option<AssemblyFlagsDeclaration>,
    pub body: YulBlock,
}

pub fn new_assembly_statement(
    assembly_keyword: AssemblyKeyword,
    label: Option<StringLiteral>,
    flags: Option<AssemblyFlagsDeclaration>,
    body: YulBlock,
) -> AssemblyStatement {
    Rc::new(AssemblyStatementStruct {
        assembly_keyword,
        label,
        flags,
        body,
    })
}

pub type AssignmentExpression = Rc<AssignmentExpressionStruct>;

#[derive(Clone, Debug)]
pub struct AssignmentExpressionStruct {
    pub left_operand: Expression,
    pub expression_assignment_expression_operator: Expression_AssignmentExpression_Operator,
    pub right_operand: Expression,
}

pub fn new_assignment_expression(
    left_operand: Expression,
    expression_assignment_expression_operator: Expression_AssignmentExpression_Operator,
    right_operand: Expression,
) -> AssignmentExpression {
    Rc::new(AssignmentExpressionStruct {
        left_operand,
        expression_assignment_expression_operator,
        right_operand,
    })
}

pub type BitwiseAndExpression = Rc<BitwiseAndExpressionStruct>;

#[derive(Clone, Debug)]
pub struct BitwiseAndExpressionStruct {
    pub left_operand: Expression,
    pub operator: Ampersand,
    pub right_operand: Expression,
}

pub fn new_bitwise_and_expression(
    left_operand: Expression,
    operator: Ampersand,
    right_operand: Expression,
) -> BitwiseAndExpression {
    Rc::new(BitwiseAndExpressionStruct {
        left_operand,
        operator,
        right_operand,
    })
}

pub type BitwiseOrExpression = Rc<BitwiseOrExpressionStruct>;

#[derive(Clone, Debug)]
pub struct BitwiseOrExpressionStruct {
    pub left_operand: Expression,
    pub operator: Bar,
    pub right_operand: Expression,
}

pub fn new_bitwise_or_expression(
    left_operand: Expression,
    operator: Bar,
    right_operand: Expression,
) -> BitwiseOrExpression {
    Rc::new(BitwiseOrExpressionStruct {
        left_operand,
        operator,
        right_operand,
    })
}

pub type BitwiseXorExpression = Rc<BitwiseXorExpressionStruct>;

#[derive(Clone, Debug)]
pub struct BitwiseXorExpressionStruct {
    pub left_operand: Expression,
    pub operator: Caret,
    pub right_operand: Expression,
}

pub fn new_bitwise_xor_expression(
    left_operand: Expression,
    operator: Caret,
    right_operand: Expression,
) -> BitwiseXorExpression {
    Rc::new(BitwiseXorExpressionStruct {
        left_operand,
        operator,
        right_operand,
    })
}

pub type Block = Rc<BlockStruct>;

#[derive(Clone, Debug)]
pub struct BlockStruct {
    pub open_brace: OpenBrace,
    pub statements: Statements,
    pub close_brace: CloseBrace,
}

pub fn new_block(open_brace: OpenBrace, statements: Statements, close_brace: CloseBrace) -> Block {
    Rc::new(BlockStruct {
        open_brace,
        statements,
        close_brace,
    })
}

pub type BreakStatement = Rc<BreakStatementStruct>;

#[derive(Clone, Debug)]
pub struct BreakStatementStruct {
    pub break_keyword: BreakKeyword,
    pub semicolon: Semicolon,
}

pub fn new_break_statement(break_keyword: BreakKeyword, semicolon: Semicolon) -> BreakStatement {
    Rc::new(BreakStatementStruct {
        break_keyword,
        semicolon,
    })
}

pub type CallOptionsExpression = Rc<CallOptionsExpressionStruct>;

#[derive(Clone, Debug)]
pub struct CallOptionsExpressionStruct {
    pub operand: Expression,
    pub open_brace: OpenBrace,
    pub options: CallOptions,
    pub close_brace: CloseBrace,
}

pub fn new_call_options_expression(
    operand: Expression,
    open_brace: OpenBrace,
    options: CallOptions,
    close_brace: CloseBrace,
) -> CallOptionsExpression {
    Rc::new(CallOptionsExpressionStruct {
        operand,
        open_brace,
        options,
        close_brace,
    })
}

pub type CatchClause = Rc<CatchClauseStruct>;

#[derive(Clone, Debug)]
pub struct CatchClauseStruct {
    pub catch_keyword: CatchKeyword,
    pub error: Option<CatchClauseError>,
    pub body: Block,
}

pub fn new_catch_clause(
    catch_keyword: CatchKeyword,
    error: Option<CatchClauseError>,
    body: Block,
) -> CatchClause {
    Rc::new(CatchClauseStruct {
        catch_keyword,
        error,
        body,
    })
}

pub type CatchClauseError = Rc<CatchClauseErrorStruct>;

#[derive(Clone, Debug)]
pub struct CatchClauseErrorStruct {
    pub name: Option<Identifier>,
    pub parameters: ParametersDeclaration,
}

pub fn new_catch_clause_error(
    name: Option<Identifier>,
    parameters: ParametersDeclaration,
) -> CatchClauseError {
    Rc::new(CatchClauseErrorStruct { name, parameters })
}

pub type ConditionalExpression = Rc<ConditionalExpressionStruct>;

#[derive(Clone, Debug)]
pub struct ConditionalExpressionStruct {
    pub operand: Expression,
    pub question_mark: QuestionMark,
    pub true_expression: Expression,
    pub colon: Colon,
    pub false_expression: Expression,
}

pub fn new_conditional_expression(
    operand: Expression,
    question_mark: QuestionMark,
    true_expression: Expression,
    colon: Colon,
    false_expression: Expression,
) -> ConditionalExpression {
    Rc::new(ConditionalExpressionStruct {
        operand,
        question_mark,
        true_expression,
        colon,
        false_expression,
    })
}

pub type ConstantDefinition = Rc<ConstantDefinitionStruct>;

#[derive(Clone, Debug)]
pub struct ConstantDefinitionStruct {
    pub type_name: TypeName,
    pub constant_keyword: ConstantKeyword,
    pub name: Identifier,
    pub equal: Equal,
    pub value: Expression,
    pub semicolon: Semicolon,
}

pub fn new_constant_definition(
    type_name: TypeName,
    constant_keyword: ConstantKeyword,
    name: Identifier,
    equal: Equal,
    value: Expression,
    semicolon: Semicolon,
) -> ConstantDefinition {
    Rc::new(ConstantDefinitionStruct {
        type_name,
        constant_keyword,
        name,
        equal,
        value,
        semicolon,
    })
}

pub type ConstructorDefinition = Rc<ConstructorDefinitionStruct>;

#[derive(Clone, Debug)]
pub struct ConstructorDefinitionStruct {
    pub constructor_keyword: ConstructorKeyword,
    pub parameters: ParametersDeclaration,
    pub attributes: ConstructorAttributes,
    pub body: Block,
}

pub fn new_constructor_definition(
    constructor_keyword: ConstructorKeyword,
    parameters: ParametersDeclaration,
    attributes: ConstructorAttributes,
    body: Block,
) -> ConstructorDefinition {
    Rc::new(ConstructorDefinitionStruct {
        constructor_keyword,
        parameters,
        attributes,
        body,
    })
}

pub type ContinueStatement = Rc<ContinueStatementStruct>;

#[derive(Clone, Debug)]
pub struct ContinueStatementStruct {
    pub continue_keyword: ContinueKeyword,
    pub semicolon: Semicolon,
}

pub fn new_continue_statement(
    continue_keyword: ContinueKeyword,
    semicolon: Semicolon,
) -> ContinueStatement {
    Rc::new(ContinueStatementStruct {
        continue_keyword,
        semicolon,
    })
}

pub type ContractDefinition = Rc<ContractDefinitionStruct>;

#[derive(Clone, Debug)]
pub struct ContractDefinitionStruct {
    pub abstract_keyword: Option<AbstractKeyword>,
    pub contract_keyword: ContractKeyword,
    pub name: Identifier,
    pub specifiers: ContractSpecifiers,
    pub open_brace: OpenBrace,
    pub members: ContractMembers,
    pub close_brace: CloseBrace,
}

pub fn new_contract_definition(
    abstract_keyword: Option<AbstractKeyword>,
    contract_keyword: ContractKeyword,
    name: Identifier,
    specifiers: ContractSpecifiers,
    open_brace: OpenBrace,
    members: ContractMembers,
    close_brace: CloseBrace,
) -> ContractDefinition {
    Rc::new(ContractDefinitionStruct {
        abstract_keyword,
        contract_keyword,
        name,
        specifiers,
        open_brace,
        members,
        close_brace,
    })
}

pub type DecimalNumberExpression = Rc<DecimalNumberExpressionStruct>;

#[derive(Clone, Debug)]
pub struct DecimalNumberExpressionStruct {
    pub literal: DecimalLiteral,
    pub unit: Option<NumberUnit>,
}

pub fn new_decimal_number_expression(
    literal: DecimalLiteral,
    unit: Option<NumberUnit>,
) -> DecimalNumberExpression {
    Rc::new(DecimalNumberExpressionStruct { literal, unit })
}

pub type DoWhileStatement = Rc<DoWhileStatementStruct>;

#[derive(Clone, Debug)]
pub struct DoWhileStatementStruct {
    pub do_keyword: DoKeyword,
    pub body: Statement,
    pub while_keyword: WhileKeyword,
    pub open_paren: OpenParen,
    pub condition: Expression,
    pub close_paren: CloseParen,
    pub semicolon: Semicolon,
}

pub fn new_do_while_statement(
    do_keyword: DoKeyword,
    body: Statement,
    while_keyword: WhileKeyword,
    open_paren: OpenParen,
    condition: Expression,
    close_paren: CloseParen,
    semicolon: Semicolon,
) -> DoWhileStatement {
    Rc::new(DoWhileStatementStruct {
        do_keyword,
        body,
        while_keyword,
        open_paren,
        condition,
        close_paren,
        semicolon,
    })
}

pub type ElseBranch = Rc<ElseBranchStruct>;

#[derive(Clone, Debug)]
pub struct ElseBranchStruct {
    pub else_keyword: ElseKeyword,
    pub body: Statement,
}

pub fn new_else_branch(else_keyword: ElseKeyword, body: Statement) -> ElseBranch {
    Rc::new(ElseBranchStruct { else_keyword, body })
}

pub type EmitStatement = Rc<EmitStatementStruct>;

#[derive(Clone, Debug)]
pub struct EmitStatementStruct {
    pub emit_keyword: EmitKeyword,
    pub event: IdentifierPath,
    pub arguments: ArgumentsDeclaration,
    pub semicolon: Semicolon,
}

pub fn new_emit_statement(
    emit_keyword: EmitKeyword,
    event: IdentifierPath,
    arguments: ArgumentsDeclaration,
    semicolon: Semicolon,
) -> EmitStatement {
    Rc::new(EmitStatementStruct {
        emit_keyword,
        event,
        arguments,
        semicolon,
    })
}

pub type EnumDefinition = Rc<EnumDefinitionStruct>;

#[derive(Clone, Debug)]
pub struct EnumDefinitionStruct {
    pub enum_keyword: EnumKeyword,
    pub name: Identifier,
    pub open_brace: OpenBrace,
    pub members: EnumMembers,
    pub close_brace: CloseBrace,
}

pub fn new_enum_definition(
    enum_keyword: EnumKeyword,
    name: Identifier,
    open_brace: OpenBrace,
    members: EnumMembers,
    close_brace: CloseBrace,
) -> EnumDefinition {
    Rc::new(EnumDefinitionStruct {
        enum_keyword,
        name,
        open_brace,
        members,
        close_brace,
    })
}

pub type EqualityExpression = Rc<EqualityExpressionStruct>;

#[derive(Clone, Debug)]
pub struct EqualityExpressionStruct {
    pub left_operand: Expression,
    pub expression_equality_expression_operator: Expression_EqualityExpression_Operator,
    pub right_operand: Expression,
}

pub fn new_equality_expression(
    left_operand: Expression,
    expression_equality_expression_operator: Expression_EqualityExpression_Operator,
    right_operand: Expression,
) -> EqualityExpression {
    Rc::new(EqualityExpressionStruct {
        left_operand,
        expression_equality_expression_operator,
        right_operand,
    })
}

pub type ErrorDefinition = Rc<ErrorDefinitionStruct>;

#[derive(Clone, Debug)]
pub struct ErrorDefinitionStruct {
    pub error_keyword: ErrorKeyword,
    pub name: Identifier,
    pub members: ErrorParametersDeclaration,
    pub semicolon: Semicolon,
}

pub fn new_error_definition(
    error_keyword: ErrorKeyword,
    name: Identifier,
    members: ErrorParametersDeclaration,
    semicolon: Semicolon,
) -> ErrorDefinition {
    Rc::new(ErrorDefinitionStruct {
        error_keyword,
        name,
        members,
        semicolon,
    })
}

pub type ErrorParameter = Rc<ErrorParameterStruct>;

#[derive(Clone, Debug)]
pub struct ErrorParameterStruct {
    pub type_name: TypeName,
    pub name: Option<Identifier>,
}

pub fn new_error_parameter(type_name: TypeName, name: Option<Identifier>) -> ErrorParameter {
    Rc::new(ErrorParameterStruct { type_name, name })
}

pub type ErrorParametersDeclaration = Rc<ErrorParametersDeclarationStruct>;

#[derive(Clone, Debug)]
pub struct ErrorParametersDeclarationStruct {
    pub open_paren: OpenParen,
    pub parameters: ErrorParameters,
    pub close_paren: CloseParen,
}

pub fn new_error_parameters_declaration(
    open_paren: OpenParen,
    parameters: ErrorParameters,
    close_paren: CloseParen,
) -> ErrorParametersDeclaration {
    Rc::new(ErrorParametersDeclarationStruct {
        open_paren,
        parameters,
        close_paren,
    })
}

pub type EventDefinition = Rc<EventDefinitionStruct>;

#[derive(Clone, Debug)]
pub struct EventDefinitionStruct {
    pub event_keyword: EventKeyword,
    pub name: Identifier,
    pub parameters: EventParametersDeclaration,
    pub anonymous_keyword: Option<AnonymousKeyword>,
    pub semicolon: Semicolon,
}

pub fn new_event_definition(
    event_keyword: EventKeyword,
    name: Identifier,
    parameters: EventParametersDeclaration,
    anonymous_keyword: Option<AnonymousKeyword>,
    semicolon: Semicolon,
) -> EventDefinition {
    Rc::new(EventDefinitionStruct {
        event_keyword,
        name,
        parameters,
        anonymous_keyword,
        semicolon,
    })
}

pub type EventParameter = Rc<EventParameterStruct>;

#[derive(Clone, Debug)]
pub struct EventParameterStruct {
    pub type_name: TypeName,
    pub indexed_keyword: Option<IndexedKeyword>,
    pub name: Option<Identifier>,
}

pub fn new_event_parameter(
    type_name: TypeName,
    indexed_keyword: Option<IndexedKeyword>,
    name: Option<Identifier>,
) -> EventParameter {
    Rc::new(EventParameterStruct {
        type_name,
        indexed_keyword,
        name,
    })
}

pub type EventParametersDeclaration = Rc<EventParametersDeclarationStruct>;

#[derive(Clone, Debug)]
pub struct EventParametersDeclarationStruct {
    pub open_paren: OpenParen,
    pub parameters: EventParameters,
    pub close_paren: CloseParen,
}

pub fn new_event_parameters_declaration(
    open_paren: OpenParen,
    parameters: EventParameters,
    close_paren: CloseParen,
) -> EventParametersDeclaration {
    Rc::new(EventParametersDeclarationStruct {
        open_paren,
        parameters,
        close_paren,
    })
}

pub type ExperimentalPragma = Rc<ExperimentalPragmaStruct>;

#[derive(Clone, Debug)]
pub struct ExperimentalPragmaStruct {
    pub experimental_keyword: ExperimentalKeyword,
    pub feature: ExperimentalFeature,
}

pub fn new_experimental_pragma(
    experimental_keyword: ExperimentalKeyword,
    feature: ExperimentalFeature,
) -> ExperimentalPragma {
    Rc::new(ExperimentalPragmaStruct {
        experimental_keyword,
        feature,
    })
}

pub type ExponentiationExpression = Rc<ExponentiationExpressionStruct>;

#[derive(Clone, Debug)]
pub struct ExponentiationExpressionStruct {
    pub left_operand: Expression,
    pub expression_exponentiation_expression_operator: Expression_ExponentiationExpression_Operator,
    pub right_operand: Expression,
}

pub fn new_exponentiation_expression(
    left_operand: Expression,
    expression_exponentiation_expression_operator: Expression_ExponentiationExpression_Operator,
    right_operand: Expression,
) -> ExponentiationExpression {
    Rc::new(ExponentiationExpressionStruct {
        left_operand,
        expression_exponentiation_expression_operator,
        right_operand,
    })
}

pub type ExpressionStatement = Rc<ExpressionStatementStruct>;

#[derive(Clone, Debug)]
pub struct ExpressionStatementStruct {
    pub expression: Expression,
    pub semicolon: Semicolon,
}

pub fn new_expression_statement(
    expression: Expression,
    semicolon: Semicolon,
) -> ExpressionStatement {
    Rc::new(ExpressionStatementStruct {
        expression,
        semicolon,
    })
}

pub type FallbackFunctionDefinition = Rc<FallbackFunctionDefinitionStruct>;

#[derive(Clone, Debug)]
pub struct FallbackFunctionDefinitionStruct {
    pub fallback_keyword: FallbackKeyword,
    pub parameters: ParametersDeclaration,
    pub attributes: FallbackFunctionAttributes,
    pub returns: Option<ReturnsDeclaration>,
    pub body: FunctionBody,
}

pub fn new_fallback_function_definition(
    fallback_keyword: FallbackKeyword,
    parameters: ParametersDeclaration,
    attributes: FallbackFunctionAttributes,
    returns: Option<ReturnsDeclaration>,
    body: FunctionBody,
) -> FallbackFunctionDefinition {
    Rc::new(FallbackFunctionDefinitionStruct {
        fallback_keyword,
        parameters,
        attributes,
        returns,
        body,
    })
}

pub type ForStatement = Rc<ForStatementStruct>;

#[derive(Clone, Debug)]
pub struct ForStatementStruct {
    pub for_keyword: ForKeyword,
    pub open_paren: OpenParen,
    pub initialization: ForStatementInitialization,
    pub condition: ForStatementCondition,
    pub iterator: Option<Expression>,
    pub close_paren: CloseParen,
    pub body: Statement,
}

pub fn new_for_statement(
    for_keyword: ForKeyword,
    open_paren: OpenParen,
    initialization: ForStatementInitialization,
    condition: ForStatementCondition,
    iterator: Option<Expression>,
    close_paren: CloseParen,
    body: Statement,
) -> ForStatement {
    Rc::new(ForStatementStruct {
        for_keyword,
        open_paren,
        initialization,
        condition,
        iterator,
        close_paren,
        body,
    })
}

pub type FunctionCallExpression = Rc<FunctionCallExpressionStruct>;

#[derive(Clone, Debug)]
pub struct FunctionCallExpressionStruct {
    pub operand: Expression,
    pub arguments: ArgumentsDeclaration,
}

pub fn new_function_call_expression(
    operand: Expression,
    arguments: ArgumentsDeclaration,
) -> FunctionCallExpression {
    Rc::new(FunctionCallExpressionStruct { operand, arguments })
}

pub type FunctionDefinition = Rc<FunctionDefinitionStruct>;

#[derive(Clone, Debug)]
pub struct FunctionDefinitionStruct {
    pub function_keyword: FunctionKeyword,
    pub name: FunctionName,
    pub parameters: ParametersDeclaration,
    pub attributes: FunctionAttributes,
    pub returns: Option<ReturnsDeclaration>,
    pub body: FunctionBody,
}

pub fn new_function_definition(
    function_keyword: FunctionKeyword,
    name: FunctionName,
    parameters: ParametersDeclaration,
    attributes: FunctionAttributes,
    returns: Option<ReturnsDeclaration>,
    body: FunctionBody,
) -> FunctionDefinition {
    Rc::new(FunctionDefinitionStruct {
        function_keyword,
        name,
        parameters,
        attributes,
        returns,
        body,
    })
}

pub type FunctionType = Rc<FunctionTypeStruct>;

#[derive(Clone, Debug)]
pub struct FunctionTypeStruct {
    pub function_keyword: FunctionKeyword,
    pub parameters: ParametersDeclaration,
    pub attributes: FunctionTypeAttributes,
    pub returns: Option<ReturnsDeclaration>,
}

pub fn new_function_type(
    function_keyword: FunctionKeyword,
    parameters: ParametersDeclaration,
    attributes: FunctionTypeAttributes,
    returns: Option<ReturnsDeclaration>,
) -> FunctionType {
    Rc::new(FunctionTypeStruct {
        function_keyword,
        parameters,
        attributes,
        returns,
    })
}

pub type HexNumberExpression = Rc<HexNumberExpressionStruct>;

#[derive(Clone, Debug)]
pub struct HexNumberExpressionStruct {
    pub literal: HexLiteral,
    pub unit: Option<NumberUnit>,
}

pub fn new_hex_number_expression(
    literal: HexLiteral,
    unit: Option<NumberUnit>,
) -> HexNumberExpression {
    Rc::new(HexNumberExpressionStruct { literal, unit })
}

pub type IfStatement = Rc<IfStatementStruct>;

#[derive(Clone, Debug)]
pub struct IfStatementStruct {
    pub if_keyword: IfKeyword,
    pub open_paren: OpenParen,
    pub condition: Expression,
    pub close_paren: CloseParen,
    pub body: Statement,
    pub else_branch: Option<ElseBranch>,
}

pub fn new_if_statement(
    if_keyword: IfKeyword,
    open_paren: OpenParen,
    condition: Expression,
    close_paren: CloseParen,
    body: Statement,
    else_branch: Option<ElseBranch>,
) -> IfStatement {
    Rc::new(IfStatementStruct {
        if_keyword,
        open_paren,
        condition,
        close_paren,
        body,
        else_branch,
    })
}

pub type ImportAlias = Rc<ImportAliasStruct>;

#[derive(Clone, Debug)]
pub struct ImportAliasStruct {
    pub as_keyword: AsKeyword,
    pub identifier: Identifier,
}

pub fn new_import_alias(as_keyword: AsKeyword, identifier: Identifier) -> ImportAlias {
    Rc::new(ImportAliasStruct {
        as_keyword,
        identifier,
    })
}

pub type ImportDeconstruction = Rc<ImportDeconstructionStruct>;

#[derive(Clone, Debug)]
pub struct ImportDeconstructionStruct {
    pub open_brace: OpenBrace,
    pub symbols: ImportDeconstructionSymbols,
    pub close_brace: CloseBrace,
    pub from_keyword: FromKeyword,
    pub path: StringLiteral,
}

pub fn new_import_deconstruction(
    open_brace: OpenBrace,
    symbols: ImportDeconstructionSymbols,
    close_brace: CloseBrace,
    from_keyword: FromKeyword,
    path: StringLiteral,
) -> ImportDeconstruction {
    Rc::new(ImportDeconstructionStruct {
        open_brace,
        symbols,
        close_brace,
        from_keyword,
        path,
    })
}

pub type ImportDeconstructionSymbol = Rc<ImportDeconstructionSymbolStruct>;

#[derive(Clone, Debug)]
pub struct ImportDeconstructionSymbolStruct {
    pub name: Identifier,
    pub alias: Option<ImportAlias>,
}

pub fn new_import_deconstruction_symbol(
    name: Identifier,
    alias: Option<ImportAlias>,
) -> ImportDeconstructionSymbol {
    Rc::new(ImportDeconstructionSymbolStruct { name, alias })
}

pub type ImportDirective = Rc<ImportDirectiveStruct>;

#[derive(Clone, Debug)]
pub struct ImportDirectiveStruct {
    pub import_keyword: ImportKeyword,
    pub clause: ImportClause,
    pub semicolon: Semicolon,
}

pub fn new_import_directive(
    import_keyword: ImportKeyword,
    clause: ImportClause,
    semicolon: Semicolon,
) -> ImportDirective {
    Rc::new(ImportDirectiveStruct {
        import_keyword,
        clause,
        semicolon,
    })
}

pub type IndexAccessEnd = Rc<IndexAccessEndStruct>;

#[derive(Clone, Debug)]
pub struct IndexAccessEndStruct {
    pub colon: Colon,
    pub end: Option<Expression>,
}

pub fn new_index_access_end(colon: Colon, end: Option<Expression>) -> IndexAccessEnd {
    Rc::new(IndexAccessEndStruct { colon, end })
}

pub type IndexAccessExpression = Rc<IndexAccessExpressionStruct>;

#[derive(Clone, Debug)]
pub struct IndexAccessExpressionStruct {
    pub operand: Expression,
    pub open_bracket: OpenBracket,
    pub start: Option<Expression>,
    pub end: Option<IndexAccessEnd>,
    pub close_bracket: CloseBracket,
}

pub fn new_index_access_expression(
    operand: Expression,
    open_bracket: OpenBracket,
    start: Option<Expression>,
    end: Option<IndexAccessEnd>,
    close_bracket: CloseBracket,
) -> IndexAccessExpression {
    Rc::new(IndexAccessExpressionStruct {
        operand,
        open_bracket,
        start,
        end,
        close_bracket,
    })
}

pub type InequalityExpression = Rc<InequalityExpressionStruct>;

#[derive(Clone, Debug)]
pub struct InequalityExpressionStruct {
    pub left_operand: Expression,
    pub expression_inequality_expression_operator: Expression_InequalityExpression_Operator,
    pub right_operand: Expression,
}

pub fn new_inequality_expression(
    left_operand: Expression,
    expression_inequality_expression_operator: Expression_InequalityExpression_Operator,
    right_operand: Expression,
) -> InequalityExpression {
    Rc::new(InequalityExpressionStruct {
        left_operand,
        expression_inequality_expression_operator,
        right_operand,
    })
}

pub type InheritanceSpecifier = Rc<InheritanceSpecifierStruct>;

#[derive(Clone, Debug)]
pub struct InheritanceSpecifierStruct {
    pub is_keyword: IsKeyword,
    pub types: InheritanceTypes,
}

pub fn new_inheritance_specifier(
    is_keyword: IsKeyword,
    types: InheritanceTypes,
) -> InheritanceSpecifier {
    Rc::new(InheritanceSpecifierStruct { is_keyword, types })
}

pub type InheritanceType = Rc<InheritanceTypeStruct>;

#[derive(Clone, Debug)]
pub struct InheritanceTypeStruct {
    pub type_name: IdentifierPath,
    pub arguments: Option<ArgumentsDeclaration>,
}

pub fn new_inheritance_type(
    type_name: IdentifierPath,
    arguments: Option<ArgumentsDeclaration>,
) -> InheritanceType {
    Rc::new(InheritanceTypeStruct {
        type_name,
        arguments,
    })
}

pub type InterfaceDefinition = Rc<InterfaceDefinitionStruct>;

#[derive(Clone, Debug)]
pub struct InterfaceDefinitionStruct {
    pub interface_keyword: InterfaceKeyword,
    pub name: Identifier,
    pub inheritance: Option<InheritanceSpecifier>,
    pub open_brace: OpenBrace,
    pub members: InterfaceMembers,
    pub close_brace: CloseBrace,
}

pub fn new_interface_definition(
    interface_keyword: InterfaceKeyword,
    name: Identifier,
    inheritance: Option<InheritanceSpecifier>,
    open_brace: OpenBrace,
    members: InterfaceMembers,
    close_brace: CloseBrace,
) -> InterfaceDefinition {
    Rc::new(InterfaceDefinitionStruct {
        interface_keyword,
        name,
        inheritance,
        open_brace,
        members,
        close_brace,
    })
}

pub type LibraryDefinition = Rc<LibraryDefinitionStruct>;

#[derive(Clone, Debug)]
pub struct LibraryDefinitionStruct {
    pub library_keyword: LibraryKeyword,
    pub name: Identifier,
    pub open_brace: OpenBrace,
    pub members: LibraryMembers,
    pub close_brace: CloseBrace,
}

pub fn new_library_definition(
    library_keyword: LibraryKeyword,
    name: Identifier,
    open_brace: OpenBrace,
    members: LibraryMembers,
    close_brace: CloseBrace,
) -> LibraryDefinition {
    Rc::new(LibraryDefinitionStruct {
        library_keyword,
        name,
        open_brace,
        members,
        close_brace,
    })
}

pub type MappingKey = Rc<MappingKeyStruct>;

#[derive(Clone, Debug)]
pub struct MappingKeyStruct {
    pub key_type: MappingKeyType,
    pub name: Option<Identifier>,
}

pub fn new_mapping_key(key_type: MappingKeyType, name: Option<Identifier>) -> MappingKey {
    Rc::new(MappingKeyStruct { key_type, name })
}

pub type MappingType = Rc<MappingTypeStruct>;

#[derive(Clone, Debug)]
pub struct MappingTypeStruct {
    pub mapping_keyword: MappingKeyword,
    pub open_paren: OpenParen,
    pub key_type: MappingKey,
    pub equal_greater_than: EqualGreaterThan,
    pub value_type: MappingValue,
    pub close_paren: CloseParen,
}

pub fn new_mapping_type(
    mapping_keyword: MappingKeyword,
    open_paren: OpenParen,
    key_type: MappingKey,
    equal_greater_than: EqualGreaterThan,
    value_type: MappingValue,
    close_paren: CloseParen,
) -> MappingType {
    Rc::new(MappingTypeStruct {
        mapping_keyword,
        open_paren,
        key_type,
        equal_greater_than,
        value_type,
        close_paren,
    })
}

pub type MappingValue = Rc<MappingValueStruct>;

#[derive(Clone, Debug)]
pub struct MappingValueStruct {
    pub type_name: TypeName,
    pub name: Option<Identifier>,
}

pub fn new_mapping_value(type_name: TypeName, name: Option<Identifier>) -> MappingValue {
    Rc::new(MappingValueStruct { type_name, name })
}

pub type MemberAccessExpression = Rc<MemberAccessExpressionStruct>;

#[derive(Clone, Debug)]
pub struct MemberAccessExpressionStruct {
    pub operand: Expression,
    pub period: Period,
    pub member: Identifier,
}

pub fn new_member_access_expression(
    operand: Expression,
    period: Period,
    member: Identifier,
) -> MemberAccessExpression {
    Rc::new(MemberAccessExpressionStruct {
        operand,
        period,
        member,
    })
}

pub type ModifierDefinition = Rc<ModifierDefinitionStruct>;

#[derive(Clone, Debug)]
pub struct ModifierDefinitionStruct {
    pub modifier_keyword: ModifierKeyword,
    pub name: Identifier,
    pub parameters: Option<ParametersDeclaration>,
    pub attributes: ModifierAttributes,
    pub body: FunctionBody,
}

pub fn new_modifier_definition(
    modifier_keyword: ModifierKeyword,
    name: Identifier,
    parameters: Option<ParametersDeclaration>,
    attributes: ModifierAttributes,
    body: FunctionBody,
) -> ModifierDefinition {
    Rc::new(ModifierDefinitionStruct {
        modifier_keyword,
        name,
        parameters,
        attributes,
        body,
    })
}

pub type ModifierInvocation = Rc<ModifierInvocationStruct>;

#[derive(Clone, Debug)]
pub struct ModifierInvocationStruct {
    pub name: IdentifierPath,
    pub arguments: Option<ArgumentsDeclaration>,
}

pub fn new_modifier_invocation(
    name: IdentifierPath,
    arguments: Option<ArgumentsDeclaration>,
) -> ModifierInvocation {
    Rc::new(ModifierInvocationStruct { name, arguments })
}

pub type MultiplicativeExpression = Rc<MultiplicativeExpressionStruct>;

#[derive(Clone, Debug)]
pub struct MultiplicativeExpressionStruct {
    pub left_operand: Expression,
    pub expression_multiplicative_expression_operator: Expression_MultiplicativeExpression_Operator,
    pub right_operand: Expression,
}

pub fn new_multiplicative_expression(
    left_operand: Expression,
    expression_multiplicative_expression_operator: Expression_MultiplicativeExpression_Operator,
    right_operand: Expression,
) -> MultiplicativeExpression {
    Rc::new(MultiplicativeExpressionStruct {
        left_operand,
        expression_multiplicative_expression_operator,
        right_operand,
    })
}

pub type NamedArgument = Rc<NamedArgumentStruct>;

#[derive(Clone, Debug)]
pub struct NamedArgumentStruct {
    pub name: Identifier,
    pub colon: Colon,
    pub value: Expression,
}

pub fn new_named_argument(name: Identifier, colon: Colon, value: Expression) -> NamedArgument {
    Rc::new(NamedArgumentStruct { name, colon, value })
}

pub type NamedArgumentGroup = Rc<NamedArgumentGroupStruct>;

#[derive(Clone, Debug)]
pub struct NamedArgumentGroupStruct {
    pub open_brace: OpenBrace,
    pub arguments: NamedArguments,
    pub close_brace: CloseBrace,
}

pub fn new_named_argument_group(
    open_brace: OpenBrace,
    arguments: NamedArguments,
    close_brace: CloseBrace,
) -> NamedArgumentGroup {
    Rc::new(NamedArgumentGroupStruct {
        open_brace,
        arguments,
        close_brace,
    })
}

pub type NamedArgumentsDeclaration = Rc<NamedArgumentsDeclarationStruct>;

#[derive(Clone, Debug)]
pub struct NamedArgumentsDeclarationStruct {
    pub open_paren: OpenParen,
    pub arguments: Option<NamedArgumentGroup>,
    pub close_paren: CloseParen,
}

pub fn new_named_arguments_declaration(
    open_paren: OpenParen,
    arguments: Option<NamedArgumentGroup>,
    close_paren: CloseParen,
) -> NamedArgumentsDeclaration {
    Rc::new(NamedArgumentsDeclarationStruct {
        open_paren,
        arguments,
        close_paren,
    })
}

pub type NamedImport = Rc<NamedImportStruct>;

#[derive(Clone, Debug)]
pub struct NamedImportStruct {
    pub asterisk: Asterisk,
    pub alias: ImportAlias,
    pub from_keyword: FromKeyword,
    pub path: StringLiteral,
}

pub fn new_named_import(
    asterisk: Asterisk,
    alias: ImportAlias,
    from_keyword: FromKeyword,
    path: StringLiteral,
) -> NamedImport {
    Rc::new(NamedImportStruct {
        asterisk,
        alias,
        from_keyword,
        path,
    })
}

pub type NewExpression = Rc<NewExpressionStruct>;

#[derive(Clone, Debug)]
pub struct NewExpressionStruct {
    pub new_keyword: NewKeyword,
    pub type_name: TypeName,
}

pub fn new_new_expression(new_keyword: NewKeyword, type_name: TypeName) -> NewExpression {
    Rc::new(NewExpressionStruct {
        new_keyword,
        type_name,
    })
}

pub type OrExpression = Rc<OrExpressionStruct>;

#[derive(Clone, Debug)]
pub struct OrExpressionStruct {
    pub left_operand: Expression,
    pub operator: BarBar,
    pub right_operand: Expression,
}

pub fn new_or_expression(
    left_operand: Expression,
    operator: BarBar,
    right_operand: Expression,
) -> OrExpression {
    Rc::new(OrExpressionStruct {
        left_operand,
        operator,
        right_operand,
    })
}

pub type OverridePathsDeclaration = Rc<OverridePathsDeclarationStruct>;

#[derive(Clone, Debug)]
pub struct OverridePathsDeclarationStruct {
    pub open_paren: OpenParen,
    pub paths: OverridePaths,
    pub close_paren: CloseParen,
}

pub fn new_override_paths_declaration(
    open_paren: OpenParen,
    paths: OverridePaths,
    close_paren: CloseParen,
) -> OverridePathsDeclaration {
    Rc::new(OverridePathsDeclarationStruct {
        open_paren,
        paths,
        close_paren,
    })
}

pub type OverrideSpecifier = Rc<OverrideSpecifierStruct>;

#[derive(Clone, Debug)]
pub struct OverrideSpecifierStruct {
    pub override_keyword: OverrideKeyword,
    pub overridden: Option<OverridePathsDeclaration>,
}

pub fn new_override_specifier(
    override_keyword: OverrideKeyword,
    overridden: Option<OverridePathsDeclaration>,
) -> OverrideSpecifier {
    Rc::new(OverrideSpecifierStruct {
        override_keyword,
        overridden,
    })
}

pub type Parameter = Rc<ParameterStruct>;

#[derive(Clone, Debug)]
pub struct ParameterStruct {
    pub type_name: TypeName,
    pub storage_location: Option<StorageLocation>,
    pub name: Option<Identifier>,
}

pub fn new_parameter(
    type_name: TypeName,
    storage_location: Option<StorageLocation>,
    name: Option<Identifier>,
) -> Parameter {
    Rc::new(ParameterStruct {
        type_name,
        storage_location,
        name,
    })
}

pub type ParametersDeclaration = Rc<ParametersDeclarationStruct>;

#[derive(Clone, Debug)]
pub struct ParametersDeclarationStruct {
    pub open_paren: OpenParen,
    pub parameters: Parameters,
    pub close_paren: CloseParen,
}

pub fn new_parameters_declaration(
    open_paren: OpenParen,
    parameters: Parameters,
    close_paren: CloseParen,
) -> ParametersDeclaration {
    Rc::new(ParametersDeclarationStruct {
        open_paren,
        parameters,
        close_paren,
    })
}

pub type PathImport = Rc<PathImportStruct>;

#[derive(Clone, Debug)]
pub struct PathImportStruct {
    pub path: StringLiteral,
    pub alias: Option<ImportAlias>,
}

pub fn new_path_import(path: StringLiteral, alias: Option<ImportAlias>) -> PathImport {
    Rc::new(PathImportStruct { path, alias })
}

pub type PositionalArgumentsDeclaration = Rc<PositionalArgumentsDeclarationStruct>;

#[derive(Clone, Debug)]
pub struct PositionalArgumentsDeclarationStruct {
    pub open_paren: OpenParen,
    pub arguments: PositionalArguments,
    pub close_paren: CloseParen,
}

pub fn new_positional_arguments_declaration(
    open_paren: OpenParen,
    arguments: PositionalArguments,
    close_paren: CloseParen,
) -> PositionalArgumentsDeclaration {
    Rc::new(PositionalArgumentsDeclarationStruct {
        open_paren,
        arguments,
        close_paren,
    })
}

pub type PostfixExpression = Rc<PostfixExpressionStruct>;

#[derive(Clone, Debug)]
pub struct PostfixExpressionStruct {
    pub operand: Expression,
    pub expression_postfix_expression_operator: Expression_PostfixExpression_Operator,
}

pub fn new_postfix_expression(
    operand: Expression,
    expression_postfix_expression_operator: Expression_PostfixExpression_Operator,
) -> PostfixExpression {
    Rc::new(PostfixExpressionStruct {
        operand,
        expression_postfix_expression_operator,
    })
}

pub type PragmaDirective = Rc<PragmaDirectiveStruct>;

#[derive(Clone, Debug)]
pub struct PragmaDirectiveStruct {
    pub pragma_keyword: PragmaKeyword,
    pub pragma: Pragma,
    pub semicolon: Semicolon,
}

pub fn new_pragma_directive(
    pragma_keyword: PragmaKeyword,
    pragma: Pragma,
    semicolon: Semicolon,
) -> PragmaDirective {
    Rc::new(PragmaDirectiveStruct {
        pragma_keyword,
        pragma,
        semicolon,
    })
}

pub type PrefixExpression = Rc<PrefixExpressionStruct>;

#[derive(Clone, Debug)]
pub struct PrefixExpressionStruct {
    pub expression_prefix_expression_operator: Expression_PrefixExpression_Operator,
    pub operand: Expression,
}

pub fn new_prefix_expression(
    expression_prefix_expression_operator: Expression_PrefixExpression_Operator,
    operand: Expression,
) -> PrefixExpression {
    Rc::new(PrefixExpressionStruct {
        expression_prefix_expression_operator,
        operand,
    })
}

pub type ReceiveFunctionDefinition = Rc<ReceiveFunctionDefinitionStruct>;

#[derive(Clone, Debug)]
pub struct ReceiveFunctionDefinitionStruct {
    pub receive_keyword: ReceiveKeyword,
    pub parameters: ParametersDeclaration,
    pub attributes: ReceiveFunctionAttributes,
    pub body: FunctionBody,
}

pub fn new_receive_function_definition(
    receive_keyword: ReceiveKeyword,
    parameters: ParametersDeclaration,
    attributes: ReceiveFunctionAttributes,
    body: FunctionBody,
) -> ReceiveFunctionDefinition {
    Rc::new(ReceiveFunctionDefinitionStruct {
        receive_keyword,
        parameters,
        attributes,
        body,
    })
}

pub type ReturnStatement = Rc<ReturnStatementStruct>;

#[derive(Clone, Debug)]
pub struct ReturnStatementStruct {
    pub return_keyword: ReturnKeyword,
    pub expression: Option<Expression>,
    pub semicolon: Semicolon,
}

pub fn new_return_statement(
    return_keyword: ReturnKeyword,
    expression: Option<Expression>,
    semicolon: Semicolon,
) -> ReturnStatement {
    Rc::new(ReturnStatementStruct {
        return_keyword,
        expression,
        semicolon,
    })
}

pub type ReturnsDeclaration = Rc<ReturnsDeclarationStruct>;

#[derive(Clone, Debug)]
pub struct ReturnsDeclarationStruct {
    pub returns_keyword: ReturnsKeyword,
    pub variables: ParametersDeclaration,
}

pub fn new_returns_declaration(
    returns_keyword: ReturnsKeyword,
    variables: ParametersDeclaration,
) -> ReturnsDeclaration {
    Rc::new(ReturnsDeclarationStruct {
        returns_keyword,
        variables,
    })
}

pub type RevertStatement = Rc<RevertStatementStruct>;

#[derive(Clone, Debug)]
pub struct RevertStatementStruct {
    pub revert_keyword: RevertKeyword,
    pub error: IdentifierPath,
    pub arguments: ArgumentsDeclaration,
    pub semicolon: Semicolon,
}

pub fn new_revert_statement(
    revert_keyword: RevertKeyword,
    error: IdentifierPath,
    arguments: ArgumentsDeclaration,
    semicolon: Semicolon,
) -> RevertStatement {
    Rc::new(RevertStatementStruct {
        revert_keyword,
        error,
        arguments,
        semicolon,
    })
}

pub type ShiftExpression = Rc<ShiftExpressionStruct>;

#[derive(Clone, Debug)]
pub struct ShiftExpressionStruct {
    pub left_operand: Expression,
    pub expression_shift_expression_operator: Expression_ShiftExpression_Operator,
    pub right_operand: Expression,
}

pub fn new_shift_expression(
    left_operand: Expression,
    expression_shift_expression_operator: Expression_ShiftExpression_Operator,
    right_operand: Expression,
) -> ShiftExpression {
    Rc::new(ShiftExpressionStruct {
        left_operand,
        expression_shift_expression_operator,
        right_operand,
    })
}

pub type SourceUnit = Rc<SourceUnitStruct>;

#[derive(Clone, Debug)]
pub struct SourceUnitStruct {
    pub members: SourceUnitMembers,
}

pub fn new_source_unit(members: SourceUnitMembers) -> SourceUnit {
    Rc::new(SourceUnitStruct { members })
}

pub type StateVariableDefinition = Rc<StateVariableDefinitionStruct>;

#[derive(Clone, Debug)]
pub struct StateVariableDefinitionStruct {
    pub type_name: TypeName,
    pub attributes: StateVariableAttributes,
    pub name: Identifier,
    pub value: Option<StateVariableDefinitionValue>,
    pub semicolon: Semicolon,
}

pub fn new_state_variable_definition(
    type_name: TypeName,
    attributes: StateVariableAttributes,
    name: Identifier,
    value: Option<StateVariableDefinitionValue>,
    semicolon: Semicolon,
) -> StateVariableDefinition {
    Rc::new(StateVariableDefinitionStruct {
        type_name,
        attributes,
        name,
        value,
        semicolon,
    })
}

pub type StateVariableDefinitionValue = Rc<StateVariableDefinitionValueStruct>;

#[derive(Clone, Debug)]
pub struct StateVariableDefinitionValueStruct {
    pub equal: Equal,
    pub value: Expression,
}

pub fn new_state_variable_definition_value(
    equal: Equal,
    value: Expression,
) -> StateVariableDefinitionValue {
    Rc::new(StateVariableDefinitionValueStruct { equal, value })
}

pub type StorageLayoutSpecifier = Rc<StorageLayoutSpecifierStruct>;

#[derive(Clone, Debug)]
pub struct StorageLayoutSpecifierStruct {
    pub layout_keyword: LayoutKeyword,
    pub at_keyword: AtKeyword,
    pub expression: Expression,
}

pub fn new_storage_layout_specifier(
    layout_keyword: LayoutKeyword,
    at_keyword: AtKeyword,
    expression: Expression,
) -> StorageLayoutSpecifier {
    Rc::new(StorageLayoutSpecifierStruct {
        layout_keyword,
        at_keyword,
        expression,
    })
}

pub type StructDefinition = Rc<StructDefinitionStruct>;

#[derive(Clone, Debug)]
pub struct StructDefinitionStruct {
    pub struct_keyword: StructKeyword,
    pub name: Identifier,
    pub open_brace: OpenBrace,
    pub members: StructMembers,
    pub close_brace: CloseBrace,
}

pub fn new_struct_definition(
    struct_keyword: StructKeyword,
    name: Identifier,
    open_brace: OpenBrace,
    members: StructMembers,
    close_brace: CloseBrace,
) -> StructDefinition {
    Rc::new(StructDefinitionStruct {
        struct_keyword,
        name,
        open_brace,
        members,
        close_brace,
    })
}

pub type StructMember = Rc<StructMemberStruct>;

#[derive(Clone, Debug)]
pub struct StructMemberStruct {
    pub type_name: TypeName,
    pub name: Identifier,
    pub semicolon: Semicolon,
}

pub fn new_struct_member(
    type_name: TypeName,
    name: Identifier,
    semicolon: Semicolon,
) -> StructMember {
    Rc::new(StructMemberStruct {
        type_name,
        name,
        semicolon,
    })
}

pub type ThrowStatement = Rc<ThrowStatementStruct>;

#[derive(Clone, Debug)]
pub struct ThrowStatementStruct {
    pub throw_keyword: ThrowKeyword,
    pub semicolon: Semicolon,
}

pub fn new_throw_statement(throw_keyword: ThrowKeyword, semicolon: Semicolon) -> ThrowStatement {
    Rc::new(ThrowStatementStruct {
        throw_keyword,
        semicolon,
    })
}

pub type TryStatement = Rc<TryStatementStruct>;

#[derive(Clone, Debug)]
pub struct TryStatementStruct {
    pub try_keyword: TryKeyword,
    pub expression: Expression,
    pub returns: Option<ReturnsDeclaration>,
    pub body: Block,
    pub catch_clauses: CatchClauses,
}

pub fn new_try_statement(
    try_keyword: TryKeyword,
    expression: Expression,
    returns: Option<ReturnsDeclaration>,
    body: Block,
    catch_clauses: CatchClauses,
) -> TryStatement {
    Rc::new(TryStatementStruct {
        try_keyword,
        expression,
        returns,
        body,
        catch_clauses,
    })
}

pub type TupleDeconstructionElement = Rc<TupleDeconstructionElementStruct>;

#[derive(Clone, Debug)]
pub struct TupleDeconstructionElementStruct {
    pub member: Option<TupleMember>,
}

pub fn new_tuple_deconstruction_element(member: Option<TupleMember>) -> TupleDeconstructionElement {
    Rc::new(TupleDeconstructionElementStruct { member })
}

pub type TupleDeconstructionStatement = Rc<TupleDeconstructionStatementStruct>;

#[derive(Clone, Debug)]
pub struct TupleDeconstructionStatementStruct {
    pub var_keyword: Option<VarKeyword>,
    pub open_paren: OpenParen,
    pub elements: TupleDeconstructionElements,
    pub close_paren: CloseParen,
    pub equal: Equal,
    pub expression: Expression,
    pub semicolon: Semicolon,
}

pub fn new_tuple_deconstruction_statement(
    var_keyword: Option<VarKeyword>,
    open_paren: OpenParen,
    elements: TupleDeconstructionElements,
    close_paren: CloseParen,
    equal: Equal,
    expression: Expression,
    semicolon: Semicolon,
) -> TupleDeconstructionStatement {
    Rc::new(TupleDeconstructionStatementStruct {
        var_keyword,
        open_paren,
        elements,
        close_paren,
        equal,
        expression,
        semicolon,
    })
}

pub type TupleExpression = Rc<TupleExpressionStruct>;

#[derive(Clone, Debug)]
pub struct TupleExpressionStruct {
    pub open_paren: OpenParen,
    pub items: TupleValues,
    pub close_paren: CloseParen,
}

pub fn new_tuple_expression(
    open_paren: OpenParen,
    items: TupleValues,
    close_paren: CloseParen,
) -> TupleExpression {
    Rc::new(TupleExpressionStruct {
        open_paren,
        items,
        close_paren,
    })
}

pub type TupleValue = Rc<TupleValueStruct>;

#[derive(Clone, Debug)]
pub struct TupleValueStruct {
    pub expression: Option<Expression>,
}

pub fn new_tuple_value(expression: Option<Expression>) -> TupleValue {
    Rc::new(TupleValueStruct { expression })
}

pub type TypeExpression = Rc<TypeExpressionStruct>;

#[derive(Clone, Debug)]
pub struct TypeExpressionStruct {
    pub type_keyword: TypeKeyword,
    pub open_paren: OpenParen,
    pub type_name: TypeName,
    pub close_paren: CloseParen,
}

pub fn new_type_expression(
    type_keyword: TypeKeyword,
    open_paren: OpenParen,
    type_name: TypeName,
    close_paren: CloseParen,
) -> TypeExpression {
    Rc::new(TypeExpressionStruct {
        type_keyword,
        open_paren,
        type_name,
        close_paren,
    })
}

pub type TypedTupleMember = Rc<TypedTupleMemberStruct>;

#[derive(Clone, Debug)]
pub struct TypedTupleMemberStruct {
    pub type_name: TypeName,
    pub storage_location: Option<StorageLocation>,
    pub name: Identifier,
}

pub fn new_typed_tuple_member(
    type_name: TypeName,
    storage_location: Option<StorageLocation>,
    name: Identifier,
) -> TypedTupleMember {
    Rc::new(TypedTupleMemberStruct {
        type_name,
        storage_location,
        name,
    })
}

pub type UncheckedBlock = Rc<UncheckedBlockStruct>;

#[derive(Clone, Debug)]
pub struct UncheckedBlockStruct {
    pub unchecked_keyword: UncheckedKeyword,
    pub block: Block,
}

pub fn new_unchecked_block(unchecked_keyword: UncheckedKeyword, block: Block) -> UncheckedBlock {
    Rc::new(UncheckedBlockStruct {
        unchecked_keyword,
        block,
    })
}

pub type UnnamedFunctionDefinition = Rc<UnnamedFunctionDefinitionStruct>;

#[derive(Clone, Debug)]
pub struct UnnamedFunctionDefinitionStruct {
    pub function_keyword: FunctionKeyword,
    pub parameters: ParametersDeclaration,
    pub attributes: UnnamedFunctionAttributes,
    pub body: FunctionBody,
}

pub fn new_unnamed_function_definition(
    function_keyword: FunctionKeyword,
    parameters: ParametersDeclaration,
    attributes: UnnamedFunctionAttributes,
    body: FunctionBody,
) -> UnnamedFunctionDefinition {
    Rc::new(UnnamedFunctionDefinitionStruct {
        function_keyword,
        parameters,
        attributes,
        body,
    })
}

pub type UntypedTupleMember = Rc<UntypedTupleMemberStruct>;

#[derive(Clone, Debug)]
pub struct UntypedTupleMemberStruct {
    pub storage_location: Option<StorageLocation>,
    pub name: Identifier,
}

pub fn new_untyped_tuple_member(
    storage_location: Option<StorageLocation>,
    name: Identifier,
) -> UntypedTupleMember {
    Rc::new(UntypedTupleMemberStruct {
        storage_location,
        name,
    })
}

pub type UserDefinedValueTypeDefinition = Rc<UserDefinedValueTypeDefinitionStruct>;

#[derive(Clone, Debug)]
pub struct UserDefinedValueTypeDefinitionStruct {
    pub type_keyword: TypeKeyword,
    pub name: Identifier,
    pub is_keyword: IsKeyword,
    pub value_type: ElementaryType,
    pub semicolon: Semicolon,
}

pub fn new_user_defined_value_type_definition(
    type_keyword: TypeKeyword,
    name: Identifier,
    is_keyword: IsKeyword,
    value_type: ElementaryType,
    semicolon: Semicolon,
) -> UserDefinedValueTypeDefinition {
    Rc::new(UserDefinedValueTypeDefinitionStruct {
        type_keyword,
        name,
        is_keyword,
        value_type,
        semicolon,
    })
}

pub type UsingAlias = Rc<UsingAliasStruct>;

#[derive(Clone, Debug)]
pub struct UsingAliasStruct {
    pub as_keyword: AsKeyword,
    pub operator: UsingOperator,
}

pub fn new_using_alias(as_keyword: AsKeyword, operator: UsingOperator) -> UsingAlias {
    Rc::new(UsingAliasStruct {
        as_keyword,
        operator,
    })
}

pub type UsingDeconstruction = Rc<UsingDeconstructionStruct>;

#[derive(Clone, Debug)]
pub struct UsingDeconstructionStruct {
    pub open_brace: OpenBrace,
    pub symbols: UsingDeconstructionSymbols,
    pub close_brace: CloseBrace,
}

pub fn new_using_deconstruction(
    open_brace: OpenBrace,
    symbols: UsingDeconstructionSymbols,
    close_brace: CloseBrace,
) -> UsingDeconstruction {
    Rc::new(UsingDeconstructionStruct {
        open_brace,
        symbols,
        close_brace,
    })
}

pub type UsingDeconstructionSymbol = Rc<UsingDeconstructionSymbolStruct>;

#[derive(Clone, Debug)]
pub struct UsingDeconstructionSymbolStruct {
    pub name: IdentifierPath,
    pub alias: Option<UsingAlias>,
}

pub fn new_using_deconstruction_symbol(
    name: IdentifierPath,
    alias: Option<UsingAlias>,
) -> UsingDeconstructionSymbol {
    Rc::new(UsingDeconstructionSymbolStruct { name, alias })
}

pub type UsingDirective = Rc<UsingDirectiveStruct>;

#[derive(Clone, Debug)]
pub struct UsingDirectiveStruct {
    pub using_keyword: UsingKeyword,
    pub clause: UsingClause,
    pub for_keyword: ForKeyword,
    pub target: UsingTarget,
    pub global_keyword: Option<GlobalKeyword>,
    pub semicolon: Semicolon,
}

pub fn new_using_directive(
    using_keyword: UsingKeyword,
    clause: UsingClause,
    for_keyword: ForKeyword,
    target: UsingTarget,
    global_keyword: Option<GlobalKeyword>,
    semicolon: Semicolon,
) -> UsingDirective {
    Rc::new(UsingDirectiveStruct {
        using_keyword,
        clause,
        for_keyword,
        target,
        global_keyword,
        semicolon,
    })
}

pub type VariableDeclarationStatement = Rc<VariableDeclarationStatementStruct>;

#[derive(Clone, Debug)]
pub struct VariableDeclarationStatementStruct {
    pub variable_type: VariableDeclarationType,
    pub storage_location: Option<StorageLocation>,
    pub name: Identifier,
    pub value: Option<VariableDeclarationValue>,
    pub semicolon: Semicolon,
}

pub fn new_variable_declaration_statement(
    variable_type: VariableDeclarationType,
    storage_location: Option<StorageLocation>,
    name: Identifier,
    value: Option<VariableDeclarationValue>,
    semicolon: Semicolon,
) -> VariableDeclarationStatement {
    Rc::new(VariableDeclarationStatementStruct {
        variable_type,
        storage_location,
        name,
        value,
        semicolon,
    })
}

pub type VariableDeclarationValue = Rc<VariableDeclarationValueStruct>;

#[derive(Clone, Debug)]
pub struct VariableDeclarationValueStruct {
    pub equal: Equal,
    pub expression: Expression,
}

pub fn new_variable_declaration_value(
    equal: Equal,
    expression: Expression,
) -> VariableDeclarationValue {
    Rc::new(VariableDeclarationValueStruct { equal, expression })
}

pub type VersionPragma = Rc<VersionPragmaStruct>;

#[derive(Clone, Debug)]
pub struct VersionPragmaStruct {
    pub solidity_keyword: SolidityKeyword,
    pub sets: VersionExpressionSets,
}

pub fn new_version_pragma(
    solidity_keyword: SolidityKeyword,
    sets: VersionExpressionSets,
) -> VersionPragma {
    Rc::new(VersionPragmaStruct {
        solidity_keyword,
        sets,
    })
}

pub type VersionRange = Rc<VersionRangeStruct>;

#[derive(Clone, Debug)]
pub struct VersionRangeStruct {
    pub start: VersionLiteral,
    pub minus: Minus,
    pub end: VersionLiteral,
}

pub fn new_version_range(start: VersionLiteral, minus: Minus, end: VersionLiteral) -> VersionRange {
    Rc::new(VersionRangeStruct { start, minus, end })
}

pub type VersionTerm = Rc<VersionTermStruct>;

#[derive(Clone, Debug)]
pub struct VersionTermStruct {
    pub operator: Option<VersionOperator>,
    pub literal: VersionLiteral,
}

pub fn new_version_term(operator: Option<VersionOperator>, literal: VersionLiteral) -> VersionTerm {
    Rc::new(VersionTermStruct { operator, literal })
}

pub type WhileStatement = Rc<WhileStatementStruct>;

#[derive(Clone, Debug)]
pub struct WhileStatementStruct {
    pub while_keyword: WhileKeyword,
    pub open_paren: OpenParen,
    pub condition: Expression,
    pub close_paren: CloseParen,
    pub body: Statement,
}

pub fn new_while_statement(
    while_keyword: WhileKeyword,
    open_paren: OpenParen,
    condition: Expression,
    close_paren: CloseParen,
    body: Statement,
) -> WhileStatement {
    Rc::new(WhileStatementStruct {
        while_keyword,
        open_paren,
        condition,
        close_paren,
        body,
    })
}

pub type YulBlock = Rc<YulBlockStruct>;

#[derive(Clone, Debug)]
pub struct YulBlockStruct {
    pub open_brace: OpenBrace,
    pub statements: YulStatements,
    pub close_brace: CloseBrace,
}

pub fn new_yul_block(
    open_brace: OpenBrace,
    statements: YulStatements,
    close_brace: CloseBrace,
) -> YulBlock {
    Rc::new(YulBlockStruct {
        open_brace,
        statements,
        close_brace,
    })
}

pub type YulBreakStatement = Rc<YulBreakStatementStruct>;

#[derive(Clone, Debug)]
pub struct YulBreakStatementStruct {
    pub break_keyword: YulBreakKeyword,
}

pub fn new_yul_break_statement(break_keyword: YulBreakKeyword) -> YulBreakStatement {
    Rc::new(YulBreakStatementStruct { break_keyword })
}

pub type YulColonAndEqual = Rc<YulColonAndEqualStruct>;

#[derive(Clone, Debug)]
pub struct YulColonAndEqualStruct {
    pub colon: Colon,
    pub equal: Equal,
}

pub fn new_yul_colon_and_equal(colon: Colon, equal: Equal) -> YulColonAndEqual {
    Rc::new(YulColonAndEqualStruct { colon, equal })
}

pub type YulContinueStatement = Rc<YulContinueStatementStruct>;

#[derive(Clone, Debug)]
pub struct YulContinueStatementStruct {
    pub continue_keyword: YulContinueKeyword,
}

pub fn new_yul_continue_statement(continue_keyword: YulContinueKeyword) -> YulContinueStatement {
    Rc::new(YulContinueStatementStruct { continue_keyword })
}

pub type YulDefaultCase = Rc<YulDefaultCaseStruct>;

#[derive(Clone, Debug)]
pub struct YulDefaultCaseStruct {
    pub default_keyword: YulDefaultKeyword,
    pub body: YulBlock,
}

pub fn new_yul_default_case(default_keyword: YulDefaultKeyword, body: YulBlock) -> YulDefaultCase {
    Rc::new(YulDefaultCaseStruct {
        default_keyword,
        body,
    })
}

pub type YulEqualAndColon = Rc<YulEqualAndColonStruct>;

#[derive(Clone, Debug)]
pub struct YulEqualAndColonStruct {
    pub equal: Equal,
    pub colon: Colon,
}

pub fn new_yul_equal_and_colon(equal: Equal, colon: Colon) -> YulEqualAndColon {
    Rc::new(YulEqualAndColonStruct { equal, colon })
}

pub type YulForStatement = Rc<YulForStatementStruct>;

#[derive(Clone, Debug)]
pub struct YulForStatementStruct {
    pub for_keyword: YulForKeyword,
    pub initialization: YulBlock,
    pub condition: YulExpression,
    pub iterator: YulBlock,
    pub body: YulBlock,
}

pub fn new_yul_for_statement(
    for_keyword: YulForKeyword,
    initialization: YulBlock,
    condition: YulExpression,
    iterator: YulBlock,
    body: YulBlock,
) -> YulForStatement {
    Rc::new(YulForStatementStruct {
        for_keyword,
        initialization,
        condition,
        iterator,
        body,
    })
}

pub type YulFunctionCallExpression = Rc<YulFunctionCallExpressionStruct>;

#[derive(Clone, Debug)]
pub struct YulFunctionCallExpressionStruct {
    pub operand: YulExpression,
    pub open_paren: OpenParen,
    pub arguments: YulArguments,
    pub close_paren: CloseParen,
}

pub fn new_yul_function_call_expression(
    operand: YulExpression,
    open_paren: OpenParen,
    arguments: YulArguments,
    close_paren: CloseParen,
) -> YulFunctionCallExpression {
    Rc::new(YulFunctionCallExpressionStruct {
        operand,
        open_paren,
        arguments,
        close_paren,
    })
}

pub type YulFunctionDefinition = Rc<YulFunctionDefinitionStruct>;

#[derive(Clone, Debug)]
pub struct YulFunctionDefinitionStruct {
    pub function_keyword: YulFunctionKeyword,
    pub name: YulIdentifier,
    pub parameters: YulParametersDeclaration,
    pub returns: Option<YulReturnsDeclaration>,
    pub body: YulBlock,
}

pub fn new_yul_function_definition(
    function_keyword: YulFunctionKeyword,
    name: YulIdentifier,
    parameters: YulParametersDeclaration,
    returns: Option<YulReturnsDeclaration>,
    body: YulBlock,
) -> YulFunctionDefinition {
    Rc::new(YulFunctionDefinitionStruct {
        function_keyword,
        name,
        parameters,
        returns,
        body,
    })
}

pub type YulIfStatement = Rc<YulIfStatementStruct>;

#[derive(Clone, Debug)]
pub struct YulIfStatementStruct {
    pub if_keyword: YulIfKeyword,
    pub condition: YulExpression,
    pub body: YulBlock,
}

pub fn new_yul_if_statement(
    if_keyword: YulIfKeyword,
    condition: YulExpression,
    body: YulBlock,
) -> YulIfStatement {
    Rc::new(YulIfStatementStruct {
        if_keyword,
        condition,
        body,
    })
}

pub type YulLabel = Rc<YulLabelStruct>;

#[derive(Clone, Debug)]
pub struct YulLabelStruct {
    pub label: YulIdentifier,
    pub colon: Colon,
}

pub fn new_yul_label(label: YulIdentifier, colon: Colon) -> YulLabel {
    Rc::new(YulLabelStruct { label, colon })
}

pub type YulLeaveStatement = Rc<YulLeaveStatementStruct>;

#[derive(Clone, Debug)]
pub struct YulLeaveStatementStruct {
    pub leave_keyword: YulLeaveKeyword,
}

pub fn new_yul_leave_statement(leave_keyword: YulLeaveKeyword) -> YulLeaveStatement {
    Rc::new(YulLeaveStatementStruct { leave_keyword })
}

pub type YulParametersDeclaration = Rc<YulParametersDeclarationStruct>;

#[derive(Clone, Debug)]
pub struct YulParametersDeclarationStruct {
    pub open_paren: OpenParen,
    pub parameters: YulParameters,
    pub close_paren: CloseParen,
}

pub fn new_yul_parameters_declaration(
    open_paren: OpenParen,
    parameters: YulParameters,
    close_paren: CloseParen,
) -> YulParametersDeclaration {
    Rc::new(YulParametersDeclarationStruct {
        open_paren,
        parameters,
        close_paren,
    })
}

pub type YulReturnsDeclaration = Rc<YulReturnsDeclarationStruct>;

#[derive(Clone, Debug)]
pub struct YulReturnsDeclarationStruct {
    pub minus_greater_than: MinusGreaterThan,
    pub variables: YulVariableNames,
}

pub fn new_yul_returns_declaration(
    minus_greater_than: MinusGreaterThan,
    variables: YulVariableNames,
) -> YulReturnsDeclaration {
    Rc::new(YulReturnsDeclarationStruct {
        minus_greater_than,
        variables,
    })
}

pub type YulStackAssignmentStatement = Rc<YulStackAssignmentStatementStruct>;

#[derive(Clone, Debug)]
pub struct YulStackAssignmentStatementStruct {
    pub assignment: YulStackAssignmentOperator,
    pub variable: YulIdentifier,
}

pub fn new_yul_stack_assignment_statement(
    assignment: YulStackAssignmentOperator,
    variable: YulIdentifier,
) -> YulStackAssignmentStatement {
    Rc::new(YulStackAssignmentStatementStruct {
        assignment,
        variable,
    })
}

pub type YulSwitchStatement = Rc<YulSwitchStatementStruct>;

#[derive(Clone, Debug)]
pub struct YulSwitchStatementStruct {
    pub switch_keyword: YulSwitchKeyword,
    pub expression: YulExpression,
    pub cases: YulSwitchCases,
}

pub fn new_yul_switch_statement(
    switch_keyword: YulSwitchKeyword,
    expression: YulExpression,
    cases: YulSwitchCases,
) -> YulSwitchStatement {
    Rc::new(YulSwitchStatementStruct {
        switch_keyword,
        expression,
        cases,
    })
}

pub type YulValueCase = Rc<YulValueCaseStruct>;

#[derive(Clone, Debug)]
pub struct YulValueCaseStruct {
    pub case_keyword: YulCaseKeyword,
    pub value: YulLiteral,
    pub body: YulBlock,
}

pub fn new_yul_value_case(
    case_keyword: YulCaseKeyword,
    value: YulLiteral,
    body: YulBlock,
) -> YulValueCase {
    Rc::new(YulValueCaseStruct {
        case_keyword,
        value,
        body,
    })
}

pub type YulVariableAssignmentStatement = Rc<YulVariableAssignmentStatementStruct>;

#[derive(Clone, Debug)]
pub struct YulVariableAssignmentStatementStruct {
    pub variables: YulPaths,
    pub assignment: YulAssignmentOperator,
    pub expression: YulExpression,
}

pub fn new_yul_variable_assignment_statement(
    variables: YulPaths,
    assignment: YulAssignmentOperator,
    expression: YulExpression,
) -> YulVariableAssignmentStatement {
    Rc::new(YulVariableAssignmentStatementStruct {
        variables,
        assignment,
        expression,
    })
}

pub type YulVariableDeclarationStatement = Rc<YulVariableDeclarationStatementStruct>;

#[derive(Clone, Debug)]
pub struct YulVariableDeclarationStatementStruct {
    pub let_keyword: YulLetKeyword,
    pub variables: YulVariableNames,
    pub value: Option<YulVariableDeclarationValue>,
}

pub fn new_yul_variable_declaration_statement(
    let_keyword: YulLetKeyword,
    variables: YulVariableNames,
    value: Option<YulVariableDeclarationValue>,
) -> YulVariableDeclarationStatement {
    Rc::new(YulVariableDeclarationStatementStruct {
        let_keyword,
        variables,
        value,
    })
}

pub type YulVariableDeclarationValue = Rc<YulVariableDeclarationValueStruct>;

#[derive(Clone, Debug)]
pub struct YulVariableDeclarationValueStruct {
    pub assignment: YulAssignmentOperator,
    pub expression: YulExpression,
}

pub fn new_yul_variable_declaration_value(
    assignment: YulAssignmentOperator,
    expression: YulExpression,
) -> YulVariableDeclarationValue {
    Rc::new(YulVariableDeclarationValueStruct {
        assignment,
        expression,
    })
}

//
// Choices:
//
// Note: We create a constructor function for each variant

#[derive(Clone, Debug)]
pub enum AbicoderVersion {
    AbicoderV1Keyword(AbicoderV1Keyword),
    AbicoderV2Keyword(AbicoderV2Keyword),
}

pub fn new_abicoder_version_abicoder_v1_keyword(element: AbicoderV1Keyword) -> AbicoderVersion {
    AbicoderVersion::AbicoderV1Keyword(element)
}

pub fn new_abicoder_version_abicoder_v2_keyword(element: AbicoderV2Keyword) -> AbicoderVersion {
    AbicoderVersion::AbicoderV2Keyword(element)
}

#[derive(Clone, Debug)]
pub enum ArgumentsDeclaration {
    PositionalArgumentsDeclaration(PositionalArgumentsDeclaration),
    NamedArgumentsDeclaration(NamedArgumentsDeclaration),
}

pub fn new_arguments_declaration_positional_arguments_declaration(
    element: PositionalArgumentsDeclaration,
) -> ArgumentsDeclaration {
    ArgumentsDeclaration::PositionalArgumentsDeclaration(element)
}

pub fn new_arguments_declaration_named_arguments_declaration(
    element: NamedArgumentsDeclaration,
) -> ArgumentsDeclaration {
    ArgumentsDeclaration::NamedArgumentsDeclaration(element)
}

#[derive(Clone, Debug)]
pub enum ConstructorAttribute {
    ModifierInvocation(ModifierInvocation),
    InternalKeyword(InternalKeyword),
    OverrideKeyword(OverrideKeyword),
    PayableKeyword(PayableKeyword),
    PublicKeyword(PublicKeyword),
    VirtualKeyword(VirtualKeyword),
}

pub fn new_constructor_attribute_modifier_invocation(
    element: ModifierInvocation,
) -> ConstructorAttribute {
    ConstructorAttribute::ModifierInvocation(element)
}

pub fn new_constructor_attribute_internal_keyword(
    element: InternalKeyword,
) -> ConstructorAttribute {
    ConstructorAttribute::InternalKeyword(element)
}

pub fn new_constructor_attribute_override_keyword(
    element: OverrideKeyword,
) -> ConstructorAttribute {
    ConstructorAttribute::OverrideKeyword(element)
}

pub fn new_constructor_attribute_payable_keyword(element: PayableKeyword) -> ConstructorAttribute {
    ConstructorAttribute::PayableKeyword(element)
}

pub fn new_constructor_attribute_public_keyword(element: PublicKeyword) -> ConstructorAttribute {
    ConstructorAttribute::PublicKeyword(element)
}

pub fn new_constructor_attribute_virtual_keyword(element: VirtualKeyword) -> ConstructorAttribute {
    ConstructorAttribute::VirtualKeyword(element)
}

#[derive(Clone, Debug)]
pub enum ContractMember {
    UsingDirective(UsingDirective),
    FunctionDefinition(FunctionDefinition),
    ConstructorDefinition(ConstructorDefinition),
    ReceiveFunctionDefinition(ReceiveFunctionDefinition),
    FallbackFunctionDefinition(FallbackFunctionDefinition),
    UnnamedFunctionDefinition(UnnamedFunctionDefinition),
    ModifierDefinition(ModifierDefinition),
    StructDefinition(StructDefinition),
    EnumDefinition(EnumDefinition),
    EventDefinition(EventDefinition),
    ErrorDefinition(ErrorDefinition),
    UserDefinedValueTypeDefinition(UserDefinedValueTypeDefinition),
    StateVariableDefinition(StateVariableDefinition),
}

pub fn new_contract_member_using_directive(element: UsingDirective) -> ContractMember {
    ContractMember::UsingDirective(element)
}

pub fn new_contract_member_function_definition(element: FunctionDefinition) -> ContractMember {
    ContractMember::FunctionDefinition(element)
}

pub fn new_contract_member_constructor_definition(
    element: ConstructorDefinition,
) -> ContractMember {
    ContractMember::ConstructorDefinition(element)
}

pub fn new_contract_member_receive_function_definition(
    element: ReceiveFunctionDefinition,
) -> ContractMember {
    ContractMember::ReceiveFunctionDefinition(element)
}

pub fn new_contract_member_fallback_function_definition(
    element: FallbackFunctionDefinition,
) -> ContractMember {
    ContractMember::FallbackFunctionDefinition(element)
}

pub fn new_contract_member_unnamed_function_definition(
    element: UnnamedFunctionDefinition,
) -> ContractMember {
    ContractMember::UnnamedFunctionDefinition(element)
}

pub fn new_contract_member_modifier_definition(element: ModifierDefinition) -> ContractMember {
    ContractMember::ModifierDefinition(element)
}

pub fn new_contract_member_struct_definition(element: StructDefinition) -> ContractMember {
    ContractMember::StructDefinition(element)
}

pub fn new_contract_member_enum_definition(element: EnumDefinition) -> ContractMember {
    ContractMember::EnumDefinition(element)
}

pub fn new_contract_member_event_definition(element: EventDefinition) -> ContractMember {
    ContractMember::EventDefinition(element)
}

pub fn new_contract_member_error_definition(element: ErrorDefinition) -> ContractMember {
    ContractMember::ErrorDefinition(element)
}

pub fn new_contract_member_user_defined_value_type_definition(
    element: UserDefinedValueTypeDefinition,
) -> ContractMember {
    ContractMember::UserDefinedValueTypeDefinition(element)
}

pub fn new_contract_member_state_variable_definition(
    element: StateVariableDefinition,
) -> ContractMember {
    ContractMember::StateVariableDefinition(element)
}

#[derive(Clone, Debug)]
pub enum ContractSpecifier {
    InheritanceSpecifier(InheritanceSpecifier),
    StorageLayoutSpecifier(StorageLayoutSpecifier),
}

pub fn new_contract_specifier_inheritance_specifier(
    element: InheritanceSpecifier,
) -> ContractSpecifier {
    ContractSpecifier::InheritanceSpecifier(element)
}

pub fn new_contract_specifier_storage_layout_specifier(
    element: StorageLayoutSpecifier,
) -> ContractSpecifier {
    ContractSpecifier::StorageLayoutSpecifier(element)
}

#[derive(Clone, Debug)]
pub enum ElementaryType {
    BoolKeyword(BoolKeyword),
    ByteKeyword(ByteKeyword),
    StringKeyword(StringKeyword),
    AddressType(AddressType),
    BytesKeyword(BytesKeyword),
    IntKeyword(IntKeyword),
    UintKeyword(UintKeyword),
    FixedKeyword(FixedKeyword),
    UfixedKeyword(UfixedKeyword),
}

pub fn new_elementary_type_bool_keyword(element: BoolKeyword) -> ElementaryType {
    ElementaryType::BoolKeyword(element)
}

pub fn new_elementary_type_byte_keyword(element: ByteKeyword) -> ElementaryType {
    ElementaryType::ByteKeyword(element)
}

pub fn new_elementary_type_string_keyword(element: StringKeyword) -> ElementaryType {
    ElementaryType::StringKeyword(element)
}

pub fn new_elementary_type_address_type(element: AddressType) -> ElementaryType {
    ElementaryType::AddressType(element)
}

pub fn new_elementary_type_bytes_keyword(element: BytesKeyword) -> ElementaryType {
    ElementaryType::BytesKeyword(element)
}

pub fn new_elementary_type_int_keyword(element: IntKeyword) -> ElementaryType {
    ElementaryType::IntKeyword(element)
}

pub fn new_elementary_type_uint_keyword(element: UintKeyword) -> ElementaryType {
    ElementaryType::UintKeyword(element)
}

pub fn new_elementary_type_fixed_keyword(element: FixedKeyword) -> ElementaryType {
    ElementaryType::FixedKeyword(element)
}

pub fn new_elementary_type_ufixed_keyword(element: UfixedKeyword) -> ElementaryType {
    ElementaryType::UfixedKeyword(element)
}

#[derive(Clone, Debug)]
pub enum ExperimentalFeature {
    ABIEncoderV2Keyword(ABIEncoderV2Keyword),
    SMTCheckerKeyword(SMTCheckerKeyword),
    StringLiteral(StringLiteral),
}

pub fn new_experimental_feature_abi_encoder_v2_keyword(
    element: ABIEncoderV2Keyword,
) -> ExperimentalFeature {
    ExperimentalFeature::ABIEncoderV2Keyword(element)
}

pub fn new_experimental_feature_smt_checker_keyword(
    element: SMTCheckerKeyword,
) -> ExperimentalFeature {
    ExperimentalFeature::SMTCheckerKeyword(element)
}

pub fn new_experimental_feature_string_literal(element: StringLiteral) -> ExperimentalFeature {
    ExperimentalFeature::StringLiteral(element)
}

#[derive(Clone, Debug)]
pub enum Expression {
    AssignmentExpression(AssignmentExpression),
    ConditionalExpression(ConditionalExpression),
    OrExpression(OrExpression),
    AndExpression(AndExpression),
    EqualityExpression(EqualityExpression),
    InequalityExpression(InequalityExpression),
    BitwiseOrExpression(BitwiseOrExpression),
    BitwiseXorExpression(BitwiseXorExpression),
    BitwiseAndExpression(BitwiseAndExpression),
    ShiftExpression(ShiftExpression),
    AdditiveExpression(AdditiveExpression),
    MultiplicativeExpression(MultiplicativeExpression),
    ExponentiationExpression(ExponentiationExpression),
    PostfixExpression(PostfixExpression),
    PrefixExpression(PrefixExpression),
    FunctionCallExpression(FunctionCallExpression),
    CallOptionsExpression(CallOptionsExpression),
    MemberAccessExpression(MemberAccessExpression),
    IndexAccessExpression(IndexAccessExpression),
    NewExpression(NewExpression),
    TupleExpression(TupleExpression),
    TypeExpression(TypeExpression),
    ArrayExpression(ArrayExpression),
    HexNumberExpression(HexNumberExpression),
    DecimalNumberExpression(DecimalNumberExpression),
    StringExpression(StringExpression),
    ElementaryType(ElementaryType),
    PayableKeyword(PayableKeyword),
    ThisKeyword(ThisKeyword),
    SuperKeyword(SuperKeyword),
    TrueKeyword(TrueKeyword),
    FalseKeyword(FalseKeyword),
    Identifier(Identifier),
}

pub fn new_expression_assignment_expression(element: AssignmentExpression) -> Expression {
    Expression::AssignmentExpression(element)
}

pub fn new_expression_conditional_expression(element: ConditionalExpression) -> Expression {
    Expression::ConditionalExpression(element)
}

pub fn new_expression_or_expression(element: OrExpression) -> Expression {
    Expression::OrExpression(element)
}

pub fn new_expression_and_expression(element: AndExpression) -> Expression {
    Expression::AndExpression(element)
}

pub fn new_expression_equality_expression(element: EqualityExpression) -> Expression {
    Expression::EqualityExpression(element)
}

pub fn new_expression_inequality_expression(element: InequalityExpression) -> Expression {
    Expression::InequalityExpression(element)
}

pub fn new_expression_bitwise_or_expression(element: BitwiseOrExpression) -> Expression {
    Expression::BitwiseOrExpression(element)
}

pub fn new_expression_bitwise_xor_expression(element: BitwiseXorExpression) -> Expression {
    Expression::BitwiseXorExpression(element)
}

pub fn new_expression_bitwise_and_expression(element: BitwiseAndExpression) -> Expression {
    Expression::BitwiseAndExpression(element)
}

pub fn new_expression_shift_expression(element: ShiftExpression) -> Expression {
    Expression::ShiftExpression(element)
}

pub fn new_expression_additive_expression(element: AdditiveExpression) -> Expression {
    Expression::AdditiveExpression(element)
}

pub fn new_expression_multiplicative_expression(element: MultiplicativeExpression) -> Expression {
    Expression::MultiplicativeExpression(element)
}

pub fn new_expression_exponentiation_expression(element: ExponentiationExpression) -> Expression {
    Expression::ExponentiationExpression(element)
}

pub fn new_expression_postfix_expression(element: PostfixExpression) -> Expression {
    Expression::PostfixExpression(element)
}

pub fn new_expression_prefix_expression(element: PrefixExpression) -> Expression {
    Expression::PrefixExpression(element)
}

pub fn new_expression_function_call_expression(element: FunctionCallExpression) -> Expression {
    Expression::FunctionCallExpression(element)
}

pub fn new_expression_call_options_expression(element: CallOptionsExpression) -> Expression {
    Expression::CallOptionsExpression(element)
}

pub fn new_expression_member_access_expression(element: MemberAccessExpression) -> Expression {
    Expression::MemberAccessExpression(element)
}

pub fn new_expression_index_access_expression(element: IndexAccessExpression) -> Expression {
    Expression::IndexAccessExpression(element)
}

pub fn new_expression_new_expression(element: NewExpression) -> Expression {
    Expression::NewExpression(element)
}

pub fn new_expression_tuple_expression(element: TupleExpression) -> Expression {
    Expression::TupleExpression(element)
}

pub fn new_expression_type_expression(element: TypeExpression) -> Expression {
    Expression::TypeExpression(element)
}

pub fn new_expression_array_expression(element: ArrayExpression) -> Expression {
    Expression::ArrayExpression(element)
}

pub fn new_expression_hex_number_expression(element: HexNumberExpression) -> Expression {
    Expression::HexNumberExpression(element)
}

pub fn new_expression_decimal_number_expression(element: DecimalNumberExpression) -> Expression {
    Expression::DecimalNumberExpression(element)
}

pub fn new_expression_string_expression(element: StringExpression) -> Expression {
    Expression::StringExpression(element)
}

pub fn new_expression_elementary_type(element: ElementaryType) -> Expression {
    Expression::ElementaryType(element)
}

pub fn new_expression_payable_keyword(element: PayableKeyword) -> Expression {
    Expression::PayableKeyword(element)
}

pub fn new_expression_this_keyword(element: ThisKeyword) -> Expression {
    Expression::ThisKeyword(element)
}

pub fn new_expression_super_keyword(element: SuperKeyword) -> Expression {
    Expression::SuperKeyword(element)
}

pub fn new_expression_true_keyword(element: TrueKeyword) -> Expression {
    Expression::TrueKeyword(element)
}

pub fn new_expression_false_keyword(element: FalseKeyword) -> Expression {
    Expression::FalseKeyword(element)
}

pub fn new_expression_identifier(element: Identifier) -> Expression {
    Expression::Identifier(element)
}

#[derive(Clone, Debug)]
pub enum Expression_AdditiveExpression_Operator {
    Minus(Minus),
    Plus(Plus),
}

pub fn new_expression_additive_expression_operator_minus(
    element: Minus,
) -> Expression_AdditiveExpression_Operator {
    Expression_AdditiveExpression_Operator::Minus(element)
}

pub fn new_expression_additive_expression_operator_plus(
    element: Plus,
) -> Expression_AdditiveExpression_Operator {
    Expression_AdditiveExpression_Operator::Plus(element)
}

#[derive(Clone, Debug)]
pub enum Expression_AssignmentExpression_Operator {
    AmpersandEqual(AmpersandEqual),
    AsteriskEqual(AsteriskEqual),
    BarEqual(BarEqual),
    CaretEqual(CaretEqual),
    Equal(Equal),
    GreaterThanGreaterThanEqual(GreaterThanGreaterThanEqual),
    GreaterThanGreaterThanGreaterThanEqual(GreaterThanGreaterThanGreaterThanEqual),
    LessThanLessThanEqual(LessThanLessThanEqual),
    MinusEqual(MinusEqual),
    PercentEqual(PercentEqual),
    PlusEqual(PlusEqual),
    SlashEqual(SlashEqual),
}

pub fn new_expression_assignment_expression_operator_ampersand_equal(
    element: AmpersandEqual,
) -> Expression_AssignmentExpression_Operator {
    Expression_AssignmentExpression_Operator::AmpersandEqual(element)
}

pub fn new_expression_assignment_expression_operator_asterisk_equal(
    element: AsteriskEqual,
) -> Expression_AssignmentExpression_Operator {
    Expression_AssignmentExpression_Operator::AsteriskEqual(element)
}

pub fn new_expression_assignment_expression_operator_bar_equal(
    element: BarEqual,
) -> Expression_AssignmentExpression_Operator {
    Expression_AssignmentExpression_Operator::BarEqual(element)
}

pub fn new_expression_assignment_expression_operator_caret_equal(
    element: CaretEqual,
) -> Expression_AssignmentExpression_Operator {
    Expression_AssignmentExpression_Operator::CaretEqual(element)
}

pub fn new_expression_assignment_expression_operator_equal(
    element: Equal,
) -> Expression_AssignmentExpression_Operator {
    Expression_AssignmentExpression_Operator::Equal(element)
}

pub fn new_expression_assignment_expression_operator_greater_than_greater_than_equal(
    element: GreaterThanGreaterThanEqual,
) -> Expression_AssignmentExpression_Operator {
    Expression_AssignmentExpression_Operator::GreaterThanGreaterThanEqual(element)
}

pub fn new_expression_assignment_expression_operator_greater_than_greater_than_greater_than_equal(
    element: GreaterThanGreaterThanGreaterThanEqual,
) -> Expression_AssignmentExpression_Operator {
    Expression_AssignmentExpression_Operator::GreaterThanGreaterThanGreaterThanEqual(element)
}

pub fn new_expression_assignment_expression_operator_less_than_less_than_equal(
    element: LessThanLessThanEqual,
) -> Expression_AssignmentExpression_Operator {
    Expression_AssignmentExpression_Operator::LessThanLessThanEqual(element)
}

pub fn new_expression_assignment_expression_operator_minus_equal(
    element: MinusEqual,
) -> Expression_AssignmentExpression_Operator {
    Expression_AssignmentExpression_Operator::MinusEqual(element)
}

pub fn new_expression_assignment_expression_operator_percent_equal(
    element: PercentEqual,
) -> Expression_AssignmentExpression_Operator {
    Expression_AssignmentExpression_Operator::PercentEqual(element)
}

pub fn new_expression_assignment_expression_operator_plus_equal(
    element: PlusEqual,
) -> Expression_AssignmentExpression_Operator {
    Expression_AssignmentExpression_Operator::PlusEqual(element)
}

pub fn new_expression_assignment_expression_operator_slash_equal(
    element: SlashEqual,
) -> Expression_AssignmentExpression_Operator {
    Expression_AssignmentExpression_Operator::SlashEqual(element)
}

#[derive(Clone, Debug)]
pub enum Expression_EqualityExpression_Operator {
    BangEqual(BangEqual),
    EqualEqual(EqualEqual),
}

pub fn new_expression_equality_expression_operator_bang_equal(
    element: BangEqual,
) -> Expression_EqualityExpression_Operator {
    Expression_EqualityExpression_Operator::BangEqual(element)
}

pub fn new_expression_equality_expression_operator_equal_equal(
    element: EqualEqual,
) -> Expression_EqualityExpression_Operator {
    Expression_EqualityExpression_Operator::EqualEqual(element)
}

#[derive(Clone, Debug)]
pub enum Expression_ExponentiationExpression_Operator {
    AsteriskAsterisk(AsteriskAsterisk),
}

pub fn new_expression_exponentiation_expression_operator_asterisk_asterisk(
    element: AsteriskAsterisk,
) -> Expression_ExponentiationExpression_Operator {
    Expression_ExponentiationExpression_Operator::AsteriskAsterisk(element)
}

#[derive(Clone, Debug)]
pub enum Expression_InequalityExpression_Operator {
    GreaterThan(GreaterThan),
    GreaterThanEqual(GreaterThanEqual),
    LessThan(LessThan),
    LessThanEqual(LessThanEqual),
}

pub fn new_expression_inequality_expression_operator_greater_than(
    element: GreaterThan,
) -> Expression_InequalityExpression_Operator {
    Expression_InequalityExpression_Operator::GreaterThan(element)
}

pub fn new_expression_inequality_expression_operator_greater_than_equal(
    element: GreaterThanEqual,
) -> Expression_InequalityExpression_Operator {
    Expression_InequalityExpression_Operator::GreaterThanEqual(element)
}

pub fn new_expression_inequality_expression_operator_less_than(
    element: LessThan,
) -> Expression_InequalityExpression_Operator {
    Expression_InequalityExpression_Operator::LessThan(element)
}

pub fn new_expression_inequality_expression_operator_less_than_equal(
    element: LessThanEqual,
) -> Expression_InequalityExpression_Operator {
    Expression_InequalityExpression_Operator::LessThanEqual(element)
}

#[derive(Clone, Debug)]
pub enum Expression_MultiplicativeExpression_Operator {
    Asterisk(Asterisk),
    Percent(Percent),
    Slash(Slash),
}

pub fn new_expression_multiplicative_expression_operator_asterisk(
    element: Asterisk,
) -> Expression_MultiplicativeExpression_Operator {
    Expression_MultiplicativeExpression_Operator::Asterisk(element)
}

pub fn new_expression_multiplicative_expression_operator_percent(
    element: Percent,
) -> Expression_MultiplicativeExpression_Operator {
    Expression_MultiplicativeExpression_Operator::Percent(element)
}

pub fn new_expression_multiplicative_expression_operator_slash(
    element: Slash,
) -> Expression_MultiplicativeExpression_Operator {
    Expression_MultiplicativeExpression_Operator::Slash(element)
}

#[derive(Clone, Debug)]
pub enum Expression_PostfixExpression_Operator {
    MinusMinus(MinusMinus),
    PlusPlus(PlusPlus),
}

pub fn new_expression_postfix_expression_operator_minus_minus(
    element: MinusMinus,
) -> Expression_PostfixExpression_Operator {
    Expression_PostfixExpression_Operator::MinusMinus(element)
}

pub fn new_expression_postfix_expression_operator_plus_plus(
    element: PlusPlus,
) -> Expression_PostfixExpression_Operator {
    Expression_PostfixExpression_Operator::PlusPlus(element)
}

#[derive(Clone, Debug)]
pub enum Expression_PrefixExpression_Operator {
    Bang(Bang),
    DeleteKeyword(DeleteKeyword),
    Minus(Minus),
    MinusMinus(MinusMinus),
    Plus(Plus),
    PlusPlus(PlusPlus),
    Tilde(Tilde),
}

pub fn new_expression_prefix_expression_operator_bang(
    element: Bang,
) -> Expression_PrefixExpression_Operator {
    Expression_PrefixExpression_Operator::Bang(element)
}

pub fn new_expression_prefix_expression_operator_delete_keyword(
    element: DeleteKeyword,
) -> Expression_PrefixExpression_Operator {
    Expression_PrefixExpression_Operator::DeleteKeyword(element)
}

pub fn new_expression_prefix_expression_operator_minus(
    element: Minus,
) -> Expression_PrefixExpression_Operator {
    Expression_PrefixExpression_Operator::Minus(element)
}

pub fn new_expression_prefix_expression_operator_minus_minus(
    element: MinusMinus,
) -> Expression_PrefixExpression_Operator {
    Expression_PrefixExpression_Operator::MinusMinus(element)
}

pub fn new_expression_prefix_expression_operator_plus(
    element: Plus,
) -> Expression_PrefixExpression_Operator {
    Expression_PrefixExpression_Operator::Plus(element)
}

pub fn new_expression_prefix_expression_operator_plus_plus(
    element: PlusPlus,
) -> Expression_PrefixExpression_Operator {
    Expression_PrefixExpression_Operator::PlusPlus(element)
}

pub fn new_expression_prefix_expression_operator_tilde(
    element: Tilde,
) -> Expression_PrefixExpression_Operator {
    Expression_PrefixExpression_Operator::Tilde(element)
}

#[derive(Clone, Debug)]
pub enum Expression_ShiftExpression_Operator {
    GreaterThanGreaterThan(GreaterThanGreaterThan),
    GreaterThanGreaterThanGreaterThan(GreaterThanGreaterThanGreaterThan),
    LessThanLessThan(LessThanLessThan),
}

pub fn new_expression_shift_expression_operator_greater_than_greater_than(
    element: GreaterThanGreaterThan,
) -> Expression_ShiftExpression_Operator {
    Expression_ShiftExpression_Operator::GreaterThanGreaterThan(element)
}

pub fn new_expression_shift_expression_operator_greater_than_greater_than_greater_than(
    element: GreaterThanGreaterThanGreaterThan,
) -> Expression_ShiftExpression_Operator {
    Expression_ShiftExpression_Operator::GreaterThanGreaterThanGreaterThan(element)
}

pub fn new_expression_shift_expression_operator_less_than_less_than(
    element: LessThanLessThan,
) -> Expression_ShiftExpression_Operator {
    Expression_ShiftExpression_Operator::LessThanLessThan(element)
}

#[derive(Clone, Debug)]
pub enum FallbackFunctionAttribute {
    ModifierInvocation(ModifierInvocation),
    OverrideSpecifier(OverrideSpecifier),
    ExternalKeyword(ExternalKeyword),
    PayableKeyword(PayableKeyword),
    PureKeyword(PureKeyword),
    ViewKeyword(ViewKeyword),
    VirtualKeyword(VirtualKeyword),
}

pub fn new_fallback_function_attribute_modifier_invocation(
    element: ModifierInvocation,
) -> FallbackFunctionAttribute {
    FallbackFunctionAttribute::ModifierInvocation(element)
}

pub fn new_fallback_function_attribute_override_specifier(
    element: OverrideSpecifier,
) -> FallbackFunctionAttribute {
    FallbackFunctionAttribute::OverrideSpecifier(element)
}

pub fn new_fallback_function_attribute_external_keyword(
    element: ExternalKeyword,
) -> FallbackFunctionAttribute {
    FallbackFunctionAttribute::ExternalKeyword(element)
}

pub fn new_fallback_function_attribute_payable_keyword(
    element: PayableKeyword,
) -> FallbackFunctionAttribute {
    FallbackFunctionAttribute::PayableKeyword(element)
}

pub fn new_fallback_function_attribute_pure_keyword(
    element: PureKeyword,
) -> FallbackFunctionAttribute {
    FallbackFunctionAttribute::PureKeyword(element)
}

pub fn new_fallback_function_attribute_view_keyword(
    element: ViewKeyword,
) -> FallbackFunctionAttribute {
    FallbackFunctionAttribute::ViewKeyword(element)
}

pub fn new_fallback_function_attribute_virtual_keyword(
    element: VirtualKeyword,
) -> FallbackFunctionAttribute {
    FallbackFunctionAttribute::VirtualKeyword(element)
}

#[derive(Clone, Debug)]
pub enum ForStatementCondition {
    ExpressionStatement(ExpressionStatement),
    Semicolon(Semicolon),
}

pub fn new_for_statement_condition_expression_statement(
    element: ExpressionStatement,
) -> ForStatementCondition {
    ForStatementCondition::ExpressionStatement(element)
}

pub fn new_for_statement_condition_semicolon(element: Semicolon) -> ForStatementCondition {
    ForStatementCondition::Semicolon(element)
}

#[derive(Clone, Debug)]
pub enum ForStatementInitialization {
    TupleDeconstructionStatement(TupleDeconstructionStatement),
    VariableDeclarationStatement(VariableDeclarationStatement),
    ExpressionStatement(ExpressionStatement),
    Semicolon(Semicolon),
}

pub fn new_for_statement_initialization_tuple_deconstruction_statement(
    element: TupleDeconstructionStatement,
) -> ForStatementInitialization {
    ForStatementInitialization::TupleDeconstructionStatement(element)
}

pub fn new_for_statement_initialization_variable_declaration_statement(
    element: VariableDeclarationStatement,
) -> ForStatementInitialization {
    ForStatementInitialization::VariableDeclarationStatement(element)
}

pub fn new_for_statement_initialization_expression_statement(
    element: ExpressionStatement,
) -> ForStatementInitialization {
    ForStatementInitialization::ExpressionStatement(element)
}

pub fn new_for_statement_initialization_semicolon(
    element: Semicolon,
) -> ForStatementInitialization {
    ForStatementInitialization::Semicolon(element)
}

#[derive(Clone, Debug)]
pub enum FunctionAttribute {
    ModifierInvocation(ModifierInvocation),
    OverrideSpecifier(OverrideSpecifier),
    ConstantKeyword(ConstantKeyword),
    ExternalKeyword(ExternalKeyword),
    InternalKeyword(InternalKeyword),
    PayableKeyword(PayableKeyword),
    PrivateKeyword(PrivateKeyword),
    PublicKeyword(PublicKeyword),
    PureKeyword(PureKeyword),
    ViewKeyword(ViewKeyword),
    VirtualKeyword(VirtualKeyword),
}

pub fn new_function_attribute_modifier_invocation(
    element: ModifierInvocation,
) -> FunctionAttribute {
    FunctionAttribute::ModifierInvocation(element)
}

pub fn new_function_attribute_override_specifier(element: OverrideSpecifier) -> FunctionAttribute {
    FunctionAttribute::OverrideSpecifier(element)
}

pub fn new_function_attribute_constant_keyword(element: ConstantKeyword) -> FunctionAttribute {
    FunctionAttribute::ConstantKeyword(element)
}

pub fn new_function_attribute_external_keyword(element: ExternalKeyword) -> FunctionAttribute {
    FunctionAttribute::ExternalKeyword(element)
}

pub fn new_function_attribute_internal_keyword(element: InternalKeyword) -> FunctionAttribute {
    FunctionAttribute::InternalKeyword(element)
}

pub fn new_function_attribute_payable_keyword(element: PayableKeyword) -> FunctionAttribute {
    FunctionAttribute::PayableKeyword(element)
}

pub fn new_function_attribute_private_keyword(element: PrivateKeyword) -> FunctionAttribute {
    FunctionAttribute::PrivateKeyword(element)
}

pub fn new_function_attribute_public_keyword(element: PublicKeyword) -> FunctionAttribute {
    FunctionAttribute::PublicKeyword(element)
}

pub fn new_function_attribute_pure_keyword(element: PureKeyword) -> FunctionAttribute {
    FunctionAttribute::PureKeyword(element)
}

pub fn new_function_attribute_view_keyword(element: ViewKeyword) -> FunctionAttribute {
    FunctionAttribute::ViewKeyword(element)
}

pub fn new_function_attribute_virtual_keyword(element: VirtualKeyword) -> FunctionAttribute {
    FunctionAttribute::VirtualKeyword(element)
}

#[derive(Clone, Debug)]
pub enum FunctionBody {
    Block(Block),
    Semicolon(Semicolon),
}

pub fn new_function_body_block(element: Block) -> FunctionBody {
    FunctionBody::Block(element)
}

pub fn new_function_body_semicolon(element: Semicolon) -> FunctionBody {
    FunctionBody::Semicolon(element)
}

#[derive(Clone, Debug)]
pub enum FunctionName {
    Identifier(Identifier),
    FallbackKeyword(FallbackKeyword),
    ReceiveKeyword(ReceiveKeyword),
}

pub fn new_function_name_identifier(element: Identifier) -> FunctionName {
    FunctionName::Identifier(element)
}

pub fn new_function_name_fallback_keyword(element: FallbackKeyword) -> FunctionName {
    FunctionName::FallbackKeyword(element)
}

pub fn new_function_name_receive_keyword(element: ReceiveKeyword) -> FunctionName {
    FunctionName::ReceiveKeyword(element)
}

#[derive(Clone, Debug)]
pub enum FunctionTypeAttribute {
    InternalKeyword(InternalKeyword),
    ExternalKeyword(ExternalKeyword),
    PrivateKeyword(PrivateKeyword),
    PublicKeyword(PublicKeyword),
    ConstantKeyword(ConstantKeyword),
    PureKeyword(PureKeyword),
    ViewKeyword(ViewKeyword),
    PayableKeyword(PayableKeyword),
}

pub fn new_function_type_attribute_internal_keyword(
    element: InternalKeyword,
) -> FunctionTypeAttribute {
    FunctionTypeAttribute::InternalKeyword(element)
}

pub fn new_function_type_attribute_external_keyword(
    element: ExternalKeyword,
) -> FunctionTypeAttribute {
    FunctionTypeAttribute::ExternalKeyword(element)
}

pub fn new_function_type_attribute_private_keyword(
    element: PrivateKeyword,
) -> FunctionTypeAttribute {
    FunctionTypeAttribute::PrivateKeyword(element)
}

pub fn new_function_type_attribute_public_keyword(element: PublicKeyword) -> FunctionTypeAttribute {
    FunctionTypeAttribute::PublicKeyword(element)
}

pub fn new_function_type_attribute_constant_keyword(
    element: ConstantKeyword,
) -> FunctionTypeAttribute {
    FunctionTypeAttribute::ConstantKeyword(element)
}

pub fn new_function_type_attribute_pure_keyword(element: PureKeyword) -> FunctionTypeAttribute {
    FunctionTypeAttribute::PureKeyword(element)
}

pub fn new_function_type_attribute_view_keyword(element: ViewKeyword) -> FunctionTypeAttribute {
    FunctionTypeAttribute::ViewKeyword(element)
}

pub fn new_function_type_attribute_payable_keyword(
    element: PayableKeyword,
) -> FunctionTypeAttribute {
    FunctionTypeAttribute::PayableKeyword(element)
}

#[derive(Clone, Debug)]
pub enum HexStringLiteral {
    SingleQuotedHexStringLiteral(SingleQuotedHexStringLiteral),
    DoubleQuotedHexStringLiteral(DoubleQuotedHexStringLiteral),
}

pub fn new_hex_string_literal_single_quoted_hex_string_literal(
    element: SingleQuotedHexStringLiteral,
) -> HexStringLiteral {
    HexStringLiteral::SingleQuotedHexStringLiteral(element)
}

pub fn new_hex_string_literal_double_quoted_hex_string_literal(
    element: DoubleQuotedHexStringLiteral,
) -> HexStringLiteral {
    HexStringLiteral::DoubleQuotedHexStringLiteral(element)
}

#[derive(Clone, Debug)]
pub enum ImportClause {
    PathImport(PathImport),
    NamedImport(NamedImport),
    ImportDeconstruction(ImportDeconstruction),
}

pub fn new_import_clause_path_import(element: PathImport) -> ImportClause {
    ImportClause::PathImport(element)
}

pub fn new_import_clause_named_import(element: NamedImport) -> ImportClause {
    ImportClause::NamedImport(element)
}

pub fn new_import_clause_import_deconstruction(element: ImportDeconstruction) -> ImportClause {
    ImportClause::ImportDeconstruction(element)
}

#[derive(Clone, Debug)]
pub enum MappingKeyType {
    ElementaryType(ElementaryType),
    IdentifierPath(IdentifierPath),
}

pub fn new_mapping_key_type_elementary_type(element: ElementaryType) -> MappingKeyType {
    MappingKeyType::ElementaryType(element)
}

pub fn new_mapping_key_type_identifier_path(element: IdentifierPath) -> MappingKeyType {
    MappingKeyType::IdentifierPath(element)
}

#[derive(Clone, Debug)]
pub enum ModifierAttribute {
    OverrideSpecifier(OverrideSpecifier),
    VirtualKeyword(VirtualKeyword),
}

pub fn new_modifier_attribute_override_specifier(element: OverrideSpecifier) -> ModifierAttribute {
    ModifierAttribute::OverrideSpecifier(element)
}

pub fn new_modifier_attribute_virtual_keyword(element: VirtualKeyword) -> ModifierAttribute {
    ModifierAttribute::VirtualKeyword(element)
}

#[derive(Clone, Debug)]
pub enum NumberUnit {
    WeiKeyword(WeiKeyword),
    GweiKeyword(GweiKeyword),
    SzaboKeyword(SzaboKeyword),
    FinneyKeyword(FinneyKeyword),
    EtherKeyword(EtherKeyword),
    SecondsKeyword(SecondsKeyword),
    MinutesKeyword(MinutesKeyword),
    HoursKeyword(HoursKeyword),
    DaysKeyword(DaysKeyword),
    WeeksKeyword(WeeksKeyword),
    YearsKeyword(YearsKeyword),
}

pub fn new_number_unit_wei_keyword(element: WeiKeyword) -> NumberUnit {
    NumberUnit::WeiKeyword(element)
}

pub fn new_number_unit_gwei_keyword(element: GweiKeyword) -> NumberUnit {
    NumberUnit::GweiKeyword(element)
}

pub fn new_number_unit_szabo_keyword(element: SzaboKeyword) -> NumberUnit {
    NumberUnit::SzaboKeyword(element)
}

pub fn new_number_unit_finney_keyword(element: FinneyKeyword) -> NumberUnit {
    NumberUnit::FinneyKeyword(element)
}

pub fn new_number_unit_ether_keyword(element: EtherKeyword) -> NumberUnit {
    NumberUnit::EtherKeyword(element)
}

pub fn new_number_unit_seconds_keyword(element: SecondsKeyword) -> NumberUnit {
    NumberUnit::SecondsKeyword(element)
}

pub fn new_number_unit_minutes_keyword(element: MinutesKeyword) -> NumberUnit {
    NumberUnit::MinutesKeyword(element)
}

pub fn new_number_unit_hours_keyword(element: HoursKeyword) -> NumberUnit {
    NumberUnit::HoursKeyword(element)
}

pub fn new_number_unit_days_keyword(element: DaysKeyword) -> NumberUnit {
    NumberUnit::DaysKeyword(element)
}

pub fn new_number_unit_weeks_keyword(element: WeeksKeyword) -> NumberUnit {
    NumberUnit::WeeksKeyword(element)
}

pub fn new_number_unit_years_keyword(element: YearsKeyword) -> NumberUnit {
    NumberUnit::YearsKeyword(element)
}

#[derive(Clone, Debug)]
pub enum Pragma {
    VersionPragma(VersionPragma),
    AbicoderPragma(AbicoderPragma),
    ExperimentalPragma(ExperimentalPragma),
}

pub fn new_pragma_version_pragma(element: VersionPragma) -> Pragma {
    Pragma::VersionPragma(element)
}

pub fn new_pragma_abicoder_pragma(element: AbicoderPragma) -> Pragma {
    Pragma::AbicoderPragma(element)
}

pub fn new_pragma_experimental_pragma(element: ExperimentalPragma) -> Pragma {
    Pragma::ExperimentalPragma(element)
}

#[derive(Clone, Debug)]
pub enum ReceiveFunctionAttribute {
    ModifierInvocation(ModifierInvocation),
    OverrideSpecifier(OverrideSpecifier),
    ExternalKeyword(ExternalKeyword),
    PayableKeyword(PayableKeyword),
    VirtualKeyword(VirtualKeyword),
}

pub fn new_receive_function_attribute_modifier_invocation(
    element: ModifierInvocation,
) -> ReceiveFunctionAttribute {
    ReceiveFunctionAttribute::ModifierInvocation(element)
}

pub fn new_receive_function_attribute_override_specifier(
    element: OverrideSpecifier,
) -> ReceiveFunctionAttribute {
    ReceiveFunctionAttribute::OverrideSpecifier(element)
}

pub fn new_receive_function_attribute_external_keyword(
    element: ExternalKeyword,
) -> ReceiveFunctionAttribute {
    ReceiveFunctionAttribute::ExternalKeyword(element)
}

pub fn new_receive_function_attribute_payable_keyword(
    element: PayableKeyword,
) -> ReceiveFunctionAttribute {
    ReceiveFunctionAttribute::PayableKeyword(element)
}

pub fn new_receive_function_attribute_virtual_keyword(
    element: VirtualKeyword,
) -> ReceiveFunctionAttribute {
    ReceiveFunctionAttribute::VirtualKeyword(element)
}

#[derive(Clone, Debug)]
pub enum SourceUnitMember {
    PragmaDirective(PragmaDirective),
    ImportDirective(ImportDirective),
    ContractDefinition(ContractDefinition),
    InterfaceDefinition(InterfaceDefinition),
    LibraryDefinition(LibraryDefinition),
    StructDefinition(StructDefinition),
    EnumDefinition(EnumDefinition),
    FunctionDefinition(FunctionDefinition),
    ErrorDefinition(ErrorDefinition),
    UserDefinedValueTypeDefinition(UserDefinedValueTypeDefinition),
    UsingDirective(UsingDirective),
    EventDefinition(EventDefinition),
    ConstantDefinition(ConstantDefinition),
}

pub fn new_source_unit_member_pragma_directive(element: PragmaDirective) -> SourceUnitMember {
    SourceUnitMember::PragmaDirective(element)
}

pub fn new_source_unit_member_import_directive(element: ImportDirective) -> SourceUnitMember {
    SourceUnitMember::ImportDirective(element)
}

pub fn new_source_unit_member_contract_definition(element: ContractDefinition) -> SourceUnitMember {
    SourceUnitMember::ContractDefinition(element)
}

pub fn new_source_unit_member_interface_definition(
    element: InterfaceDefinition,
) -> SourceUnitMember {
    SourceUnitMember::InterfaceDefinition(element)
}

pub fn new_source_unit_member_library_definition(element: LibraryDefinition) -> SourceUnitMember {
    SourceUnitMember::LibraryDefinition(element)
}

pub fn new_source_unit_member_struct_definition(element: StructDefinition) -> SourceUnitMember {
    SourceUnitMember::StructDefinition(element)
}

pub fn new_source_unit_member_enum_definition(element: EnumDefinition) -> SourceUnitMember {
    SourceUnitMember::EnumDefinition(element)
}

pub fn new_source_unit_member_function_definition(element: FunctionDefinition) -> SourceUnitMember {
    SourceUnitMember::FunctionDefinition(element)
}

pub fn new_source_unit_member_error_definition(element: ErrorDefinition) -> SourceUnitMember {
    SourceUnitMember::ErrorDefinition(element)
}

pub fn new_source_unit_member_user_defined_value_type_definition(
    element: UserDefinedValueTypeDefinition,
) -> SourceUnitMember {
    SourceUnitMember::UserDefinedValueTypeDefinition(element)
}

pub fn new_source_unit_member_using_directive(element: UsingDirective) -> SourceUnitMember {
    SourceUnitMember::UsingDirective(element)
}

pub fn new_source_unit_member_event_definition(element: EventDefinition) -> SourceUnitMember {
    SourceUnitMember::EventDefinition(element)
}

pub fn new_source_unit_member_constant_definition(element: ConstantDefinition) -> SourceUnitMember {
    SourceUnitMember::ConstantDefinition(element)
}

#[derive(Clone, Debug)]
pub enum StateVariableAttribute {
    OverrideSpecifier(OverrideSpecifier),
    ConstantKeyword(ConstantKeyword),
    InternalKeyword(InternalKeyword),
    PrivateKeyword(PrivateKeyword),
    PublicKeyword(PublicKeyword),
    ImmutableKeyword(ImmutableKeyword),
    TransientKeyword(TransientKeyword),
}

pub fn new_state_variable_attribute_override_specifier(
    element: OverrideSpecifier,
) -> StateVariableAttribute {
    StateVariableAttribute::OverrideSpecifier(element)
}

pub fn new_state_variable_attribute_constant_keyword(
    element: ConstantKeyword,
) -> StateVariableAttribute {
    StateVariableAttribute::ConstantKeyword(element)
}

pub fn new_state_variable_attribute_internal_keyword(
    element: InternalKeyword,
) -> StateVariableAttribute {
    StateVariableAttribute::InternalKeyword(element)
}

pub fn new_state_variable_attribute_private_keyword(
    element: PrivateKeyword,
) -> StateVariableAttribute {
    StateVariableAttribute::PrivateKeyword(element)
}

pub fn new_state_variable_attribute_public_keyword(
    element: PublicKeyword,
) -> StateVariableAttribute {
    StateVariableAttribute::PublicKeyword(element)
}

pub fn new_state_variable_attribute_immutable_keyword(
    element: ImmutableKeyword,
) -> StateVariableAttribute {
    StateVariableAttribute::ImmutableKeyword(element)
}

pub fn new_state_variable_attribute_transient_keyword(
    element: TransientKeyword,
) -> StateVariableAttribute {
    StateVariableAttribute::TransientKeyword(element)
}

#[derive(Clone, Debug)]
pub enum Statement {
    IfStatement(IfStatement),
    ForStatement(ForStatement),
    WhileStatement(WhileStatement),
    DoWhileStatement(DoWhileStatement),
    ContinueStatement(ContinueStatement),
    BreakStatement(BreakStatement),
    ReturnStatement(ReturnStatement),
    ThrowStatement(ThrowStatement),
    EmitStatement(EmitStatement),
    TryStatement(TryStatement),
    RevertStatement(RevertStatement),
    AssemblyStatement(AssemblyStatement),
    Block(Block),
    UncheckedBlock(UncheckedBlock),
    TupleDeconstructionStatement(TupleDeconstructionStatement),
    VariableDeclarationStatement(VariableDeclarationStatement),
    ExpressionStatement(ExpressionStatement),
}

pub fn new_statement_if_statement(element: IfStatement) -> Statement {
    Statement::IfStatement(element)
}

pub fn new_statement_for_statement(element: ForStatement) -> Statement {
    Statement::ForStatement(element)
}

pub fn new_statement_while_statement(element: WhileStatement) -> Statement {
    Statement::WhileStatement(element)
}

pub fn new_statement_do_while_statement(element: DoWhileStatement) -> Statement {
    Statement::DoWhileStatement(element)
}

pub fn new_statement_continue_statement(element: ContinueStatement) -> Statement {
    Statement::ContinueStatement(element)
}

pub fn new_statement_break_statement(element: BreakStatement) -> Statement {
    Statement::BreakStatement(element)
}

pub fn new_statement_return_statement(element: ReturnStatement) -> Statement {
    Statement::ReturnStatement(element)
}

pub fn new_statement_throw_statement(element: ThrowStatement) -> Statement {
    Statement::ThrowStatement(element)
}

pub fn new_statement_emit_statement(element: EmitStatement) -> Statement {
    Statement::EmitStatement(element)
}

pub fn new_statement_try_statement(element: TryStatement) -> Statement {
    Statement::TryStatement(element)
}

pub fn new_statement_revert_statement(element: RevertStatement) -> Statement {
    Statement::RevertStatement(element)
}

pub fn new_statement_assembly_statement(element: AssemblyStatement) -> Statement {
    Statement::AssemblyStatement(element)
}

pub fn new_statement_block(element: Block) -> Statement {
    Statement::Block(element)
}

pub fn new_statement_unchecked_block(element: UncheckedBlock) -> Statement {
    Statement::UncheckedBlock(element)
}

pub fn new_statement_tuple_deconstruction_statement(
    element: TupleDeconstructionStatement,
) -> Statement {
    Statement::TupleDeconstructionStatement(element)
}

pub fn new_statement_variable_declaration_statement(
    element: VariableDeclarationStatement,
) -> Statement {
    Statement::VariableDeclarationStatement(element)
}

pub fn new_statement_expression_statement(element: ExpressionStatement) -> Statement {
    Statement::ExpressionStatement(element)
}

#[derive(Clone, Debug)]
pub enum StorageLocation {
    MemoryKeyword(MemoryKeyword),
    StorageKeyword(StorageKeyword),
    CallDataKeyword(CallDataKeyword),
}

pub fn new_storage_location_memory_keyword(element: MemoryKeyword) -> StorageLocation {
    StorageLocation::MemoryKeyword(element)
}

pub fn new_storage_location_storage_keyword(element: StorageKeyword) -> StorageLocation {
    StorageLocation::StorageKeyword(element)
}

pub fn new_storage_location_call_data_keyword(element: CallDataKeyword) -> StorageLocation {
    StorageLocation::CallDataKeyword(element)
}

#[derive(Clone, Debug)]
pub enum StringExpression {
    StringLiteral(StringLiteral),
    StringLiterals(StringLiterals),
    HexStringLiteral(HexStringLiteral),
    HexStringLiterals(HexStringLiterals),
    UnicodeStringLiterals(UnicodeStringLiterals),
}

pub fn new_string_expression_string_literal(element: StringLiteral) -> StringExpression {
    StringExpression::StringLiteral(element)
}

pub fn new_string_expression_string_literals(element: StringLiterals) -> StringExpression {
    StringExpression::StringLiterals(element)
}

pub fn new_string_expression_hex_string_literal(element: HexStringLiteral) -> StringExpression {
    StringExpression::HexStringLiteral(element)
}

pub fn new_string_expression_hex_string_literals(element: HexStringLiterals) -> StringExpression {
    StringExpression::HexStringLiterals(element)
}

pub fn new_string_expression_unicode_string_literals(
    element: UnicodeStringLiterals,
) -> StringExpression {
    StringExpression::UnicodeStringLiterals(element)
}

#[derive(Clone, Debug)]
pub enum StringLiteral {
    SingleQuotedStringLiteral(SingleQuotedStringLiteral),
    DoubleQuotedStringLiteral(DoubleQuotedStringLiteral),
}

pub fn new_string_literal_single_quoted_string_literal(
    element: SingleQuotedStringLiteral,
) -> StringLiteral {
    StringLiteral::SingleQuotedStringLiteral(element)
}

pub fn new_string_literal_double_quoted_string_literal(
    element: DoubleQuotedStringLiteral,
) -> StringLiteral {
    StringLiteral::DoubleQuotedStringLiteral(element)
}

#[derive(Clone, Debug)]
pub enum TupleMember {
    TypedTupleMember(TypedTupleMember),
    UntypedTupleMember(UntypedTupleMember),
}

pub fn new_tuple_member_typed_tuple_member(element: TypedTupleMember) -> TupleMember {
    TupleMember::TypedTupleMember(element)
}

pub fn new_tuple_member_untyped_tuple_member(element: UntypedTupleMember) -> TupleMember {
    TupleMember::UntypedTupleMember(element)
}

#[derive(Clone, Debug)]
pub enum TypeName {
    ArrayTypeName(ArrayTypeName),
    FunctionType(FunctionType),
    MappingType(MappingType),
    ElementaryType(ElementaryType),
    IdentifierPath(IdentifierPath),
}

pub fn new_type_name_array_type_name(element: ArrayTypeName) -> TypeName {
    TypeName::ArrayTypeName(element)
}

pub fn new_type_name_function_type(element: FunctionType) -> TypeName {
    TypeName::FunctionType(element)
}

pub fn new_type_name_mapping_type(element: MappingType) -> TypeName {
    TypeName::MappingType(element)
}

pub fn new_type_name_elementary_type(element: ElementaryType) -> TypeName {
    TypeName::ElementaryType(element)
}

pub fn new_type_name_identifier_path(element: IdentifierPath) -> TypeName {
    TypeName::IdentifierPath(element)
}

#[derive(Clone, Debug)]
pub enum UnicodeStringLiteral {
    SingleQuotedUnicodeStringLiteral(SingleQuotedUnicodeStringLiteral),
    DoubleQuotedUnicodeStringLiteral(DoubleQuotedUnicodeStringLiteral),
}

pub fn new_unicode_string_literal_single_quoted_unicode_string_literal(
    element: SingleQuotedUnicodeStringLiteral,
) -> UnicodeStringLiteral {
    UnicodeStringLiteral::SingleQuotedUnicodeStringLiteral(element)
}

pub fn new_unicode_string_literal_double_quoted_unicode_string_literal(
    element: DoubleQuotedUnicodeStringLiteral,
) -> UnicodeStringLiteral {
    UnicodeStringLiteral::DoubleQuotedUnicodeStringLiteral(element)
}

#[derive(Clone, Debug)]
pub enum UnnamedFunctionAttribute {
    ModifierInvocation(ModifierInvocation),
    ConstantKeyword(ConstantKeyword),
    ExternalKeyword(ExternalKeyword),
    InternalKeyword(InternalKeyword),
    PayableKeyword(PayableKeyword),
    PrivateKeyword(PrivateKeyword),
    PublicKeyword(PublicKeyword),
    PureKeyword(PureKeyword),
    ViewKeyword(ViewKeyword),
}

pub fn new_unnamed_function_attribute_modifier_invocation(
    element: ModifierInvocation,
) -> UnnamedFunctionAttribute {
    UnnamedFunctionAttribute::ModifierInvocation(element)
}

pub fn new_unnamed_function_attribute_constant_keyword(
    element: ConstantKeyword,
) -> UnnamedFunctionAttribute {
    UnnamedFunctionAttribute::ConstantKeyword(element)
}

pub fn new_unnamed_function_attribute_external_keyword(
    element: ExternalKeyword,
) -> UnnamedFunctionAttribute {
    UnnamedFunctionAttribute::ExternalKeyword(element)
}

pub fn new_unnamed_function_attribute_internal_keyword(
    element: InternalKeyword,
) -> UnnamedFunctionAttribute {
    UnnamedFunctionAttribute::InternalKeyword(element)
}

pub fn new_unnamed_function_attribute_payable_keyword(
    element: PayableKeyword,
) -> UnnamedFunctionAttribute {
    UnnamedFunctionAttribute::PayableKeyword(element)
}

pub fn new_unnamed_function_attribute_private_keyword(
    element: PrivateKeyword,
) -> UnnamedFunctionAttribute {
    UnnamedFunctionAttribute::PrivateKeyword(element)
}

pub fn new_unnamed_function_attribute_public_keyword(
    element: PublicKeyword,
) -> UnnamedFunctionAttribute {
    UnnamedFunctionAttribute::PublicKeyword(element)
}

pub fn new_unnamed_function_attribute_pure_keyword(
    element: PureKeyword,
) -> UnnamedFunctionAttribute {
    UnnamedFunctionAttribute::PureKeyword(element)
}

pub fn new_unnamed_function_attribute_view_keyword(
    element: ViewKeyword,
) -> UnnamedFunctionAttribute {
    UnnamedFunctionAttribute::ViewKeyword(element)
}

#[derive(Clone, Debug)]
pub enum UsingClause {
    IdentifierPath(IdentifierPath),
    UsingDeconstruction(UsingDeconstruction),
}

pub fn new_using_clause_identifier_path(element: IdentifierPath) -> UsingClause {
    UsingClause::IdentifierPath(element)
}

pub fn new_using_clause_using_deconstruction(element: UsingDeconstruction) -> UsingClause {
    UsingClause::UsingDeconstruction(element)
}

#[derive(Clone, Debug)]
pub enum UsingOperator {
    Ampersand(Ampersand),
    Asterisk(Asterisk),
    BangEqual(BangEqual),
    Bar(Bar),
    Caret(Caret),
    EqualEqual(EqualEqual),
    GreaterThan(GreaterThan),
    GreaterThanEqual(GreaterThanEqual),
    LessThan(LessThan),
    LessThanEqual(LessThanEqual),
    Minus(Minus),
    Percent(Percent),
    Plus(Plus),
    Slash(Slash),
    Tilde(Tilde),
}

pub fn new_using_operator_ampersand(element: Ampersand) -> UsingOperator {
    UsingOperator::Ampersand(element)
}

pub fn new_using_operator_asterisk(element: Asterisk) -> UsingOperator {
    UsingOperator::Asterisk(element)
}

pub fn new_using_operator_bang_equal(element: BangEqual) -> UsingOperator {
    UsingOperator::BangEqual(element)
}

pub fn new_using_operator_bar(element: Bar) -> UsingOperator {
    UsingOperator::Bar(element)
}

pub fn new_using_operator_caret(element: Caret) -> UsingOperator {
    UsingOperator::Caret(element)
}

pub fn new_using_operator_equal_equal(element: EqualEqual) -> UsingOperator {
    UsingOperator::EqualEqual(element)
}

pub fn new_using_operator_greater_than(element: GreaterThan) -> UsingOperator {
    UsingOperator::GreaterThan(element)
}

pub fn new_using_operator_greater_than_equal(element: GreaterThanEqual) -> UsingOperator {
    UsingOperator::GreaterThanEqual(element)
}

pub fn new_using_operator_less_than(element: LessThan) -> UsingOperator {
    UsingOperator::LessThan(element)
}

pub fn new_using_operator_less_than_equal(element: LessThanEqual) -> UsingOperator {
    UsingOperator::LessThanEqual(element)
}

pub fn new_using_operator_minus(element: Minus) -> UsingOperator {
    UsingOperator::Minus(element)
}

pub fn new_using_operator_percent(element: Percent) -> UsingOperator {
    UsingOperator::Percent(element)
}

pub fn new_using_operator_plus(element: Plus) -> UsingOperator {
    UsingOperator::Plus(element)
}

pub fn new_using_operator_slash(element: Slash) -> UsingOperator {
    UsingOperator::Slash(element)
}

pub fn new_using_operator_tilde(element: Tilde) -> UsingOperator {
    UsingOperator::Tilde(element)
}

#[derive(Clone, Debug)]
pub enum UsingTarget {
    TypeName(TypeName),
    Asterisk(Asterisk),
}

pub fn new_using_target_type_name(element: TypeName) -> UsingTarget {
    UsingTarget::TypeName(element)
}

pub fn new_using_target_asterisk(element: Asterisk) -> UsingTarget {
    UsingTarget::Asterisk(element)
}

#[derive(Clone, Debug)]
pub enum VariableDeclarationType {
    TypeName(TypeName),
    VarKeyword(VarKeyword),
}

pub fn new_variable_declaration_type_type_name(element: TypeName) -> VariableDeclarationType {
    VariableDeclarationType::TypeName(element)
}

pub fn new_variable_declaration_type_var_keyword(element: VarKeyword) -> VariableDeclarationType {
    VariableDeclarationType::VarKeyword(element)
}

#[derive(Clone, Debug)]
pub enum VersionExpression {
    VersionRange(VersionRange),
    VersionTerm(VersionTerm),
}

pub fn new_version_expression_version_range(element: VersionRange) -> VersionExpression {
    VersionExpression::VersionRange(element)
}

pub fn new_version_expression_version_term(element: VersionTerm) -> VersionExpression {
    VersionExpression::VersionTerm(element)
}

#[derive(Clone, Debug)]
pub enum VersionLiteral {
    SimpleVersionLiteral(SimpleVersionLiteral),
    SingleQuotedVersionLiteral(SingleQuotedVersionLiteral),
    DoubleQuotedVersionLiteral(DoubleQuotedVersionLiteral),
}

pub fn new_version_literal_simple_version_literal(element: SimpleVersionLiteral) -> VersionLiteral {
    VersionLiteral::SimpleVersionLiteral(element)
}

pub fn new_version_literal_single_quoted_version_literal(
    element: SingleQuotedVersionLiteral,
) -> VersionLiteral {
    VersionLiteral::SingleQuotedVersionLiteral(element)
}

pub fn new_version_literal_double_quoted_version_literal(
    element: DoubleQuotedVersionLiteral,
) -> VersionLiteral {
    VersionLiteral::DoubleQuotedVersionLiteral(element)
}

#[derive(Clone, Debug)]
pub enum VersionOperator {
    Caret(Caret),
    Tilde(Tilde),
    Equal(Equal),
    LessThan(LessThan),
    GreaterThan(GreaterThan),
    LessThanEqual(LessThanEqual),
    GreaterThanEqual(GreaterThanEqual),
}

pub fn new_version_operator_caret(element: Caret) -> VersionOperator {
    VersionOperator::Caret(element)
}

pub fn new_version_operator_tilde(element: Tilde) -> VersionOperator {
    VersionOperator::Tilde(element)
}

pub fn new_version_operator_equal(element: Equal) -> VersionOperator {
    VersionOperator::Equal(element)
}

pub fn new_version_operator_less_than(element: LessThan) -> VersionOperator {
    VersionOperator::LessThan(element)
}

pub fn new_version_operator_greater_than(element: GreaterThan) -> VersionOperator {
    VersionOperator::GreaterThan(element)
}

pub fn new_version_operator_less_than_equal(element: LessThanEqual) -> VersionOperator {
    VersionOperator::LessThanEqual(element)
}

pub fn new_version_operator_greater_than_equal(element: GreaterThanEqual) -> VersionOperator {
    VersionOperator::GreaterThanEqual(element)
}

#[derive(Clone, Debug)]
pub enum YulAssignmentOperator {
    ColonEqual(ColonEqual),
    YulColonAndEqual(YulColonAndEqual),
}

pub fn new_yul_assignment_operator_colon_equal(element: ColonEqual) -> YulAssignmentOperator {
    YulAssignmentOperator::ColonEqual(element)
}

pub fn new_yul_assignment_operator_yul_colon_and_equal(
    element: YulColonAndEqual,
) -> YulAssignmentOperator {
    YulAssignmentOperator::YulColonAndEqual(element)
}

#[derive(Clone, Debug)]
pub enum YulExpression {
    YulFunctionCallExpression(YulFunctionCallExpression),
    YulLiteral(YulLiteral),
    YulPath(YulPath),
}

pub fn new_yul_expression_yul_function_call_expression(
    element: YulFunctionCallExpression,
) -> YulExpression {
    YulExpression::YulFunctionCallExpression(element)
}

pub fn new_yul_expression_yul_literal(element: YulLiteral) -> YulExpression {
    YulExpression::YulLiteral(element)
}

pub fn new_yul_expression_yul_path(element: YulPath) -> YulExpression {
    YulExpression::YulPath(element)
}

#[derive(Clone, Debug)]
pub enum YulLiteral {
    YulTrueKeyword(YulTrueKeyword),
    YulFalseKeyword(YulFalseKeyword),
    YulDecimalLiteral(YulDecimalLiteral),
    YulHexLiteral(YulHexLiteral),
    HexStringLiteral(HexStringLiteral),
    StringLiteral(StringLiteral),
}

pub fn new_yul_literal_yul_true_keyword(element: YulTrueKeyword) -> YulLiteral {
    YulLiteral::YulTrueKeyword(element)
}

pub fn new_yul_literal_yul_false_keyword(element: YulFalseKeyword) -> YulLiteral {
    YulLiteral::YulFalseKeyword(element)
}

pub fn new_yul_literal_yul_decimal_literal(element: YulDecimalLiteral) -> YulLiteral {
    YulLiteral::YulDecimalLiteral(element)
}

pub fn new_yul_literal_yul_hex_literal(element: YulHexLiteral) -> YulLiteral {
    YulLiteral::YulHexLiteral(element)
}

pub fn new_yul_literal_hex_string_literal(element: HexStringLiteral) -> YulLiteral {
    YulLiteral::HexStringLiteral(element)
}

pub fn new_yul_literal_string_literal(element: StringLiteral) -> YulLiteral {
    YulLiteral::StringLiteral(element)
}

#[derive(Clone, Debug)]
pub enum YulStackAssignmentOperator {
    EqualColon(EqualColon),
    YulEqualAndColon(YulEqualAndColon),
}

pub fn new_yul_stack_assignment_operator_equal_colon(
    element: EqualColon,
) -> YulStackAssignmentOperator {
    YulStackAssignmentOperator::EqualColon(element)
}

pub fn new_yul_stack_assignment_operator_yul_equal_and_colon(
    element: YulEqualAndColon,
) -> YulStackAssignmentOperator {
    YulStackAssignmentOperator::YulEqualAndColon(element)
}

#[derive(Clone, Debug)]
pub enum YulStatement {
    YulBlock(YulBlock),
    YulFunctionDefinition(YulFunctionDefinition),
    YulStackAssignmentStatement(YulStackAssignmentStatement),
    YulIfStatement(YulIfStatement),
    YulForStatement(YulForStatement),
    YulSwitchStatement(YulSwitchStatement),
    YulLeaveStatement(YulLeaveStatement),
    YulBreakStatement(YulBreakStatement),
    YulContinueStatement(YulContinueStatement),
    YulVariableAssignmentStatement(YulVariableAssignmentStatement),
    YulLabel(YulLabel),
    YulVariableDeclarationStatement(YulVariableDeclarationStatement),
    YulExpression(YulExpression),
}

pub fn new_yul_statement_yul_block(element: YulBlock) -> YulStatement {
    YulStatement::YulBlock(element)
}

pub fn new_yul_statement_yul_function_definition(element: YulFunctionDefinition) -> YulStatement {
    YulStatement::YulFunctionDefinition(element)
}

pub fn new_yul_statement_yul_stack_assignment_statement(
    element: YulStackAssignmentStatement,
) -> YulStatement {
    YulStatement::YulStackAssignmentStatement(element)
}

pub fn new_yul_statement_yul_if_statement(element: YulIfStatement) -> YulStatement {
    YulStatement::YulIfStatement(element)
}

pub fn new_yul_statement_yul_for_statement(element: YulForStatement) -> YulStatement {
    YulStatement::YulForStatement(element)
}

pub fn new_yul_statement_yul_switch_statement(element: YulSwitchStatement) -> YulStatement {
    YulStatement::YulSwitchStatement(element)
}

pub fn new_yul_statement_yul_leave_statement(element: YulLeaveStatement) -> YulStatement {
    YulStatement::YulLeaveStatement(element)
}

pub fn new_yul_statement_yul_break_statement(element: YulBreakStatement) -> YulStatement {
    YulStatement::YulBreakStatement(element)
}

pub fn new_yul_statement_yul_continue_statement(element: YulContinueStatement) -> YulStatement {
    YulStatement::YulContinueStatement(element)
}

pub fn new_yul_statement_yul_variable_assignment_statement(
    element: YulVariableAssignmentStatement,
) -> YulStatement {
    YulStatement::YulVariableAssignmentStatement(element)
}

pub fn new_yul_statement_yul_label(element: YulLabel) -> YulStatement {
    YulStatement::YulLabel(element)
}

pub fn new_yul_statement_yul_variable_declaration_statement(
    element: YulVariableDeclarationStatement,
) -> YulStatement {
    YulStatement::YulVariableDeclarationStatement(element)
}

pub fn new_yul_statement_yul_expression(element: YulExpression) -> YulStatement {
    YulStatement::YulExpression(element)
}

#[derive(Clone, Debug)]
pub enum YulSwitchCase {
    YulDefaultCase(YulDefaultCase),
    YulValueCase(YulValueCase),
}

pub fn new_yul_switch_case_yul_default_case(element: YulDefaultCase) -> YulSwitchCase {
    YulSwitchCase::YulDefaultCase(element)
}

pub fn new_yul_switch_case_yul_value_case(element: YulValueCase) -> YulSwitchCase {
    YulSwitchCase::YulValueCase(element)
}

//
// Repeated & Separated
//
// TODO(v2): consider using a transparent representation

#[derive(Clone, Debug)]
pub struct ArrayValues {
    pub elements: Vec<Expression>,
}

pub fn new_array_values(elements: Vec<Expression>) -> ArrayValues {
    ArrayValues { elements }
}

#[derive(Clone, Debug)]
pub struct AssemblyFlags {
    pub elements: Vec<StringLiteral>,
}

pub fn new_assembly_flags(elements: Vec<StringLiteral>) -> AssemblyFlags {
    AssemblyFlags { elements }
}

#[derive(Clone, Debug)]
pub struct CallOptions {
    pub elements: Vec<NamedArgument>,
}

pub fn new_call_options(elements: Vec<NamedArgument>) -> CallOptions {
    CallOptions { elements }
}

#[derive(Clone, Debug)]
pub struct CatchClauses {
    pub elements: Vec<CatchClause>,
}

pub fn new_catch_clauses(elements: Vec<CatchClause>) -> CatchClauses {
    CatchClauses { elements }
}

#[derive(Clone, Debug)]
pub struct ConstructorAttributes {
    pub elements: Vec<ConstructorAttribute>,
}

pub fn new_constructor_attributes(elements: Vec<ConstructorAttribute>) -> ConstructorAttributes {
    ConstructorAttributes { elements }
}

#[derive(Clone, Debug)]
pub struct ContractMembers {
    pub elements: Vec<ContractMember>,
}

pub fn new_contract_members(elements: Vec<ContractMember>) -> ContractMembers {
    ContractMembers { elements }
}

#[derive(Clone, Debug)]
pub struct ContractSpecifiers {
    pub elements: Vec<ContractSpecifier>,
}

pub fn new_contract_specifiers(elements: Vec<ContractSpecifier>) -> ContractSpecifiers {
    ContractSpecifiers { elements }
}

#[derive(Clone, Debug)]
pub struct EnumMembers {
    pub elements: Vec<Identifier>,
}

pub fn new_enum_members(elements: Vec<Identifier>) -> EnumMembers {
    EnumMembers { elements }
}

#[derive(Clone, Debug)]
pub struct ErrorParameters {
    pub elements: Vec<ErrorParameter>,
}

pub fn new_error_parameters(elements: Vec<ErrorParameter>) -> ErrorParameters {
    ErrorParameters { elements }
}

#[derive(Clone, Debug)]
pub struct EventParameters {
    pub elements: Vec<EventParameter>,
}

pub fn new_event_parameters(elements: Vec<EventParameter>) -> EventParameters {
    EventParameters { elements }
}

#[derive(Clone, Debug)]
pub struct FallbackFunctionAttributes {
    pub elements: Vec<FallbackFunctionAttribute>,
}

pub fn new_fallback_function_attributes(
    elements: Vec<FallbackFunctionAttribute>,
) -> FallbackFunctionAttributes {
    FallbackFunctionAttributes { elements }
}

#[derive(Clone, Debug)]
pub struct FunctionAttributes {
    pub elements: Vec<FunctionAttribute>,
}

pub fn new_function_attributes(elements: Vec<FunctionAttribute>) -> FunctionAttributes {
    FunctionAttributes { elements }
}

#[derive(Clone, Debug)]
pub struct FunctionTypeAttributes {
    pub elements: Vec<FunctionTypeAttribute>,
}

pub fn new_function_type_attributes(
    elements: Vec<FunctionTypeAttribute>,
) -> FunctionTypeAttributes {
    FunctionTypeAttributes { elements }
}

#[derive(Clone, Debug)]
pub struct HexStringLiterals {
    pub elements: Vec<HexStringLiteral>,
}

pub fn new_hex_string_literals(elements: Vec<HexStringLiteral>) -> HexStringLiterals {
    HexStringLiterals { elements }
}

#[derive(Clone, Debug)]
pub struct IdentifierPath {
    pub elements: Vec<Identifier>,
}

pub fn new_identifier_path(elements: Vec<Identifier>) -> IdentifierPath {
    IdentifierPath { elements }
}

#[derive(Clone, Debug)]
pub struct ImportDeconstructionSymbols {
    pub elements: Vec<ImportDeconstructionSymbol>,
}

pub fn new_import_deconstruction_symbols(
    elements: Vec<ImportDeconstructionSymbol>,
) -> ImportDeconstructionSymbols {
    ImportDeconstructionSymbols { elements }
}

#[derive(Clone, Debug)]
pub struct InheritanceTypes {
    pub elements: Vec<InheritanceType>,
}

pub fn new_inheritance_types(elements: Vec<InheritanceType>) -> InheritanceTypes {
    InheritanceTypes { elements }
}

#[derive(Clone, Debug)]
pub struct InterfaceMembers {
    pub elements: Vec<ContractMember>,
}

pub fn new_interface_members(elements: Vec<ContractMember>) -> InterfaceMembers {
    InterfaceMembers { elements }
}

#[derive(Clone, Debug)]
pub struct LibraryMembers {
    pub elements: Vec<ContractMember>,
}

pub fn new_library_members(elements: Vec<ContractMember>) -> LibraryMembers {
    LibraryMembers { elements }
}

#[derive(Clone, Debug)]
pub struct ModifierAttributes {
    pub elements: Vec<ModifierAttribute>,
}

pub fn new_modifier_attributes(elements: Vec<ModifierAttribute>) -> ModifierAttributes {
    ModifierAttributes { elements }
}

#[derive(Clone, Debug)]
pub struct NamedArguments {
    pub elements: Vec<NamedArgument>,
}

pub fn new_named_arguments(elements: Vec<NamedArgument>) -> NamedArguments {
    NamedArguments { elements }
}

#[derive(Clone, Debug)]
pub struct OverridePaths {
    pub elements: Vec<IdentifierPath>,
}

pub fn new_override_paths(elements: Vec<IdentifierPath>) -> OverridePaths {
    OverridePaths { elements }
}

#[derive(Clone, Debug)]
pub struct Parameters {
    pub elements: Vec<Parameter>,
}

pub fn new_parameters(elements: Vec<Parameter>) -> Parameters {
    Parameters { elements }
}

#[derive(Clone, Debug)]
pub struct PositionalArguments {
    pub elements: Vec<Expression>,
}

pub fn new_positional_arguments(elements: Vec<Expression>) -> PositionalArguments {
    PositionalArguments { elements }
}

#[derive(Clone, Debug)]
pub struct ReceiveFunctionAttributes {
    pub elements: Vec<ReceiveFunctionAttribute>,
}

pub fn new_receive_function_attributes(
    elements: Vec<ReceiveFunctionAttribute>,
) -> ReceiveFunctionAttributes {
    ReceiveFunctionAttributes { elements }
}

#[derive(Clone, Debug)]
pub struct SimpleVersionLiteral {
    pub elements: Vec<VersionSpecifier>,
}

pub fn new_simple_version_literal(elements: Vec<VersionSpecifier>) -> SimpleVersionLiteral {
    SimpleVersionLiteral { elements }
}

#[derive(Clone, Debug)]
pub struct SourceUnitMembers {
    pub elements: Vec<SourceUnitMember>,
}

pub fn new_source_unit_members(elements: Vec<SourceUnitMember>) -> SourceUnitMembers {
    SourceUnitMembers { elements }
}

#[derive(Clone, Debug)]
pub struct StateVariableAttributes {
    pub elements: Vec<StateVariableAttribute>,
}

pub fn new_state_variable_attributes(
    elements: Vec<StateVariableAttribute>,
) -> StateVariableAttributes {
    StateVariableAttributes { elements }
}

#[derive(Clone, Debug)]
pub struct Statements {
    pub elements: Vec<Statement>,
}

pub fn new_statements(elements: Vec<Statement>) -> Statements {
    Statements { elements }
}

#[derive(Clone, Debug)]
pub struct StringLiterals {
    pub elements: Vec<StringLiteral>,
}

pub fn new_string_literals(elements: Vec<StringLiteral>) -> StringLiterals {
    StringLiterals { elements }
}

#[derive(Clone, Debug)]
pub struct StructMembers {
    pub elements: Vec<StructMember>,
}

pub fn new_struct_members(elements: Vec<StructMember>) -> StructMembers {
    StructMembers { elements }
}

#[derive(Clone, Debug)]
pub struct TupleDeconstructionElements {
    pub elements: Vec<TupleDeconstructionElement>,
}

pub fn new_tuple_deconstruction_elements(
    elements: Vec<TupleDeconstructionElement>,
) -> TupleDeconstructionElements {
    TupleDeconstructionElements { elements }
}

#[derive(Clone, Debug)]
pub struct TupleValues {
    pub elements: Vec<TupleValue>,
}

pub fn new_tuple_values(elements: Vec<TupleValue>) -> TupleValues {
    TupleValues { elements }
}

#[derive(Clone, Debug)]
pub struct UnicodeStringLiterals {
    pub elements: Vec<UnicodeStringLiteral>,
}

pub fn new_unicode_string_literals(elements: Vec<UnicodeStringLiteral>) -> UnicodeStringLiterals {
    UnicodeStringLiterals { elements }
}

#[derive(Clone, Debug)]
pub struct UnnamedFunctionAttributes {
    pub elements: Vec<UnnamedFunctionAttribute>,
}

pub fn new_unnamed_function_attributes(
    elements: Vec<UnnamedFunctionAttribute>,
) -> UnnamedFunctionAttributes {
    UnnamedFunctionAttributes { elements }
}

#[derive(Clone, Debug)]
pub struct UsingDeconstructionSymbols {
    pub elements: Vec<UsingDeconstructionSymbol>,
}

pub fn new_using_deconstruction_symbols(
    elements: Vec<UsingDeconstructionSymbol>,
) -> UsingDeconstructionSymbols {
    UsingDeconstructionSymbols { elements }
}

#[derive(Clone, Debug)]
pub struct VersionExpressionSet {
    pub elements: Vec<VersionExpression>,
}

pub fn new_version_expression_set(elements: Vec<VersionExpression>) -> VersionExpressionSet {
    VersionExpressionSet { elements }
}

#[derive(Clone, Debug)]
pub struct VersionExpressionSets {
    pub elements: Vec<VersionExpressionSet>,
}

pub fn new_version_expression_sets(elements: Vec<VersionExpressionSet>) -> VersionExpressionSets {
    VersionExpressionSets { elements }
}

#[derive(Clone, Debug)]
pub struct YulArguments {
    pub elements: Vec<YulExpression>,
}

pub fn new_yul_arguments(elements: Vec<YulExpression>) -> YulArguments {
    YulArguments { elements }
}

#[derive(Clone, Debug)]
pub struct YulParameters {
    pub elements: Vec<YulIdentifier>,
}

pub fn new_yul_parameters(elements: Vec<YulIdentifier>) -> YulParameters {
    YulParameters { elements }
}

#[derive(Clone, Debug)]
pub struct YulPath {
    pub elements: Vec<YulIdentifier>,
}

pub fn new_yul_path(elements: Vec<YulIdentifier>) -> YulPath {
    YulPath { elements }
}

#[derive(Clone, Debug)]
pub struct YulPaths {
    pub elements: Vec<YulPath>,
}

pub fn new_yul_paths(elements: Vec<YulPath>) -> YulPaths {
    YulPaths { elements }
}

#[derive(Clone, Debug)]
pub struct YulStatements {
    pub elements: Vec<YulStatement>,
}

pub fn new_yul_statements(elements: Vec<YulStatement>) -> YulStatements {
    YulStatements { elements }
}

#[derive(Clone, Debug)]
pub struct YulSwitchCases {
    pub elements: Vec<YulSwitchCase>,
}

pub fn new_yul_switch_cases(elements: Vec<YulSwitchCase>) -> YulSwitchCases {
    YulSwitchCases { elements }
}

#[derive(Clone, Debug)]
pub struct YulVariableNames {
    pub elements: Vec<YulIdentifier>,
}

pub fn new_yul_variable_names(elements: Vec<YulIdentifier>) -> YulVariableNames {
    YulVariableNames { elements }
}

//
// Terminals
//
// Note: _source is unused on the constructor methods, but kept for uniformity with other constructors
// and because it may be needed in the future

#[derive(Clone, Debug)]
pub struct ABIEncoderV2Keyword {
    pub range: Range<usize>,
}

pub fn new_abi_encoder_v2_keyword(range: Range<usize>, _source: &str) -> ABIEncoderV2Keyword {
    ABIEncoderV2Keyword { range }
}

#[derive(Clone, Debug)]
pub struct AbicoderKeyword {
    pub range: Range<usize>,
}

pub fn new_abicoder_keyword(range: Range<usize>, _source: &str) -> AbicoderKeyword {
    AbicoderKeyword { range }
}

#[derive(Clone, Debug)]
pub struct AbicoderV1Keyword {
    pub range: Range<usize>,
}

pub fn new_abicoder_v1_keyword(range: Range<usize>, _source: &str) -> AbicoderV1Keyword {
    AbicoderV1Keyword { range }
}

#[derive(Clone, Debug)]
pub struct AbicoderV2Keyword {
    pub range: Range<usize>,
}

pub fn new_abicoder_v2_keyword(range: Range<usize>, _source: &str) -> AbicoderV2Keyword {
    AbicoderV2Keyword { range }
}

#[derive(Clone, Debug)]
pub struct AbstractKeyword {
    pub range: Range<usize>,
}

pub fn new_abstract_keyword(range: Range<usize>, _source: &str) -> AbstractKeyword {
    AbstractKeyword { range }
}

#[derive(Clone, Debug)]
pub struct AddressKeyword {
    pub range: Range<usize>,
}

pub fn new_address_keyword(range: Range<usize>, _source: &str) -> AddressKeyword {
    AddressKeyword { range }
}

#[derive(Clone, Debug)]
pub struct AfterKeyword {
    pub range: Range<usize>,
}

pub fn new_after_keyword(range: Range<usize>, _source: &str) -> AfterKeyword {
    AfterKeyword { range }
}

#[derive(Clone, Debug)]
pub struct AliasKeyword {
    pub range: Range<usize>,
}

pub fn new_alias_keyword(range: Range<usize>, _source: &str) -> AliasKeyword {
    AliasKeyword { range }
}

#[derive(Clone, Debug)]
pub struct Ampersand {
    pub range: Range<usize>,
}

pub fn new_ampersand(range: Range<usize>, _source: &str) -> Ampersand {
    Ampersand { range }
}

#[derive(Clone, Debug)]
pub struct AmpersandAmpersand {
    pub range: Range<usize>,
}

pub fn new_ampersand_ampersand(range: Range<usize>, _source: &str) -> AmpersandAmpersand {
    AmpersandAmpersand { range }
}

#[derive(Clone, Debug)]
pub struct AmpersandEqual {
    pub range: Range<usize>,
}

pub fn new_ampersand_equal(range: Range<usize>, _source: &str) -> AmpersandEqual {
    AmpersandEqual { range }
}

#[derive(Clone, Debug)]
pub struct AnonymousKeyword {
    pub range: Range<usize>,
}

pub fn new_anonymous_keyword(range: Range<usize>, _source: &str) -> AnonymousKeyword {
    AnonymousKeyword { range }
}

#[derive(Clone, Debug)]
pub struct ApplyKeyword {
    pub range: Range<usize>,
}

pub fn new_apply_keyword(range: Range<usize>, _source: &str) -> ApplyKeyword {
    ApplyKeyword { range }
}

#[derive(Clone, Debug)]
pub struct AsKeyword {
    pub range: Range<usize>,
}

pub fn new_as_keyword(range: Range<usize>, _source: &str) -> AsKeyword {
    AsKeyword { range }
}

#[derive(Clone, Debug)]
pub struct AssemblyKeyword {
    pub range: Range<usize>,
}

pub fn new_assembly_keyword(range: Range<usize>, _source: &str) -> AssemblyKeyword {
    AssemblyKeyword { range }
}

#[derive(Clone, Debug)]
pub struct Asterisk {
    pub range: Range<usize>,
}

pub fn new_asterisk(range: Range<usize>, _source: &str) -> Asterisk {
    Asterisk { range }
}

#[derive(Clone, Debug)]
pub struct AsteriskAsterisk {
    pub range: Range<usize>,
}

pub fn new_asterisk_asterisk(range: Range<usize>, _source: &str) -> AsteriskAsterisk {
    AsteriskAsterisk { range }
}

#[derive(Clone, Debug)]
pub struct AsteriskEqual {
    pub range: Range<usize>,
}

pub fn new_asterisk_equal(range: Range<usize>, _source: &str) -> AsteriskEqual {
    AsteriskEqual { range }
}

#[derive(Clone, Debug)]
pub struct AtKeyword {
    pub range: Range<usize>,
}

pub fn new_at_keyword(range: Range<usize>, _source: &str) -> AtKeyword {
    AtKeyword { range }
}

#[derive(Clone, Debug)]
pub struct AutoKeyword {
    pub range: Range<usize>,
}

pub fn new_auto_keyword(range: Range<usize>, _source: &str) -> AutoKeyword {
    AutoKeyword { range }
}

#[derive(Clone, Debug)]
pub struct Bang {
    pub range: Range<usize>,
}

pub fn new_bang(range: Range<usize>, _source: &str) -> Bang {
    Bang { range }
}

#[derive(Clone, Debug)]
pub struct BangEqual {
    pub range: Range<usize>,
}

pub fn new_bang_equal(range: Range<usize>, _source: &str) -> BangEqual {
    BangEqual { range }
}

#[derive(Clone, Debug)]
pub struct Bar {
    pub range: Range<usize>,
}

pub fn new_bar(range: Range<usize>, _source: &str) -> Bar {
    Bar { range }
}

#[derive(Clone, Debug)]
pub struct BarBar {
    pub range: Range<usize>,
}

pub fn new_bar_bar(range: Range<usize>, _source: &str) -> BarBar {
    BarBar { range }
}

#[derive(Clone, Debug)]
pub struct BarEqual {
    pub range: Range<usize>,
}

pub fn new_bar_equal(range: Range<usize>, _source: &str) -> BarEqual {
    BarEqual { range }
}

#[derive(Clone, Debug)]
pub struct BoolKeyword {
    pub range: Range<usize>,
}

pub fn new_bool_keyword(range: Range<usize>, _source: &str) -> BoolKeyword {
    BoolKeyword { range }
}

#[derive(Clone, Debug)]
pub struct BreakKeyword {
    pub range: Range<usize>,
}

pub fn new_break_keyword(range: Range<usize>, _source: &str) -> BreakKeyword {
    BreakKeyword { range }
}

#[derive(Clone, Debug)]
pub struct ByteKeyword {
    pub range: Range<usize>,
}

pub fn new_byte_keyword(range: Range<usize>, _source: &str) -> ByteKeyword {
    ByteKeyword { range }
}

#[derive(Clone, Debug)]
pub struct BytesKeyword {
    pub range: Range<usize>,
}

pub fn new_bytes_keyword(range: Range<usize>, _source: &str) -> BytesKeyword {
    BytesKeyword { range }
}

#[derive(Clone, Debug)]
pub struct CallDataKeyword {
    pub range: Range<usize>,
}

pub fn new_call_data_keyword(range: Range<usize>, _source: &str) -> CallDataKeyword {
    CallDataKeyword { range }
}

#[derive(Clone, Debug)]
pub struct Caret {
    pub range: Range<usize>,
}

pub fn new_caret(range: Range<usize>, _source: &str) -> Caret {
    Caret { range }
}

#[derive(Clone, Debug)]
pub struct CaretEqual {
    pub range: Range<usize>,
}

pub fn new_caret_equal(range: Range<usize>, _source: &str) -> CaretEqual {
    CaretEqual { range }
}

#[derive(Clone, Debug)]
pub struct CaseKeyword {
    pub range: Range<usize>,
}

pub fn new_case_keyword(range: Range<usize>, _source: &str) -> CaseKeyword {
    CaseKeyword { range }
}

#[derive(Clone, Debug)]
pub struct CatchKeyword {
    pub range: Range<usize>,
}

pub fn new_catch_keyword(range: Range<usize>, _source: &str) -> CatchKeyword {
    CatchKeyword { range }
}

#[derive(Clone, Debug)]
pub struct CloseBrace {
    pub range: Range<usize>,
}

pub fn new_close_brace(range: Range<usize>, _source: &str) -> CloseBrace {
    CloseBrace { range }
}

#[derive(Clone, Debug)]
pub struct CloseBracket {
    pub range: Range<usize>,
}

pub fn new_close_bracket(range: Range<usize>, _source: &str) -> CloseBracket {
    CloseBracket { range }
}

#[derive(Clone, Debug)]
pub struct CloseParen {
    pub range: Range<usize>,
}

pub fn new_close_paren(range: Range<usize>, _source: &str) -> CloseParen {
    CloseParen { range }
}

#[derive(Clone, Debug)]
pub struct Colon {
    pub range: Range<usize>,
}

pub fn new_colon(range: Range<usize>, _source: &str) -> Colon {
    Colon { range }
}

#[derive(Clone, Debug)]
pub struct ColonEqual {
    pub range: Range<usize>,
}

pub fn new_colon_equal(range: Range<usize>, _source: &str) -> ColonEqual {
    ColonEqual { range }
}

#[derive(Clone, Debug)]
pub struct Comma {
    pub range: Range<usize>,
}

pub fn new_comma(range: Range<usize>, _source: &str) -> Comma {
    Comma { range }
}

#[derive(Clone, Debug)]
pub struct ConstantKeyword {
    pub range: Range<usize>,
}

pub fn new_constant_keyword(range: Range<usize>, _source: &str) -> ConstantKeyword {
    ConstantKeyword { range }
}

#[derive(Clone, Debug)]
pub struct ConstructorKeyword {
    pub range: Range<usize>,
}

pub fn new_constructor_keyword(range: Range<usize>, _source: &str) -> ConstructorKeyword {
    ConstructorKeyword { range }
}

#[derive(Clone, Debug)]
pub struct ContinueKeyword {
    pub range: Range<usize>,
}

pub fn new_continue_keyword(range: Range<usize>, _source: &str) -> ContinueKeyword {
    ContinueKeyword { range }
}

#[derive(Clone, Debug)]
pub struct ContractKeyword {
    pub range: Range<usize>,
}

pub fn new_contract_keyword(range: Range<usize>, _source: &str) -> ContractKeyword {
    ContractKeyword { range }
}

#[derive(Clone, Debug)]
pub struct CopyOfKeyword {
    pub range: Range<usize>,
}

pub fn new_copy_of_keyword(range: Range<usize>, _source: &str) -> CopyOfKeyword {
    CopyOfKeyword { range }
}

#[derive(Clone, Debug)]
pub struct DaysKeyword {
    pub range: Range<usize>,
}

pub fn new_days_keyword(range: Range<usize>, _source: &str) -> DaysKeyword {
    DaysKeyword { range }
}

#[derive(Clone, Debug)]
pub struct DecimalLiteral {
    pub range: Range<usize>,
}

pub fn new_decimal_literal(range: Range<usize>, _source: &str) -> DecimalLiteral {
    DecimalLiteral { range }
}

#[derive(Clone, Debug)]
pub struct DefaultKeyword {
    pub range: Range<usize>,
}

pub fn new_default_keyword(range: Range<usize>, _source: &str) -> DefaultKeyword {
    DefaultKeyword { range }
}

#[derive(Clone, Debug)]
pub struct DefineKeyword {
    pub range: Range<usize>,
}

pub fn new_define_keyword(range: Range<usize>, _source: &str) -> DefineKeyword {
    DefineKeyword { range }
}

#[derive(Clone, Debug)]
pub struct DeleteKeyword {
    pub range: Range<usize>,
}

pub fn new_delete_keyword(range: Range<usize>, _source: &str) -> DeleteKeyword {
    DeleteKeyword { range }
}

#[derive(Clone, Debug)]
pub struct DoKeyword {
    pub range: Range<usize>,
}

pub fn new_do_keyword(range: Range<usize>, _source: &str) -> DoKeyword {
    DoKeyword { range }
}

#[derive(Clone, Debug)]
pub struct DoubleQuotedHexStringLiteral {
    pub range: Range<usize>,
}

pub fn new_double_quoted_hex_string_literal(
    range: Range<usize>,
    _source: &str,
) -> DoubleQuotedHexStringLiteral {
    DoubleQuotedHexStringLiteral { range }
}

#[derive(Clone, Debug)]
pub struct DoubleQuotedStringLiteral {
    pub range: Range<usize>,
}

pub fn new_double_quoted_string_literal(
    range: Range<usize>,
    _source: &str,
) -> DoubleQuotedStringLiteral {
    DoubleQuotedStringLiteral { range }
}

#[derive(Clone, Debug)]
pub struct DoubleQuotedUnicodeStringLiteral {
    pub range: Range<usize>,
}

pub fn new_double_quoted_unicode_string_literal(
    range: Range<usize>,
    _source: &str,
) -> DoubleQuotedUnicodeStringLiteral {
    DoubleQuotedUnicodeStringLiteral { range }
}

#[derive(Clone, Debug)]
pub struct DoubleQuotedVersionLiteral {
    pub range: Range<usize>,
}

pub fn new_double_quoted_version_literal(
    range: Range<usize>,
    _source: &str,
) -> DoubleQuotedVersionLiteral {
    DoubleQuotedVersionLiteral { range }
}

#[derive(Clone, Debug)]
pub struct ElseKeyword {
    pub range: Range<usize>,
}

pub fn new_else_keyword(range: Range<usize>, _source: &str) -> ElseKeyword {
    ElseKeyword { range }
}

#[derive(Clone, Debug)]
pub struct EmitKeyword {
    pub range: Range<usize>,
}

pub fn new_emit_keyword(range: Range<usize>, _source: &str) -> EmitKeyword {
    EmitKeyword { range }
}

#[derive(Clone, Debug)]
pub struct EndOfLine {
    pub range: Range<usize>,
}

pub fn new_end_of_line(range: Range<usize>, _source: &str) -> EndOfLine {
    EndOfLine { range }
}

#[derive(Clone, Debug)]
pub struct EnumKeyword {
    pub range: Range<usize>,
}

pub fn new_enum_keyword(range: Range<usize>, _source: &str) -> EnumKeyword {
    EnumKeyword { range }
}

#[derive(Clone, Debug)]
pub struct Equal {
    pub range: Range<usize>,
}

pub fn new_equal(range: Range<usize>, _source: &str) -> Equal {
    Equal { range }
}

#[derive(Clone, Debug)]
pub struct EqualColon {
    pub range: Range<usize>,
}

pub fn new_equal_colon(range: Range<usize>, _source: &str) -> EqualColon {
    EqualColon { range }
}

#[derive(Clone, Debug)]
pub struct EqualEqual {
    pub range: Range<usize>,
}

pub fn new_equal_equal(range: Range<usize>, _source: &str) -> EqualEqual {
    EqualEqual { range }
}

#[derive(Clone, Debug)]
pub struct EqualGreaterThan {
    pub range: Range<usize>,
}

pub fn new_equal_greater_than(range: Range<usize>, _source: &str) -> EqualGreaterThan {
    EqualGreaterThan { range }
}

#[derive(Clone, Debug)]
pub struct ErrorKeyword {
    pub range: Range<usize>,
}

pub fn new_error_keyword(range: Range<usize>, _source: &str) -> ErrorKeyword {
    ErrorKeyword { range }
}

#[derive(Clone, Debug)]
pub struct EtherKeyword {
    pub range: Range<usize>,
}

pub fn new_ether_keyword(range: Range<usize>, _source: &str) -> EtherKeyword {
    EtherKeyword { range }
}

#[derive(Clone, Debug)]
pub struct EventKeyword {
    pub range: Range<usize>,
}

pub fn new_event_keyword(range: Range<usize>, _source: &str) -> EventKeyword {
    EventKeyword { range }
}

#[derive(Clone, Debug)]
pub struct ExperimentalKeyword {
    pub range: Range<usize>,
}

pub fn new_experimental_keyword(range: Range<usize>, _source: &str) -> ExperimentalKeyword {
    ExperimentalKeyword { range }
}

#[derive(Clone, Debug)]
pub struct ExternalKeyword {
    pub range: Range<usize>,
}

pub fn new_external_keyword(range: Range<usize>, _source: &str) -> ExternalKeyword {
    ExternalKeyword { range }
}

#[derive(Clone, Debug)]
pub struct FallbackKeyword {
    pub range: Range<usize>,
}

pub fn new_fallback_keyword(range: Range<usize>, _source: &str) -> FallbackKeyword {
    FallbackKeyword { range }
}

#[derive(Clone, Debug)]
pub struct FalseKeyword {
    pub range: Range<usize>,
}

pub fn new_false_keyword(range: Range<usize>, _source: &str) -> FalseKeyword {
    FalseKeyword { range }
}

#[derive(Clone, Debug)]
pub struct FinalKeyword {
    pub range: Range<usize>,
}

pub fn new_final_keyword(range: Range<usize>, _source: &str) -> FinalKeyword {
    FinalKeyword { range }
}

#[derive(Clone, Debug)]
pub struct FinneyKeyword {
    pub range: Range<usize>,
}

pub fn new_finney_keyword(range: Range<usize>, _source: &str) -> FinneyKeyword {
    FinneyKeyword { range }
}

#[derive(Clone, Debug)]
pub struct FixedKeyword {
    pub range: Range<usize>,
}

pub fn new_fixed_keyword(range: Range<usize>, _source: &str) -> FixedKeyword {
    FixedKeyword { range }
}

#[derive(Clone, Debug)]
pub struct ForKeyword {
    pub range: Range<usize>,
}

pub fn new_for_keyword(range: Range<usize>, _source: &str) -> ForKeyword {
    ForKeyword { range }
}

#[derive(Clone, Debug)]
pub struct FromKeyword {
    pub range: Range<usize>,
}

pub fn new_from_keyword(range: Range<usize>, _source: &str) -> FromKeyword {
    FromKeyword { range }
}

#[derive(Clone, Debug)]
pub struct FunctionKeyword {
    pub range: Range<usize>,
}

pub fn new_function_keyword(range: Range<usize>, _source: &str) -> FunctionKeyword {
    FunctionKeyword { range }
}

#[derive(Clone, Debug)]
pub struct GlobalKeyword {
    pub range: Range<usize>,
}

pub fn new_global_keyword(range: Range<usize>, _source: &str) -> GlobalKeyword {
    GlobalKeyword { range }
}

#[derive(Clone, Debug)]
pub struct GreaterThan {
    pub range: Range<usize>,
}

pub fn new_greater_than(range: Range<usize>, _source: &str) -> GreaterThan {
    GreaterThan { range }
}

#[derive(Clone, Debug)]
pub struct GreaterThanEqual {
    pub range: Range<usize>,
}

pub fn new_greater_than_equal(range: Range<usize>, _source: &str) -> GreaterThanEqual {
    GreaterThanEqual { range }
}

#[derive(Clone, Debug)]
pub struct GreaterThanGreaterThan {
    pub range: Range<usize>,
}

pub fn new_greater_than_greater_than(range: Range<usize>, _source: &str) -> GreaterThanGreaterThan {
    GreaterThanGreaterThan { range }
}

#[derive(Clone, Debug)]
pub struct GreaterThanGreaterThanEqual {
    pub range: Range<usize>,
}

pub fn new_greater_than_greater_than_equal(
    range: Range<usize>,
    _source: &str,
) -> GreaterThanGreaterThanEqual {
    GreaterThanGreaterThanEqual { range }
}

#[derive(Clone, Debug)]
pub struct GreaterThanGreaterThanGreaterThan {
    pub range: Range<usize>,
}

pub fn new_greater_than_greater_than_greater_than(
    range: Range<usize>,
    _source: &str,
) -> GreaterThanGreaterThanGreaterThan {
    GreaterThanGreaterThanGreaterThan { range }
}

#[derive(Clone, Debug)]
pub struct GreaterThanGreaterThanGreaterThanEqual {
    pub range: Range<usize>,
}

pub fn new_greater_than_greater_than_greater_than_equal(
    range: Range<usize>,
    _source: &str,
) -> GreaterThanGreaterThanGreaterThanEqual {
    GreaterThanGreaterThanGreaterThanEqual { range }
}

#[derive(Clone, Debug)]
pub struct GweiKeyword {
    pub range: Range<usize>,
}

pub fn new_gwei_keyword(range: Range<usize>, _source: &str) -> GweiKeyword {
    GweiKeyword { range }
}

#[derive(Clone, Debug)]
pub struct HexKeyword {
    pub range: Range<usize>,
}

pub fn new_hex_keyword(range: Range<usize>, _source: &str) -> HexKeyword {
    HexKeyword { range }
}

#[derive(Clone, Debug)]
pub struct HexLiteral {
    pub range: Range<usize>,
}

pub fn new_hex_literal(range: Range<usize>, _source: &str) -> HexLiteral {
    HexLiteral { range }
}

#[derive(Clone, Debug)]
pub struct HoursKeyword {
    pub range: Range<usize>,
}

pub fn new_hours_keyword(range: Range<usize>, _source: &str) -> HoursKeyword {
    HoursKeyword { range }
}

#[derive(Clone, Debug)]
pub struct Identifier {
    pub range: Range<usize>,
}

pub fn new_identifier(range: Range<usize>, _source: &str) -> Identifier {
    Identifier { range }
}

#[derive(Clone, Debug)]
pub struct IfKeyword {
    pub range: Range<usize>,
}

pub fn new_if_keyword(range: Range<usize>, _source: &str) -> IfKeyword {
    IfKeyword { range }
}

#[derive(Clone, Debug)]
pub struct ImmutableKeyword {
    pub range: Range<usize>,
}

pub fn new_immutable_keyword(range: Range<usize>, _source: &str) -> ImmutableKeyword {
    ImmutableKeyword { range }
}

#[derive(Clone, Debug)]
pub struct ImplementsKeyword {
    pub range: Range<usize>,
}

pub fn new_implements_keyword(range: Range<usize>, _source: &str) -> ImplementsKeyword {
    ImplementsKeyword { range }
}

#[derive(Clone, Debug)]
pub struct ImportKeyword {
    pub range: Range<usize>,
}

pub fn new_import_keyword(range: Range<usize>, _source: &str) -> ImportKeyword {
    ImportKeyword { range }
}

#[derive(Clone, Debug)]
pub struct InKeyword {
    pub range: Range<usize>,
}

pub fn new_in_keyword(range: Range<usize>, _source: &str) -> InKeyword {
    InKeyword { range }
}

#[derive(Clone, Debug)]
pub struct IndexedKeyword {
    pub range: Range<usize>,
}

pub fn new_indexed_keyword(range: Range<usize>, _source: &str) -> IndexedKeyword {
    IndexedKeyword { range }
}

#[derive(Clone, Debug)]
pub struct InlineKeyword {
    pub range: Range<usize>,
}

pub fn new_inline_keyword(range: Range<usize>, _source: &str) -> InlineKeyword {
    InlineKeyword { range }
}

#[derive(Clone, Debug)]
pub struct IntKeyword {
    pub range: Range<usize>,
}

pub fn new_int_keyword(range: Range<usize>, _source: &str) -> IntKeyword {
    IntKeyword { range }
}

#[derive(Clone, Debug)]
pub struct InterfaceKeyword {
    pub range: Range<usize>,
}

pub fn new_interface_keyword(range: Range<usize>, _source: &str) -> InterfaceKeyword {
    InterfaceKeyword { range }
}

#[derive(Clone, Debug)]
pub struct InternalKeyword {
    pub range: Range<usize>,
}

pub fn new_internal_keyword(range: Range<usize>, _source: &str) -> InternalKeyword {
    InternalKeyword { range }
}

#[derive(Clone, Debug)]
pub struct IsKeyword {
    pub range: Range<usize>,
}

pub fn new_is_keyword(range: Range<usize>, _source: &str) -> IsKeyword {
    IsKeyword { range }
}

#[derive(Clone, Debug)]
pub struct LayoutKeyword {
    pub range: Range<usize>,
}

pub fn new_layout_keyword(range: Range<usize>, _source: &str) -> LayoutKeyword {
    LayoutKeyword { range }
}

#[derive(Clone, Debug)]
pub struct LessThan {
    pub range: Range<usize>,
}

pub fn new_less_than(range: Range<usize>, _source: &str) -> LessThan {
    LessThan { range }
}

#[derive(Clone, Debug)]
pub struct LessThanEqual {
    pub range: Range<usize>,
}

pub fn new_less_than_equal(range: Range<usize>, _source: &str) -> LessThanEqual {
    LessThanEqual { range }
}

#[derive(Clone, Debug)]
pub struct LessThanLessThan {
    pub range: Range<usize>,
}

pub fn new_less_than_less_than(range: Range<usize>, _source: &str) -> LessThanLessThan {
    LessThanLessThan { range }
}

#[derive(Clone, Debug)]
pub struct LessThanLessThanEqual {
    pub range: Range<usize>,
}

pub fn new_less_than_less_than_equal(range: Range<usize>, _source: &str) -> LessThanLessThanEqual {
    LessThanLessThanEqual { range }
}

#[derive(Clone, Debug)]
pub struct LetKeyword {
    pub range: Range<usize>,
}

pub fn new_let_keyword(range: Range<usize>, _source: &str) -> LetKeyword {
    LetKeyword { range }
}

#[derive(Clone, Debug)]
pub struct LibraryKeyword {
    pub range: Range<usize>,
}

pub fn new_library_keyword(range: Range<usize>, _source: &str) -> LibraryKeyword {
    LibraryKeyword { range }
}

#[derive(Clone, Debug)]
pub struct MacroKeyword {
    pub range: Range<usize>,
}

pub fn new_macro_keyword(range: Range<usize>, _source: &str) -> MacroKeyword {
    MacroKeyword { range }
}

#[derive(Clone, Debug)]
pub struct MappingKeyword {
    pub range: Range<usize>,
}

pub fn new_mapping_keyword(range: Range<usize>, _source: &str) -> MappingKeyword {
    MappingKeyword { range }
}

#[derive(Clone, Debug)]
pub struct MatchKeyword {
    pub range: Range<usize>,
}

pub fn new_match_keyword(range: Range<usize>, _source: &str) -> MatchKeyword {
    MatchKeyword { range }
}

#[derive(Clone, Debug)]
pub struct MemoryKeyword {
    pub range: Range<usize>,
}

pub fn new_memory_keyword(range: Range<usize>, _source: &str) -> MemoryKeyword {
    MemoryKeyword { range }
}

#[derive(Clone, Debug)]
pub struct Minus {
    pub range: Range<usize>,
}

pub fn new_minus(range: Range<usize>, _source: &str) -> Minus {
    Minus { range }
}

#[derive(Clone, Debug)]
pub struct MinusEqual {
    pub range: Range<usize>,
}

pub fn new_minus_equal(range: Range<usize>, _source: &str) -> MinusEqual {
    MinusEqual { range }
}

#[derive(Clone, Debug)]
pub struct MinusGreaterThan {
    pub range: Range<usize>,
}

pub fn new_minus_greater_than(range: Range<usize>, _source: &str) -> MinusGreaterThan {
    MinusGreaterThan { range }
}

#[derive(Clone, Debug)]
pub struct MinusMinus {
    pub range: Range<usize>,
}

pub fn new_minus_minus(range: Range<usize>, _source: &str) -> MinusMinus {
    MinusMinus { range }
}

#[derive(Clone, Debug)]
pub struct MinutesKeyword {
    pub range: Range<usize>,
}

pub fn new_minutes_keyword(range: Range<usize>, _source: &str) -> MinutesKeyword {
    MinutesKeyword { range }
}

#[derive(Clone, Debug)]
pub struct ModifierKeyword {
    pub range: Range<usize>,
}

pub fn new_modifier_keyword(range: Range<usize>, _source: &str) -> ModifierKeyword {
    ModifierKeyword { range }
}

#[derive(Clone, Debug)]
pub struct MultiLineComment {
    pub range: Range<usize>,
}

pub fn new_multi_line_comment(range: Range<usize>, _source: &str) -> MultiLineComment {
    MultiLineComment { range }
}

#[derive(Clone, Debug)]
pub struct MultiLineNatSpecComment {
    pub range: Range<usize>,
}

pub fn new_multi_line_nat_spec_comment(
    range: Range<usize>,
    _source: &str,
) -> MultiLineNatSpecComment {
    MultiLineNatSpecComment { range }
}

#[derive(Clone, Debug)]
pub struct MutableKeyword {
    pub range: Range<usize>,
}

pub fn new_mutable_keyword(range: Range<usize>, _source: &str) -> MutableKeyword {
    MutableKeyword { range }
}

#[derive(Clone, Debug)]
pub struct NewKeyword {
    pub range: Range<usize>,
}

pub fn new_new_keyword(range: Range<usize>, _source: &str) -> NewKeyword {
    NewKeyword { range }
}

#[derive(Clone, Debug)]
pub struct NullKeyword {
    pub range: Range<usize>,
}

pub fn new_null_keyword(range: Range<usize>, _source: &str) -> NullKeyword {
    NullKeyword { range }
}

#[derive(Clone, Debug)]
pub struct OfKeyword {
    pub range: Range<usize>,
}

pub fn new_of_keyword(range: Range<usize>, _source: &str) -> OfKeyword {
    OfKeyword { range }
}

#[derive(Clone, Debug)]
pub struct OpenBrace {
    pub range: Range<usize>,
}

pub fn new_open_brace(range: Range<usize>, _source: &str) -> OpenBrace {
    OpenBrace { range }
}

#[derive(Clone, Debug)]
pub struct OpenBracket {
    pub range: Range<usize>,
}

pub fn new_open_bracket(range: Range<usize>, _source: &str) -> OpenBracket {
    OpenBracket { range }
}

#[derive(Clone, Debug)]
pub struct OpenParen {
    pub range: Range<usize>,
}

pub fn new_open_paren(range: Range<usize>, _source: &str) -> OpenParen {
    OpenParen { range }
}

#[derive(Clone, Debug)]
pub struct OverrideKeyword {
    pub range: Range<usize>,
}

pub fn new_override_keyword(range: Range<usize>, _source: &str) -> OverrideKeyword {
    OverrideKeyword { range }
}

#[derive(Clone, Debug)]
pub struct PartialKeyword {
    pub range: Range<usize>,
}

pub fn new_partial_keyword(range: Range<usize>, _source: &str) -> PartialKeyword {
    PartialKeyword { range }
}

#[derive(Clone, Debug)]
pub struct PayableKeyword {
    pub range: Range<usize>,
}

pub fn new_payable_keyword(range: Range<usize>, _source: &str) -> PayableKeyword {
    PayableKeyword { range }
}

#[derive(Clone, Debug)]
pub struct Percent {
    pub range: Range<usize>,
}

pub fn new_percent(range: Range<usize>, _source: &str) -> Percent {
    Percent { range }
}

#[derive(Clone, Debug)]
pub struct PercentEqual {
    pub range: Range<usize>,
}

pub fn new_percent_equal(range: Range<usize>, _source: &str) -> PercentEqual {
    PercentEqual { range }
}

#[derive(Clone, Debug)]
pub struct Period {
    pub range: Range<usize>,
}

pub fn new_period(range: Range<usize>, _source: &str) -> Period {
    Period { range }
}

#[derive(Clone, Debug)]
pub struct Plus {
    pub range: Range<usize>,
}

pub fn new_plus(range: Range<usize>, _source: &str) -> Plus {
    Plus { range }
}

#[derive(Clone, Debug)]
pub struct PlusEqual {
    pub range: Range<usize>,
}

pub fn new_plus_equal(range: Range<usize>, _source: &str) -> PlusEqual {
    PlusEqual { range }
}

#[derive(Clone, Debug)]
pub struct PlusPlus {
    pub range: Range<usize>,
}

pub fn new_plus_plus(range: Range<usize>, _source: &str) -> PlusPlus {
    PlusPlus { range }
}

#[derive(Clone, Debug)]
pub struct PragmaKeyword {
    pub range: Range<usize>,
}

pub fn new_pragma_keyword(range: Range<usize>, _source: &str) -> PragmaKeyword {
    PragmaKeyword { range }
}

#[derive(Clone, Debug)]
pub struct PrivateKeyword {
    pub range: Range<usize>,
}

pub fn new_private_keyword(range: Range<usize>, _source: &str) -> PrivateKeyword {
    PrivateKeyword { range }
}

#[derive(Clone, Debug)]
pub struct PromiseKeyword {
    pub range: Range<usize>,
}

pub fn new_promise_keyword(range: Range<usize>, _source: &str) -> PromiseKeyword {
    PromiseKeyword { range }
}

#[derive(Clone, Debug)]
pub struct PublicKeyword {
    pub range: Range<usize>,
}

pub fn new_public_keyword(range: Range<usize>, _source: &str) -> PublicKeyword {
    PublicKeyword { range }
}

#[derive(Clone, Debug)]
pub struct PureKeyword {
    pub range: Range<usize>,
}

pub fn new_pure_keyword(range: Range<usize>, _source: &str) -> PureKeyword {
    PureKeyword { range }
}

#[derive(Clone, Debug)]
pub struct QuestionMark {
    pub range: Range<usize>,
}

pub fn new_question_mark(range: Range<usize>, _source: &str) -> QuestionMark {
    QuestionMark { range }
}

#[derive(Clone, Debug)]
pub struct ReceiveKeyword {
    pub range: Range<usize>,
}

pub fn new_receive_keyword(range: Range<usize>, _source: &str) -> ReceiveKeyword {
    ReceiveKeyword { range }
}

#[derive(Clone, Debug)]
pub struct ReferenceKeyword {
    pub range: Range<usize>,
}

pub fn new_reference_keyword(range: Range<usize>, _source: &str) -> ReferenceKeyword {
    ReferenceKeyword { range }
}

#[derive(Clone, Debug)]
pub struct RelocatableKeyword {
    pub range: Range<usize>,
}

pub fn new_relocatable_keyword(range: Range<usize>, _source: &str) -> RelocatableKeyword {
    RelocatableKeyword { range }
}

#[derive(Clone, Debug)]
pub struct ReturnKeyword {
    pub range: Range<usize>,
}

pub fn new_return_keyword(range: Range<usize>, _source: &str) -> ReturnKeyword {
    ReturnKeyword { range }
}

#[derive(Clone, Debug)]
pub struct ReturnsKeyword {
    pub range: Range<usize>,
}

pub fn new_returns_keyword(range: Range<usize>, _source: &str) -> ReturnsKeyword {
    ReturnsKeyword { range }
}

#[derive(Clone, Debug)]
pub struct RevertKeyword {
    pub range: Range<usize>,
}

pub fn new_revert_keyword(range: Range<usize>, _source: &str) -> RevertKeyword {
    RevertKeyword { range }
}

#[derive(Clone, Debug)]
pub struct SMTCheckerKeyword {
    pub range: Range<usize>,
}

pub fn new_smt_checker_keyword(range: Range<usize>, _source: &str) -> SMTCheckerKeyword {
    SMTCheckerKeyword { range }
}

#[derive(Clone, Debug)]
pub struct SealedKeyword {
    pub range: Range<usize>,
}

pub fn new_sealed_keyword(range: Range<usize>, _source: &str) -> SealedKeyword {
    SealedKeyword { range }
}

#[derive(Clone, Debug)]
pub struct SecondsKeyword {
    pub range: Range<usize>,
}

pub fn new_seconds_keyword(range: Range<usize>, _source: &str) -> SecondsKeyword {
    SecondsKeyword { range }
}

#[derive(Clone, Debug)]
pub struct Semicolon {
    pub range: Range<usize>,
}

pub fn new_semicolon(range: Range<usize>, _source: &str) -> Semicolon {
    Semicolon { range }
}

#[derive(Clone, Debug)]
pub struct SingleLineComment {
    pub range: Range<usize>,
}

pub fn new_single_line_comment(range: Range<usize>, _source: &str) -> SingleLineComment {
    SingleLineComment { range }
}

#[derive(Clone, Debug)]
pub struct SingleLineNatSpecComment {
    pub range: Range<usize>,
}

pub fn new_single_line_nat_spec_comment(
    range: Range<usize>,
    _source: &str,
) -> SingleLineNatSpecComment {
    SingleLineNatSpecComment { range }
}

#[derive(Clone, Debug)]
pub struct SingleQuotedHexStringLiteral {
    pub range: Range<usize>,
}

pub fn new_single_quoted_hex_string_literal(
    range: Range<usize>,
    _source: &str,
) -> SingleQuotedHexStringLiteral {
    SingleQuotedHexStringLiteral { range }
}

#[derive(Clone, Debug)]
pub struct SingleQuotedStringLiteral {
    pub range: Range<usize>,
}

pub fn new_single_quoted_string_literal(
    range: Range<usize>,
    _source: &str,
) -> SingleQuotedStringLiteral {
    SingleQuotedStringLiteral { range }
}

#[derive(Clone, Debug)]
pub struct SingleQuotedUnicodeStringLiteral {
    pub range: Range<usize>,
}

pub fn new_single_quoted_unicode_string_literal(
    range: Range<usize>,
    _source: &str,
) -> SingleQuotedUnicodeStringLiteral {
    SingleQuotedUnicodeStringLiteral { range }
}

#[derive(Clone, Debug)]
pub struct SingleQuotedVersionLiteral {
    pub range: Range<usize>,
}

pub fn new_single_quoted_version_literal(
    range: Range<usize>,
    _source: &str,
) -> SingleQuotedVersionLiteral {
    SingleQuotedVersionLiteral { range }
}

#[derive(Clone, Debug)]
pub struct SizeOfKeyword {
    pub range: Range<usize>,
}

pub fn new_size_of_keyword(range: Range<usize>, _source: &str) -> SizeOfKeyword {
    SizeOfKeyword { range }
}

#[derive(Clone, Debug)]
pub struct Slash {
    pub range: Range<usize>,
}

pub fn new_slash(range: Range<usize>, _source: &str) -> Slash {
    Slash { range }
}

#[derive(Clone, Debug)]
pub struct SlashEqual {
    pub range: Range<usize>,
}

pub fn new_slash_equal(range: Range<usize>, _source: &str) -> SlashEqual {
    SlashEqual { range }
}

#[derive(Clone, Debug)]
pub struct SolidityKeyword {
    pub range: Range<usize>,
}

pub fn new_solidity_keyword(range: Range<usize>, _source: &str) -> SolidityKeyword {
    SolidityKeyword { range }
}

#[derive(Clone, Debug)]
pub struct StaticKeyword {
    pub range: Range<usize>,
}

pub fn new_static_keyword(range: Range<usize>, _source: &str) -> StaticKeyword {
    StaticKeyword { range }
}

#[derive(Clone, Debug)]
pub struct StorageKeyword {
    pub range: Range<usize>,
}

pub fn new_storage_keyword(range: Range<usize>, _source: &str) -> StorageKeyword {
    StorageKeyword { range }
}

#[derive(Clone, Debug)]
pub struct StringKeyword {
    pub range: Range<usize>,
}

pub fn new_string_keyword(range: Range<usize>, _source: &str) -> StringKeyword {
    StringKeyword { range }
}

#[derive(Clone, Debug)]
pub struct StructKeyword {
    pub range: Range<usize>,
}

pub fn new_struct_keyword(range: Range<usize>, _source: &str) -> StructKeyword {
    StructKeyword { range }
}

#[derive(Clone, Debug)]
pub struct SuperKeyword {
    pub range: Range<usize>,
}

pub fn new_super_keyword(range: Range<usize>, _source: &str) -> SuperKeyword {
    SuperKeyword { range }
}

#[derive(Clone, Debug)]
pub struct SupportsKeyword {
    pub range: Range<usize>,
}

pub fn new_supports_keyword(range: Range<usize>, _source: &str) -> SupportsKeyword {
    SupportsKeyword { range }
}

#[derive(Clone, Debug)]
pub struct SwitchKeyword {
    pub range: Range<usize>,
}

pub fn new_switch_keyword(range: Range<usize>, _source: &str) -> SwitchKeyword {
    SwitchKeyword { range }
}

#[derive(Clone, Debug)]
pub struct SzaboKeyword {
    pub range: Range<usize>,
}

pub fn new_szabo_keyword(range: Range<usize>, _source: &str) -> SzaboKeyword {
    SzaboKeyword { range }
}

#[derive(Clone, Debug)]
pub struct ThisKeyword {
    pub range: Range<usize>,
}

pub fn new_this_keyword(range: Range<usize>, _source: &str) -> ThisKeyword {
    ThisKeyword { range }
}

#[derive(Clone, Debug)]
pub struct ThrowKeyword {
    pub range: Range<usize>,
}

pub fn new_throw_keyword(range: Range<usize>, _source: &str) -> ThrowKeyword {
    ThrowKeyword { range }
}

#[derive(Clone, Debug)]
pub struct Tilde {
    pub range: Range<usize>,
}

pub fn new_tilde(range: Range<usize>, _source: &str) -> Tilde {
    Tilde { range }
}

#[derive(Clone, Debug)]
pub struct TransientKeyword {
    pub range: Range<usize>,
}

pub fn new_transient_keyword(range: Range<usize>, _source: &str) -> TransientKeyword {
    TransientKeyword { range }
}

#[derive(Clone, Debug)]
pub struct TrueKeyword {
    pub range: Range<usize>,
}

pub fn new_true_keyword(range: Range<usize>, _source: &str) -> TrueKeyword {
    TrueKeyword { range }
}

#[derive(Clone, Debug)]
pub struct TryKeyword {
    pub range: Range<usize>,
}

pub fn new_try_keyword(range: Range<usize>, _source: &str) -> TryKeyword {
    TryKeyword { range }
}

#[derive(Clone, Debug)]
pub struct TypeDefKeyword {
    pub range: Range<usize>,
}

pub fn new_type_def_keyword(range: Range<usize>, _source: &str) -> TypeDefKeyword {
    TypeDefKeyword { range }
}

#[derive(Clone, Debug)]
pub struct TypeKeyword {
    pub range: Range<usize>,
}

pub fn new_type_keyword(range: Range<usize>, _source: &str) -> TypeKeyword {
    TypeKeyword { range }
}

#[derive(Clone, Debug)]
pub struct TypeOfKeyword {
    pub range: Range<usize>,
}

pub fn new_type_of_keyword(range: Range<usize>, _source: &str) -> TypeOfKeyword {
    TypeOfKeyword { range }
}

#[derive(Clone, Debug)]
pub struct UfixedKeyword {
    pub range: Range<usize>,
}

pub fn new_ufixed_keyword(range: Range<usize>, _source: &str) -> UfixedKeyword {
    UfixedKeyword { range }
}

#[derive(Clone, Debug)]
pub struct UintKeyword {
    pub range: Range<usize>,
}

pub fn new_uint_keyword(range: Range<usize>, _source: &str) -> UintKeyword {
    UintKeyword { range }
}

#[derive(Clone, Debug)]
pub struct UncheckedKeyword {
    pub range: Range<usize>,
}

pub fn new_unchecked_keyword(range: Range<usize>, _source: &str) -> UncheckedKeyword {
    UncheckedKeyword { range }
}

#[derive(Clone, Debug)]
pub struct UsingKeyword {
    pub range: Range<usize>,
}

pub fn new_using_keyword(range: Range<usize>, _source: &str) -> UsingKeyword {
    UsingKeyword { range }
}

#[derive(Clone, Debug)]
pub struct VarKeyword {
    pub range: Range<usize>,
}

pub fn new_var_keyword(range: Range<usize>, _source: &str) -> VarKeyword {
    VarKeyword { range }
}

#[derive(Clone, Debug)]
pub struct VersionSpecifier {
    pub range: Range<usize>,
}

pub fn new_version_specifier(range: Range<usize>, _source: &str) -> VersionSpecifier {
    VersionSpecifier { range }
}

#[derive(Clone, Debug)]
pub struct ViewKeyword {
    pub range: Range<usize>,
}

pub fn new_view_keyword(range: Range<usize>, _source: &str) -> ViewKeyword {
    ViewKeyword { range }
}

#[derive(Clone, Debug)]
pub struct VirtualKeyword {
    pub range: Range<usize>,
}

pub fn new_virtual_keyword(range: Range<usize>, _source: &str) -> VirtualKeyword {
    VirtualKeyword { range }
}

#[derive(Clone, Debug)]
pub struct WeeksKeyword {
    pub range: Range<usize>,
}

pub fn new_weeks_keyword(range: Range<usize>, _source: &str) -> WeeksKeyword {
    WeeksKeyword { range }
}

#[derive(Clone, Debug)]
pub struct WeiKeyword {
    pub range: Range<usize>,
}

pub fn new_wei_keyword(range: Range<usize>, _source: &str) -> WeiKeyword {
    WeiKeyword { range }
}

#[derive(Clone, Debug)]
pub struct WhileKeyword {
    pub range: Range<usize>,
}

pub fn new_while_keyword(range: Range<usize>, _source: &str) -> WhileKeyword {
    WhileKeyword { range }
}

#[derive(Clone, Debug)]
pub struct Whitespace {
    pub range: Range<usize>,
}

pub fn new_whitespace(range: Range<usize>, _source: &str) -> Whitespace {
    Whitespace { range }
}

#[derive(Clone, Debug)]
pub struct YearsKeyword {
    pub range: Range<usize>,
}

pub fn new_years_keyword(range: Range<usize>, _source: &str) -> YearsKeyword {
    YearsKeyword { range }
}

#[derive(Clone, Debug)]
pub struct YulAbstractKeyword {
    pub range: Range<usize>,
}

pub fn new_yul_abstract_keyword(range: Range<usize>, _source: &str) -> YulAbstractKeyword {
    YulAbstractKeyword { range }
}

#[derive(Clone, Debug)]
pub struct YulAfterKeyword {
    pub range: Range<usize>,
}

pub fn new_yul_after_keyword(range: Range<usize>, _source: &str) -> YulAfterKeyword {
    YulAfterKeyword { range }
}

#[derive(Clone, Debug)]
pub struct YulAliasKeyword {
    pub range: Range<usize>,
}

pub fn new_yul_alias_keyword(range: Range<usize>, _source: &str) -> YulAliasKeyword {
    YulAliasKeyword { range }
}

#[derive(Clone, Debug)]
pub struct YulAnonymousKeyword {
    pub range: Range<usize>,
}

pub fn new_yul_anonymous_keyword(range: Range<usize>, _source: &str) -> YulAnonymousKeyword {
    YulAnonymousKeyword { range }
}

#[derive(Clone, Debug)]
pub struct YulApplyKeyword {
    pub range: Range<usize>,
}

pub fn new_yul_apply_keyword(range: Range<usize>, _source: &str) -> YulApplyKeyword {
    YulApplyKeyword { range }
}

#[derive(Clone, Debug)]
pub struct YulAsKeyword {
    pub range: Range<usize>,
}

pub fn new_yul_as_keyword(range: Range<usize>, _source: &str) -> YulAsKeyword {
    YulAsKeyword { range }
}

#[derive(Clone, Debug)]
pub struct YulAssemblyKeyword {
    pub range: Range<usize>,
}

pub fn new_yul_assembly_keyword(range: Range<usize>, _source: &str) -> YulAssemblyKeyword {
    YulAssemblyKeyword { range }
}

#[derive(Clone, Debug)]
pub struct YulAutoKeyword {
    pub range: Range<usize>,
}

pub fn new_yul_auto_keyword(range: Range<usize>, _source: &str) -> YulAutoKeyword {
    YulAutoKeyword { range }
}

#[derive(Clone, Debug)]
pub struct YulBoolKeyword {
    pub range: Range<usize>,
}

pub fn new_yul_bool_keyword(range: Range<usize>, _source: &str) -> YulBoolKeyword {
    YulBoolKeyword { range }
}

#[derive(Clone, Debug)]
pub struct YulBreakKeyword {
    pub range: Range<usize>,
}

pub fn new_yul_break_keyword(range: Range<usize>, _source: &str) -> YulBreakKeyword {
    YulBreakKeyword { range }
}

#[derive(Clone, Debug)]
pub struct YulBytesKeyword {
    pub range: Range<usize>,
}

pub fn new_yul_bytes_keyword(range: Range<usize>, _source: &str) -> YulBytesKeyword {
    YulBytesKeyword { range }
}

#[derive(Clone, Debug)]
pub struct YulCallDataKeyword {
    pub range: Range<usize>,
}

pub fn new_yul_call_data_keyword(range: Range<usize>, _source: &str) -> YulCallDataKeyword {
    YulCallDataKeyword { range }
}

#[derive(Clone, Debug)]
pub struct YulCaseKeyword {
    pub range: Range<usize>,
}

pub fn new_yul_case_keyword(range: Range<usize>, _source: &str) -> YulCaseKeyword {
    YulCaseKeyword { range }
}

#[derive(Clone, Debug)]
pub struct YulCatchKeyword {
    pub range: Range<usize>,
}

pub fn new_yul_catch_keyword(range: Range<usize>, _source: &str) -> YulCatchKeyword {
    YulCatchKeyword { range }
}

#[derive(Clone, Debug)]
pub struct YulConstantKeyword {
    pub range: Range<usize>,
}

pub fn new_yul_constant_keyword(range: Range<usize>, _source: &str) -> YulConstantKeyword {
    YulConstantKeyword { range }
}

#[derive(Clone, Debug)]
pub struct YulConstructorKeyword {
    pub range: Range<usize>,
}

pub fn new_yul_constructor_keyword(range: Range<usize>, _source: &str) -> YulConstructorKeyword {
    YulConstructorKeyword { range }
}

#[derive(Clone, Debug)]
pub struct YulContinueKeyword {
    pub range: Range<usize>,
}

pub fn new_yul_continue_keyword(range: Range<usize>, _source: &str) -> YulContinueKeyword {
    YulContinueKeyword { range }
}

#[derive(Clone, Debug)]
pub struct YulContractKeyword {
    pub range: Range<usize>,
}

pub fn new_yul_contract_keyword(range: Range<usize>, _source: &str) -> YulContractKeyword {
    YulContractKeyword { range }
}

#[derive(Clone, Debug)]
pub struct YulCopyOfKeyword {
    pub range: Range<usize>,
}

pub fn new_yul_copy_of_keyword(range: Range<usize>, _source: &str) -> YulCopyOfKeyword {
    YulCopyOfKeyword { range }
}

#[derive(Clone, Debug)]
pub struct YulDaysKeyword {
    pub range: Range<usize>,
}

pub fn new_yul_days_keyword(range: Range<usize>, _source: &str) -> YulDaysKeyword {
    YulDaysKeyword { range }
}

#[derive(Clone, Debug)]
pub struct YulDecimalLiteral {
    pub range: Range<usize>,
}

pub fn new_yul_decimal_literal(range: Range<usize>, _source: &str) -> YulDecimalLiteral {
    YulDecimalLiteral { range }
}

#[derive(Clone, Debug)]
pub struct YulDefaultKeyword {
    pub range: Range<usize>,
}

pub fn new_yul_default_keyword(range: Range<usize>, _source: &str) -> YulDefaultKeyword {
    YulDefaultKeyword { range }
}

#[derive(Clone, Debug)]
pub struct YulDefineKeyword {
    pub range: Range<usize>,
}

pub fn new_yul_define_keyword(range: Range<usize>, _source: &str) -> YulDefineKeyword {
    YulDefineKeyword { range }
}

#[derive(Clone, Debug)]
pub struct YulDeleteKeyword {
    pub range: Range<usize>,
}

pub fn new_yul_delete_keyword(range: Range<usize>, _source: &str) -> YulDeleteKeyword {
    YulDeleteKeyword { range }
}

#[derive(Clone, Debug)]
pub struct YulDoKeyword {
    pub range: Range<usize>,
}

pub fn new_yul_do_keyword(range: Range<usize>, _source: &str) -> YulDoKeyword {
    YulDoKeyword { range }
}

#[derive(Clone, Debug)]
pub struct YulElseKeyword {
    pub range: Range<usize>,
}

pub fn new_yul_else_keyword(range: Range<usize>, _source: &str) -> YulElseKeyword {
    YulElseKeyword { range }
}

#[derive(Clone, Debug)]
pub struct YulEmitKeyword {
    pub range: Range<usize>,
}

pub fn new_yul_emit_keyword(range: Range<usize>, _source: &str) -> YulEmitKeyword {
    YulEmitKeyword { range }
}

#[derive(Clone, Debug)]
pub struct YulEnumKeyword {
    pub range: Range<usize>,
}

pub fn new_yul_enum_keyword(range: Range<usize>, _source: &str) -> YulEnumKeyword {
    YulEnumKeyword { range }
}

#[derive(Clone, Debug)]
pub struct YulEtherKeyword {
    pub range: Range<usize>,
}

pub fn new_yul_ether_keyword(range: Range<usize>, _source: &str) -> YulEtherKeyword {
    YulEtherKeyword { range }
}

#[derive(Clone, Debug)]
pub struct YulEventKeyword {
    pub range: Range<usize>,
}

pub fn new_yul_event_keyword(range: Range<usize>, _source: &str) -> YulEventKeyword {
    YulEventKeyword { range }
}

#[derive(Clone, Debug)]
pub struct YulExternalKeyword {
    pub range: Range<usize>,
}

pub fn new_yul_external_keyword(range: Range<usize>, _source: &str) -> YulExternalKeyword {
    YulExternalKeyword { range }
}

#[derive(Clone, Debug)]
pub struct YulFallbackKeyword {
    pub range: Range<usize>,
}

pub fn new_yul_fallback_keyword(range: Range<usize>, _source: &str) -> YulFallbackKeyword {
    YulFallbackKeyword { range }
}

#[derive(Clone, Debug)]
pub struct YulFalseKeyword {
    pub range: Range<usize>,
}

pub fn new_yul_false_keyword(range: Range<usize>, _source: &str) -> YulFalseKeyword {
    YulFalseKeyword { range }
}

#[derive(Clone, Debug)]
pub struct YulFinalKeyword {
    pub range: Range<usize>,
}

pub fn new_yul_final_keyword(range: Range<usize>, _source: &str) -> YulFinalKeyword {
    YulFinalKeyword { range }
}

#[derive(Clone, Debug)]
pub struct YulFinneyKeyword {
    pub range: Range<usize>,
}

pub fn new_yul_finney_keyword(range: Range<usize>, _source: &str) -> YulFinneyKeyword {
    YulFinneyKeyword { range }
}

#[derive(Clone, Debug)]
pub struct YulFixedKeyword {
    pub range: Range<usize>,
}

pub fn new_yul_fixed_keyword(range: Range<usize>, _source: &str) -> YulFixedKeyword {
    YulFixedKeyword { range }
}

#[derive(Clone, Debug)]
pub struct YulForKeyword {
    pub range: Range<usize>,
}

pub fn new_yul_for_keyword(range: Range<usize>, _source: &str) -> YulForKeyword {
    YulForKeyword { range }
}

#[derive(Clone, Debug)]
pub struct YulFunctionKeyword {
    pub range: Range<usize>,
}

pub fn new_yul_function_keyword(range: Range<usize>, _source: &str) -> YulFunctionKeyword {
    YulFunctionKeyword { range }
}

#[derive(Clone, Debug)]
pub struct YulGweiKeyword {
    pub range: Range<usize>,
}

pub fn new_yul_gwei_keyword(range: Range<usize>, _source: &str) -> YulGweiKeyword {
    YulGweiKeyword { range }
}

#[derive(Clone, Debug)]
pub struct YulHexKeyword {
    pub range: Range<usize>,
}

pub fn new_yul_hex_keyword(range: Range<usize>, _source: &str) -> YulHexKeyword {
    YulHexKeyword { range }
}

#[derive(Clone, Debug)]
pub struct YulHexLiteral {
    pub range: Range<usize>,
}

pub fn new_yul_hex_literal(range: Range<usize>, _source: &str) -> YulHexLiteral {
    YulHexLiteral { range }
}

#[derive(Clone, Debug)]
pub struct YulHoursKeyword {
    pub range: Range<usize>,
}

pub fn new_yul_hours_keyword(range: Range<usize>, _source: &str) -> YulHoursKeyword {
    YulHoursKeyword { range }
}

#[derive(Clone, Debug)]
pub struct YulIdentifier {
    pub range: Range<usize>,
}

pub fn new_yul_identifier(range: Range<usize>, _source: &str) -> YulIdentifier {
    YulIdentifier { range }
}

#[derive(Clone, Debug)]
pub struct YulIfKeyword {
    pub range: Range<usize>,
}

pub fn new_yul_if_keyword(range: Range<usize>, _source: &str) -> YulIfKeyword {
    YulIfKeyword { range }
}

#[derive(Clone, Debug)]
pub struct YulImmutableKeyword {
    pub range: Range<usize>,
}

pub fn new_yul_immutable_keyword(range: Range<usize>, _source: &str) -> YulImmutableKeyword {
    YulImmutableKeyword { range }
}

#[derive(Clone, Debug)]
pub struct YulImplementsKeyword {
    pub range: Range<usize>,
}

pub fn new_yul_implements_keyword(range: Range<usize>, _source: &str) -> YulImplementsKeyword {
    YulImplementsKeyword { range }
}

#[derive(Clone, Debug)]
pub struct YulImportKeyword {
    pub range: Range<usize>,
}

pub fn new_yul_import_keyword(range: Range<usize>, _source: &str) -> YulImportKeyword {
    YulImportKeyword { range }
}

#[derive(Clone, Debug)]
pub struct YulInKeyword {
    pub range: Range<usize>,
}

pub fn new_yul_in_keyword(range: Range<usize>, _source: &str) -> YulInKeyword {
    YulInKeyword { range }
}

#[derive(Clone, Debug)]
pub struct YulIndexedKeyword {
    pub range: Range<usize>,
}

pub fn new_yul_indexed_keyword(range: Range<usize>, _source: &str) -> YulIndexedKeyword {
    YulIndexedKeyword { range }
}

#[derive(Clone, Debug)]
pub struct YulInlineKeyword {
    pub range: Range<usize>,
}

pub fn new_yul_inline_keyword(range: Range<usize>, _source: &str) -> YulInlineKeyword {
    YulInlineKeyword { range }
}

#[derive(Clone, Debug)]
pub struct YulIntKeyword {
    pub range: Range<usize>,
}

pub fn new_yul_int_keyword(range: Range<usize>, _source: &str) -> YulIntKeyword {
    YulIntKeyword { range }
}

#[derive(Clone, Debug)]
pub struct YulInterfaceKeyword {
    pub range: Range<usize>,
}

pub fn new_yul_interface_keyword(range: Range<usize>, _source: &str) -> YulInterfaceKeyword {
    YulInterfaceKeyword { range }
}

#[derive(Clone, Debug)]
pub struct YulInternalKeyword {
    pub range: Range<usize>,
}

pub fn new_yul_internal_keyword(range: Range<usize>, _source: &str) -> YulInternalKeyword {
    YulInternalKeyword { range }
}

#[derive(Clone, Debug)]
pub struct YulIsKeyword {
    pub range: Range<usize>,
}

pub fn new_yul_is_keyword(range: Range<usize>, _source: &str) -> YulIsKeyword {
    YulIsKeyword { range }
}

#[derive(Clone, Debug)]
pub struct YulLeaveKeyword {
    pub range: Range<usize>,
}

pub fn new_yul_leave_keyword(range: Range<usize>, _source: &str) -> YulLeaveKeyword {
    YulLeaveKeyword { range }
}

#[derive(Clone, Debug)]
pub struct YulLetKeyword {
    pub range: Range<usize>,
}

pub fn new_yul_let_keyword(range: Range<usize>, _source: &str) -> YulLetKeyword {
    YulLetKeyword { range }
}

#[derive(Clone, Debug)]
pub struct YulLibraryKeyword {
    pub range: Range<usize>,
}

pub fn new_yul_library_keyword(range: Range<usize>, _source: &str) -> YulLibraryKeyword {
    YulLibraryKeyword { range }
}

#[derive(Clone, Debug)]
pub struct YulMacroKeyword {
    pub range: Range<usize>,
}

pub fn new_yul_macro_keyword(range: Range<usize>, _source: &str) -> YulMacroKeyword {
    YulMacroKeyword { range }
}

#[derive(Clone, Debug)]
pub struct YulMappingKeyword {
    pub range: Range<usize>,
}

pub fn new_yul_mapping_keyword(range: Range<usize>, _source: &str) -> YulMappingKeyword {
    YulMappingKeyword { range }
}

#[derive(Clone, Debug)]
pub struct YulMatchKeyword {
    pub range: Range<usize>,
}

pub fn new_yul_match_keyword(range: Range<usize>, _source: &str) -> YulMatchKeyword {
    YulMatchKeyword { range }
}

#[derive(Clone, Debug)]
pub struct YulMemoryKeyword {
    pub range: Range<usize>,
}

pub fn new_yul_memory_keyword(range: Range<usize>, _source: &str) -> YulMemoryKeyword {
    YulMemoryKeyword { range }
}

#[derive(Clone, Debug)]
pub struct YulMinutesKeyword {
    pub range: Range<usize>,
}

pub fn new_yul_minutes_keyword(range: Range<usize>, _source: &str) -> YulMinutesKeyword {
    YulMinutesKeyword { range }
}

#[derive(Clone, Debug)]
pub struct YulModifierKeyword {
    pub range: Range<usize>,
}

pub fn new_yul_modifier_keyword(range: Range<usize>, _source: &str) -> YulModifierKeyword {
    YulModifierKeyword { range }
}

#[derive(Clone, Debug)]
pub struct YulMutableKeyword {
    pub range: Range<usize>,
}

pub fn new_yul_mutable_keyword(range: Range<usize>, _source: &str) -> YulMutableKeyword {
    YulMutableKeyword { range }
}

#[derive(Clone, Debug)]
pub struct YulNewKeyword {
    pub range: Range<usize>,
}

pub fn new_yul_new_keyword(range: Range<usize>, _source: &str) -> YulNewKeyword {
    YulNewKeyword { range }
}

#[derive(Clone, Debug)]
pub struct YulNullKeyword {
    pub range: Range<usize>,
}

pub fn new_yul_null_keyword(range: Range<usize>, _source: &str) -> YulNullKeyword {
    YulNullKeyword { range }
}

#[derive(Clone, Debug)]
pub struct YulOfKeyword {
    pub range: Range<usize>,
}

pub fn new_yul_of_keyword(range: Range<usize>, _source: &str) -> YulOfKeyword {
    YulOfKeyword { range }
}

#[derive(Clone, Debug)]
pub struct YulOverrideKeyword {
    pub range: Range<usize>,
}

pub fn new_yul_override_keyword(range: Range<usize>, _source: &str) -> YulOverrideKeyword {
    YulOverrideKeyword { range }
}

#[derive(Clone, Debug)]
pub struct YulPartialKeyword {
    pub range: Range<usize>,
}

pub fn new_yul_partial_keyword(range: Range<usize>, _source: &str) -> YulPartialKeyword {
    YulPartialKeyword { range }
}

#[derive(Clone, Debug)]
pub struct YulPayableKeyword {
    pub range: Range<usize>,
}

pub fn new_yul_payable_keyword(range: Range<usize>, _source: &str) -> YulPayableKeyword {
    YulPayableKeyword { range }
}

#[derive(Clone, Debug)]
pub struct YulPragmaKeyword {
    pub range: Range<usize>,
}

pub fn new_yul_pragma_keyword(range: Range<usize>, _source: &str) -> YulPragmaKeyword {
    YulPragmaKeyword { range }
}

#[derive(Clone, Debug)]
pub struct YulPrivateKeyword {
    pub range: Range<usize>,
}

pub fn new_yul_private_keyword(range: Range<usize>, _source: &str) -> YulPrivateKeyword {
    YulPrivateKeyword { range }
}

#[derive(Clone, Debug)]
pub struct YulPromiseKeyword {
    pub range: Range<usize>,
}

pub fn new_yul_promise_keyword(range: Range<usize>, _source: &str) -> YulPromiseKeyword {
    YulPromiseKeyword { range }
}

#[derive(Clone, Debug)]
pub struct YulPublicKeyword {
    pub range: Range<usize>,
}

pub fn new_yul_public_keyword(range: Range<usize>, _source: &str) -> YulPublicKeyword {
    YulPublicKeyword { range }
}

#[derive(Clone, Debug)]
pub struct YulPureKeyword {
    pub range: Range<usize>,
}

pub fn new_yul_pure_keyword(range: Range<usize>, _source: &str) -> YulPureKeyword {
    YulPureKeyword { range }
}

#[derive(Clone, Debug)]
pub struct YulReceiveKeyword {
    pub range: Range<usize>,
}

pub fn new_yul_receive_keyword(range: Range<usize>, _source: &str) -> YulReceiveKeyword {
    YulReceiveKeyword { range }
}

#[derive(Clone, Debug)]
pub struct YulReferenceKeyword {
    pub range: Range<usize>,
}

pub fn new_yul_reference_keyword(range: Range<usize>, _source: &str) -> YulReferenceKeyword {
    YulReferenceKeyword { range }
}

#[derive(Clone, Debug)]
pub struct YulRelocatableKeyword {
    pub range: Range<usize>,
}

pub fn new_yul_relocatable_keyword(range: Range<usize>, _source: &str) -> YulRelocatableKeyword {
    YulRelocatableKeyword { range }
}

#[derive(Clone, Debug)]
pub struct YulReturnsKeyword {
    pub range: Range<usize>,
}

pub fn new_yul_returns_keyword(range: Range<usize>, _source: &str) -> YulReturnsKeyword {
    YulReturnsKeyword { range }
}

#[derive(Clone, Debug)]
pub struct YulSealedKeyword {
    pub range: Range<usize>,
}

pub fn new_yul_sealed_keyword(range: Range<usize>, _source: &str) -> YulSealedKeyword {
    YulSealedKeyword { range }
}

#[derive(Clone, Debug)]
pub struct YulSecondsKeyword {
    pub range: Range<usize>,
}

pub fn new_yul_seconds_keyword(range: Range<usize>, _source: &str) -> YulSecondsKeyword {
    YulSecondsKeyword { range }
}

#[derive(Clone, Debug)]
pub struct YulSizeOfKeyword {
    pub range: Range<usize>,
}

pub fn new_yul_size_of_keyword(range: Range<usize>, _source: &str) -> YulSizeOfKeyword {
    YulSizeOfKeyword { range }
}

#[derive(Clone, Debug)]
pub struct YulStaticKeyword {
    pub range: Range<usize>,
}

pub fn new_yul_static_keyword(range: Range<usize>, _source: &str) -> YulStaticKeyword {
    YulStaticKeyword { range }
}

#[derive(Clone, Debug)]
pub struct YulStorageKeyword {
    pub range: Range<usize>,
}

pub fn new_yul_storage_keyword(range: Range<usize>, _source: &str) -> YulStorageKeyword {
    YulStorageKeyword { range }
}

#[derive(Clone, Debug)]
pub struct YulStringKeyword {
    pub range: Range<usize>,
}

pub fn new_yul_string_keyword(range: Range<usize>, _source: &str) -> YulStringKeyword {
    YulStringKeyword { range }
}

#[derive(Clone, Debug)]
pub struct YulStructKeyword {
    pub range: Range<usize>,
}

pub fn new_yul_struct_keyword(range: Range<usize>, _source: &str) -> YulStructKeyword {
    YulStructKeyword { range }
}

#[derive(Clone, Debug)]
pub struct YulSuperKeyword {
    pub range: Range<usize>,
}

pub fn new_yul_super_keyword(range: Range<usize>, _source: &str) -> YulSuperKeyword {
    YulSuperKeyword { range }
}

#[derive(Clone, Debug)]
pub struct YulSupportsKeyword {
    pub range: Range<usize>,
}

pub fn new_yul_supports_keyword(range: Range<usize>, _source: &str) -> YulSupportsKeyword {
    YulSupportsKeyword { range }
}

#[derive(Clone, Debug)]
pub struct YulSwitchKeyword {
    pub range: Range<usize>,
}

pub fn new_yul_switch_keyword(range: Range<usize>, _source: &str) -> YulSwitchKeyword {
    YulSwitchKeyword { range }
}

#[derive(Clone, Debug)]
pub struct YulSzaboKeyword {
    pub range: Range<usize>,
}

pub fn new_yul_szabo_keyword(range: Range<usize>, _source: &str) -> YulSzaboKeyword {
    YulSzaboKeyword { range }
}

#[derive(Clone, Debug)]
pub struct YulThisKeyword {
    pub range: Range<usize>,
}

pub fn new_yul_this_keyword(range: Range<usize>, _source: &str) -> YulThisKeyword {
    YulThisKeyword { range }
}

#[derive(Clone, Debug)]
pub struct YulThrowKeyword {
    pub range: Range<usize>,
}

pub fn new_yul_throw_keyword(range: Range<usize>, _source: &str) -> YulThrowKeyword {
    YulThrowKeyword { range }
}

#[derive(Clone, Debug)]
pub struct YulTrueKeyword {
    pub range: Range<usize>,
}

pub fn new_yul_true_keyword(range: Range<usize>, _source: &str) -> YulTrueKeyword {
    YulTrueKeyword { range }
}

#[derive(Clone, Debug)]
pub struct YulTryKeyword {
    pub range: Range<usize>,
}

pub fn new_yul_try_keyword(range: Range<usize>, _source: &str) -> YulTryKeyword {
    YulTryKeyword { range }
}

#[derive(Clone, Debug)]
pub struct YulTypeDefKeyword {
    pub range: Range<usize>,
}

pub fn new_yul_type_def_keyword(range: Range<usize>, _source: &str) -> YulTypeDefKeyword {
    YulTypeDefKeyword { range }
}

#[derive(Clone, Debug)]
pub struct YulTypeKeyword {
    pub range: Range<usize>,
}

pub fn new_yul_type_keyword(range: Range<usize>, _source: &str) -> YulTypeKeyword {
    YulTypeKeyword { range }
}

#[derive(Clone, Debug)]
pub struct YulTypeOfKeyword {
    pub range: Range<usize>,
}

pub fn new_yul_type_of_keyword(range: Range<usize>, _source: &str) -> YulTypeOfKeyword {
    YulTypeOfKeyword { range }
}

#[derive(Clone, Debug)]
pub struct YulUfixedKeyword {
    pub range: Range<usize>,
}

pub fn new_yul_ufixed_keyword(range: Range<usize>, _source: &str) -> YulUfixedKeyword {
    YulUfixedKeyword { range }
}

#[derive(Clone, Debug)]
pub struct YulUintKeyword {
    pub range: Range<usize>,
}

pub fn new_yul_uint_keyword(range: Range<usize>, _source: &str) -> YulUintKeyword {
    YulUintKeyword { range }
}

#[derive(Clone, Debug)]
pub struct YulUncheckedKeyword {
    pub range: Range<usize>,
}

pub fn new_yul_unchecked_keyword(range: Range<usize>, _source: &str) -> YulUncheckedKeyword {
    YulUncheckedKeyword { range }
}

#[derive(Clone, Debug)]
pub struct YulUsingKeyword {
    pub range: Range<usize>,
}

pub fn new_yul_using_keyword(range: Range<usize>, _source: &str) -> YulUsingKeyword {
    YulUsingKeyword { range }
}

#[derive(Clone, Debug)]
pub struct YulVarKeyword {
    pub range: Range<usize>,
}

pub fn new_yul_var_keyword(range: Range<usize>, _source: &str) -> YulVarKeyword {
    YulVarKeyword { range }
}

#[derive(Clone, Debug)]
pub struct YulViewKeyword {
    pub range: Range<usize>,
}

pub fn new_yul_view_keyword(range: Range<usize>, _source: &str) -> YulViewKeyword {
    YulViewKeyword { range }
}

#[derive(Clone, Debug)]
pub struct YulVirtualKeyword {
    pub range: Range<usize>,
}

pub fn new_yul_virtual_keyword(range: Range<usize>, _source: &str) -> YulVirtualKeyword {
    YulVirtualKeyword { range }
}

#[derive(Clone, Debug)]
pub struct YulWeeksKeyword {
    pub range: Range<usize>,
}

pub fn new_yul_weeks_keyword(range: Range<usize>, _source: &str) -> YulWeeksKeyword {
    YulWeeksKeyword { range }
}

#[derive(Clone, Debug)]
pub struct YulWeiKeyword {
    pub range: Range<usize>,
}

pub fn new_yul_wei_keyword(range: Range<usize>, _source: &str) -> YulWeiKeyword {
    YulWeiKeyword { range }
}

#[derive(Clone, Debug)]
pub struct YulWhileKeyword {
    pub range: Range<usize>,
}

pub fn new_yul_while_keyword(range: Range<usize>, _source: &str) -> YulWhileKeyword {
    YulWhileKeyword { range }
}

#[derive(Clone, Debug)]
pub struct YulYearsKeyword {
    pub range: Range<usize>,
}

pub fn new_yul_years_keyword(range: Range<usize>, _source: &str) -> YulYearsKeyword {
    YulYearsKeyword { range }
}

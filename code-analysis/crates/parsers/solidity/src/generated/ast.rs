// This file is generated via `cargo build`. Please don't edit by hand.

#[allow(unused_imports)]
use super::kinds;
use std::ops::Range;
use std::rc::Rc;
pub type Token = Range<usize>;
pub type SeparatedBy<T> = Rc<T>;
pub type DelimitedBy<T> = Rc<T>;

pub enum ConstructorAttribute {
    ModifierInvocation(Rc<ModifierInvocation>),
    Internal(Token),
    Payable(Token),
    Public(Token),
}
pub enum ContractBodyElement {
    UsingDirective(Rc<UsingDirective>),
    ConstructorDefinition(Rc<ConstructorDefinition>),
    FunctionDefinition(Rc<FunctionDefinition>),
    FallbackFunctionDefinition(Rc<FallbackFunctionDefinition>),
    ReceiveFunctionDefinition(Rc<ReceiveFunctionDefinition>),
    ModifierDefinition(Rc<ModifierDefinition>),
    StructDefinition(Rc<StructDefinition>),
    EnumDefinition(Rc<EnumDefinition>),
    UserDefinedValueTypeDefinition(Rc<UserDefinedValueTypeDefinition>),
    EventDefinition(Rc<EventDefinition>),
    ErrorDefinition(Rc<ErrorDefinition>),
    StateVariableDeclaration(Rc<StateVariableDeclaration>),
}
pub enum DataLocation {
    Memory(Token),
    Storage(Token),
    Calldata(Token),
}
pub enum Definition {
    ContractDefinition(Rc<ContractDefinition>),
    InterfaceDefinition(Rc<InterfaceDefinition>),
    LibraryDefinition(Rc<LibraryDefinition>),
    FunctionDefinition(Rc<FunctionDefinition>),
    ConstantDefinition(Rc<ConstantDefinition>),
    StructDefinition(Rc<StructDefinition>),
    EnumDefinition(Rc<EnumDefinition>),
    UserDefinedValueTypeDefinition(Rc<UserDefinedValueTypeDefinition>),
    ErrorDefinition(Rc<ErrorDefinition>),
}
pub enum Directive {
    PragmaDirective(Rc<PragmaDirective>),
    ImportDirective(Rc<ImportDirective>),
    UsingDirective(Rc<UsingDirective>),
}
pub enum ElementaryType {
    Bool(Token),
    String(Token),
    AddressType(Rc<AddressType>),
    FixedBytesType(Token),
    SignedIntegerType(Token),
    UnsignedIntegerType(Token),
    SignedFixedType(Token),
    UnsignedFixedType(Token),
}
pub enum Expression {
    None,
}
pub enum FallbackFunctionAttribute {
    ModifierInvocation(Rc<ModifierInvocation>),
    OverrideSpecifier(Rc<OverrideSpecifier>),
    External(Token),
    Payable(Token),
    Pure(Token),
    View(Token),
    Virtual(Token),
}
pub enum FunctionAttribute {
    ModifierInvocation(Rc<ModifierInvocation>),
    OverrideSpecifier(Rc<OverrideSpecifier>),
    External(Token),
    Internal(Token),
    Payable(Token),
    Private(Token),
    Public(Token),
    Pure(Token),
    View(Token),
    Virtual(Token),
}
pub enum ModifierAttribute {
    OverrideSpecifier(Rc<OverrideSpecifier>),
    Virtual(Token),
}
pub enum PrimaryExpression {
    PayableExpression(Rc<PayableExpression>),
    TypeExpression(Rc<TypeExpression>),
    NewExpression(Rc<NewExpression>),
    ParenthesisExpression(Rc<ParenthesisExpression>),
    ArrayLiteral(Rc<ArrayLiteral>),
    AsciiStringLiteral(Token),
    UnicodeStringLiteral(Token),
    HexStringLiteral(Token),
    NumericLiteral(Token),
    BooleanLiteral(Token),
    Identifier(Token),
}
pub enum ReceiveFunctionAttribute {
    ModifierInvocation(Rc<ModifierInvocation>),
    OverrideSpecifier(Rc<OverrideSpecifier>),
    External(Token),
    Payable(Token),
    Virtual(Token),
}
pub enum SimpleStatement {
    TupleDeconstructionStatement(Rc<TupleDeconstructionStatement>),
    VariableDeclarationStatement(Rc<VariableDeclarationStatement>),
    ExpressionStatement(Rc<ExpressionStatement>),
}
pub enum StateVariableAttribute {
    OverrideSpecifier(Rc<OverrideSpecifier>),
    Constant(Token),
    Immutable(Token),
    Internal(Token),
    Private(Token),
    Public(Token),
}
pub enum Statement {
    Block(Rc<Block>),
    SimpleStatement(Rc<SimpleStatement>),
    IfStatement(Rc<IfStatement>),
    ForStatement(Rc<ForStatement>),
    WhileStatement(Rc<WhileStatement>),
    DoWhileStatement(Rc<DoWhileStatement>),
    ContinueStatement(Rc<ContinueStatement>),
    BreakStatement(Rc<BreakStatement>),
    TryStatement(Rc<TryStatement>),
    ReturnStatement(Rc<ReturnStatement>),
    EmitStatement(Rc<EmitStatement>),
    RevertStatement(Rc<RevertStatement>),
    DeleteStatement(Rc<DeleteStatement>),
    AssemblyStatement(Rc<AssemblyStatement>),
}
pub enum YulExpression {
    YulIdentifierPath(Rc<YulIdentifierPath>),
    YulFunctionCall(Rc<YulFunctionCall>),
    YulLiteral(Rc<YulLiteral>),
}
pub enum YulLiteral {
    YulDecimalNumberLiteral(Token),
    YulHexLiteral(Token),
    AsciiStringLiteral(Token),
    BooleanLiteral(Token),
    HexStringLiteral(Token),
}
pub enum YulStatement {
    YulBlock(Rc<YulBlock>),
    YulVariableDeclaration(Rc<YulVariableDeclaration>),
    YulFunctionDefinition(Rc<YulFunctionDefinition>),
    YulAssignmentStatement(Rc<YulAssignmentStatement>),
    YulFunctionCall(Rc<YulFunctionCall>),
    YulIfStatement(Rc<YulIfStatement>),
    YulForStatement(Rc<YulForStatement>),
    YulSwitchStatement(Rc<YulSwitchStatement>),
    YulLeaveStatement(Rc<YulLeaveStatement>),
    YulBreakStatement(Rc<YulBreakStatement>),
    YulContinueStatement(Rc<YulContinueStatement>),
}
pub struct AbicoderPragmaSpecifier {
    abicoder: Token,
    identifier: Token,
}
pub struct AddressType {
    address: Token,
    optional_payable: Option<Token>,
}
pub struct AssemblyStatement {
    assembly: Token,
    optional_double_quote_evmasm_double_quote: Option<Token>,
    optional_assembly_flags: Option<Rc<AssemblyFlags>>,
    yul_block: Rc<YulBlock>,
}
pub struct BreakStatement {
    r#break: Token,
    semicolon: Token,
}
pub struct ConstantDefinition {
    type_name: Rc<TypeName>,
    constant: Token,
    identifier: Token,
    equal: Token,
    expression: Rc<Expression>,
    semicolon: Token,
}
pub struct ConstructorDefinition {
    constructor: Token,
    parameter_list: Rc<ParameterList>,
    constructor_attributes: ConstructorAttributes,
    block: Rc<Block>,
}
pub struct ContinueStatement {
    r#continue: Token,
    semicolon: Token,
}
pub struct ContractDefinition {
    optional_abstract: Option<Token>,
    contract: Token,
    identifier: Token,
    optional_inheritance_specifier_list: Option<Rc<InheritanceSpecifierList>>,
    open_brace_and_contract_body_elements_and_close_brace:
        OpenBraceAndContractBodyElementsAndCloseBrace,
}
pub struct DeleteStatement {
    delete: Token,
    identifier: Token,
    semicolon: Token,
}
pub struct DoWhileStatement {
    r#do: Token,
    statement: Rc<Statement>,
    r#while: Token,
    open_paren_and_expression_and_close_paren: OpenParenAndExpressionAndCloseParen,
    semicolon: Token,
}
pub struct EmitStatement {
    emit: Token,
    identifier_path: Rc<IdentifierPath>,
    argument_list: Rc<ArgumentList>,
    semicolon: Token,
}
pub struct EnumDefinition {
    r#enum: Token,
    identifier: Token,
    open_brace_and_identifiers_and_commas_and_close_brace:
        OpenBraceAndIdentifiersAndCommasAndCloseBrace,
}
pub struct ErrorDefinition {
    error: Token,
    identifier: Token,
    open_paren_and_optional_error_parameters_and_commas_and_close_paren:
        OpenParenAndOptionalErrorParametersAndCommasAndCloseParen,
    semicolon: Token,
}
pub struct ErrorParameter {
    type_name: Rc<TypeName>,
    optional_identifier: Option<Token>,
}
pub struct EventDefinition {
    event: Token,
    identifier: Token,
    open_paren_and_optional_event_parameters_and_commas_and_close_paren:
        OpenParenAndOptionalEventParametersAndCommasAndCloseParen,
    optional_anonymous: Option<Token>,
    semicolon: Token,
}
pub struct EventParameter {
    type_name: Rc<TypeName>,
    optional_indexed: Option<Token>,
    optional_identifier: Option<Token>,
}
pub struct ExperimentalPragmaSpecifier {
    experimental: Token,
    identifier: Token,
}
pub struct ExpressionStatement {
    expression: Rc<Expression>,
    semicolon: Token,
}
pub struct InheritanceSpecifier {
    identifier_path: Rc<IdentifierPath>,
    optional_argument_list: Option<Rc<ArgumentList>>,
}
pub struct InheritanceSpecifierList {
    is: Token,
    inheritance_specifiers_and_commas: InheritanceSpecifiersAndCommas,
}
pub struct InterfaceDefinition {
    interface: Token,
    identifier: Token,
    optional_inheritance_specifier_list: Option<Rc<InheritanceSpecifierList>>,
    open_brace_and_contract_body_elements_and_close_brace:
        OpenBraceAndContractBodyElementsAndCloseBrace,
}
pub struct LibraryDefinition {
    library: Token,
    identifier: Token,
    open_brace_and_contract_body_elements_and_close_brace:
        OpenBraceAndContractBodyElementsAndCloseBrace,
}
pub struct ModifierInvocation {
    identifier_path: Rc<IdentifierPath>,
    optional_argument_list: Option<Rc<ArgumentList>>,
}
pub struct NamedArgument {
    identifier: Token,
    colon: Token,
    expression: Rc<Expression>,
}
pub struct NewExpression {
    new: Token,
    identifier_path: Rc<IdentifierPath>,
    argument_list: Rc<ArgumentList>,
}
pub struct OverrideSpecifier {
    r#override: Token,
    optional_open_paren_and_identifier_paths_and_commas_and_close_paren:
        Option<OpenParenAndIdentifierPathsAndCommasAndCloseParen>,
}
pub struct ParameterDeclaration {
    type_name: Rc<TypeName>,
    optional_data_location: Option<Rc<DataLocation>>,
    optional_identifier: Option<Token>,
}
pub struct PayableExpression {
    payable: Token,
    argument_list: Rc<ArgumentList>,
}
pub struct ReturnStatement {
    r#return: Token,
    optional_expression: Option<Rc<Expression>>,
    semicolon: Token,
}
pub struct RevertStatement {
    revert: Token,
    optional_identifier_path: Option<Rc<IdentifierPath>>,
    argument_list: Rc<ArgumentList>,
    semicolon: Token,
}
pub struct SelectingImportDirective {
    open_brace_and_selected_imports_and_commas_and_close_brace:
        OpenBraceAndSelectedImportsAndCommasAndCloseBrace,
    from: Token,
    import_path: Rc<ImportPath>,
}
pub struct StarImportDirective {
    star: Token,
    r#as: Token,
    identifier: Token,
    from: Token,
    import_path: Rc<ImportPath>,
}
pub struct StructDefinition {
    r#struct: Token,
    identifier: Token,
    open_brace_and_struct_members_and_close_brace: OpenBraceAndStructMembersAndCloseBrace,
}
pub struct StructMember {
    type_name: Rc<TypeName>,
    identifier: Token,
    semicolon: Token,
}
pub struct TypeExpression {
    r#type: Token,
    open_paren_and_type_name_and_close_paren: OpenParenAndTypeNameAndCloseParen,
}
pub struct UncheckedBlock {
    unchecked: Token,
    block: Rc<Block>,
}
pub struct UserDefinedValueTypeDefinition {
    r#type: Token,
    identifier: Token,
    is: Token,
    elementary_type: Rc<ElementaryType>,
    semicolon: Token,
}
pub struct WhileStatement {
    r#while: Token,
    open_paren_and_expression_and_close_paren: OpenParenAndExpressionAndCloseParen,
    statement: Rc<Statement>,
}
pub struct YulAssignmentStatement {
    yul_identifier_paths_and_commas: YulIdentifierPathsAndCommas,
    colon_equal: Token,
    yul_expression: Rc<YulExpression>,
}
pub struct YulForStatement {
    r#for: Token,
    yul_block: Rc<YulBlock>,
    yul_expression: Rc<YulExpression>,
    yul_block: Rc<YulBlock>,
    yul_block: Rc<YulBlock>,
}
pub struct YulFunctionCall {
    yul_identifier: Token,
    open_paren_and_optional_yul_expressions_and_commas_and_close_paren:
        OpenParenAndOptionalYulExpressionsAndCommasAndCloseParen,
}
pub struct YulIfStatement {
    r#if: Token,
    yul_expression: Rc<YulExpression>,
    yul_block: Rc<YulBlock>,
}
pub type ArgumentList = DelimitedBy<
    Option<(
        Option<Rc<PositionalArgumentList>>,
        Option<Rc<NamedArgumentList>>,
    )>,
>;
pub type Arguments = SeparatedBy<Token>;
pub type ArrayLiteral = DelimitedBy<ExpressionsAndCommas>;
pub type AssemblyFlags = DelimitedBy<DoubleQuotedAsciiStringLiteralsAndCommas>;
pub type Block = DelimitedBy<Vec<(Option<Rc<Statement>>, Option<Rc<UncheckedBlock>>)>>;
pub type CatchClause = (Token, Option<(Option<Token>, Rc<ParameterList>)>, Rc<Block>);
pub type CatchClauses = Vec<Rc<CatchClause>>;
pub type ConstructorAttributes = Vec<Rc<ConstructorAttribute>>;
pub type ContractBodyElements = Vec<Rc<ContractBodyElement>>;
pub type DoubleQuotedAsciiStringLiteralsAndCommas = SeparatedBy<Token>;
pub type EndOfFileTrivia = Vec<Token>;
pub type ErrorParametersAndCommas = SeparatedBy<Rc<ErrorParameter>>;
pub type EventParametersAndCommas = SeparatedBy<Rc<EventParameter>>;
pub type ExpressionsAndCommas = SeparatedBy<Rc<Expression>>;
pub type FallbackFunctionAttributes = Vec<Rc<FallbackFunctionAttribute>>;
pub type FallbackFunctionDefinition = (
    Token,
    Rc<ParameterList>,
    FallbackFunctionAttributes,
    Option<(Token, Rc<ParameterList>)>,
    (Option<Token>, Option<Rc<Block>>),
);
pub type ForStatement = (
    Token,
    DelimitedBy<(
        (Option<Rc<SimpleStatement>>, Option<Token>),
        (Option<Rc<ExpressionStatement>>, Option<Token>),
        Option<Rc<Expression>>,
    )>,
    Rc<Statement>,
);
pub type FunctionAttributes = Vec<Rc<FunctionAttribute>>;
pub type FunctionDefinition = (
    Token,
    Token,
    Rc<ParameterList>,
    FunctionAttributes,
    Option<(Token, Rc<ParameterList>)>,
    (Option<Token>, Option<Rc<Block>>),
);
pub type FunctionType = (
    Token,
    Rc<ParameterList>,
    Vec<Token>,
    Option<(Token, Rc<ParameterList>)>,
);
pub type IdentifierPath = SeparatedBy<Token>;
pub type IdentifierPathsAndCommas = SeparatedBy<Rc<IdentifierPath>>;
pub type IdentifiersAndCommas = SeparatedBy<Token>;
pub type IfStatement = (
    Token,
    OpenParenAndExpressionAndCloseParen,
    Rc<Statement>,
    Option<(Token, Rc<Statement>)>,
);
pub type ImportDirective = (
    Token,
    (
        Option<Rc<SimpleImportDirective>>,
        Option<Rc<StarImportDirective>>,
        Option<Rc<SelectingImportDirective>>,
    ),
    Token,
);
pub type InheritanceSpecifiersAndCommas = SeparatedBy<Rc<InheritanceSpecifier>>;
pub type LeadingTrivia = Vec<Token>;
pub type MappingType = (
    Token,
    DelimitedBy<(
        (Option<Rc<ElementaryType>>, Option<Rc<IdentifierPath>>),
        Token,
        Rc<TypeName>,
    )>,
);
pub type ModifierAttributes = Vec<Rc<ModifierAttribute>>;
pub type ModifierDefinition = (
    Token,
    Token,
    Option<Rc<ParameterList>>,
    ModifierAttributes,
    (Option<Token>, Option<Rc<Block>>),
);
pub type NamedArgumentList = DelimitedBy<Option<NamedArgumentsAndCommas>>;
pub type NamedArgumentsAndCommas = SeparatedBy<Rc<NamedArgument>>;
pub type OpenBraceAndContractBodyElementsAndCloseBrace = DelimitedBy<ContractBodyElements>;
pub type OpenBraceAndIdentifierPathsAndCommasAndCloseBrace = DelimitedBy<IdentifierPathsAndCommas>;
pub type OpenBraceAndIdentifiersAndCommasAndCloseBrace = DelimitedBy<IdentifiersAndCommas>;
pub type OpenBraceAndNamedArgumentsAndCommasAndCloseBrace = DelimitedBy<NamedArgumentsAndCommas>;
pub type OpenBraceAndSelectedImportsAndCommasAndCloseBrace = DelimitedBy<SelectedImportsAndCommas>;
pub type OpenBraceAndStructMembersAndCloseBrace = DelimitedBy<StructMembers>;
pub type OpenBracketAndOptionalExpressionAndCloseBracket = DelimitedBy<Option<Rc<Expression>>>;
pub type OpenBracketAndOptionalExpressionAndCloseBrackets =
    Vec<OpenBracketAndOptionalExpressionAndCloseBracket>;
pub type OpenParenAndExpressionAndCloseParen = DelimitedBy<Rc<Expression>>;
pub type OpenParenAndIdentifierPathsAndCommasAndCloseParen = DelimitedBy<IdentifierPathsAndCommas>;
pub type OpenParenAndOptionalArgumentsAndCloseParen = DelimitedBy<Option<Arguments>>;
pub type OpenParenAndOptionalErrorParametersAndCommasAndCloseParen =
    DelimitedBy<Option<ErrorParametersAndCommas>>;
pub type OpenParenAndOptionalEventParametersAndCommasAndCloseParen =
    DelimitedBy<Option<EventParametersAndCommas>>;
pub type OpenParenAndOptionalYulExpressionsAndCommasAndCloseParen =
    DelimitedBy<Option<YulExpressionsAndCommas>>;
pub type OpenParenAndTypeNameAndCloseParen = DelimitedBy<Rc<TypeName>>;
pub type OptionalExpressionsAndCommas = SeparatedBy<Option<Rc<Expression>>>;
pub type ParameterDeclarationsAndCommas = SeparatedBy<Rc<ParameterDeclaration>>;
pub type ParameterList = DelimitedBy<Option<ParameterDeclarationsAndCommas>>;
pub type ParenthesisExpression = DelimitedBy<OptionalExpressionsAndCommas>;
pub type PositionalArgumentList = SeparatedBy<Rc<Expression>>;
pub type PragmaDirective = (
    Token,
    (
        Option<Rc<VersionPragmaSpecifier>>,
        Option<Rc<AbicoderPragmaSpecifier>>,
        Option<Rc<ExperimentalPragmaSpecifier>>,
    ),
    Token,
);
pub type ReceiveFunctionAttributes = Vec<Rc<ReceiveFunctionAttribute>>;
pub type ReceiveFunctionDefinition = (
    Token,
    Rc<ParameterList>,
    ReceiveFunctionAttributes,
    (Option<Token>, Option<Rc<Block>>),
);
pub type Results = SeparatedBy<Token>;
pub type SelectedImport = (Token, Option<(Token, Token)>);
pub type SelectedImportsAndCommas = SeparatedBy<Rc<SelectedImport>>;
pub type SimpleImportDirective = (Rc<ImportPath>, Vec<(Token, Token)>);
pub type SourceUnit = (
    Rc<LeadingTrivia>,
    Vec<(Option<Rc<Directive>>, Option<Rc<Definition>>)>,
    Rc<EndOfFileTrivia>,
);
pub type StateVariableAttributes = Vec<Rc<StateVariableAttribute>>;
pub type StateVariableDeclaration = (
    Rc<TypeName>,
    StateVariableAttributes,
    Token,
    Option<(Token, Rc<Expression>)>,
    Token,
);
pub type StructMembers = Vec<Rc<StructMember>>;
pub type TryStatement = (
    Token,
    Rc<Expression>,
    Option<(Token, Rc<ParameterList>)>,
    Rc<Block>,
    CatchClauses,
);
pub type TupleDeconstructionStatement = (
    DelimitedBy<Option<SeparatedBy<Option<(Option<Rc<TypeName>>, Token)>>>>,
    Token,
    Rc<Expression>,
    Token,
);
pub type TypeName = (
    (
        Option<Rc<ElementaryType>>,
        Option<Rc<FunctionType>>,
        Option<Rc<MappingType>>,
        Option<Rc<IdentifierPath>>,
    ),
    OpenBracketAndOptionalExpressionAndCloseBrackets,
);
pub type UsingDirective = (
    Token,
    (
        Option<Rc<IdentifierPath>>,
        Option<OpenBraceAndIdentifierPathsAndCommasAndCloseBrace>,
    ),
    Token,
    (Option<Token>, Option<Rc<TypeName>>),
    Option<Token>,
    Token,
);
pub type VariableDeclarationStatement = (
    Rc<TypeName>,
    Option<Rc<DataLocation>>,
    Token,
    Option<(Token, Rc<Expression>)>,
    Token,
);
pub type VersionPragmaSpecifier = (Token, Vec<(Token, Token)>);
pub type YulBlock = DelimitedBy<YulStatements>;
pub type YulExpressionsAndCommas = SeparatedBy<Rc<YulExpression>>;
pub type YulFunctionDefinition = (
    Token,
    Token,
    OpenParenAndOptionalArgumentsAndCloseParen,
    Option<(Token, Results)>,
    Rc<YulBlock>,
);
pub type YulIdentifierPath = SeparatedBy<Token>;
pub type YulIdentifierPathsAndCommas = SeparatedBy<Rc<YulIdentifierPath>>;
pub type YulStatements = Vec<Rc<YulStatement>>;
pub type YulSwitchStatement = (
    Token,
    Rc<YulExpression>,
    Vec<(
        (Option<(Token, Rc<YulLiteral>)>, Option<Token>),
        Rc<YulBlock>,
    )>,
);
pub type YulVariableDeclaration = (
    Token,
    YulIdentifierPathsAndCommas,
    Option<(Token, Rc<YulExpression>)>,
);

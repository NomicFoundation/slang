// This file is generated via `cargo build`. Please don't edit by hand.

#[allow(unused_imports)]
use super::kinds;

pub struct AbicoderPragmaSpecifier {
    abicoder: usize,
    identifier: usize,
}
pub enum Anon {
    Plus,
    Minus,
}
pub struct AddressType {
    address: usize,
    payable: usize,
}
pub enum Anon {
    PositionalArgumentList,
    NamedArgumentList,
}
pub struct AssemblyStatement {
    assembly: usize,
    double_quote_evmasm_double_quote: usize,
    assembly_flags: usize,
    yul_block: usize,
}
pub enum Anon {
    Equal,
    PipeEqual,
    CaretEqual,
    AmpersandEqual,
    LessLessEqual,
    GreaterGreaterEqual,
    GreaterGreaterGreaterEqual,
    PlusEqual,
    MinusEqual,
    StarEqual,
    SlashEqual,
    PercentEqual,
}
pub enum Anon {
    Statement,
    UncheckedBlock,
}
pub struct BreakStatement {
    r#break: usize,
    semicolon: usize,
}
pub struct Anon {
    identifier: usize,
    parameter_list: usize,
}
pub struct CatchClause {
    catch: usize,
    anon: usize,
    block: usize,
}
pub struct Anon {
    question: usize,
    expression: usize,
    colon: usize,
    expression: usize,
}
pub struct ConstantDefinition {
    type_name: usize,
    constant: usize,
    identifier: usize,
    equal: usize,
    expression: usize,
    semicolon: usize,
}
pub enum ConstructorAttribute {
    ModifierInvocation,
    Internal,
    Payable,
    Public,
}
pub struct ConstructorDefinition {
    constructor: usize,
    parameter_list: usize,
    constructor_attribute_repeated: usize,
    block: usize,
}
pub struct ContinueStatement {
    r#continue: usize,
    semicolon: usize,
}
pub enum ContractBodyElement {
    UsingDirective,
    ConstructorDefinition,
    FunctionDefinition,
    FallbackFunctionDefinition,
    ReceiveFunctionDefinition,
    ModifierDefinition,
    StructDefinition,
    EnumDefinition,
    UserDefinedValueTypeDefinition,
    EventDefinition,
    ErrorDefinition,
    StateVariableDeclaration,
}
pub struct ContractDefinition {
    r#abstract: usize,
    contract: usize,
    identifier: usize,
    inheritance_specifier_list: usize,
    open_brace_and_contract_body_element_repeated_and_close_brace: usize,
}
pub enum DataLocation {
    Memory,
    Storage,
    Calldata,
}
pub enum Definition {
    ContractDefinition,
    InterfaceDefinition,
    LibraryDefinition,
    FunctionDefinition,
    ConstantDefinition,
    StructDefinition,
    EnumDefinition,
    UserDefinedValueTypeDefinition,
    ErrorDefinition,
}
pub struct DeleteStatement {
    delete: usize,
    identifier: usize,
    semicolon: usize,
}
pub enum Directive {
    PragmaDirective,
    ImportDirective,
    UsingDirective,
}
pub struct DoWhileStatement {
    r#do: usize,
    statement: usize,
    r#while: usize,
    open_paren_and_expression_and_close_paren: usize,
    semicolon: usize,
}
pub enum ElementaryType {
    Bool,
    String,
    AddressType,
    FixedBytesType,
    SignedIntegerType,
    UnsignedIntegerType,
    SignedFixedType,
    UnsignedFixedType,
}
pub struct EmitStatement {
    emit: usize,
    identifier_path: usize,
    argument_list: usize,
    semicolon: usize,
}
pub struct EnumDefinition {
    r#enum: usize,
    identifier: usize,
    open_brace_and_identifier_repeated_and_comma_repeated_and_close_brace: usize,
}
pub enum Anon {
    EqualEqual,
    BangEqual,
}
pub struct ErrorDefinition {
    error: usize,
    identifier: usize,
    open_paren_and_error_parameter_repeated_and_comma_repeated_and_close_paren: usize,
    semicolon: usize,
}
pub struct ErrorParameter {
    type_name: usize,
    identifier: usize,
}
pub struct EventDefinition {
    event: usize,
    identifier: usize,
    open_paren_and_event_parameter_repeated_and_comma_repeated_and_close_paren: usize,
    anonymous: usize,
    semicolon: usize,
}
pub struct EventParameter {
    type_name: usize,
    indexed: usize,
    identifier: usize,
}
pub struct ExperimentalPragmaSpecifier {
    experimental: usize,
    identifier: usize,
}
pub struct ExpressionStatement {
    expression: usize,
    semicolon: usize,
}
pub enum FallbackFunctionAttribute {
    ModifierInvocation,
    OverrideSpecifier,
    External,
    Payable,
    Pure,
    View,
    Virtual,
}
pub struct Anon {
    returns: usize,
    parameter_list: usize,
}
pub enum Anon {
    Semicolon,
    Block,
}
pub struct FallbackFunctionDefinition {
    fallback: usize,
    parameter_list: usize,
    fallback_function_attribute_repeated: usize,
    anon: usize,
    anon: usize,
}
pub enum Anon {
    SimpleStatement,
    Semicolon,
}
pub enum Anon {
    ExpressionStatement,
    Semicolon,
}
pub struct Anon {
    anon: usize,
    anon: usize,
    expression: usize,
}
pub struct ForStatement {
    r#for: usize,
    open_paren_and_anon_and_close_paren: usize,
    statement: usize,
}
pub enum FunctionAttribute {
    ModifierInvocation,
    OverrideSpecifier,
    External,
    Internal,
    Payable,
    Private,
    Public,
    Pure,
    View,
    Virtual,
}
pub struct Operator {
    open_brace_and_named_argument_repeated_and_comma_repeated_and_close_brace: usize,
    argument_list: usize,
}
pub enum Anon {
    Identifier,
    Fallback,
    Receive,
}
pub struct Anon {
    returns: usize,
    parameter_list: usize,
}
pub enum Anon {
    Semicolon,
    Block,
}
pub struct FunctionDefinition {
    function: usize,
    anon: usize,
    parameter_list: usize,
    function_attribute_repeated: usize,
    anon: usize,
    anon: usize,
}
pub enum Anon {
    Internal,
    External,
    Private,
    Public,
    Pure,
    View,
    Payable,
}
pub struct Anon {
    returns: usize,
    parameter_list: usize,
}
pub struct FunctionType {
    function: usize,
    parameter_list: usize,
    anon_repeated: usize,
    anon: usize,
}
pub struct Anon {
    r#else: usize,
    statement: usize,
}
pub struct IfStatement {
    r#if: usize,
    open_paren_and_expression_and_close_paren: usize,
    statement: usize,
    anon: usize,
}
pub enum Anon {
    SimpleImportDirective,
    StarImportDirective,
    SelectingImportDirective,
}
pub struct ImportDirective {
    import: usize,
    anon: usize,
    semicolon: usize,
}
pub struct Anon {
    colon: usize,
    expression: usize,
}
pub struct Anon {
    expression: usize,
    anon: usize,
}
pub struct InheritanceSpecifier {
    identifier_path: usize,
    argument_list: usize,
}
pub struct InheritanceSpecifierList {
    is: usize,
    inheritance_specifier_repeated_and_comma_repeated: usize,
}
pub struct InterfaceDefinition {
    interface: usize,
    identifier: usize,
    inheritance_specifier_list: usize,
    open_brace_and_contract_body_element_repeated_and_close_brace: usize,
}
pub struct LibraryDefinition {
    library: usize,
    identifier: usize,
    open_brace_and_contract_body_element_repeated_and_close_brace: usize,
}
pub enum Anon {
    ElementaryType,
    IdentifierPath,
}
pub struct Anon {
    anon: usize,
    equal_greater: usize,
    type_name: usize,
}
pub struct MappingType {
    mapping: usize,
    open_paren_and_anon_and_close_paren: usize,
}
pub enum Anon {
    Identifier,
    Address,
}
pub struct Operator {
    period: usize,
    anon: usize,
}
pub enum ModifierAttribute {
    OverrideSpecifier,
    Virtual,
}
pub enum Anon {
    Semicolon,
    Block,
}
pub struct ModifierDefinition {
    modifier: usize,
    identifier: usize,
    parameter_list: usize,
    modifier_attribute_repeated: usize,
    anon: usize,
}
pub struct ModifierInvocation {
    identifier_path: usize,
    argument_list: usize,
}
pub enum Anon {
    Star,
    Slash,
    Percent,
}
pub struct NamedArgument {
    identifier: usize,
    colon: usize,
    expression: usize,
}
pub struct NewExpression {
    new: usize,
    identifier_path: usize,
    argument_list: usize,
}
pub enum Anon {
    Less,
    Greater,
    LessEqual,
    GreaterEqual,
}
pub struct OverrideSpecifier {
    r#override: usize,
    open_paren_and_identifier_path_repeated_and_comma_repeated_and_close_paren: usize,
}
pub struct ParameterDeclaration {
    type_name: usize,
    data_location: usize,
    identifier: usize,
}
pub struct PayableExpression {
    payable: usize,
    argument_list: usize,
}
pub enum Anon {
    VersionPragmaSpecifier,
    AbicoderPragmaSpecifier,
    ExperimentalPragmaSpecifier,
}
pub struct PragmaDirective {
    pragma: usize,
    anon: usize,
    semicolon: usize,
}
pub enum PrimaryExpression {
    PayableExpression,
    TypeExpression,
    NewExpression,
    ParenthesisExpression,
    ArrayLiteral,
    AsciiStringLiteral,
    UnicodeStringLiteral,
    HexStringLiteral,
    NumericLiteral,
    BooleanLiteral,
    Identifier,
}
pub enum ReceiveFunctionAttribute {
    ModifierInvocation,
    OverrideSpecifier,
    External,
    Payable,
    Virtual,
}
pub enum Anon {
    Semicolon,
    Block,
}
pub struct ReceiveFunctionDefinition {
    receive: usize,
    parameter_list: usize,
    receive_function_attribute_repeated: usize,
    anon: usize,
}
pub struct ReturnStatement {
    r#return: usize,
    expression: usize,
    semicolon: usize,
}
pub struct RevertStatement {
    revert: usize,
    identifier_path: usize,
    argument_list: usize,
    semicolon: usize,
}
pub struct Anon {
    r#as: usize,
    identifier: usize,
}
pub struct SelectedImport {
    identifier: usize,
    anon: usize,
}
pub struct SelectingImportDirective {
    open_brace_and_selected_import_repeated_and_comma_repeated_and_close_brace: usize,
    from: usize,
    import_path: usize,
}
pub enum Anon {
    LessLess,
    GreaterGreater,
    GreaterGreaterGreater,
}
pub struct Anon {
    r#as: usize,
    identifier: usize,
}
pub struct SimpleImportDirective {
    import_path: usize,
    anon_repeated: usize,
}
pub enum SimpleStatement {
    TupleDeconstructionStatement,
    VariableDeclarationStatement,
    ExpressionStatement,
}
pub enum Anon {
    Directive,
    Definition,
}
pub struct SourceUnit {
    leading_trivia: usize,
    anon_repeated: usize,
    end_of_file_trivia: usize,
}
pub struct StarImportDirective {
    star: usize,
    r#as: usize,
    identifier: usize,
    from: usize,
    import_path: usize,
}
pub enum StateVariableAttribute {
    OverrideSpecifier,
    Constant,
    Immutable,
    Internal,
    Private,
    Public,
}
pub struct Anon {
    equal: usize,
    expression: usize,
}
pub struct StateVariableDeclaration {
    type_name: usize,
    state_variable_attribute_repeated: usize,
    identifier: usize,
    anon: usize,
    semicolon: usize,
}
pub enum Statement {
    Block,
    SimpleStatement,
    IfStatement,
    ForStatement,
    WhileStatement,
    DoWhileStatement,
    ContinueStatement,
    BreakStatement,
    TryStatement,
    ReturnStatement,
    EmitStatement,
    RevertStatement,
    DeleteStatement,
    AssemblyStatement,
}
pub struct StructDefinition {
    r#struct: usize,
    identifier: usize,
    open_brace_and_struct_member_repeated_and_close_brace: usize,
}
pub struct StructMember {
    type_name: usize,
    identifier: usize,
    semicolon: usize,
}
pub struct Anon {
    returns: usize,
    parameter_list: usize,
}
pub struct TryStatement {
    r#try: usize,
    expression: usize,
    anon: usize,
    block: usize,
    catch_clause_repeated: usize,
}
pub struct Anon {
    type_name: usize,
    identifier: usize,
}
pub struct TupleDeconstructionStatement {
    open_paren_and_anon_repeated_and_comma_repeated_and_close_paren: usize,
    equal: usize,
    expression: usize,
    semicolon: usize,
}
pub struct TypeExpression {
    r#type: usize,
    open_paren_and_type_name_and_close_paren: usize,
}
pub enum Anon {
    ElementaryType,
    FunctionType,
    MappingType,
    IdentifierPath,
}
pub struct TypeName {
    anon: usize,
    open_bracket_and_expression_and_close_bracket_repeated: usize,
}
pub enum Anon {
    PlusPlus,
    MinusMinus,
    Bang,
    Tilde,
    Minus,
}
pub enum Anon {
    PlusPlus,
    MinusMinus,
}
pub struct UncheckedBlock {
    unchecked: usize,
    block: usize,
}
pub struct UserDefinedValueTypeDefinition {
    r#type: usize,
    identifier: usize,
    is: usize,
    elementary_type: usize,
    semicolon: usize,
}
pub enum Anon {
    IdentifierPath,
    OpenBraceAndIdentifierPathRepeatedAndCommaRepeatedAndCloseBrace,
}
pub enum Anon {
    Star,
    TypeName,
}
pub struct UsingDirective {
    using: usize,
    anon: usize,
    r#for: usize,
    anon: usize,
    global: usize,
    semicolon: usize,
}
pub struct Anon {
    equal: usize,
    expression: usize,
}
pub struct VariableDeclarationStatement {
    type_name: usize,
    data_location: usize,
    identifier: usize,
    anon: usize,
    semicolon: usize,
}
pub struct Anon {
    version_pragma_operator: usize,
    version_pragma_value: usize,
}
pub struct VersionPragmaSpecifier {
    solidity: usize,
    anon_repeated: usize,
}
pub struct WhileStatement {
    r#while: usize,
    open_paren_and_expression_and_close_paren: usize,
    statement: usize,
}
pub struct YulAssignmentStatement {
    yul_identifier_path_repeated_and_comma_repeated: usize,
    colon_equal: usize,
    yul_expression: usize,
}
pub enum YulExpression {
    YulIdentifierPath,
    YulFunctionCall,
    YulLiteral,
}
pub struct YulForStatement {
    r#for: usize,
    yul_block: usize,
    yul_expression: usize,
    yul_block: usize,
    yul_block: usize,
}
pub struct YulFunctionCall {
    yul_identifier: usize,
    open_paren_and_yul_expression_repeated_and_comma_repeated_and_close_paren: usize,
}
pub struct Anon {
    minus_greater: usize,
    results: usize,
}
pub struct YulFunctionDefinition {
    function: usize,
    yul_identifier: usize,
    open_paren_and_arguments_and_close_paren: usize,
    anon: usize,
    yul_block: usize,
}
pub struct YulIfStatement {
    r#if: usize,
    yul_expression: usize,
    yul_block: usize,
}
pub enum YulLiteral {
    YulDecimalNumberLiteral,
    YulHexLiteral,
    AsciiStringLiteral,
    BooleanLiteral,
    HexStringLiteral,
}
pub enum YulStatement {
    YulBlock,
    YulVariableDeclaration,
    YulFunctionDefinition,
    YulAssignmentStatement,
    YulFunctionCall,
    YulIfStatement,
    YulForStatement,
    YulSwitchStatement,
    YulLeaveStatement,
    YulBreakStatement,
    YulContinueStatement,
}
pub struct Anon {
    case: usize,
    yul_literal: usize,
}
pub enum Anon {
    Anon,
    Default,
}
pub struct Anon {
    anon: usize,
    yul_block: usize,
}
pub struct YulSwitchStatement {
    switch: usize,
    yul_expression: usize,
    anon_repeated: usize,
}
pub struct Anon {
    colon_equal: usize,
    yul_expression: usize,
}
pub struct YulVariableDeclaration {
    r#let: usize,
    yul_identifier_path_repeated_and_comma_repeated: usize,
    anon: usize,
}

// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use serde::Serialize;
use strum_macros::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize)]
pub enum TokenKind {
    Abicoder,
    Abstract,
    Address,
    Ampersand,
    AmpersandAmpersand,
    AmpersandEqual,
    Anonymous,
    As,
    AsciiEscape,
    AsciiStringLiteral,
    Assembly,
    Bang,
    BangEqual,
    Bool,
    BooleanLiteral,
    Break,
    Calldata,
    Caret,
    CaretEqual,
    Case,
    Catch,
    Char,
    CloseBrace,
    CloseBracket,
    CloseParen,
    Colon,
    ColonEqual,
    Comma,
    Constant,
    Constructor,
    Content,
    Continue,
    Contract,
    DecimalExponent,
    DecimalFloat,
    DecimalInteger,
    DecimalNumber,
    Default,
    Delete,
    Do,
    DoubleQuote,
    DoubleQuoteEvmasmDoubleQuote,
    DoubleQuotedAsciiStringLiteral,
    DoubleQuotedUnicodeStringLiteral,
    Else,
    Emit,
    EndOfLine,
    Enum,
    Equal,
    EqualEqual,
    EqualGreater,
    Error,
    EscapeSequence,
    Event,
    Experimental,
    External,
    Fallback,
    FixedBytesType,
    For,
    From,
    Function,
    Global,
    Greater,
    GreaterEqual,
    GreaterGreater,
    GreaterGreaterEqual,
    GreaterGreaterGreater,
    GreaterGreaterGreaterEqual,
    HexByteEscape,
    HexCharacter,
    HexNumber,
    HexStringLiteral,
    Identifier,
    IdentifierPart,
    IdentifierStart,
    If,
    Immutable,
    Import,
    Indexed,
    Interface,
    Internal,
    Is,
    Keyword,
    Leave,
    Less,
    LessEqual,
    LessLess,
    LessLessEqual,
    Let,
    Library,
    Mapping,
    Memory,
    Minus,
    MinusEqual,
    MinusGreater,
    MinusMinus,
    Modifier,
    MultilineComment,
    New,
    NumberUnit,
    OpenBrace,
    OpenBracket,
    OpenParen,
    Override,
    Payable,
    Percent,
    PercentEqual,
    Period,
    Pipe,
    PipeEqual,
    PipePipe,
    Plus,
    PlusEqual,
    PlusPlus,
    PossiblySeparatedPairsOfHexDigits,
    Pragma,
    Private,
    Public,
    Pure,
    Question,
    Quote,
    RawIdentifier,
    Receive,
    ReservedKeyword,
    Return,
    Returns,
    Revert,
    Run,
    Semicolon,
    SignedFixedType,
    SignedIntegerType,
    SingleLineComment,
    SingleQuotedAsciiStringLiteral,
    SingleQuotedUnicodeStringLiteral,
    Slash,
    SlashEqual,
    SlashStar,
    Solidity,
    Star,
    StarEqual,
    StarSlash,
    StarStar,
    Storage,
    String,
    Struct,
    Switch,
    Tilde,
    Try,
    Type,
    Unchecked,
    UnicodeDoubleQuote,
    UnicodeEscape,
    UnicodeQuote,
    UnicodeStringLiteral,
    UnsignedFixedType,
    UnsignedIntegerType,
    Using,
    VersionPragmaOperator,
    VersionPragmaValue,
    View,
    Virtual,
    While,
    Whitespace,
    YulDecimalNumberLiteral,
    YulHexLiteral,
    YulIdentifier,
    YulKeyword,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize)]
pub enum RuleKind {
    ABICoderPragma,
    AddSubExpression,
    AddressType,
    AndExpression,
    ArgumentList,
    Arguments,
    ArrayLiteral,
    AssemblyFlags,
    AssemblyStatement,
    AssignmentExpression,
    BitAndExpression,
    BitOrExpression,
    BitXOrExpression,
    Block,
    BreakStatement,
    CatchClause,
    ConditionalExpression,
    ConstantDefinition,
    ConstructorAttribute,
    ConstructorDefinition,
    ContinueStatement,
    ContractBodyElement,
    ContractDefinition,
    DataLocation,
    Definition,
    DeleteStatement,
    Directive,
    DoWhileStatement,
    ElementaryType,
    EmitStatement,
    EndOfFileTrivia,
    EnumDefinition,
    EqualityComparisonExpression,
    ErrorDefinition,
    ErrorParameter,
    EventDefinition,
    EventParameter,
    ExperimentalPragma,
    ExponentiationExpression,
    Expression,
    ExpressionStatement,
    FallbackFunctionAttribute,
    FallbackFunctionDefinition,
    ForStatement,
    FunctionAttribute,
    FunctionCallExpression,
    FunctionDefinition,
    FunctionType,
    IdentifierPath,
    IfStatement,
    ImportDirective,
    ImportPath,
    IndexAccessExpression,
    InheritanceSpecifier,
    InheritanceSpecifierList,
    InterfaceDefinition,
    LeadingTrivia,
    LibraryDefinition,
    MappingType,
    MemberAccessExpression,
    ModifierAttribute,
    ModifierDefinition,
    ModifierInvocation,
    MulDivModExpression,
    NamedArgument,
    NamedArgumentList,
    NewExpression,
    NumericLiteral,
    OrExpression,
    OrderComparisonExpression,
    OverrideSpecifier,
    ParameterDeclaration,
    ParameterList,
    ParenthesisExpression,
    PayableExpression,
    PositionalArgumentList,
    PragmaDirective,
    PrimaryExpression,
    ReceiveFunctionAttribute,
    ReceiveFunctionDefinition,
    Results,
    ReturnStatement,
    RevertStatement,
    SelectedImport,
    SelectingImportDirective,
    ShiftExpression,
    SimpleImportDirective,
    SimpleStatement,
    SourceUnit,
    StarImportDirective,
    StateVariableAttribute,
    StateVariableDeclaration,
    Statement,
    StructDefinition,
    StructMember,
    TrailingTrivia,
    TryStatement,
    TupleDeconstructionStatement,
    TypeExpression,
    TypeName,
    UnaryPrefixExpression,
    UnarySuffixExpression,
    UncheckedBlock,
    UserDefinedValueTypeDefinition,
    UsingDirective,
    VariableDeclarationStatement,
    VersionPragma,
    VersionPragmaSpecifier,
    WhileStatement,
    YulAssignmentStatement,
    YulBlock,
    YulBreakStatement,
    YulContinueStatement,
    YulExpression,
    YulForStatement,
    YulFunctionCall,
    YulFunctionDefinition,
    YulIdentifierPath,
    YulIfStatement,
    YulLeaveStatement,
    YulLiteral,
    YulStatement,
    YulSwitchStatement,
    YulVariableDeclaration,
}

#[derive(
    Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, EnumString, AsRefStr, Display,
)]
pub enum ProductionKind {
    ABICoderPragma,
    AddSubExpression,
    AddressType,
    AndExpression,
    ArgumentList,
    ArrayLiteral,
    AsciiEscape,
    AsciiStringLiteral,
    AssemblyFlags,
    AssemblyStatement,
    AssignmentExpression,
    BitAndExpression,
    BitOrExpression,
    BitXOrExpression,
    Block,
    BooleanLiteral,
    BreakStatement,
    CatchClause,
    ConditionalExpression,
    ConstantDefinition,
    ConstructorAttribute,
    ConstructorDefinition,
    ContinueStatement,
    ContractBodyElement,
    ContractDefinition,
    DataLocation,
    DecimalExponent,
    DecimalFloat,
    DecimalInteger,
    DecimalNumber,
    Definition,
    DeleteStatement,
    Directive,
    DoWhileStatement,
    DoubleQuotedAsciiStringLiteral,
    DoubleQuotedUnicodeStringLiteral,
    ElementaryType,
    EmitStatement,
    EndOfFileTrivia,
    EndOfLine,
    EnumDefinition,
    EqualityComparisonExpression,
    ErrorDefinition,
    ErrorParameter,
    EscapeSequence,
    EventDefinition,
    EventParameter,
    ExperimentalPragma,
    ExponentiationExpression,
    Expression,
    ExpressionStatement,
    FallbackFunctionAttribute,
    FallbackFunctionDefinition,
    FixedBytesType,
    ForStatement,
    FunctionAttribute,
    FunctionCallExpression,
    FunctionDefinition,
    FunctionType,
    HexByteEscape,
    HexCharacter,
    HexNumber,
    HexStringLiteral,
    Identifier,
    IdentifierPart,
    IdentifierPath,
    IdentifierStart,
    IfStatement,
    ImportDirective,
    ImportPath,
    IndexAccessExpression,
    InheritanceSpecifier,
    InheritanceSpecifierList,
    InterfaceDefinition,
    Keyword,
    LeadingTrivia,
    LibraryDefinition,
    MappingType,
    MemberAccessExpression,
    ModifierAttribute,
    ModifierDefinition,
    ModifierInvocation,
    MulDivModExpression,
    MultilineComment,
    NamedArgument,
    NamedArgumentList,
    NewExpression,
    NumberUnit,
    NumericLiteral,
    OrExpression,
    OrderComparisonExpression,
    OverrideSpecifier,
    ParameterDeclaration,
    ParameterList,
    ParenthesisExpression,
    PayableExpression,
    PositionalArgumentList,
    PossiblySeparatedPairsOfHexDigits,
    PragmaDirective,
    PrimaryExpression,
    RawIdentifier,
    ReceiveFunctionAttribute,
    ReceiveFunctionDefinition,
    ReservedKeyword,
    ReturnStatement,
    RevertStatement,
    SelectedImport,
    SelectingImportDirective,
    ShiftExpression,
    SignedFixedType,
    SignedIntegerType,
    SimpleImportDirective,
    SimpleStatement,
    SingleLineComment,
    SingleQuotedAsciiStringLiteral,
    SingleQuotedUnicodeStringLiteral,
    SourceUnit,
    StarImportDirective,
    StateVariableAttribute,
    StateVariableDeclaration,
    Statement,
    StructDefinition,
    StructMember,
    TrailingTrivia,
    TryStatement,
    TupleDeconstructionStatement,
    TypeExpression,
    TypeName,
    UnaryPrefixExpression,
    UnarySuffixExpression,
    UncheckedBlock,
    UnicodeEscape,
    UnicodeStringLiteral,
    UnsignedFixedType,
    UnsignedIntegerType,
    UserDefinedValueTypeDefinition,
    UsingDirective,
    VariableDeclarationStatement,
    VersionPragma,
    VersionPragmaOperator,
    VersionPragmaSpecifier,
    VersionPragmaValue,
    WhileStatement,
    Whitespace,
    YulAssignmentStatement,
    YulBlock,
    YulBreakStatement,
    YulContinueStatement,
    YulDecimalNumberLiteral,
    YulExpression,
    YulForStatement,
    YulFunctionCall,
    YulFunctionDefinition,
    YulHexLiteral,
    YulIdentifier,
    YulIdentifierPath,
    YulIfStatement,
    YulKeyword,
    YulLeaveStatement,
    YulLiteral,
    YulStatement,
    YulSwitchStatement,
    YulVariableDeclaration,
}

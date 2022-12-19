// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use napi::bindgen_prelude::*;
use napi_derive::napi;
use serde::Serialize;

#[derive(Debug, PartialEq, Eq, Serialize)]
#[napi]
pub enum TokenKind {
    Abicoder,
    Abstract,
    Address,
    After,
    Alias,
    Ampersand,
    AmpersandAmpersand,
    AmpersandEqual,
    Anonymous,
    Apply,
    As,
    AsciiEscape,
    AsciiStringLiteral,
    Assembly,
    Auto,
    Backslash,
    Bang,
    BangEqual,
    Bool,
    BooleanLiteral,
    Break,
    Byte,
    ByteCount,
    Bytes,
    Calldata,
    Caret,
    CaretEqual,
    CarriageReturn,
    Case,
    Catch,
    Char,
    Chars,
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
    Copyof,
    Count,
    Days,
    DecimalExponent,
    DecimalFloat,
    DecimalInteger,
    DecimalNumber,
    Default,
    Define,
    Delete,
    DelimitedPossiblySeparatedPairsOfHexDigits,
    Do,
    DoubleQuote,
    DoubleQuoteEvmasmDoubleQuote,
    DoubleQuotedAsciiStringLiteral,
    DoubleQuotedUnicodeStringLiteral,
    Eight,
    EightEight,
    EightZero,
    Else,
    Emit,
    EndOfLine,
    Enum,
    Equal,
    EqualEqual,
    EqualGreater,
    Error,
    EscapeSequence,
    Ether,
    Event,
    Experimental,
    External,
    Fallback,
    False,
    Final,
    Finney,
    Five,
    FiveSix,
    Fixed,
    FixedBytesType,
    For,
    Four,
    FourEight,
    FourZero,
    From,
    Function,
    Global,
    Greater,
    GreaterEqual,
    GreaterGreater,
    GreaterGreaterEqual,
    GreaterGreaterGreater,
    GreaterGreaterGreaterEqual,
    Gwei,
    Hex,
    HexByteEscape,
    HexCharacter,
    HexNumber,
    HexStringLiteral,
    Hours,
    Identifier,
    IdentifierPart,
    IdentifierStart,
    If,
    Immutable,
    Implements,
    Import,
    In,
    Indexed,
    Inline,
    Int,
    Interface,
    Internal,
    Is,
    Keyword,
    LatinSmallLetterN,
    LatinSmallLetterR,
    LatinSmallLetterT,
    LatinSmallLetterU,
    LatinSmallLetterX,
    Leave,
    Less,
    LessEqual,
    LessLess,
    LessLessEqual,
    Let,
    Library,
    Linefeed,
    Macro,
    Mapping,
    Match,
    Memory,
    Minus,
    MinusEqual,
    MinusGreater,
    MinusMinus,
    Minutes,
    Modifier,
    MultilineComment,
    Mutable,
    New,
    Nine,
    NineSix,
    NotSlash,
    NotStar,
    Null,
    NumberUnit,
    Of,
    One,
    OneEight,
    OneEightFour,
    OneFive,
    OneFiveTwo,
    OneFour,
    OneFourFour,
    OneNine,
    OneNineTwo,
    OneOne,
    OneOneTwo,
    OneSeven,
    OneSevenSix,
    OneSix,
    OneSixEight,
    OneSixZero,
    OneThree,
    OneThreeSix,
    OneTwo,
    OneTwoEight,
    OneTwoZero,
    OneZero,
    OneZeroFour,
    OpenBrace,
    OpenBracket,
    OpenParen,
    Override,
    Partial,
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
    Promise,
    Public,
    Pure,
    Question,
    Quote,
    RawIdentifier,
    Receive,
    Reference,
    Relocatable,
    ReservedKeyword,
    Return,
    Returns,
    Revert,
    Run,
    Runs,
    Sealed,
    Seconds,
    Semicolon,
    Seven,
    SevenTwo,
    SignedFixedType,
    SignedIntegerType,
    SingleLineComment,
    SingleQuotedAsciiStringLiteral,
    SingleQuotedUnicodeStringLiteral,
    Six,
    SixFour,
    Sizeof,
    Slash,
    SlashEqual,
    SlashSlash,
    SlashStar,
    Solidity,
    Star,
    StarEqual,
    StarSlash,
    StarStar,
    Static,
    Storage,
    String,
    Struct,
    Supports,
    Switch,
    Szabo,
    Three,
    ThreeOne,
    ThreeTwo,
    ThreeZero,
    Tilde,
    True,
    Try,
    Two,
    TwoEight,
    TwoFive,
    TwoFiveSix,
    TwoFour,
    TwoFourEight,
    TwoFourZero,
    TwoNine,
    TwoOne,
    TwoOneSix,
    TwoSeven,
    TwoSix,
    TwoThree,
    TwoThreeTwo,
    TwoTwo,
    TwoTwoFour,
    TwoZero,
    TwoZeroEight,
    TwoZeroZero,
    Type,
    Typedef,
    Typeof,
    Unchecked,
    Underscore,
    UnicodeDoubleQuote,
    UnicodeEscape,
    UnicodeQuote,
    UnicodeStringLiteral,
    UnsignedFixedType,
    UnsignedIntegerType,
    Using,
    Var,
    VersionPragmaOperator,
    VersionPragmaValue,
    View,
    Virtual,
    Weeks,
    Wei,
    While,
    Whitespace,
    Years,
    YulDecimalNumberLiteral,
    YulHexLiteral,
    YulIdentifier,
    YulKeyword,
    Zero,
    ZeroX,
}

#[derive(Debug, PartialEq, Eq, Serialize)]
#[napi]
pub enum RuleKind {
    ABICoderPragma,
    Abicoder,
    Abstract,
    AddSubExpression,
    Address,
    AddressType,
    Ampersand,
    AmpersandAmpersand,
    AmpersandEqual,
    AndExpression,
    Anonymous,
    ArgumentList,
    Arguments,
    ArrayLiteral,
    As,
    Assembly,
    AssemblyFlags,
    AssemblyStatement,
    AssignmentExpression,
    Bang,
    BangEqual,
    BitAndExpression,
    BitOrExpression,
    BitXOrExpression,
    Block,
    Bool,
    Break,
    BreakStatement,
    Calldata,
    Caret,
    CaretEqual,
    Case,
    Catch,
    CatchClause,
    CatchClauses,
    Colon,
    ColonEqual,
    ConditionalExpression,
    Constant,
    ConstantDefinition,
    Constructor,
    ConstructorAttribute,
    ConstructorAttributes,
    ConstructorDefinition,
    Continue,
    ContinueStatement,
    Contract,
    ContractBodyElement,
    ContractBodyElements,
    ContractDefinition,
    DataLocation,
    Default,
    Definition,
    Delete,
    DeleteStatement,
    DelimitedArguments,
    DelimitedContractBodyElements,
    DelimitedExpression,
    DelimitedExpressions,
    DelimitedSeparatedErrorParameters,
    DelimitedSeparatedEventParameters,
    DelimitedSeparatedIdentifierPaths,
    DelimitedSeparatedIdentifiers,
    DelimitedSeparatedNamedArguments,
    DelimitedSeparatedSelectedImports,
    DelimitedSeparatedYulExpressions,
    DelimitedStructMembers,
    DelimitedTypeName,
    Directive,
    Do,
    DoWhileStatement,
    DoubleQuoteEvmasmDoubleQuote,
    ElementaryType,
    Else,
    Emit,
    EmitStatement,
    EndOfFileTrivia,
    Enum,
    EnumDefinition,
    Equal,
    EqualEqual,
    EqualGreater,
    EqualityComparisonExpression,
    Error,
    ErrorDefinition,
    ErrorParameter,
    Event,
    EventDefinition,
    EventParameter,
    Experimental,
    ExperimentalPragma,
    ExponentiationExpression,
    Expression,
    ExpressionStatement,
    External,
    Fallback,
    FallbackFunctionAttribute,
    FallbackFunctionAttributes,
    FallbackFunctionDefinition,
    For,
    ForStatement,
    From,
    Function,
    FunctionAttribute,
    FunctionAttributes,
    FunctionCallExpression,
    FunctionDefinition,
    FunctionType,
    Global,
    Greater,
    GreaterEqual,
    GreaterGreater,
    GreaterGreaterEqual,
    GreaterGreaterGreater,
    GreaterGreaterGreaterEqual,
    IdentifierPath,
    If,
    IfStatement,
    Immutable,
    Import,
    ImportDirective,
    ImportPath,
    IndexAccessExpression,
    Indexed,
    InheritanceSpecifier,
    InheritanceSpecifierList,
    Interface,
    InterfaceDefinition,
    Internal,
    Is,
    LeadingTrivia,
    Less,
    LessEqual,
    LessLess,
    LessLessEqual,
    Let,
    Library,
    LibraryDefinition,
    Mapping,
    MappingType,
    MemberAccessExpression,
    Memory,
    Minus,
    MinusEqual,
    MinusGreater,
    MinusMinus,
    Modifier,
    ModifierAttribute,
    ModifierAttributes,
    ModifierDefinition,
    ModifierInvocation,
    MulDivModExpression,
    NamedArgument,
    NamedArgumentList,
    New,
    NewExpression,
    NumericLiteral,
    OrExpression,
    OrderComparisonExpression,
    Override,
    OverrideSpecifier,
    ParameterDeclaration,
    ParameterList,
    ParenthesisExpression,
    Payable,
    PayableExpression,
    Percent,
    PercentEqual,
    Period,
    Pipe,
    PipeEqual,
    PipePipe,
    Plus,
    PlusEqual,
    PlusPlus,
    PositionalArgumentList,
    Pragma,
    PragmaDirective,
    PrimaryExpression,
    Private,
    Public,
    Pure,
    Question,
    Receive,
    ReceiveFunctionAttribute,
    ReceiveFunctionAttributes,
    ReceiveFunctionDefinition,
    Results,
    Return,
    ReturnStatement,
    Returns,
    Revert,
    RevertStatement,
    SelectedImport,
    SelectingImportDirective,
    Semicolon,
    SeparatedDoubleQuotedAsciiStringLiterals,
    SeparatedErrorParameters,
    SeparatedEventParameters,
    SeparatedExpressions,
    SeparatedIdentifierPaths,
    SeparatedIdentifiers,
    SeparatedInheritanceSpecifiers,
    SeparatedNamedArguments,
    SeparatedParameterDeclarations,
    SeparatedSelectedImports,
    SeparatedVersionPragmaValues,
    SeparatedYulExpressions,
    SeparatedYulIdentifierPaths,
    ShiftExpression,
    SimpleImportDirective,
    SimpleStatement,
    Slash,
    SlashEqual,
    Solidity,
    SourceUnit,
    Star,
    StarEqual,
    StarImportDirective,
    StarStar,
    StateVariableAttribute,
    StateVariableAttributes,
    StateVariableDeclaration,
    Statement,
    Storage,
    String,
    Struct,
    StructDefinition,
    StructMember,
    StructMembers,
    Switch,
    Tilde,
    TrailingTrivia,
    Try,
    TryStatement,
    TupleDeconstructionStatement,
    Type,
    TypeExpression,
    TypeName,
    UnaryPrefixExpression,
    UnarySuffixExpression,
    Unchecked,
    UncheckedBlock,
    UserDefinedValueTypeDefinition,
    Using,
    UsingDirective,
    VariableDeclarationStatement,
    VersionPragma,
    VersionPragmaSpecifier,
    VersionPragmaSpecifiers,
    View,
    Virtual,
    While,
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
    YulStatements,
    YulSwitchStatement,
    YulVariableDeclaration,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize)]
#[napi]
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

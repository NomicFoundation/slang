// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use napi::bindgen_prelude::*;
use napi_derive::napi;
use serde::Serialize;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize)]
#[napi]
pub enum TokenKind {
    AbicoderKeyword,
    AbstractKeyword,
    AddressKeyword,
    Ampersand,
    AmpersandAmpersand,
    AmpersandEqual,
    AnonymousKeyword,
    AsKeyword,
    AsciiEscape,
    AsciiStringLiteral,
    AssemblyKeyword,
    Bang,
    BangEqual,
    BoolKeyword,
    BreakKeyword,
    CalldataKeyword,
    Caret,
    CaretEqual,
    CaseKeyword,
    CatchKeyword,
    CloseBrace,
    CloseBracket,
    CloseParen,
    Colon,
    ColonEqual,
    Comma,
    ConstantKeyword,
    ConstructorKeyword,
    ContinueKeyword,
    ContractKeyword,
    DaysKeyword,
    DecimalExponent,
    DecimalLiteral,
    DecimalNumber,
    DefaultKeyword,
    DeleteKeyword,
    DoKeyword,
    DoubleQuoteEvmasmDoubleQuote,
    DoubleQuotedAsciiStringLiteral,
    DoubleQuotedUnicodeStringLiteral,
    ElseKeyword,
    EmitKeyword,
    EndOfLine,
    EnumKeyword,
    Equal,
    EqualEqual,
    EqualGreater,
    ErrorKeyword,
    EscapeSequence,
    EtherKeyword,
    EventKeyword,
    ExperimentalKeyword,
    ExternalKeyword,
    FallbackKeyword,
    FalseKeyword,
    FinneyKeyword,
    FixedBytesType,
    ForKeyword,
    FromKeyword,
    FunctionKeyword,
    GlobalKeyword,
    Greater,
    GreaterEqual,
    GreaterGreater,
    GreaterGreaterEqual,
    GreaterGreaterGreater,
    GreaterGreaterGreaterEqual,
    GweiKeyword,
    HexByteEscape,
    HexCharacter,
    HexLiteral,
    HexStringLiteral,
    HoursKeyword,
    Identifier,
    IdentifierPart,
    IdentifierStart,
    IfKeyword,
    ImmutableKeyword,
    ImportKeyword,
    IndexedKeyword,
    InterfaceKeyword,
    InternalKeyword,
    IsKeyword,
    Keyword,
    LeaveKeyword,
    Less,
    LessEqual,
    LessLess,
    LessLessEqual,
    LetKeyword,
    LibraryKeyword,
    MappingKeyword,
    MemoryKeyword,
    Minus,
    MinusEqual,
    MinusGreater,
    MinusMinus,
    MinutesKeyword,
    ModifierKeyword,
    MultilineComment,
    NewKeyword,
    OpenBrace,
    OpenBracket,
    OpenParen,
    OverrideKeyword,
    PayableKeyword,
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
    PragmaKeyword,
    PrivateKeyword,
    PublicKeyword,
    PureKeyword,
    Question,
    RawIdentifier,
    ReceiveKeyword,
    ReservedKeyword,
    ReturnKeyword,
    ReturnsKeyword,
    RevertKeyword,
    SecondsKeyword,
    Semicolon,
    SignedFixedType,
    SignedIntegerType,
    SingleLineComment,
    SingleQuotedAsciiStringLiteral,
    SingleQuotedUnicodeStringLiteral,
    Slash,
    SlashEqual,
    SolidityKeyword,
    Star,
    StarEqual,
    StarStar,
    StorageKeyword,
    StringKeyword,
    StructKeyword,
    SwitchKeyword,
    SzaboKeyword,
    Tilde,
    TrueKeyword,
    TryKeyword,
    TypeKeyword,
    UncheckedKeyword,
    UnicodeEscape,
    UnicodeStringLiteral,
    UnsignedFixedType,
    UnsignedIntegerType,
    UsingKeyword,
    VersionPragmaOperator,
    VersionPragmaValue,
    ViewKeyword,
    VirtualKeyword,
    WeeksKeyword,
    WeiKeyword,
    WhileKeyword,
    Whitespace,
    YearsKeyword,
    YulDecimalLiteral,
    YulHexLiteral,
    YulIdentifier,
    YulKeyword,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize)]
#[napi]
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
    MappingKeyType,
    MappingType,
    MappingValueType,
    MemberAccessExpression,
    ModifierAttribute,
    ModifierDefinition,
    ModifierInvocation,
    MulDivModExpression,
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
    PayableType,
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
    StringExpression,
    StructDefinition,
    StructMember,
    TrailingTrivia,
    TryStatement,
    TupleDeconstructionStatement,
    TupleExpression,
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
    _DELIMITEDBY,
    _OPTIONAL,
    _REPEATED,
    _SEPARATEDBY,
    _SEQUENCE,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize)]
#[napi]
pub enum ProductionKind {
    ABICoderPragma,
    AbicoderKeyword,
    AbstractKeyword,
    AddressKeyword,
    AddressType,
    AnonymousKeyword,
    ArgumentList,
    ArrayLiteral,
    AsKeyword,
    AsciiEscape,
    AsciiStringLiteral,
    AssemblyFlags,
    AssemblyKeyword,
    AssemblyStatement,
    Block,
    BoolKeyword,
    BooleanLiteral,
    BreakKeyword,
    BreakStatement,
    CalldataKeyword,
    CaseKeyword,
    CatchClause,
    CatchKeyword,
    ConstantDefinition,
    ConstantKeyword,
    ConstructorAttribute,
    ConstructorDefinition,
    ConstructorKeyword,
    ContinueKeyword,
    ContinueStatement,
    ContractBodyElement,
    ContractDefinition,
    ContractKeyword,
    DataLocation,
    DaysKeyword,
    DecimalExponent,
    DecimalLiteral,
    DecimalNumber,
    DefaultKeyword,
    Definition,
    DeleteKeyword,
    DeleteStatement,
    Directive,
    DoKeyword,
    DoWhileStatement,
    DoubleQuotedAsciiStringLiteral,
    DoubleQuotedUnicodeStringLiteral,
    ElementaryType,
    ElseKeyword,
    EmitKeyword,
    EmitStatement,
    EndOfFileTrivia,
    EndOfLine,
    EnumDefinition,
    EnumKeyword,
    ErrorDefinition,
    ErrorKeyword,
    ErrorParameter,
    EscapeSequence,
    EtherKeyword,
    EventDefinition,
    EventKeyword,
    EventParameter,
    ExperimentalKeyword,
    ExperimentalPragma,
    Expression,
    ExpressionStatement,
    ExternalKeyword,
    FallbackFunctionAttribute,
    FallbackFunctionDefinition,
    FallbackKeyword,
    FalseKeyword,
    FinneyKeyword,
    FixedBytesType,
    ForKeyword,
    ForStatement,
    FromKeyword,
    FunctionAttribute,
    FunctionDefinition,
    FunctionKeyword,
    FunctionType,
    GlobalKeyword,
    GweiKeyword,
    HexByteEscape,
    HexCharacter,
    HexLiteral,
    HexStringLiteral,
    HoursKeyword,
    Identifier,
    IdentifierPart,
    IdentifierPath,
    IdentifierStart,
    IfKeyword,
    IfStatement,
    ImmutableKeyword,
    ImportDirective,
    ImportKeyword,
    ImportPath,
    IndexedKeyword,
    InheritanceSpecifier,
    InheritanceSpecifierList,
    InterfaceDefinition,
    InterfaceKeyword,
    InternalKeyword,
    IsKeyword,
    Keyword,
    LeadingTrivia,
    LeaveKeyword,
    LetKeyword,
    LibraryDefinition,
    LibraryKeyword,
    MappingKeyType,
    MappingKeyword,
    MappingType,
    MappingValueType,
    MemoryKeyword,
    MinutesKeyword,
    ModifierAttribute,
    ModifierDefinition,
    ModifierInvocation,
    ModifierKeyword,
    MultilineComment,
    NamedArgument,
    NamedArgumentList,
    NewExpression,
    NewKeyword,
    NumberUnit,
    NumericLiteral,
    OverrideKeyword,
    OverrideSpecifier,
    ParameterDeclaration,
    ParameterList,
    PayableKeyword,
    PayableType,
    PositionalArgumentList,
    PossiblySeparatedPairsOfHexDigits,
    PragmaDirective,
    PragmaKeyword,
    PrimaryExpression,
    PrivateKeyword,
    PublicKeyword,
    PureKeyword,
    RawIdentifier,
    ReceiveFunctionAttribute,
    ReceiveFunctionDefinition,
    ReceiveKeyword,
    ReservedKeyword,
    ReturnKeyword,
    ReturnStatement,
    ReturnsKeyword,
    RevertKeyword,
    RevertStatement,
    SecondsKeyword,
    SelectedImport,
    SelectingImportDirective,
    SignedFixedType,
    SignedIntegerType,
    SimpleImportDirective,
    SimpleStatement,
    SingleLineComment,
    SingleQuotedAsciiStringLiteral,
    SingleQuotedUnicodeStringLiteral,
    SolidityKeyword,
    SourceUnit,
    StarImportDirective,
    StateVariableAttribute,
    StateVariableDeclaration,
    Statement,
    StorageKeyword,
    StringExpression,
    StringKeyword,
    StructDefinition,
    StructKeyword,
    StructMember,
    SwitchKeyword,
    SzaboKeyword,
    TrailingTrivia,
    TrueKeyword,
    TryKeyword,
    TryStatement,
    TupleDeconstructionStatement,
    TupleExpression,
    TypeExpression,
    TypeKeyword,
    TypeName,
    UncheckedBlock,
    UncheckedKeyword,
    UnicodeEscape,
    UnicodeStringLiteral,
    UnsignedFixedType,
    UnsignedIntegerType,
    UserDefinedValueTypeDefinition,
    UsingDirective,
    UsingKeyword,
    VariableDeclarationStatement,
    VersionPragma,
    VersionPragmaOperator,
    VersionPragmaSpecifier,
    VersionPragmaValue,
    ViewKeyword,
    VirtualKeyword,
    WeeksKeyword,
    WeiKeyword,
    WhileKeyword,
    WhileStatement,
    Whitespace,
    YearsKeyword,
    YulAssignmentStatement,
    YulBlock,
    YulBreakStatement,
    YulContinueStatement,
    YulDecimalLiteral,
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

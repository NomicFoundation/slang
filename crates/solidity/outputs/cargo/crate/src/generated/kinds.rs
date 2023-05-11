// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use serde::Serialize;
use strum_macros::*;
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize)]
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
    Asterisk,
    AsteriskAsterisk,
    AsteriskEqual,
    Bang,
    BangEqual,
    Bar,
    BarBar,
    BarEqual,
    BoolKeyword,
    BreakKeyword,
    ByteType,
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
    DoubleQuotedAsciiStringLiteral,
    DoubleQuotedUnicodeStringLiteral,
    ElseKeyword,
    EmitKeyword,
    EndOfLine,
    EnumKeyword,
    Equal,
    EqualEqual,
    EqualGreaterThan,
    ErrorKeyword,
    EscapeSequence,
    EtherKeyword,
    EventKeyword,
    Evmasm,
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
    GreaterThan,
    GreaterThanEqual,
    GreaterThanGreaterThan,
    GreaterThanGreaterThanEqual,
    GreaterThanGreaterThanGreaterThan,
    GreaterThanGreaterThanGreaterThanEqual,
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
    LeaveKeyword,
    LessThan,
    LessThanEqual,
    LessThanLessThan,
    LessThanLessThanEqual,
    LetKeyword,
    LibraryKeyword,
    MappingKeyword,
    MemoryKeyword,
    Minus,
    MinusEqual,
    MinusGreaterThan,
    MinusMinus,
    MinutesKeyword,
    ModifierKeyword,
    MultilineComment,
    NewKeyword,
    NotAnIdentifierInAnyVersion,
    NotAnIdentifierInSomeVersions,
    OpenBrace,
    OpenBracket,
    OpenParen,
    OverrideKeyword,
    PayableKeyword,
    Percent,
    PercentEqual,
    Period,
    Plus,
    PlusEqual,
    PlusPlus,
    PossiblySeparatedPairsOfHexDigits,
    PragmaKeyword,
    PrivateKeyword,
    PublicKeyword,
    PureKeyword,
    QuestionMark,
    RawIdentifier,
    ReceiveKeyword,
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
    StorageKeyword,
    StringKeyword,
    StructKeyword,
    SwitchKeyword,
    SzaboKeyword,
    ThrowKeyword,
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
    VarKeyword,
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
    YulReservedKeyword,
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
    ArrayTypeName,
    AssemblyFlags,
    AssemblyStatement,
    AssignmentExpression,
    AsteriskImport,
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
    FunctionCallOptions,
    FunctionDefinition,
    FunctionType,
    IdentifierPath,
    IfStatement,
    ImportAlias,
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
    NumericExpression,
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
    SelectiveImport,
    ShiftExpression,
    SimpleImport,
    SimpleStatement,
    SourceUnit,
    StateVariableAttribute,
    StateVariableDeclaration,
    Statement,
    StringExpression,
    StructDefinition,
    StructMember,
    ThrowStatement,
    TrailingTrivia,
    TryStatement,
    TupleDeconstructionStatement,
    TupleExpression,
    TypeExpression,
    TypeName,
    UnaryPostfixExpression,
    UnaryPrefixExpression,
    UncheckedBlock,
    UnnamedFunctionAttribute,
    UnnamedFunctionDefinition,
    UserDefinedOperator,
    UserDefinedValueTypeDefinition,
    UsingDirective,
    VariableDeclarationStatement,
    VersionPragma,
    VersionPragmaAlternatives,
    VersionPragmaComparator,
    VersionPragmaExpression,
    VersionPragmaRange,
    VersionPragmaSpecifier,
    WhileStatement,
    YulAssignmentStatement,
    YulBlock,
    YulBreakStatement,
    YulContinueStatement,
    YulDeclarationStatement,
    YulExpression,
    YulForStatement,
    YulFunctionCallExpression,
    YulFunctionDefinition,
    YulIdentifierPath,
    YulIfStatement,
    YulLeaveStatement,
    YulLiteral,
    YulStatement,
    YulSwitchStatement,
    _DELIMITEDBY,
    _OPTIONAL,
    _REPEATED,
    _SEPARATEDBY,
    _SEQUENCE,
    _TERMINATEDBY,
}
#[derive(
    Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, EnumString, AsRefStr, Display,
)]
pub enum ProductionKind {
    ABICoderPragma,
    AbicoderKeyword,
    AbstractKeyword,
    AddressKeyword,
    AddressType,
    Ampersand,
    AmpersandAmpersand,
    AmpersandEqual,
    AnonymousKeyword,
    ArgumentList,
    ArrayLiteral,
    AsKeyword,
    AsciiEscape,
    AsciiStringLiteral,
    AssemblyFlags,
    AssemblyKeyword,
    AssemblyStatement,
    Asterisk,
    AsteriskAsterisk,
    AsteriskEqual,
    AsteriskImport,
    Bang,
    BangEqual,
    Bar,
    BarBar,
    BarEqual,
    Block,
    BoolKeyword,
    BooleanLiteral,
    BreakKeyword,
    BreakStatement,
    ByteType,
    CalldataKeyword,
    Caret,
    CaretEqual,
    CaseKeyword,
    CatchClause,
    CatchKeyword,
    CloseBrace,
    CloseBracket,
    CloseParen,
    Colon,
    ColonEqual,
    Comma,
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
    Equal,
    EqualEqual,
    EqualGreaterThan,
    ErrorDefinition,
    ErrorKeyword,
    ErrorParameter,
    EscapeSequence,
    EtherKeyword,
    EventDefinition,
    EventKeyword,
    EventParameter,
    Evmasm,
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
    FunctionCallOptions,
    FunctionDefinition,
    FunctionKeyword,
    FunctionType,
    GlobalKeyword,
    GreaterThan,
    GreaterThanEqual,
    GreaterThanGreaterThan,
    GreaterThanGreaterThanEqual,
    GreaterThanGreaterThanGreaterThan,
    GreaterThanGreaterThanGreaterThanEqual,
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
    ImportAlias,
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
    LeadingTrivia,
    LeaveKeyword,
    LessThan,
    LessThanEqual,
    LessThanLessThan,
    LessThanLessThanEqual,
    LetKeyword,
    LibraryDefinition,
    LibraryKeyword,
    MappingKeyType,
    MappingKeyword,
    MappingType,
    MappingValueType,
    MemoryKeyword,
    Minus,
    MinusEqual,
    MinusGreaterThan,
    MinusMinus,
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
    NotAnIdentifierInAnyVersion,
    NotAnIdentifierInSomeVersions,
    NumberUnit,
    NumericExpression,
    OpenBrace,
    OpenBracket,
    OpenParen,
    OverrideKeyword,
    OverrideSpecifier,
    ParameterDeclaration,
    ParameterList,
    PayableKeyword,
    PayableType,
    Percent,
    PercentEqual,
    Period,
    Plus,
    PlusEqual,
    PlusPlus,
    PositionalArgumentList,
    PossiblySeparatedPairsOfHexDigits,
    PragmaDirective,
    PragmaKeyword,
    PrimaryExpression,
    PrivateKeyword,
    PublicKeyword,
    PureKeyword,
    QuestionMark,
    RawIdentifier,
    ReceiveFunctionAttribute,
    ReceiveFunctionDefinition,
    ReceiveKeyword,
    ReturnKeyword,
    ReturnStatement,
    ReturnsKeyword,
    RevertKeyword,
    RevertStatement,
    SecondsKeyword,
    SelectiveImport,
    Semicolon,
    SignedFixedType,
    SignedIntegerType,
    SimpleImport,
    SimpleStatement,
    SingleLineComment,
    SingleQuotedAsciiStringLiteral,
    SingleQuotedUnicodeStringLiteral,
    Slash,
    SlashEqual,
    SolidityKeyword,
    SourceUnit,
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
    ThrowKeyword,
    ThrowStatement,
    Tilde,
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
    UnnamedFunctionAttribute,
    UnnamedFunctionDefinition,
    UnsignedFixedType,
    UnsignedIntegerType,
    UserDefinedOperator,
    UserDefinedValueTypeDefinition,
    UsingDirective,
    UsingKeyword,
    VarKeyword,
    VariableDeclarationStatement,
    VersionPragma,
    VersionPragmaExpression,
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
    YulDeclarationStatement,
    YulExpression,
    YulForStatement,
    YulFunctionDefinition,
    YulHexLiteral,
    YulIdentifier,
    YulIdentifierPath,
    YulIfStatement,
    YulKeyword,
    YulLeaveStatement,
    YulLiteral,
    YulReservedKeyword,
    YulStatement,
    YulSwitchStatement,
}

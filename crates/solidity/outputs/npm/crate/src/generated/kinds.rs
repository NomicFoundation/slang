// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

#[cfg(feature = "slang_napi_interfaces")]
use {napi::bindgen_prelude::*, napi_derive::napi};

#[derive(
    Debug,
    Eq,
    Ord,
    PartialEq,
    PartialOrd,
    serde::Serialize,
    strum_macros::AsRefStr,
    strum_macros::Display,
    strum_macros::EnumString,
)]
#[cfg_attr(feature = "slang_napi_interfaces", /* derives `Clone` and `Copy` */ napi(string_enum, namespace = "kinds"))]
#[cfg_attr(not(feature = "slang_napi_interfaces"), derive(Clone, Copy))]
pub enum ProductionKind {
    ABICoderPragma,
    AddressType,
    ArgumentsDeclaration,
    ArrayExpression,
    ArrayValues,
    AsciiStringLiterals,
    AssemblyFlags,
    AssemblyFlagsDeclaration,
    AssemblyStatement,
    Block,
    BreakStatement,
    CatchClause,
    CatchClauseError,
    CatchClauses,
    ConstantDefinition,
    ConstructorAttributes,
    ConstructorDefinition,
    ContinueStatement,
    ContractDefinition,
    ContractMembers,
    DecimalNumberExpression,
    DeconstructionImport,
    DeconstructionImportSymbol,
    DeconstructionImportSymbols,
    DeleteStatement,
    DoWhileStatement,
    EmitStatement,
    EndOfFileTrivia,
    EnumDefinition,
    EnumMembers,
    ErrorDefinition,
    ErrorParameter,
    ErrorParameters,
    ErrorParametersDeclaration,
    EventDefinition,
    EventParameter,
    EventParameters,
    EventParametersDeclaration,
    ExperimentalPragma,
    Expression,
    ExpressionStatement,
    FallbackFunctionAttributes,
    FallbackFunctionDefinition,
    ForStatement,
    FunctionAttributes,
    FunctionCallOptions,
    FunctionDefinition,
    FunctionType,
    FunctionTypeAttributes,
    HexNumberExpression,
    HexStringLiterals,
    IdentifierPath,
    IfStatement,
    ImportDirective,
    InheritanceSpecifier,
    InheritanceType,
    InheritanceTypes,
    InterfaceDefinition,
    InterfaceMembers,
    LeadingTrivia,
    LibraryDefinition,
    LibraryMembers,
    MappingKeyType,
    MappingType,
    MappingValueType,
    ModifierAttributes,
    ModifierDefinition,
    ModifierInvocation,
    NamedArgument,
    NamedArguments,
    NamedArgumentsDeclaration,
    NamedImport,
    NewExpression,
    OverridePaths,
    OverrideSpecifier,
    Parameter,
    Parameters,
    ParametersDeclaration,
    PathImport,
    PositionalArguments,
    PragmaDirective,
    ReceiveFunctionAttributes,
    ReceiveFunctionDefinition,
    ReturnStatement,
    ReturnsDeclaration,
    RevertStatement,
    SourceUnit,
    SourceUnitMembers,
    StateVariableAttributes,
    StateVariableDefinition,
    Statements,
    StructDefinition,
    StructMember,
    StructMembers,
    ThrowStatement,
    TrailingTrivia,
    TryStatement,
    TupleDeconstructionStatement,
    TupleExpression,
    TupleMember,
    TupleMembers,
    TupleValues,
    TypeExpression,
    TypeName,
    UncheckedBlock,
    UnicodeStringLiterals,
    UnnamedFunctionAttributes,
    UnnamedFunctionDefinition,
    UserDefinedValueTypeDefinition,
    UsingDirective,
    UsingDirectiveDeconstruction,
    UsingDirectiveSymbol,
    UsingDirectiveSymbols,
    VariableDeclaration,
    VariableDeclarationStatement,
    VersionPragma,
    VersionPragmaExpression,
    VersionPragmaExpressions,
    VersionPragmaSpecifier,
    WhileStatement,
    YulArguments,
    YulAssignmentStatement,
    YulBlock,
    YulBreakStatement,
    YulContinueStatement,
    YulDeclarationStatement,
    YulExpression,
    YulForStatement,
    YulFunctionDefinition,
    YulIdentifierPath,
    YulIdentifierPaths,
    YulIdentifiers,
    YulIfStatement,
    YulLeaveStatement,
    YulParametersDeclaration,
    YulReturnsDeclaration,
    YulStatements,
    YulSwitchCase,
    YulSwitchCases,
    YulSwitchStatement,
}

#[derive(
    Debug,
    Eq,
    Ord,
    PartialEq,
    PartialOrd,
    serde::Serialize,
    strum_macros::AsRefStr,
    strum_macros::Display,
    strum_macros::EnumString,
)]
#[cfg_attr(feature = "slang_napi_interfaces", /* derives `Clone` and `Copy` */ napi(string_enum, namespace = "kinds"))]
#[cfg_attr(not(feature = "slang_napi_interfaces"), derive(Clone, Copy))]
pub enum RuleKind {
    ABICoderPragma,
    AddressType,
    ArgumentsDeclaration,
    ArrayExpression,
    ArrayTypeName,
    ArrayValues,
    AsciiStringLiterals,
    AssemblyFlags,
    AssemblyFlagsDeclaration,
    AssemblyStatement,
    BinaryExpression,
    Block,
    BreakStatement,
    CatchClause,
    CatchClauseError,
    CatchClauses,
    ConditionalExpression,
    ConstantDefinition,
    ConstructorAttributes,
    ConstructorDefinition,
    ContinueStatement,
    ContractDefinition,
    ContractMembers,
    DecimalNumberExpression,
    DeconstructionImport,
    DeconstructionImportSymbol,
    DeconstructionImportSymbols,
    DeleteStatement,
    DoWhileStatement,
    EmitStatement,
    EndOfFileTrivia,
    EnumDefinition,
    EnumMembers,
    ErrorDefinition,
    ErrorParameter,
    ErrorParameters,
    ErrorParametersDeclaration,
    EventDefinition,
    EventParameter,
    EventParameters,
    EventParametersDeclaration,
    ExperimentalPragma,
    Expression,
    ExpressionStatement,
    FallbackFunctionAttributes,
    FallbackFunctionDefinition,
    ForStatement,
    FunctionAttributes,
    FunctionCallExpression,
    FunctionCallOptions,
    FunctionDefinition,
    FunctionType,
    FunctionTypeAttributes,
    HexNumberExpression,
    HexStringLiterals,
    IdentifierPath,
    IfStatement,
    ImportDirective,
    IndexAccessExpression,
    InheritanceSpecifier,
    InheritanceType,
    InheritanceTypes,
    InterfaceDefinition,
    InterfaceMembers,
    LeadingTrivia,
    LibraryDefinition,
    LibraryMembers,
    MappingKeyType,
    MappingType,
    MappingValueType,
    MemberAccessExpression,
    ModifierAttributes,
    ModifierDefinition,
    ModifierInvocation,
    NamedArgument,
    NamedArguments,
    NamedArgumentsDeclaration,
    NamedImport,
    NewExpression,
    OverridePaths,
    OverrideSpecifier,
    Parameter,
    Parameters,
    ParametersDeclaration,
    PathImport,
    PositionalArguments,
    PragmaDirective,
    ReceiveFunctionAttributes,
    ReceiveFunctionDefinition,
    ReturnStatement,
    ReturnsDeclaration,
    RevertStatement,
    SourceUnit,
    SourceUnitMembers,
    StateVariableAttributes,
    StateVariableDefinition,
    Statements,
    StructDefinition,
    StructMember,
    StructMembers,
    ThrowStatement,
    TrailingTrivia,
    TryStatement,
    TupleDeconstructionStatement,
    TupleExpression,
    TupleMember,
    TupleMembers,
    TupleValues,
    TypeExpression,
    TypeName,
    UnaryPostfixExpression,
    UnaryPrefixExpression,
    UncheckedBlock,
    UnicodeStringLiterals,
    UnnamedFunctionAttributes,
    UnnamedFunctionDefinition,
    UserDefinedValueTypeDefinition,
    UsingDirective,
    UsingDirectiveDeconstruction,
    UsingDirectiveSymbol,
    UsingDirectiveSymbols,
    VariableDeclaration,
    VariableDeclarationStatement,
    VersionPragma,
    VersionPragmaBinaryExpression,
    VersionPragmaExpression,
    VersionPragmaExpressions,
    VersionPragmaSpecifier,
    VersionPragmaUnaryExpression,
    WhileStatement,
    YulArguments,
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
    YulIdentifierPaths,
    YulIdentifiers,
    YulIfStatement,
    YulLeaveStatement,
    YulParametersDeclaration,
    YulReturnsDeclaration,
    YulStatements,
    YulSwitchCase,
    YulSwitchCases,
    YulSwitchStatement,
}

impl RuleKind {
    pub fn is_trivia(&self) -> bool {
        match self {
            Self::EndOfFileTrivia => true,
            Self::LeadingTrivia => true,
            Self::TrailingTrivia => true,
            _ => false,
        }
    }
}

#[derive(
    Debug,
    Eq,
    Ord,
    PartialEq,
    PartialOrd,
    serde::Serialize,
    strum_macros::AsRefStr,
    strum_macros::Display,
    strum_macros::EnumString,
)]
#[cfg_attr(feature = "slang_napi_interfaces", /* derives `Clone` and `Copy` */ napi(string_enum, namespace = "kinds"))]
#[cfg_attr(not(feature = "slang_napi_interfaces"), derive(Clone, Copy))]
pub enum TokenKind {
    SKIPPED,
    AbicoderKeyword,
    AbstractKeyword,
    AddressKeyword,
    AfterKeyword,
    AliasKeyword,
    Ampersand,
    AmpersandAmpersand,
    AmpersandEqual,
    AnonymousKeyword,
    ApplyKeyword,
    AsKeyword,
    AsciiStringLiteral,
    AssemblyKeyword,
    Asterisk,
    AsteriskAsterisk,
    AsteriskEqual,
    AutoKeyword,
    Bang,
    BangEqual,
    Bar,
    BarBar,
    BarEqual,
    BoolKeyword,
    BreakKeyword,
    ByteKeyword,
    BytesKeyword,
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
    CopyofKeyword,
    DaysKeyword,
    DecimalLiteral,
    DefaultKeyword,
    DefineKeyword,
    DeleteKeyword,
    DoKeyword,
    ElseKeyword,
    EmitKeyword,
    EndOfLine,
    EnumKeyword,
    Equal,
    EqualEqual,
    EqualGreaterThan,
    ErrorKeyword,
    EtherKeyword,
    EventKeyword,
    ExperimentalKeyword,
    ExternalKeyword,
    FallbackKeyword,
    FalseKeyword,
    FinalKeyword,
    FinneyKeyword,
    FixedKeyword,
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
    HexKeyword,
    HexLiteral,
    HexStringLiteral,
    HoursKeyword,
    Identifier,
    IfKeyword,
    ImmutableKeyword,
    ImplementsKeyword,
    ImportKeyword,
    InKeyword,
    IndexedKeyword,
    InlineKeyword,
    IntKeyword,
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
    MacroKeyword,
    MappingKeyword,
    MatchKeyword,
    MemoryKeyword,
    Minus,
    MinusEqual,
    MinusGreaterThan,
    MinusMinus,
    MinutesKeyword,
    ModifierKeyword,
    MultilineComment,
    MutableKeyword,
    NewKeyword,
    NullKeyword,
    OfKeyword,
    OpenBrace,
    OpenBracket,
    OpenParen,
    OverrideKeyword,
    PartialKeyword,
    PayableKeyword,
    Percent,
    PercentEqual,
    Period,
    Plus,
    PlusEqual,
    PlusPlus,
    PragmaKeyword,
    PrivateKeyword,
    PromiseKeyword,
    PublicKeyword,
    PureKeyword,
    QuestionMark,
    ReceiveKeyword,
    ReferenceKeyword,
    RelocatableKeyword,
    ReturnKeyword,
    ReturnsKeyword,
    RevertKeyword,
    SealedKeyword,
    SecondsKeyword,
    Semicolon,
    SingleLineComment,
    SizeofKeyword,
    Slash,
    SlashEqual,
    SolidityKeyword,
    StaticKeyword,
    StorageKeyword,
    StringKeyword,
    StructKeyword,
    SupportsKeyword,
    SwitchKeyword,
    SzaboKeyword,
    ThrowKeyword,
    Tilde,
    TrueKeyword,
    TryKeyword,
    TypeKeyword,
    TypedefKeyword,
    TypeofKeyword,
    UncheckedKeyword,
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
}

#[derive(strum_macros::FromRepr)]
/// The lexical context of the scanner.
#[cfg_attr(feature = "slang_napi_interfaces", /* derives `Clone` and `Copy` */ napi(string_enum, namespace = "language"))]
#[cfg_attr(not(feature = "slang_napi_interfaces"), derive(Clone, Copy))]
pub enum LexicalContext {
    Default,
    VersionPragma,
    YulBlock,
}

/// Marker trait for type-level [`LexicalContext`] variants.
pub trait IsLexicalContext {
    /// Returns a run-time [`LexicalContext`] value.
    fn value() -> LexicalContext;
}

#[allow(non_snake_case)]
pub mod LexicalContextType {
    use super::*;
    pub struct Default {}
    impl IsLexicalContext for Default {
        fn value() -> LexicalContext {
            LexicalContext::Default
        }
    }
    pub struct VersionPragma {}
    impl IsLexicalContext for VersionPragma {
        fn value() -> LexicalContext {
            LexicalContext::VersionPragma
        }
    }
    pub struct YulBlock {}
    impl IsLexicalContext for YulBlock {
        fn value() -> LexicalContext {
            LexicalContext::YulBlock
        }
    }
}

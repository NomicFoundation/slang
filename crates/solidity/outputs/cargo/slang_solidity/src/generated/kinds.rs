// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

#[cfg(feature = "slang_napi_interfaces")]
use napi_derive::napi;

#[derive(
    Debug,
    Eq,
    Hash,
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
    AdditiveExpression,
    AddressType,
    AndExpression,
    ArgumentsDeclaration,
    ArrayExpression,
    ArrayTypeName,
    ArrayValues,
    AssemblyFlags,
    AssemblyFlagsDeclaration,
    AssemblyStatement,
    AssignmentExpression,
    BitwiseAndExpression,
    BitwiseOrExpression,
    BitwiseXorExpression,
    Block,
    BreakStatement,
    CallOptions,
    CallOptionsExpression,
    CatchClause,
    CatchClauseError,
    CatchClauses,
    ComparisonExpression,
    ConditionalExpression,
    ConstantDefinition,
    ConstructorAttribute,
    ConstructorAttributes,
    ConstructorDefinition,
    ContinueStatement,
    ContractDefinition,
    ContractMember,
    ContractMembers,
    DecimalNumberExpression,
    DoWhileStatement,
    ElementaryType,
    ElseBranch,
    EmitStatement,
    EnumDefinition,
    EnumMembers,
    EqualityExpression,
    ErrorDefinition,
    ErrorParameter,
    ErrorParameters,
    ErrorParametersDeclaration,
    EventDefinition,
    EventParameter,
    EventParameters,
    EventParametersDeclaration,
    ExperimentalFeature,
    ExperimentalPragma,
    ExponentiationExpression,
    Expression,
    ExpressionStatement,
    FallbackFunctionAttribute,
    FallbackFunctionAttributes,
    FallbackFunctionDefinition,
    ForStatement,
    ForStatementCondition,
    ForStatementInitialization,
    FunctionAttribute,
    FunctionAttributes,
    FunctionBody,
    FunctionCallExpression,
    FunctionDefinition,
    FunctionName,
    FunctionType,
    FunctionTypeAttribute,
    FunctionTypeAttributes,
    HexNumberExpression,
    HexStringLiteral,
    HexStringLiterals,
    IdentifierPath,
    IfStatement,
    ImportAlias,
    ImportClause,
    ImportDeconstruction,
    ImportDeconstructionSymbol,
    ImportDeconstructionSymbols,
    ImportDirective,
    IndexAccessEnd,
    IndexAccessExpression,
    InheritanceSpecifier,
    InheritanceType,
    InheritanceTypes,
    InterfaceDefinition,
    InterfaceMembers,
    LibraryDefinition,
    LibraryMembers,
    MappingKey,
    MappingKeyType,
    MappingType,
    MappingValue,
    MemberAccess,
    MemberAccessExpression,
    ModifierAttribute,
    ModifierAttributes,
    ModifierDefinition,
    ModifierInvocation,
    MultiplicativeExpression,
    NamedArgument,
    NamedArgumentGroup,
    NamedArguments,
    NamedArgumentsDeclaration,
    NamedImport,
    NewExpression,
    NumberUnit,
    OrExpression,
    OverridePaths,
    OverridePathsDeclaration,
    OverrideSpecifier,
    Parameter,
    Parameters,
    ParametersDeclaration,
    PathImport,
    PositionalArguments,
    PositionalArgumentsDeclaration,
    PostfixExpression,
    Pragma,
    PragmaDirective,
    PrefixExpression,
    ReceiveFunctionAttribute,
    ReceiveFunctionAttributes,
    ReceiveFunctionDefinition,
    ReturnStatement,
    ReturnsDeclaration,
    RevertStatement,
    ShiftExpression,
    SourceUnit,
    SourceUnitMember,
    SourceUnitMembers,
    StateVariableAttribute,
    StateVariableAttributes,
    StateVariableDefinition,
    StateVariableDefinitionValue,
    Statement,
    Statements,
    StorageLocation,
    StringExpression,
    StringLiteral,
    StringLiterals,
    StructDefinition,
    StructMember,
    StructMembers,
    ThrowStatement,
    TryStatement,
    TupleDeconstructionElement,
    TupleDeconstructionElements,
    TupleDeconstructionStatement,
    TupleExpression,
    TupleMember,
    TupleValue,
    TupleValues,
    TypeExpression,
    TypeName,
    TypedTupleMember,
    UncheckedBlock,
    UnicodeStringLiteral,
    UnicodeStringLiterals,
    UnnamedFunctionAttribute,
    UnnamedFunctionAttributes,
    UnnamedFunctionDefinition,
    UntypedTupleMember,
    UserDefinedValueTypeDefinition,
    UsingAlias,
    UsingClause,
    UsingDeconstruction,
    UsingDeconstructionSymbol,
    UsingDeconstructionSymbols,
    UsingDirective,
    UsingOperator,
    UsingTarget,
    VariableDeclarationStatement,
    VariableDeclarationType,
    VariableDeclarationValue,
    VersionComparator,
    VersionExpression,
    VersionExpressionSet,
    VersionExpressionSets,
    VersionPragma,
    VersionRange,
    VersionSpecifiers,
    WhileStatement,
    YulArguments,
    YulAssignmentOperator,
    YulAssignmentStatement,
    YulBlock,
    YulBreakStatement,
    YulBuiltInFunction,
    YulColonAndEqual,
    YulContinueStatement,
    YulDefaultCase,
    YulExpression,
    YulForStatement,
    YulFunctionCallExpression,
    YulFunctionDefinition,
    YulIfStatement,
    YulLabel,
    YulLeaveStatement,
    YulLiteral,
    YulParameters,
    YulParametersDeclaration,
    YulPath,
    YulPathComponent,
    YulPaths,
    YulReturnVariables,
    YulReturnsDeclaration,
    YulStatement,
    YulStatements,
    YulSwitchCase,
    YulSwitchCases,
    YulSwitchStatement,
    YulValueCase,
    YulVariableDeclarationStatement,
    YulVariableDeclarationValue,
}

impl metaslang_cst::NonTerminalKind for RuleKind {}

#[derive(
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    serde::Serialize,
    strum_macros::AsRefStr,
    strum_macros::Display,
    strum_macros::EnumString,
)]
#[strum(serialize_all = "snake_case")]
#[cfg_attr(feature = "slang_napi_interfaces", /* derives `Clone` and `Copy` */ napi(string_enum, namespace = "kinds"))]
#[cfg_attr(not(feature = "slang_napi_interfaces"), derive(Clone, Copy))]
pub enum NodeLabel {
    // Built-in labels
    Item,
    Variant,
    Separator,
    Operand,
    LeftOperand,
    RightOperand,
    LeadingTrivia,
    TrailingTrivia,
    // Generated
    AbicoderKeyword,
    AbstractKeyword,
    AddressKeyword,
    Alias,
    AnonymousKeyword,
    Arguments,
    AsKeyword,
    AssemblyKeyword,
    Assignment,
    Asterisk,
    Attributes,
    Block,
    Body,
    BreakKeyword,
    CaseKeyword,
    Cases,
    CatchClauses,
    CatchKeyword,
    Clause,
    CloseBrace,
    CloseBracket,
    CloseParen,
    Colon,
    Condition,
    ConstantKeyword,
    ConstructorKeyword,
    ContinueKeyword,
    ContractKeyword,
    DefaultKeyword,
    DoKeyword,
    Elements,
    ElseBranch,
    ElseKeyword,
    EmitKeyword,
    End,
    EnumKeyword,
    Equal,
    EqualGreaterThan,
    Error,
    ErrorKeyword,
    Event,
    EventKeyword,
    ExperimentalKeyword,
    Expression,
    FallbackKeyword,
    FalseExpression,
    Feature,
    Flags,
    ForKeyword,
    FromKeyword,
    FunctionKeyword,
    GlobalKeyword,
    Identifier,
    IfKeyword,
    ImportKeyword,
    Index,
    IndexedKeyword,
    Inheritence,
    Initialization,
    InterfaceKeyword,
    IsKeyword,
    Items,
    Iterator,
    KeyType,
    Label,
    LeaveKeyword,
    LetKeyword,
    LibraryKeyword,
    Literal,
    MappingKeyword,
    Member,
    Members,
    MinusGreaterThan,
    ModifierKeyword,
    Name,
    Names,
    NewKeyword,
    OpenBrace,
    OpenBracket,
    OpenParen,
    Operator,
    Options,
    Overridden,
    OverrideKeyword,
    Parameters,
    Path,
    Paths,
    PayableKeyword,
    Period,
    Pragma,
    PragmaKeyword,
    QuestionMark,
    ReceiveKeyword,
    ReturnKeyword,
    Returns,
    ReturnsKeyword,
    RevertKeyword,
    Semicolon,
    Sets,
    SolidityKeyword,
    Start,
    Statements,
    StorageLocation,
    StructKeyword,
    SwitchKeyword,
    Symbols,
    Target,
    ThrowKeyword,
    TrueExpression,
    TryKeyword,
    TypeKeyword,
    TypeName,
    Types,
    UncheckedKeyword,
    Unit,
    UsingKeyword,
    Value,
    ValueType,
    VarKeyword,
    VariableType,
    Variables,
    Version,
    WhileKeyword,
}

impl metaslang_cst::EdgeKind for NodeLabel {}

#[derive(
    Debug,
    Eq,
    Hash,
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
    CallDataKeyword,
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
    CopyOfKeyword,
    DaysKeyword,
    DecimalLiteral,
    DefaultKeyword,
    DefineKeyword,
    DeleteKeyword,
    DoKeyword,
    DoubleQuotedHexStringLiteral,
    DoubleQuotedStringLiteral,
    DoubleQuotedUnicodeStringLiteral,
    DoubleQuotedVersionLiteral,
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
    MultiLineComment,
    MultiLineNatSpecComment,
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
    SingleLineNatSpecComment,
    SingleQuotedHexStringLiteral,
    SingleQuotedStringLiteral,
    SingleQuotedUnicodeStringLiteral,
    SingleQuotedVersionLiteral,
    SizeOfKeyword,
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
    TypeDefKeyword,
    TypeKeyword,
    TypeOfKeyword,
    UfixedKeyword,
    UintKeyword,
    UncheckedKeyword,
    UsingKeyword,
    VarKeyword,
    VersionSpecifier,
    ViewKeyword,
    VirtualKeyword,
    WeeksKeyword,
    WeiKeyword,
    WhileKeyword,
    Whitespace,
    YearsKeyword,
    YulAbstractKeyword,
    YulAddKeyword,
    YulAddModKeyword,
    YulAddressKeyword,
    YulAfterKeyword,
    YulAliasKeyword,
    YulAndKeyword,
    YulAnonymousKeyword,
    YulApplyKeyword,
    YulAsKeyword,
    YulAssemblyKeyword,
    YulAutoKeyword,
    YulBalanceKeyword,
    YulBaseFeeKeyword,
    YulBlobBaseFeeKeyword,
    YulBlobHashKeyword,
    YulBlockHashKeyword,
    YulBoolKeyword,
    YulBreakKeyword,
    YulByteKeyword,
    YulBytesKeyword,
    YulCallCodeKeyword,
    YulCallDataCopyKeyword,
    YulCallDataKeyword,
    YulCallDataLoadKeyword,
    YulCallDataSizeKeyword,
    YulCallKeyword,
    YulCallValueKeyword,
    YulCallerKeyword,
    YulCaseKeyword,
    YulCatchKeyword,
    YulChainIdKeyword,
    YulCoinBaseKeyword,
    YulConstantKeyword,
    YulConstructorKeyword,
    YulContinueKeyword,
    YulContractKeyword,
    YulCopyOfKeyword,
    YulCreate2Keyword,
    YulCreateKeyword,
    YulDaysKeyword,
    YulDecimalLiteral,
    YulDefaultKeyword,
    YulDefineKeyword,
    YulDelegateCallKeyword,
    YulDeleteKeyword,
    YulDifficultyKeyword,
    YulDivKeyword,
    YulDoKeyword,
    YulElseKeyword,
    YulEmitKeyword,
    YulEnumKeyword,
    YulEqKeyword,
    YulEtherKeyword,
    YulEventKeyword,
    YulExpKeyword,
    YulExtCodeCopyKeyword,
    YulExtCodeHashKeyword,
    YulExtCodeSizeKeyword,
    YulExternalKeyword,
    YulFallbackKeyword,
    YulFalseKeyword,
    YulFinalKeyword,
    YulFinneyKeyword,
    YulFixedKeyword,
    YulForKeyword,
    YulFunctionKeyword,
    YulGasKeyword,
    YulGasLimitKeyword,
    YulGasPriceKeyword,
    YulGtKeyword,
    YulGweiKeyword,
    YulHexKeyword,
    YulHexLiteral,
    YulHoursKeyword,
    YulIdentifier,
    YulIfKeyword,
    YulImmutableKeyword,
    YulImplementsKeyword,
    YulImportKeyword,
    YulInKeyword,
    YulIndexedKeyword,
    YulInlineKeyword,
    YulIntKeyword,
    YulInterfaceKeyword,
    YulInternalKeyword,
    YulInvalidKeyword,
    YulIsKeyword,
    YulIsZeroKeyword,
    YulKeccak256Keyword,
    YulLeaveKeyword,
    YulLetKeyword,
    YulLibraryKeyword,
    YulLog0Keyword,
    YulLog1Keyword,
    YulLog2Keyword,
    YulLog3Keyword,
    YulLog4Keyword,
    YulLtKeyword,
    YulMCopyKeyword,
    YulMLoadKeyword,
    YulMSizeKeyword,
    YulMStore8Keyword,
    YulMStoreKeyword,
    YulMacroKeyword,
    YulMappingKeyword,
    YulMatchKeyword,
    YulMemoryKeyword,
    YulMinutesKeyword,
    YulModKeyword,
    YulModifierKeyword,
    YulMulKeyword,
    YulMulModKeyword,
    YulMutableKeyword,
    YulNewKeyword,
    YulNotKeyword,
    YulNullKeyword,
    YulNumberKeyword,
    YulOfKeyword,
    YulOrKeyword,
    YulOriginKeyword,
    YulOverrideKeyword,
    YulPartialKeyword,
    YulPayableKeyword,
    YulPopKeyword,
    YulPragmaKeyword,
    YulPrevRandaoKeyword,
    YulPrivateKeyword,
    YulPromiseKeyword,
    YulPublicKeyword,
    YulPureKeyword,
    YulReceiveKeyword,
    YulReferenceKeyword,
    YulRelocatableKeyword,
    YulReturnDataCopyKeyword,
    YulReturnDataSizeKeyword,
    YulReturnKeyword,
    YulReturnsKeyword,
    YulRevertKeyword,
    YulSDivKeyword,
    YulSLoadKeyword,
    YulSModKeyword,
    YulSStoreKeyword,
    YulSarKeyword,
    YulSealedKeyword,
    YulSecondsKeyword,
    YulSelfBalanceKeyword,
    YulSelfDestructKeyword,
    YulSgtKeyword,
    YulSha3Keyword,
    YulShlKeyword,
    YulShrKeyword,
    YulSignExtendKeyword,
    YulSizeOfKeyword,
    YulSltKeyword,
    YulStaticCallKeyword,
    YulStaticKeyword,
    YulStopKeyword,
    YulStorageKeyword,
    YulStringKeyword,
    YulStructKeyword,
    YulSubKeyword,
    YulSuicideKeyword,
    YulSupportsKeyword,
    YulSwitchKeyword,
    YulSzaboKeyword,
    YulTLoadKeyword,
    YulTStoreKeyword,
    YulThrowKeyword,
    YulTimestampKeyword,
    YulTrueKeyword,
    YulTryKeyword,
    YulTypeDefKeyword,
    YulTypeKeyword,
    YulTypeOfKeyword,
    YulUfixedKeyword,
    YulUintKeyword,
    YulUncheckedKeyword,
    YulUsingKeyword,
    YulVarKeyword,
    YulViewKeyword,
    YulVirtualKeyword,
    YulWeeksKeyword,
    YulWeiKeyword,
    YulWhileKeyword,
    YulXorKeyword,
    YulYearsKeyword,
}

impl metaslang_cst::TerminalKind for TokenKind {
    fn is_trivia(&self) -> bool {
        #[allow(clippy::match_like_matches_macro)]
        match self {
            Self::EndOfLine => true,
            Self::MultiLineComment => true,
            Self::MultiLineNatSpecComment => true,
            Self::SingleLineComment => true,
            Self::SingleLineNatSpecComment => true,
            Self::Whitespace => true,
            _ => false,
        }
    }
}

/// The lexical context of the scanner.
#[derive(strum_macros::FromRepr, Clone, Copy)]
pub(crate) enum LexicalContext {
    Default,
    Pragma,
    Yul,
}

/// Marker trait for type-level [`LexicalContext`] variants.
pub(crate) trait IsLexicalContext {
    /// Returns a run-time [`LexicalContext`] value.
    fn value() -> LexicalContext;
}

#[allow(non_snake_case)]
pub(crate) mod LexicalContextType {
    use super::{IsLexicalContext, LexicalContext};
    pub struct Default {}
    impl IsLexicalContext for Default {
        fn value() -> LexicalContext {
            LexicalContext::Default
        }
    }
    pub struct Pragma {}
    impl IsLexicalContext for Pragma {
        fn value() -> LexicalContext {
            LexicalContext::Pragma
        }
    }
    pub struct Yul {}
    impl IsLexicalContext for Yul {
        fn value() -> LexicalContext {
            LexicalContext::Yul
        }
    }
}

// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

#[cfg(feature = "slang_napi_interfaces")]
use napi::bindgen_prelude::*;
#[derive(
    Debug,
    Eq,
    Ord,
    PartialEq,
    PartialOrd,
    serde :: Serialize,
    strum_macros :: AsRefStr,
    strum_macros :: Display,
    strum_macros :: EnumString,
)]
#[cfg_attr(
    feature = "slang_napi_interfaces",
    napi(string_enum, namespace = "syntax$nodes")
)]
#[cfg_attr(not(feature = "slang_napi_interfaces"), derive(Clone, Copy))]
pub enum TokenKind {
    SKIPPED,
    AbicoderKeyword,
    AbstractKeyword,
    AddressKeyword,
    Ampersand,
    AmpersandAmpersand,
    AmpersandEqual,
    AnonymousKeyword,
    AsKeyword,
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
    ByteKeyword,
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
    DecimalLiteral,
    DefaultKeyword,
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
    HexLiteral,
    HexStringLiteral,
    HoursKeyword,
    Identifier,
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
    PragmaKeyword,
    PrivateKeyword,
    PublicKeyword,
    PureKeyword,
    QuestionMark,
    ReceiveKeyword,
    ReturnKeyword,
    ReturnsKeyword,
    RevertKeyword,
    SecondsKeyword,
    Semicolon,
    SignedFixedType,
    SignedIntegerType,
    SingleLineComment,
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

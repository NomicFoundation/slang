// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

#[cfg(feature = "slang_napi_interfaces")]
use napi::bindgen_prelude::*;

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
#[cfg_attr(
    // If feature is enabled, derive the NAPI version.
    // This also derives `Clone` and `Copy` automatically.
    feature = "slang_napi_interfaces",
    napi(string_enum, namespace = "syntax$nodes")
)]
#[cfg_attr(
    // If feature is not enabled, derive `Clone` and `Copy` manually.
    not(feature = "slang_napi_interfaces"),
    derive(Clone, Copy),
)]
pub enum TokenKind {
    /// Used to hold parts of input that cannot be parsed (incomplete, or erroneous).
    SKIPPED,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// ABICODER_KEYWORD = "abicoder";
    /// ```
    AbicoderKeyword,
    /// ## v0.6.0
    ///
    /// ```ebnf
    /// ABSTRACT_KEYWORD = "abstract";
    /// ```
    AbstractKeyword,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// ADDRESS_KEYWORD = "address";
    /// ```
    AddressKeyword,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// AMPERSAND = "&";
    /// ```
    Ampersand,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// AMPERSAND_AMPERSAND = "&&";
    /// ```
    AmpersandAmpersand,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// AMPERSAND_EQUAL = "&=";
    /// ```
    AmpersandEqual,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// ANONYMOUS_KEYWORD = "anonymous";
    /// ```
    AnonymousKeyword,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// AS_KEYWORD = "as";
    /// ```
    AsKeyword,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// ASCII_STRING_LITERAL = «SINGLE_QUOTED_ASCII_STRING_LITERAL»
    ///                      | «DOUBLE_QUOTED_ASCII_STRING_LITERAL»;
    /// ```
    AsciiStringLiteral,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// ASSEMBLY_KEYWORD = "assembly";
    /// ```
    AssemblyKeyword,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// ASTERISK = "*";
    /// ```
    Asterisk,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// ASTERISK_ASTERISK = "**";
    /// ```
    AsteriskAsterisk,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// ASTERISK_EQUAL = "*=";
    /// ```
    AsteriskEqual,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// BANG = "!";
    /// ```
    Bang,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// BANG_EQUAL = "!=";
    /// ```
    BangEqual,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// BAR = "|";
    /// ```
    Bar,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// BAR_BAR = "||";
    /// ```
    BarBar,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// BAR_EQUAL = "|=";
    /// ```
    BarEqual,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// BOOL_KEYWORD = "bool";
    /// ```
    BoolKeyword,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// BREAK_KEYWORD = "break";
    /// ```
    BreakKeyword,
    /// ## v0.4.11
    ///
    /// ```ebnf
    /// BYTE_KEYWORD = "byte";
    /// ```
    ///
    /// ## v0.8.0
    ///
    /// ```ebnf
    /// (* DELETED *)
    /// ```
    ByteKeyword,
    /// ## v0.5.0
    ///
    /// ```ebnf
    /// CALLDATA_KEYWORD = "calldata";
    /// ```
    CalldataKeyword,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// CARET = "^";
    /// ```
    Caret,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// CARET_EQUAL = "^=";
    /// ```
    CaretEqual,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// CASE_KEYWORD = "case";
    /// ```
    CaseKeyword,
    /// ## v0.6.0
    ///
    /// ```ebnf
    /// CATCH_KEYWORD = "catch";
    /// ```
    CatchKeyword,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// CLOSE_BRACE = "}";
    /// ```
    CloseBrace,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// CLOSE_BRACKET = "]";
    /// ```
    CloseBracket,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// CLOSE_PAREN = ")";
    /// ```
    CloseParen,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// COLON = ":";
    /// ```
    Colon,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// COLON_EQUAL = ":=";
    /// ```
    ColonEqual,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// COMMA = ",";
    /// ```
    Comma,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// CONSTANT_KEYWORD = "constant";
    /// ```
    ConstantKeyword,
    /// ## v0.4.22
    ///
    /// ```ebnf
    /// CONSTRUCTOR_KEYWORD = "constructor";
    /// ```
    ConstructorKeyword,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// CONTINUE_KEYWORD = "continue";
    /// ```
    ContinueKeyword,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// CONTRACT_KEYWORD = "contract";
    /// ```
    ContractKeyword,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// DAYS_KEYWORD = "days";
    /// ```
    DaysKeyword,
    /// ## v0.4.11
    ///
    /// ```ebnf
    /// DECIMAL_LITERAL = ((«DECIMAL_DIGITS» ("." «DECIMAL_DIGITS»?)?) | ("." «DECIMAL_DIGITS»)) «DECIMAL_EXPONENT»?;
    /// ```
    ///
    /// ## v0.5.0
    ///
    /// ```ebnf
    /// DECIMAL_LITERAL = ((«DECIMAL_DIGITS» ("." «DECIMAL_DIGITS»)?) | ("." «DECIMAL_DIGITS»)) «DECIMAL_EXPONENT»?;
    /// ```
    DecimalLiteral,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// DEFAULT_KEYWORD = "default";
    /// ```
    DefaultKeyword,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// DELETE_KEYWORD = "delete";
    /// ```
    DeleteKeyword,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// DO_KEYWORD = "do";
    /// ```
    DoKeyword,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// ELSE_KEYWORD = "else";
    /// ```
    ElseKeyword,
    /// ## v0.4.21
    ///
    /// ```ebnf
    /// EMIT_KEYWORD = "emit";
    /// ```
    EmitKeyword,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// END_OF_LINE = "\r"? "\n";
    /// ```
    EndOfLine,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// ENUM_KEYWORD = "enum";
    /// ```
    EnumKeyword,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// EQUAL = "=";
    /// ```
    Equal,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// EQUAL_EQUAL = "==";
    /// ```
    EqualEqual,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// EQUAL_GREATER_THAN = "=>";
    /// ```
    EqualGreaterThan,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// ERROR_KEYWORD = "error";
    /// ```
    ErrorKeyword,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// ETHER_KEYWORD = "ether";
    /// ```
    EtherKeyword,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// EVENT_KEYWORD = "event";
    /// ```
    EventKeyword,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// EXPERIMENTAL_KEYWORD = "experimental";
    /// ```
    ExperimentalKeyword,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// EXTERNAL_KEYWORD = "external";
    /// ```
    ExternalKeyword,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// FALLBACK_KEYWORD = "fallback";
    /// ```
    FallbackKeyword,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// FALSE_KEYWORD = "false";
    /// ```
    FalseKeyword,
    /// ## v0.4.11
    ///
    /// ```ebnf
    /// FINNEY_KEYWORD = "finney";
    /// ```
    ///
    /// ## v0.7.0
    ///
    /// ```ebnf
    /// (* DELETED *)
    /// ```
    FinneyKeyword,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// FIXED_BYTES_TYPE = "bytes" «FIXED_BYTES_TYPE_SIZE»;
    /// ```
    FixedBytesType,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// FOR_KEYWORD = "for";
    /// ```
    ForKeyword,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// FROM_KEYWORD = "from";
    /// ```
    FromKeyword,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// FUNCTION_KEYWORD = "function";
    /// ```
    FunctionKeyword,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// GLOBAL_KEYWORD = "global";
    /// ```
    GlobalKeyword,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// GREATER_THAN = ">";
    /// ```
    GreaterThan,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// GREATER_THAN_EQUAL = ">=";
    /// ```
    GreaterThanEqual,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// GREATER_THAN_GREATER_THAN = ">>";
    /// ```
    GreaterThanGreaterThan,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// GREATER_THAN_GREATER_THAN_EQUAL = ">>=";
    /// ```
    GreaterThanGreaterThanEqual,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// GREATER_THAN_GREATER_THAN_GREATER_THAN = ">>>";
    /// ```
    GreaterThanGreaterThanGreaterThan,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// GREATER_THAN_GREATER_THAN_GREATER_THAN_EQUAL = ">>>=";
    /// ```
    GreaterThanGreaterThanGreaterThanEqual,
    /// ## v0.6.11
    ///
    /// ```ebnf
    /// GWEI_KEYWORD = "gwei";
    /// ```
    GweiKeyword,
    /// ## v0.4.11
    ///
    /// ```ebnf
    /// HEX_LITERAL = ("0x" | "0X") «HEX_CHARACTER»+ ("_" «HEX_CHARACTER»+)*;
    /// ```
    ///
    /// ## v0.5.0
    ///
    /// ```ebnf
    /// HEX_LITERAL = "0x" «HEX_CHARACTER»+ ("_" «HEX_CHARACTER»+)*;
    /// ```
    HexLiteral,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// HEX_STRING_LITERAL = «SINGLE_QUOTED_HEX_STRING_LITERAL»
    ///                    | «DOUBLE_QUOTED_HEX_STRING_LITERAL»;
    /// ```
    HexStringLiteral,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// HOURS_KEYWORD = "hours";
    /// ```
    HoursKeyword,
    /// ## v0.4.11
    ///
    /// ```ebnf
    /// IDENTIFIER = «RAW_IDENTIFIER» - («KEYWORD_IN_ANY_VERSION» | «KEYWORD_IN_SOME_VERSION» | «RESERVED_WORD_IN_ANY_VERSION»);
    /// ```
    ///
    /// ## v0.5.0
    ///
    /// ```ebnf
    /// IDENTIFIER = «RAW_IDENTIFIER» - («KEYWORD_IN_ANY_VERSION» | «KEYWORD_IN_SOME_VERSION» | «RESERVED_WORD_IN_ANY_VERSION» | «RESERVED_WORD_IN_SOME_VERSION»);
    /// ```
    Identifier,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// IF_KEYWORD = "if";
    /// ```
    IfKeyword,
    /// ## v0.6.5
    ///
    /// ```ebnf
    /// IMMUTABLE_KEYWORD = "immutable";
    /// ```
    ImmutableKeyword,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// IMPORT_KEYWORD = "import";
    /// ```
    ImportKeyword,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// INDEXED_KEYWORD = "indexed";
    /// ```
    IndexedKeyword,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// INTERFACE_KEYWORD = "interface";
    /// ```
    InterfaceKeyword,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// INTERNAL_KEYWORD = "internal";
    /// ```
    InternalKeyword,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// IS_KEYWORD = "is";
    /// ```
    IsKeyword,
    /// ## v0.6.0
    ///
    /// ```ebnf
    /// LEAVE_KEYWORD = "leave";
    /// ```
    LeaveKeyword,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// LESS_THAN = "<";
    /// ```
    LessThan,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// LESS_THAN_EQUAL = "<=";
    /// ```
    LessThanEqual,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// LESS_THAN_LESS_THAN = "<<";
    /// ```
    LessThanLessThan,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// LESS_THAN_LESS_THAN_EQUAL = "<<=";
    /// ```
    LessThanLessThanEqual,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// LET_KEYWORD = "let";
    /// ```
    LetKeyword,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// LIBRARY_KEYWORD = "library";
    /// ```
    LibraryKeyword,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// MAPPING_KEYWORD = "mapping";
    /// ```
    MappingKeyword,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// MEMORY_KEYWORD = "memory";
    /// ```
    MemoryKeyword,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// MINUS = "-";
    /// ```
    Minus,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// MINUS_EQUAL = "-=";
    /// ```
    MinusEqual,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// MINUS_GREATER_THAN = "->";
    /// ```
    MinusGreaterThan,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// MINUS_MINUS = "--";
    /// ```
    MinusMinus,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// MINUTES_KEYWORD = "minutes";
    /// ```
    MinutesKeyword,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// MODIFIER_KEYWORD = "modifier";
    /// ```
    ModifierKeyword,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// MULTILINE_COMMENT = "/" "*" (!"*" | "*")* "*" "/";
    /// ```
    MultilineComment,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// NEW_KEYWORD = "new";
    /// ```
    NewKeyword,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// OPEN_BRACE = "{";
    /// ```
    OpenBrace,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// OPEN_BRACKET = "[";
    /// ```
    OpenBracket,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// OPEN_PAREN = "(";
    /// ```
    OpenParen,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// OVERRIDE_KEYWORD = "override";
    /// ```
    OverrideKeyword,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// PAYABLE_KEYWORD = "payable";
    /// ```
    PayableKeyword,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// PERCENT = "%";
    /// ```
    Percent,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// PERCENT_EQUAL = "%=";
    /// ```
    PercentEqual,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// PERIOD = ".";
    /// ```
    Period,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// PLUS = "+";
    /// ```
    Plus,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// PLUS_EQUAL = "+=";
    /// ```
    PlusEqual,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// PLUS_PLUS = "++";
    /// ```
    PlusPlus,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// PRAGMA_KEYWORD = "pragma";
    /// ```
    PragmaKeyword,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// PRIVATE_KEYWORD = "private";
    /// ```
    PrivateKeyword,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// PUBLIC_KEYWORD = "public";
    /// ```
    PublicKeyword,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// PURE_KEYWORD = "pure";
    /// ```
    PureKeyword,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// QUESTION_MARK = "?";
    /// ```
    QuestionMark,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// RECEIVE_KEYWORD = "receive";
    /// ```
    ReceiveKeyword,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// RETURN_KEYWORD = "return";
    /// ```
    ReturnKeyword,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// RETURNS_KEYWORD = "returns";
    /// ```
    ReturnsKeyword,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// REVERT_KEYWORD = "revert";
    /// ```
    RevertKeyword,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// SECONDS_KEYWORD = "seconds";
    /// ```
    SecondsKeyword,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// SEMICOLON = ";";
    /// ```
    Semicolon,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// SIGNED_FIXED_TYPE = "fixed" «FIXED_TYPE_SIZE»?;
    /// ```
    SignedFixedType,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// SIGNED_INTEGER_TYPE = "int" «INTEGER_TYPE_SIZE»?;
    /// ```
    SignedIntegerType,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// SINGLE_LINE_COMMENT = "//" (!("\r" | "\n"))*;
    /// ```
    SingleLineComment,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// SLASH = "/";
    /// ```
    Slash,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// SLASH_EQUAL = "/=";
    /// ```
    SlashEqual,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// SOLIDITY_KEYWORD = "solidity";
    /// ```
    SolidityKeyword,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// STORAGE_KEYWORD = "storage";
    /// ```
    StorageKeyword,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// STRING_KEYWORD = "string";
    /// ```
    StringKeyword,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// STRUCT_KEYWORD = "struct";
    /// ```
    StructKeyword,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// SWITCH_KEYWORD = "switch";
    /// ```
    SwitchKeyword,
    /// ## v0.4.11
    ///
    /// ```ebnf
    /// SZABO_KEYWORD = "szabo";
    /// ```
    ///
    /// ## v0.7.0
    ///
    /// ```ebnf
    /// (* DELETED *)
    /// ```
    SzaboKeyword,
    /// ## v0.4.11
    ///
    /// ```ebnf
    /// THROW_KEYWORD = "throw";
    /// ```
    ///
    /// ## v0.5.0
    ///
    /// ```ebnf
    /// (* DELETED *)
    /// ```
    ThrowKeyword,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// TILDE = "~";
    /// ```
    Tilde,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// TRUE_KEYWORD = "true";
    /// ```
    TrueKeyword,
    /// ## v0.6.0
    ///
    /// ```ebnf
    /// TRY_KEYWORD = "try";
    /// ```
    TryKeyword,
    /// ## v0.5.3
    ///
    /// ```ebnf
    /// TYPE_KEYWORD = "type";
    /// ```
    TypeKeyword,
    /// ## v0.8.0
    ///
    /// ```ebnf
    /// UNCHECKED_KEYWORD = "unchecked";
    /// ```
    UncheckedKeyword,
    /// ## v0.7.0
    ///
    /// ```ebnf
    /// UNICODE_STRING_LITERAL = «SINGLE_QUOTED_UNICODE_STRING_LITERAL»
    ///                        | «DOUBLE_QUOTED_UNICODE_STRING_LITERAL»;
    /// ```
    UnicodeStringLiteral,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// UNSIGNED_FIXED_TYPE = "ufixed" «FIXED_TYPE_SIZE»?;
    /// ```
    UnsignedFixedType,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// UNSIGNED_INTEGER_TYPE = "uint" «INTEGER_TYPE_SIZE»?;
    /// ```
    UnsignedIntegerType,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// USING_KEYWORD = "using";
    /// ```
    UsingKeyword,
    /// ## v0.4.11
    ///
    /// ```ebnf
    /// VAR_KEYWORD = "var";
    /// ```
    ///
    /// ## v0.5.0
    ///
    /// ```ebnf
    /// (* DELETED *)
    /// ```
    VarKeyword,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// VERSION_PRAGMA_VALUE = ("0"…"9" | "x" | "X" | "*")+;
    /// ```
    VersionPragmaValue,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// VIEW_KEYWORD = "view";
    /// ```
    ViewKeyword,
    /// ## v0.6.0
    ///
    /// ```ebnf
    /// VIRTUAL_KEYWORD = "virtual";
    /// ```
    VirtualKeyword,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// WEEKS_KEYWORD = "weeks";
    /// ```
    WeeksKeyword,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// WEI_KEYWORD = "wei";
    /// ```
    WeiKeyword,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// WHILE_KEYWORD = "while";
    /// ```
    WhileKeyword,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// WHITESPACE = (" " | "\t")+;
    /// ```
    Whitespace,
    /// ## v0.4.11
    ///
    /// ```ebnf
    /// YEARS_KEYWORD = "years";
    /// ```
    ///
    /// ## v0.5.0
    ///
    /// ```ebnf
    /// (* DELETED *)
    /// ```
    YearsKeyword,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// YUL_DECIMAL_LITERAL = "0" | ("1"…"9" "0"…"9"*);
    /// ```
    YulDecimalLiteral,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// YUL_HEX_LITERAL = "0x" «HEX_CHARACTER»+;
    /// ```
    YulHexLiteral,
    /// ## Unversioned
    ///
    /// ```ebnf
    /// YUL_IDENTIFIER = «RAW_IDENTIFIER» - («YUL_KEYWORD» | «YUL_RESERVED_WORD»);
    /// ```
    YulIdentifier,
}

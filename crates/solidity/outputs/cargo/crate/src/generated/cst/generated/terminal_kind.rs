// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

#[allow(clippy::doc_markdown)]
#[allow(clippy::doc_link_with_quotes)]
#[repr(u16)]
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
    strum_macros::IntoStaticStr,
    Clone,
    Copy,
)]
#[allow(clippy::upper_case_acronyms)]
pub enum TerminalKind {
    // Built-in:
    UNRECOGNIZED,
    MISSING,

    // Generated:
    /// ```ebnf
    /// (* Never reserved *)
    /// ABICODER_KEYWORD = "abicoder";
    /// ```
    AbicoderKeyword,
    /// ```ebnf
    /// (* Introduced in 0.6.0 *)
    /// ABSTRACT_KEYWORD = "abstract";
    /// ```
    AbstractKeyword,
    /// ```ebnf
    /// (* Never reserved *)
    /// ADDRESS_KEYWORD = "address";
    /// ```
    AddressKeyword,
    /// ```ebnf
    /// AFTER_KEYWORD = "after";
    /// ```
    AfterKeyword,
    /// ```ebnf
    /// (* Reserved in 0.5.0 *)
    /// ALIAS_KEYWORD = "alias";
    /// ```
    AliasKeyword,
    /// ```ebnf
    /// AMPERSAND = "&";
    /// ```
    Ampersand,
    /// ```ebnf
    /// AMPERSAND_AMPERSAND = "&&";
    /// ```
    AmpersandAmpersand,
    /// ```ebnf
    /// AMPERSAND_EQUAL = "&=";
    /// ```
    AmpersandEqual,
    /// ```ebnf
    /// ANONYMOUS_KEYWORD = "anonymous";
    /// ```
    AnonymousKeyword,
    /// ```ebnf
    /// (* Reserved in 0.5.0 *)
    /// APPLY_KEYWORD = "apply";
    /// ```
    ApplyKeyword,
    /// ```ebnf
    /// AS_KEYWORD = "as";
    /// ```
    AsKeyword,
    /// ```ebnf
    /// ASSEMBLY_KEYWORD = "assembly";
    /// ```
    AssemblyKeyword,
    /// ```ebnf
    /// ASTERISK = "*";
    /// ```
    Asterisk,
    /// ```ebnf
    /// ASTERISK_ASTERISK = "**";
    /// ```
    AsteriskAsterisk,
    /// ```ebnf
    /// ASTERISK_EQUAL = "*=";
    /// ```
    AsteriskEqual,
    /// ```ebnf
    /// (* Reserved in 0.5.0 *)
    /// AUTO_KEYWORD = "auto";
    /// ```
    AutoKeyword,
    /// ```ebnf
    /// BANG = "!";
    /// ```
    Bang,
    /// ```ebnf
    /// BANG_EQUAL = "!=";
    /// ```
    BangEqual,
    /// ```ebnf
    /// BAR = "|";
    /// ```
    Bar,
    /// ```ebnf
    /// BAR_BAR = "||";
    /// ```
    BarBar,
    /// ```ebnf
    /// BAR_EQUAL = "|=";
    /// ```
    BarEqual,
    /// ```ebnf
    /// BOOL_KEYWORD = "bool";
    /// ```
    BoolKeyword,
    /// ```ebnf
    /// BREAK_KEYWORD = "break";
    /// ```
    BreakKeyword,
    /// ```ebnf
    /// (* Deprecated in 0.8.0 *)
    /// BYTE_KEYWORD = "byte";
    /// ```
    ByteKeyword,
    /// ```ebnf
    /// BYTES_KEYWORD = "bytes" ("1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9" | "10" | "11" | "12" | "13" | "14" | "15" | "16" | "17" | "18" | "19" | "20" | "21" | "22" | "23" | "24" | "25" | "26" | "27" | "28" | "29" | "30" | "31" | "32")?;
    /// ```
    BytesKeyword,
    /// ```ebnf
    /// (* Introduced in 0.5.0 *)
    /// (* Reserved in 0.5.0 *)
    /// CALL_DATA_KEYWORD = "calldata";
    /// ```
    CallDataKeyword,
    /// ```ebnf
    /// CARET = "^";
    /// ```
    Caret,
    /// ```ebnf
    /// CARET_EQUAL = "^=";
    /// ```
    CaretEqual,
    /// ```ebnf
    /// CASE_KEYWORD = "case";
    /// ```
    CaseKeyword,
    /// ```ebnf
    /// (* Introduced in 0.6.0 *)
    /// CATCH_KEYWORD = "catch";
    /// ```
    CatchKeyword,
    /// ```ebnf
    /// CLOSE_BRACE = "}";
    /// ```
    CloseBrace,
    /// ```ebnf
    /// CLOSE_BRACKET = "]";
    /// ```
    CloseBracket,
    /// ```ebnf
    /// CLOSE_PAREN = ")";
    /// ```
    CloseParen,
    /// ```ebnf
    /// COLON = ":";
    /// ```
    Colon,
    /// ```ebnf
    /// COLON_EQUAL = ":=";
    /// ```
    ColonEqual,
    /// ```ebnf
    /// COMMA = ",";
    /// ```
    Comma,
    /// ```ebnf
    /// CONSTANT_KEYWORD = "constant";
    /// ```
    ConstantKeyword,
    /// ```ebnf
    /// (* Introduced in 0.4.22 *)
    /// (* Reserved in 0.5.0 *)
    /// CONSTRUCTOR_KEYWORD = "constructor";
    /// ```
    ConstructorKeyword,
    /// ```ebnf
    /// CONTINUE_KEYWORD = "continue";
    /// ```
    ContinueKeyword,
    /// ```ebnf
    /// CONTRACT_KEYWORD = "contract";
    /// ```
    ContractKeyword,
    /// ```ebnf
    /// (* Reserved in 0.5.0 *)
    /// COPY_OF_KEYWORD = "copyof";
    /// ```
    CopyOfKeyword,
    /// ```ebnf
    /// DAYS_KEYWORD = "days";
    /// ```
    DaysKeyword,
    /// ```ebnf
    /// DECIMAL_LITERAL = "." «DECIMAL_DIGITS» «DECIMAL_EXPONENT»?;
    ///
    /// DECIMAL_LITERAL = «DECIMAL_DIGITS» «DECIMAL_EXPONENT»?;
    ///
    /// (* Deprecated in 0.5.0 *)
    /// DECIMAL_LITERAL = «DECIMAL_DIGITS» "." «DECIMAL_EXPONENT»?;
    ///
    /// (* Deprecated in 0.5.0 *)
    /// DECIMAL_LITERAL = «DECIMAL_DIGITS» "." «DECIMAL_DIGITS» «DECIMAL_EXPONENT»?;
    ///
    /// (* Introduced in 0.5.0 *)
    /// DECIMAL_LITERAL = «DECIMAL_DIGITS» ("." «DECIMAL_DIGITS»)? «DECIMAL_EXPONENT»?;
    /// ```
    DecimalLiteral,
    /// ```ebnf
    /// DEFAULT_KEYWORD = "default";
    /// ```
    DefaultKeyword,
    /// ```ebnf
    /// (* Reserved in 0.5.0 *)
    /// DEFINE_KEYWORD = "define";
    /// ```
    DefineKeyword,
    /// ```ebnf
    /// DELETE_KEYWORD = "delete";
    /// ```
    DeleteKeyword,
    /// ```ebnf
    /// DO_KEYWORD = "do";
    /// ```
    DoKeyword,
    /// ```ebnf
    /// DOUBLE_QUOTED_HEX_STRING_LITERAL = 'hex"' «HEX_STRING_CONTENTS»? '"';
    /// ```
    DoubleQuotedHexStringLiteral,
    /// ```ebnf
    /// (* Deprecated in 0.4.25 *)
    /// DOUBLE_QUOTED_STRING_LITERAL = '"' («ESCAPE_SEQUENCE_ARBITRARY» | !('"' "\\" "\r" "\n"))* '"';
    ///
    /// (* Introduced in 0.4.25 and deprecated in 0.7.0. *)
    /// DOUBLE_QUOTED_STRING_LITERAL = '"' («ESCAPE_SEQUENCE» | !('"' "\\" "\r" "\n"))* '"';
    ///
    /// DOUBLE_QUOTED_STRING_LITERAL = '"' («ESCAPE_SEQUENCE» | (" "…"!") | ("#"…"[") | ("]"…"~"))* '"';
    /// ```
    DoubleQuotedStringLiteral,
    /// ```ebnf
    /// (* Introduced in 0.7.0 *)
    /// DOUBLE_QUOTED_UNICODE_STRING_LITERAL = 'unicode"' («ESCAPE_SEQUENCE» | !('"' "\\" "\r" "\n"))* '"';
    /// ```
    DoubleQuotedUnicodeStringLiteral,
    /// ```ebnf
    /// DOUBLE_QUOTED_VERSION_LITERAL = '"' «VERSION_SPECIFIER_FRAGMENT» ("." «VERSION_SPECIFIER_FRAGMENT»)* '"';
    /// ```
    DoubleQuotedVersionLiteral,
    /// ```ebnf
    /// ELSE_KEYWORD = "else";
    /// ```
    ElseKeyword,
    /// ```ebnf
    /// (* Introduced in 0.4.21 *)
    /// (* Reserved in 0.5.0 *)
    /// EMIT_KEYWORD = "emit";
    /// ```
    EmitKeyword,
    /// ```ebnf
    /// END_OF_LINE = "\n" | ("\r" "\n"?);
    /// ```
    EndOfLine,
    /// ```ebnf
    /// ENUM_KEYWORD = "enum";
    /// ```
    EnumKeyword,
    /// ```ebnf
    /// EQUAL = "=";
    /// ```
    Equal,
    /// ```ebnf
    /// (* Deprecated in 0.5.0 *)
    /// EQUAL_COLON = "=:";
    /// ```
    EqualColon,
    /// ```ebnf
    /// EQUAL_EQUAL = "==";
    /// ```
    EqualEqual,
    /// ```ebnf
    /// EQUAL_GREATER_THAN = "=>";
    /// ```
    EqualGreaterThan,
    /// ```ebnf
    /// (* Introduced in 0.8.4 *)
    /// (* Never reserved *)
    /// ERROR_KEYWORD = "error";
    /// ```
    ErrorKeyword,
    /// ```ebnf
    /// ETHER_KEYWORD = "ether";
    /// ```
    EtherKeyword,
    /// ```ebnf
    /// EVENT_KEYWORD = "event";
    /// ```
    EventKeyword,
    /// ```ebnf
    /// (* Never reserved *)
    /// EXPERIMENTAL_KEYWORD = "experimental";
    /// ```
    ExperimentalKeyword,
    /// ```ebnf
    /// EXTERNAL_KEYWORD = "external";
    /// ```
    ExternalKeyword,
    /// ```ebnf
    /// (* Reserved in 0.6.0 *)
    /// FALLBACK_KEYWORD = "fallback";
    /// ```
    FallbackKeyword,
    /// ```ebnf
    /// FALSE_KEYWORD = "false";
    /// ```
    FalseKeyword,
    /// ```ebnf
    /// FINAL_KEYWORD = "final";
    /// ```
    FinalKeyword,
    /// ```ebnf
    /// (* Deprecated in 0.7.0 *)
    /// (* Reserved until 0.7.0 *)
    /// FINNEY_KEYWORD = "finney";
    /// ```
    FinneyKeyword,
    /// ```ebnf
    /// FIXED_KEYWORD = "fixed";
    ///
    /// FIXED_KEYWORD = "fixed" ("8" | "16" | "24" | "32" | "40" | "48" | "56" | "64" | "72" | "80" | "88" | "96" | "104" | "112" | "120" | "128" | "136" | "144" | "152" | "160" | "168" | "176") "x" ("8" | "16" | "24" | "32" | "40" | "48" | "56" | "64" | "72" | "80");
    ///
    /// FIXED_KEYWORD = "fixed" ("184x8" | "184x16" | "184x24" | "184x32" | "184x40" | "184x48" | "184x56" | "184x64" | "184x72" | "192x8" | "192x16" | "192x24" | "192x32" | "192x40" | "192x48" | "192x56" | "192x64" | "200x8" | "200x16" | "200x24" | "200x32" | "200x40" | "200x48" | "200x56" | "208x8" | "208x16" | "208x24" | "208x32" | "208x40" | "208x48" | "216x8" | "216x16" | "216x24" | "216x32" | "216x40" | "224x8" | "224x16" | "224x24" | "224x32" | "232x8" | "232x16" | "232x24" | "240x8" | "240x16" | "248x8");
    ///
    /// (* Reserved in 0.4.14 *)
    /// FIXED_KEYWORD = "fixed" ("184x80" | "192x72" | "192x80" | "200x64" | "200x72" | "200x80" | "208x56" | "208x64" | "208x72" | "208x80" | "216x48" | "216x56" | "216x64" | "216x72" | "216x80" | "224x40" | "224x48" | "224x56" | "224x64" | "224x72" | "224x80" | "232x32" | "232x40" | "232x48" | "232x56" | "232x64" | "232x72" | "232x80" | "240x24" | "240x32" | "240x40" | "240x48" | "240x56" | "240x64" | "240x72" | "240x80" | "248x16" | "248x24" | "248x32" | "248x40" | "248x48" | "248x56" | "248x64" | "248x72" | "248x80" | "256x8" | "256x16" | "256x24" | "256x32" | "256x40" | "256x48" | "256x56" | "256x64" | "256x72" | "256x80");
    ///
    /// (* Reserved in 0.4.14 *)
    /// FIXED_KEYWORD = "fixed" ("8" | "16" | "24" | "32" | "40" | "48" | "56" | "64" | "72" | "80" | "88" | "96" | "104" | "112" | "120" | "128" | "136" | "144" | "152" | "160" | "168" | "176" | "184" | "192" | "200" | "208" | "216" | "224" | "232" | "240" | "248" | "256") "x" ("0" | "1" | "2" | "3" | "4" | "5" | "6" | "7" | "9" | "10" | "11" | "12" | "13" | "14" | "15" | "17" | "18" | "19" | "20" | "21" | "22" | "23" | "25" | "26" | "27" | "28" | "29" | "30" | "31" | "33" | "34" | "35" | "36" | "37" | "38" | "39" | "41" | "42" | "43" | "44" | "45" | "46" | "47" | "49" | "50" | "51" | "52" | "53" | "54" | "55" | "57" | "58" | "59" | "60" | "61" | "62" | "63" | "65" | "66" | "67" | "68" | "69" | "70" | "71" | "73" | "74" | "75" | "76" | "77" | "78" | "79");
    /// ```
    FixedKeyword,
    /// ```ebnf
    /// FOR_KEYWORD = "for";
    /// ```
    ForKeyword,
    /// ```ebnf
    /// (* Never reserved *)
    /// FROM_KEYWORD = "from";
    /// ```
    FromKeyword,
    /// ```ebnf
    /// FUNCTION_KEYWORD = "function";
    /// ```
    FunctionKeyword,
    /// ```ebnf
    /// (* Introduced in 0.8.13 *)
    /// (* Never reserved *)
    /// GLOBAL_KEYWORD = "global";
    /// ```
    GlobalKeyword,
    /// ```ebnf
    /// GREATER_THAN = ">";
    /// ```
    GreaterThan,
    /// ```ebnf
    /// GREATER_THAN_EQUAL = ">=";
    /// ```
    GreaterThanEqual,
    /// ```ebnf
    /// GREATER_THAN_GREATER_THAN = ">>";
    /// ```
    GreaterThanGreaterThan,
    /// ```ebnf
    /// GREATER_THAN_GREATER_THAN_EQUAL = ">>=";
    /// ```
    GreaterThanGreaterThanEqual,
    /// ```ebnf
    /// GREATER_THAN_GREATER_THAN_GREATER_THAN = ">>>";
    /// ```
    GreaterThanGreaterThanGreaterThan,
    /// ```ebnf
    /// GREATER_THAN_GREATER_THAN_GREATER_THAN_EQUAL = ">>>=";
    /// ```
    GreaterThanGreaterThanGreaterThanEqual,
    /// ```ebnf
    /// (* Introduced in 0.6.11 *)
    /// (* Reserved in 0.7.0 *)
    /// GWEI_KEYWORD = "gwei";
    /// ```
    GweiKeyword,
    /// ```ebnf
    /// HEX_KEYWORD = "hex";
    /// ```
    HexKeyword,
    /// ```ebnf
    /// HEX_LITERAL = "0x" «HEX_CHARACTER»+ ("_" «HEX_CHARACTER»+)*;
    ///
    /// (* Deprecated in 0.5.0 *)
    /// HEX_LITERAL = "0X" «HEX_CHARACTER»+ ("_" «HEX_CHARACTER»+)*;
    /// ```
    HexLiteral,
    /// ```ebnf
    /// HOURS_KEYWORD = "hours";
    /// ```
    HoursKeyword,
    /// ```ebnf
    /// IDENTIFIER = «IDENTIFIER_START» «IDENTIFIER_PART»*;
    /// ```
    Identifier,
    /// ```ebnf
    /// IF_KEYWORD = "if";
    /// ```
    IfKeyword,
    /// ```ebnf
    /// (* Introduced in 0.6.5 *)
    /// (* Reserved in 0.5.0 *)
    /// IMMUTABLE_KEYWORD = "immutable";
    /// ```
    ImmutableKeyword,
    /// ```ebnf
    /// (* Reserved in 0.5.0 *)
    /// IMPLEMENTS_KEYWORD = "implements";
    /// ```
    ImplementsKeyword,
    /// ```ebnf
    /// IMPORT_KEYWORD = "import";
    /// ```
    ImportKeyword,
    /// ```ebnf
    /// IN_KEYWORD = "in";
    /// ```
    InKeyword,
    /// ```ebnf
    /// INDEXED_KEYWORD = "indexed";
    /// ```
    IndexedKeyword,
    /// ```ebnf
    /// INLINE_KEYWORD = "inline";
    /// ```
    InlineKeyword,
    /// ```ebnf
    /// INT_KEYWORD = "int" ("8" | "16" | "24" | "32" | "40" | "48" | "56" | "64" | "72" | "80" | "88" | "96" | "104" | "112" | "120" | "128" | "136" | "144" | "152" | "160" | "168" | "176" | "184" | "192" | "200" | "208" | "216" | "224" | "232" | "240" | "248" | "256")?;
    /// ```
    IntKeyword,
    /// ```ebnf
    /// INTERFACE_KEYWORD = "interface";
    /// ```
    InterfaceKeyword,
    /// ```ebnf
    /// INTERNAL_KEYWORD = "internal";
    /// ```
    InternalKeyword,
    /// ```ebnf
    /// IS_KEYWORD = "is";
    /// ```
    IsKeyword,
    /// ```ebnf
    /// LESS_THAN = "<";
    /// ```
    LessThan,
    /// ```ebnf
    /// LESS_THAN_EQUAL = "<=";
    /// ```
    LessThanEqual,
    /// ```ebnf
    /// LESS_THAN_LESS_THAN = "<<";
    /// ```
    LessThanLessThan,
    /// ```ebnf
    /// LESS_THAN_LESS_THAN_EQUAL = "<<=";
    /// ```
    LessThanLessThanEqual,
    /// ```ebnf
    /// LET_KEYWORD = "let";
    /// ```
    LetKeyword,
    /// ```ebnf
    /// LIBRARY_KEYWORD = "library";
    /// ```
    LibraryKeyword,
    /// ```ebnf
    /// (* Reserved in 0.5.0 *)
    /// MACRO_KEYWORD = "macro";
    /// ```
    MacroKeyword,
    /// ```ebnf
    /// MAPPING_KEYWORD = "mapping";
    /// ```
    MappingKeyword,
    /// ```ebnf
    /// MATCH_KEYWORD = "match";
    /// ```
    MatchKeyword,
    /// ```ebnf
    /// MEMORY_KEYWORD = "memory";
    /// ```
    MemoryKeyword,
    /// ```ebnf
    /// MINUS = "-";
    /// ```
    Minus,
    /// ```ebnf
    /// MINUS_EQUAL = "-=";
    /// ```
    MinusEqual,
    /// ```ebnf
    /// MINUS_GREATER_THAN = "->";
    /// ```
    MinusGreaterThan,
    /// ```ebnf
    /// MINUS_MINUS = "--";
    /// ```
    MinusMinus,
    /// ```ebnf
    /// MINUTES_KEYWORD = "minutes";
    /// ```
    MinutesKeyword,
    /// ```ebnf
    /// MODIFIER_KEYWORD = "modifier";
    /// ```
    ModifierKeyword,
    /// ```ebnf
    /// MULTI_LINE_COMMENT = "/*" (!"*" | "*")* "*/";
    /// ```
    MultiLineComment,
    /// ```ebnf
    /// MULTI_LINE_NAT_SPEC_COMMENT = "/**" (!"*" | "*")* "*/";
    /// ```
    MultiLineNatSpecComment,
    /// ```ebnf
    /// (* Reserved in 0.5.0 *)
    /// MUTABLE_KEYWORD = "mutable";
    /// ```
    MutableKeyword,
    /// ```ebnf
    /// NEW_KEYWORD = "new";
    /// ```
    NewKeyword,
    /// ```ebnf
    /// NULL_KEYWORD = "null";
    /// ```
    NullKeyword,
    /// ```ebnf
    /// OF_KEYWORD = "of";
    /// ```
    OfKeyword,
    /// ```ebnf
    /// OPEN_BRACE = "{";
    /// ```
    OpenBrace,
    /// ```ebnf
    /// OPEN_BRACKET = "[";
    /// ```
    OpenBracket,
    /// ```ebnf
    /// OPEN_PAREN = "(";
    /// ```
    OpenParen,
    /// ```ebnf
    /// (* Introduced in 0.6.0 *)
    /// (* Reserved in 0.5.0 *)
    /// OVERRIDE_KEYWORD = "override";
    /// ```
    OverrideKeyword,
    /// ```ebnf
    /// (* Reserved in 0.5.0 *)
    /// PARTIAL_KEYWORD = "partial";
    /// ```
    PartialKeyword,
    /// ```ebnf
    /// PAYABLE_KEYWORD = "payable";
    /// ```
    PayableKeyword,
    /// ```ebnf
    /// PERCENT = "%";
    /// ```
    Percent,
    /// ```ebnf
    /// PERCENT_EQUAL = "%=";
    /// ```
    PercentEqual,
    /// ```ebnf
    /// PERIOD = ".";
    /// ```
    Period,
    /// ```ebnf
    /// PLUS = "+";
    /// ```
    Plus,
    /// ```ebnf
    /// PLUS_EQUAL = "+=";
    /// ```
    PlusEqual,
    /// ```ebnf
    /// PLUS_PLUS = "++";
    /// ```
    PlusPlus,
    /// ```ebnf
    /// PRAGMA_KEYWORD = "pragma";
    /// ```
    PragmaKeyword,
    /// ```ebnf
    /// PRIVATE_KEYWORD = "private";
    /// ```
    PrivateKeyword,
    /// ```ebnf
    /// (* Reserved in 0.5.0 *)
    /// PROMISE_KEYWORD = "promise";
    /// ```
    PromiseKeyword,
    /// ```ebnf
    /// PUBLIC_KEYWORD = "public";
    /// ```
    PublicKeyword,
    /// ```ebnf
    /// (* Introduced in 0.4.16 *)
    /// PURE_KEYWORD = "pure";
    /// ```
    PureKeyword,
    /// ```ebnf
    /// QUESTION_MARK = "?";
    /// ```
    QuestionMark,
    /// ```ebnf
    /// (* Reserved in 0.6.0 *)
    /// RECEIVE_KEYWORD = "receive";
    /// ```
    ReceiveKeyword,
    /// ```ebnf
    /// (* Reserved in 0.5.0 *)
    /// REFERENCE_KEYWORD = "reference";
    /// ```
    ReferenceKeyword,
    /// ```ebnf
    /// RELOCATABLE_KEYWORD = "relocatable";
    /// ```
    RelocatableKeyword,
    /// ```ebnf
    /// RETURN_KEYWORD = "return";
    /// ```
    ReturnKeyword,
    /// ```ebnf
    /// RETURNS_KEYWORD = "returns";
    /// ```
    ReturnsKeyword,
    /// ```ebnf
    /// (* Introduced in 0.8.4 *)
    /// (* Never reserved *)
    /// REVERT_KEYWORD = "revert";
    /// ```
    RevertKeyword,
    /// ```ebnf
    /// (* Reserved in 0.5.0 *)
    /// SEALED_KEYWORD = "sealed";
    /// ```
    SealedKeyword,
    /// ```ebnf
    /// SECONDS_KEYWORD = "seconds";
    /// ```
    SecondsKeyword,
    /// ```ebnf
    /// SEMICOLON = ";";
    /// ```
    Semicolon,
    /// ```ebnf
    /// SINGLE_LINE_COMMENT = "//" (!("\r" "\n"))*;
    /// ```
    SingleLineComment,
    /// ```ebnf
    /// SINGLE_LINE_NAT_SPEC_COMMENT = "///" (!("\r" "\n"))*;
    /// ```
    SingleLineNatSpecComment,
    /// ```ebnf
    /// SINGLE_QUOTED_HEX_STRING_LITERAL = "hex'" «HEX_STRING_CONTENTS»? "'";
    /// ```
    SingleQuotedHexStringLiteral,
    /// ```ebnf
    /// (* Deprecated in 0.4.25 *)
    /// SINGLE_QUOTED_STRING_LITERAL = "'" («ESCAPE_SEQUENCE_ARBITRARY» | !("'" "\\" "\r" "\n"))* "'";
    ///
    /// (* Introduced in 0.4.25 and deprecated in 0.7.0. *)
    /// SINGLE_QUOTED_STRING_LITERAL = "'" («ESCAPE_SEQUENCE» | !("'" "\\" "\r" "\n"))* "'";
    ///
    /// SINGLE_QUOTED_STRING_LITERAL = "'" («ESCAPE_SEQUENCE» | (" "…"&") | ("("…"[") | ("]"…"~"))* "'";
    /// ```
    SingleQuotedStringLiteral,
    /// ```ebnf
    /// (* Introduced in 0.7.0 *)
    /// SINGLE_QUOTED_UNICODE_STRING_LITERAL = "unicode'" («ESCAPE_SEQUENCE» | !("'" "\\" "\r" "\n"))* "'";
    /// ```
    SingleQuotedUnicodeStringLiteral,
    /// ```ebnf
    /// SINGLE_QUOTED_VERSION_LITERAL = "'" «VERSION_SPECIFIER_FRAGMENT» ("." «VERSION_SPECIFIER_FRAGMENT»)* "'";
    /// ```
    SingleQuotedVersionLiteral,
    /// ```ebnf
    /// (* Reserved in 0.5.0 *)
    /// SIZE_OF_KEYWORD = "sizeof";
    /// ```
    SizeOfKeyword,
    /// ```ebnf
    /// SLASH = "/";
    /// ```
    Slash,
    /// ```ebnf
    /// SLASH_EQUAL = "/=";
    /// ```
    SlashEqual,
    /// ```ebnf
    /// (* Never reserved *)
    /// SOLIDITY_KEYWORD = "solidity";
    /// ```
    SolidityKeyword,
    /// ```ebnf
    /// STATIC_KEYWORD = "static";
    /// ```
    StaticKeyword,
    /// ```ebnf
    /// STORAGE_KEYWORD = "storage";
    /// ```
    StorageKeyword,
    /// ```ebnf
    /// STRING_KEYWORD = "string";
    /// ```
    StringKeyword,
    /// ```ebnf
    /// STRUCT_KEYWORD = "struct";
    /// ```
    StructKeyword,
    /// ```ebnf
    /// SUPER_KEYWORD = "super";
    /// ```
    SuperKeyword,
    /// ```ebnf
    /// (* Reserved in 0.5.0 *)
    /// SUPPORTS_KEYWORD = "supports";
    /// ```
    SupportsKeyword,
    /// ```ebnf
    /// SWITCH_KEYWORD = "switch";
    /// ```
    SwitchKeyword,
    /// ```ebnf
    /// (* Deprecated in 0.7.0 *)
    /// (* Reserved until 0.7.0 *)
    /// SZABO_KEYWORD = "szabo";
    /// ```
    SzaboKeyword,
    /// ```ebnf
    /// THIS_KEYWORD = "this";
    /// ```
    ThisKeyword,
    /// ```ebnf
    /// (* Deprecated in 0.5.0 *)
    /// THROW_KEYWORD = "throw";
    /// ```
    ThrowKeyword,
    /// ```ebnf
    /// TILDE = "~";
    /// ```
    Tilde,
    /// ```ebnf
    /// (* Introduced in 0.8.27 *)
    /// (* Never reserved *)
    /// TRANSIENT_KEYWORD = "transient";
    /// ```
    TransientKeyword,
    /// ```ebnf
    /// TRUE_KEYWORD = "true";
    /// ```
    TrueKeyword,
    /// ```ebnf
    /// (* Introduced in 0.6.0 *)
    /// TRY_KEYWORD = "try";
    /// ```
    TryKeyword,
    /// ```ebnf
    /// (* Reserved in 0.5.0 *)
    /// TYPE_DEF_KEYWORD = "typedef";
    /// ```
    TypeDefKeyword,
    /// ```ebnf
    /// (* Introduced in 0.5.3 *)
    /// TYPE_KEYWORD = "type";
    /// ```
    TypeKeyword,
    /// ```ebnf
    /// TYPE_OF_KEYWORD = "typeof";
    /// ```
    TypeOfKeyword,
    /// ```ebnf
    /// UFIXED_KEYWORD = "ufixed";
    ///
    /// UFIXED_KEYWORD = "ufixed" ("8" | "16" | "24" | "32" | "40" | "48" | "56" | "64" | "72" | "80" | "88" | "96" | "104" | "112" | "120" | "128" | "136" | "144" | "152" | "160" | "168" | "176") "x" ("8" | "16" | "24" | "32" | "40" | "48" | "56" | "64" | "72" | "80");
    ///
    /// UFIXED_KEYWORD = "ufixed" ("184x8" | "184x16" | "184x24" | "184x32" | "184x40" | "184x48" | "184x56" | "184x64" | "184x72" | "192x8" | "192x16" | "192x24" | "192x32" | "192x40" | "192x48" | "192x56" | "192x64" | "200x8" | "200x16" | "200x24" | "200x32" | "200x40" | "200x48" | "200x56" | "208x8" | "208x16" | "208x24" | "208x32" | "208x40" | "208x48" | "216x8" | "216x16" | "216x24" | "216x32" | "216x40" | "224x8" | "224x16" | "224x24" | "224x32" | "232x8" | "232x16" | "232x24" | "240x8" | "240x16" | "248x8");
    ///
    /// (* Reserved in 0.4.14 *)
    /// UFIXED_KEYWORD = "ufixed" ("184x80" | "192x72" | "192x80" | "200x64" | "200x72" | "200x80" | "208x56" | "208x64" | "208x72" | "208x80" | "216x48" | "216x56" | "216x64" | "216x72" | "216x80" | "224x40" | "224x48" | "224x56" | "224x64" | "224x72" | "224x80" | "232x32" | "232x40" | "232x48" | "232x56" | "232x64" | "232x72" | "232x80" | "240x24" | "240x32" | "240x40" | "240x48" | "240x56" | "240x64" | "240x72" | "240x80" | "248x16" | "248x24" | "248x32" | "248x40" | "248x48" | "248x56" | "248x64" | "248x72" | "248x80" | "256x8" | "256x16" | "256x24" | "256x32" | "256x40" | "256x48" | "256x56" | "256x64" | "256x72" | "256x80");
    ///
    /// (* Reserved in 0.4.14 *)
    /// UFIXED_KEYWORD = "ufixed" ("8" | "16" | "24" | "32" | "40" | "48" | "56" | "64" | "72" | "80" | "88" | "96" | "104" | "112" | "120" | "128" | "136" | "144" | "152" | "160" | "168" | "176" | "184" | "192" | "200" | "208" | "216" | "224" | "232" | "240" | "248" | "256") "x" ("0" | "1" | "2" | "3" | "4" | "5" | "6" | "7" | "9" | "10" | "11" | "12" | "13" | "14" | "15" | "17" | "18" | "19" | "20" | "21" | "22" | "23" | "25" | "26" | "27" | "28" | "29" | "30" | "31" | "33" | "34" | "35" | "36" | "37" | "38" | "39" | "41" | "42" | "43" | "44" | "45" | "46" | "47" | "49" | "50" | "51" | "52" | "53" | "54" | "55" | "57" | "58" | "59" | "60" | "61" | "62" | "63" | "65" | "66" | "67" | "68" | "69" | "70" | "71" | "73" | "74" | "75" | "76" | "77" | "78" | "79");
    /// ```
    UfixedKeyword,
    /// ```ebnf
    /// UINT_KEYWORD = "uint" ("8" | "16" | "24" | "32" | "40" | "48" | "56" | "64" | "72" | "80" | "88" | "96" | "104" | "112" | "120" | "128" | "136" | "144" | "152" | "160" | "168" | "176" | "184" | "192" | "200" | "208" | "216" | "224" | "232" | "240" | "248" | "256")?;
    /// ```
    UintKeyword,
    /// ```ebnf
    /// (* Introduced in 0.8.0 *)
    /// (* Reserved in 0.5.0 *)
    /// UNCHECKED_KEYWORD = "unchecked";
    /// ```
    UncheckedKeyword,
    /// ```ebnf
    /// USING_KEYWORD = "using";
    /// ```
    UsingKeyword,
    /// ```ebnf
    /// (* Deprecated in 0.5.0 *)
    /// VAR_KEYWORD = "var";
    /// ```
    VarKeyword,
    /// ```ebnf
    /// VERSION_SPECIFIER = «VERSION_SPECIFIER_FRAGMENT»;
    /// ```
    VersionSpecifier,
    /// ```ebnf
    /// (* Introduced in 0.4.16 *)
    /// VIEW_KEYWORD = "view";
    /// ```
    ViewKeyword,
    /// ```ebnf
    /// (* Introduced in 0.6.0 *)
    /// (* Reserved in 0.6.0 *)
    /// VIRTUAL_KEYWORD = "virtual";
    /// ```
    VirtualKeyword,
    /// ```ebnf
    /// WEEKS_KEYWORD = "weeks";
    /// ```
    WeeksKeyword,
    /// ```ebnf
    /// WEI_KEYWORD = "wei";
    /// ```
    WeiKeyword,
    /// ```ebnf
    /// WHILE_KEYWORD = "while";
    /// ```
    WhileKeyword,
    /// ```ebnf
    /// WHITESPACE = (" " | "\t")+;
    /// ```
    Whitespace,
    /// ```ebnf
    /// (* Deprecated in 0.5.0 *)
    /// YEARS_KEYWORD = "years";
    /// ```
    YearsKeyword,
    /// ```ebnf
    /// (* Reserved until 0.7.1 *)
    /// YUL_ABSTRACT_KEYWORD = "abstract";
    /// ```
    YulAbstractKeyword,
    /// ```ebnf
    /// YUL_ADD_KEYWORD = "add";
    /// ```
    YulAddKeyword,
    /// ```ebnf
    /// YUL_ADD_MOD_KEYWORD = "addmod";
    /// ```
    YulAddModKeyword,
    /// ```ebnf
    /// (* Never reserved *)
    /// YUL_ADDRESS_KEYWORD = "address";
    /// ```
    YulAddressKeyword,
    /// ```ebnf
    /// (* Reserved until 0.7.1 *)
    /// YUL_AFTER_KEYWORD = "after";
    /// ```
    YulAfterKeyword,
    /// ```ebnf
    /// (* Reserved from 0.5.0 until 0.7.1 *)
    /// YUL_ALIAS_KEYWORD = "alias";
    /// ```
    YulAliasKeyword,
    /// ```ebnf
    /// YUL_AND_KEYWORD = "and";
    /// ```
    YulAndKeyword,
    /// ```ebnf
    /// (* Reserved until 0.7.1 *)
    /// YUL_ANONYMOUS_KEYWORD = "anonymous";
    /// ```
    YulAnonymousKeyword,
    /// ```ebnf
    /// (* Reserved from 0.5.0 until 0.7.1 *)
    /// YUL_APPLY_KEYWORD = "apply";
    /// ```
    YulApplyKeyword,
    /// ```ebnf
    /// (* Reserved until 0.7.1 *)
    /// YUL_AS_KEYWORD = "as";
    /// ```
    YulAsKeyword,
    /// ```ebnf
    /// (* Reserved until 0.7.1 *)
    /// YUL_ASSEMBLY_KEYWORD = "assembly";
    /// ```
    YulAssemblyKeyword,
    /// ```ebnf
    /// (* Reserved from 0.5.0 until 0.7.1 *)
    /// YUL_AUTO_KEYWORD = "auto";
    /// ```
    YulAutoKeyword,
    /// ```ebnf
    /// YUL_BALANCE_KEYWORD = "balance";
    /// ```
    YulBalanceKeyword,
    /// ```ebnf
    /// (* Introduced in 0.8.7 *)
    /// (* Reserved in 0.8.7 *)
    /// YUL_BASE_FEE_KEYWORD = "basefee";
    /// ```
    YulBaseFeeKeyword,
    /// ```ebnf
    /// (* Introduced in 0.8.24 *)
    /// (* Reserved in 0.8.25 *)
    /// YUL_BLOB_BASE_FEE_KEYWORD = "blobbasefee";
    /// ```
    YulBlobBaseFeeKeyword,
    /// ```ebnf
    /// (* Introduced in 0.8.24 *)
    /// (* Reserved in 0.8.25 *)
    /// YUL_BLOB_HASH_KEYWORD = "blobhash";
    /// ```
    YulBlobHashKeyword,
    /// ```ebnf
    /// YUL_BLOCK_HASH_KEYWORD = "blockhash";
    /// ```
    YulBlockHashKeyword,
    /// ```ebnf
    /// (* Reserved until 0.5.10 *)
    /// YUL_BOOL_KEYWORD = "bool";
    /// ```
    YulBoolKeyword,
    /// ```ebnf
    /// YUL_BREAK_KEYWORD = "break";
    /// ```
    YulBreakKeyword,
    /// ```ebnf
    /// YUL_BYTE_KEYWORD = "byte";
    /// ```
    YulByteKeyword,
    /// ```ebnf
    /// (* Reserved until 0.7.1 *)
    /// YUL_BYTES_KEYWORD = "bytes" ("1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9" | "10" | "11" | "12" | "13" | "14" | "15" | "16" | "17" | "18" | "19" | "20" | "21" | "22" | "23" | "24" | "25" | "26" | "27" | "28" | "29" | "30" | "31" | "32")?;
    /// ```
    YulBytesKeyword,
    /// ```ebnf
    /// YUL_CALL_CODE_KEYWORD = "callcode";
    /// ```
    YulCallCodeKeyword,
    /// ```ebnf
    /// YUL_CALL_DATA_COPY_KEYWORD = "calldatacopy";
    /// ```
    YulCallDataCopyKeyword,
    /// ```ebnf
    /// (* Reserved from 0.5.0 until 0.7.1 *)
    /// YUL_CALL_DATA_KEYWORD = "calldata";
    /// ```
    YulCallDataKeyword,
    /// ```ebnf
    /// YUL_CALL_DATA_LOAD_KEYWORD = "calldataload";
    /// ```
    YulCallDataLoadKeyword,
    /// ```ebnf
    /// YUL_CALL_DATA_SIZE_KEYWORD = "calldatasize";
    /// ```
    YulCallDataSizeKeyword,
    /// ```ebnf
    /// YUL_CALL_KEYWORD = "call";
    /// ```
    YulCallKeyword,
    /// ```ebnf
    /// YUL_CALL_VALUE_KEYWORD = "callvalue";
    /// ```
    YulCallValueKeyword,
    /// ```ebnf
    /// YUL_CALLER_KEYWORD = "caller";
    /// ```
    YulCallerKeyword,
    /// ```ebnf
    /// YUL_CASE_KEYWORD = "case";
    /// ```
    YulCaseKeyword,
    /// ```ebnf
    /// (* Reserved until 0.7.1 *)
    /// YUL_CATCH_KEYWORD = "catch";
    /// ```
    YulCatchKeyword,
    /// ```ebnf
    /// (* Reserved in 0.5.12 *)
    /// YUL_CHAIN_ID_KEYWORD = "chainid";
    /// ```
    YulChainIdKeyword,
    /// ```ebnf
    /// YUL_COIN_BASE_KEYWORD = "coinbase";
    /// ```
    YulCoinBaseKeyword,
    /// ```ebnf
    /// (* Reserved until 0.7.1 *)
    /// YUL_CONSTANT_KEYWORD = "constant";
    /// ```
    YulConstantKeyword,
    /// ```ebnf
    /// (* Reserved from 0.5.0 until 0.7.1 *)
    /// YUL_CONSTRUCTOR_KEYWORD = "constructor";
    /// ```
    YulConstructorKeyword,
    /// ```ebnf
    /// YUL_CONTINUE_KEYWORD = "continue";
    /// ```
    YulContinueKeyword,
    /// ```ebnf
    /// (* Reserved until 0.7.1 *)
    /// YUL_CONTRACT_KEYWORD = "contract";
    /// ```
    YulContractKeyword,
    /// ```ebnf
    /// (* Reserved from 0.5.0 until 0.7.1 *)
    /// YUL_COPY_OF_KEYWORD = "copyof";
    /// ```
    YulCopyOfKeyword,
    /// ```ebnf
    /// (* Introduced in 0.4.12 *)
    /// (* Reserved in 0.4.12 *)
    /// YUL_CREATE_2_KEYWORD = "create2";
    /// ```
    YulCreate2Keyword,
    /// ```ebnf
    /// YUL_CREATE_KEYWORD = "create";
    /// ```
    YulCreateKeyword,
    /// ```ebnf
    /// (* Reserved until 0.7.1 *)
    /// YUL_DAYS_KEYWORD = "days";
    /// ```
    YulDaysKeyword,
    /// ```ebnf
    /// YUL_DECIMAL_LITERAL = "0" | (("1"…"9") ("0"…"9")*);
    /// ```
    YulDecimalLiteral,
    /// ```ebnf
    /// YUL_DEFAULT_KEYWORD = "default";
    /// ```
    YulDefaultKeyword,
    /// ```ebnf
    /// (* Reserved from 0.5.0 until 0.7.1 *)
    /// YUL_DEFINE_KEYWORD = "define";
    /// ```
    YulDefineKeyword,
    /// ```ebnf
    /// YUL_DELEGATE_CALL_KEYWORD = "delegatecall";
    /// ```
    YulDelegateCallKeyword,
    /// ```ebnf
    /// (* Reserved until 0.7.1 *)
    /// YUL_DELETE_KEYWORD = "delete";
    /// ```
    YulDeleteKeyword,
    /// ```ebnf
    /// (* Deprecated in 0.8.18 *)
    /// YUL_DIFFICULTY_KEYWORD = "difficulty";
    /// ```
    YulDifficultyKeyword,
    /// ```ebnf
    /// YUL_DIV_KEYWORD = "div";
    /// ```
    YulDivKeyword,
    /// ```ebnf
    /// (* Reserved until 0.7.1 *)
    /// YUL_DO_KEYWORD = "do";
    /// ```
    YulDoKeyword,
    /// ```ebnf
    /// (* Reserved until 0.7.1 *)
    /// YUL_ELSE_KEYWORD = "else";
    /// ```
    YulElseKeyword,
    /// ```ebnf
    /// (* Reserved from 0.5.0 until 0.7.1 *)
    /// YUL_EMIT_KEYWORD = "emit";
    /// ```
    YulEmitKeyword,
    /// ```ebnf
    /// (* Reserved until 0.7.1 *)
    /// YUL_ENUM_KEYWORD = "enum";
    /// ```
    YulEnumKeyword,
    /// ```ebnf
    /// YUL_EQ_KEYWORD = "eq";
    /// ```
    YulEqKeyword,
    /// ```ebnf
    /// (* Reserved until 0.7.1 *)
    /// YUL_ETHER_KEYWORD = "ether";
    /// ```
    YulEtherKeyword,
    /// ```ebnf
    /// (* Reserved until 0.7.1 *)
    /// YUL_EVENT_KEYWORD = "event";
    /// ```
    YulEventKeyword,
    /// ```ebnf
    /// YUL_EXP_KEYWORD = "exp";
    /// ```
    YulExpKeyword,
    /// ```ebnf
    /// YUL_EXT_CODE_COPY_KEYWORD = "extcodecopy";
    /// ```
    YulExtCodeCopyKeyword,
    /// ```ebnf
    /// (* Introduced in 0.5.0 *)
    /// (* Reserved in 0.5.0 *)
    /// YUL_EXT_CODE_HASH_KEYWORD = "extcodehash";
    /// ```
    YulExtCodeHashKeyword,
    /// ```ebnf
    /// YUL_EXT_CODE_SIZE_KEYWORD = "extcodesize";
    /// ```
    YulExtCodeSizeKeyword,
    /// ```ebnf
    /// (* Reserved until 0.7.1 *)
    /// YUL_EXTERNAL_KEYWORD = "external";
    /// ```
    YulExternalKeyword,
    /// ```ebnf
    /// (* Reserved from 0.6.0 until 0.7.1 *)
    /// YUL_FALLBACK_KEYWORD = "fallback";
    /// ```
    YulFallbackKeyword,
    /// ```ebnf
    /// YUL_FALSE_KEYWORD = "false";
    /// ```
    YulFalseKeyword,
    /// ```ebnf
    /// (* Reserved until 0.7.1 *)
    /// YUL_FINAL_KEYWORD = "final";
    /// ```
    YulFinalKeyword,
    /// ```ebnf
    /// (* Reserved until 0.7.0 *)
    /// YUL_FINNEY_KEYWORD = "finney";
    /// ```
    YulFinneyKeyword,
    /// ```ebnf
    /// (* Reserved until 0.7.1 *)
    /// YUL_FIXED_KEYWORD = "fixed";
    ///
    /// (* Reserved until 0.7.1 *)
    /// YUL_FIXED_KEYWORD = "fixed" ("8" | "16" | "24" | "32" | "40" | "48" | "56" | "64" | "72" | "80" | "88" | "96" | "104" | "112" | "120" | "128" | "136" | "144" | "152" | "160" | "168" | "176") "x" ("8" | "16" | "24" | "32" | "40" | "48" | "56" | "64" | "72" | "80");
    ///
    /// (* Reserved until 0.7.1 *)
    /// YUL_FIXED_KEYWORD = "fixed" ("184x8" | "184x16" | "184x24" | "184x32" | "184x40" | "184x48" | "184x56" | "184x64" | "184x72" | "192x8" | "192x16" | "192x24" | "192x32" | "192x40" | "192x48" | "192x56" | "192x64" | "200x8" | "200x16" | "200x24" | "200x32" | "200x40" | "200x48" | "200x56" | "208x8" | "208x16" | "208x24" | "208x32" | "208x40" | "208x48" | "216x8" | "216x16" | "216x24" | "216x32" | "216x40" | "224x8" | "224x16" | "224x24" | "224x32" | "232x8" | "232x16" | "232x24" | "240x8" | "240x16" | "248x8");
    ///
    /// (* Reserved from 0.4.14 until 0.7.1 *)
    /// YUL_FIXED_KEYWORD = "fixed" ("184x80" | "192x72" | "192x80" | "200x64" | "200x72" | "200x80" | "208x56" | "208x64" | "208x72" | "208x80" | "216x48" | "216x56" | "216x64" | "216x72" | "216x80" | "224x40" | "224x48" | "224x56" | "224x64" | "224x72" | "224x80" | "232x32" | "232x40" | "232x48" | "232x56" | "232x64" | "232x72" | "232x80" | "240x24" | "240x32" | "240x40" | "240x48" | "240x56" | "240x64" | "240x72" | "240x80" | "248x16" | "248x24" | "248x32" | "248x40" | "248x48" | "248x56" | "248x64" | "248x72" | "248x80" | "256x8" | "256x16" | "256x24" | "256x32" | "256x40" | "256x48" | "256x56" | "256x64" | "256x72" | "256x80");
    ///
    /// (* Reserved from 0.4.14 until 0.7.1 *)
    /// YUL_FIXED_KEYWORD = "fixed" ("8" | "16" | "24" | "32" | "40" | "48" | "56" | "64" | "72" | "80" | "88" | "96" | "104" | "112" | "120" | "128" | "136" | "144" | "152" | "160" | "168" | "176" | "184" | "192" | "200" | "208" | "216" | "224" | "232" | "240" | "248" | "256") "x" ("0" | "1" | "2" | "3" | "4" | "5" | "6" | "7" | "9" | "10" | "11" | "12" | "13" | "14" | "15" | "17" | "18" | "19" | "20" | "21" | "22" | "23" | "25" | "26" | "27" | "28" | "29" | "30" | "31" | "33" | "34" | "35" | "36" | "37" | "38" | "39" | "41" | "42" | "43" | "44" | "45" | "46" | "47" | "49" | "50" | "51" | "52" | "53" | "54" | "55" | "57" | "58" | "59" | "60" | "61" | "62" | "63" | "65" | "66" | "67" | "68" | "69" | "70" | "71" | "73" | "74" | "75" | "76" | "77" | "78" | "79");
    /// ```
    YulFixedKeyword,
    /// ```ebnf
    /// YUL_FOR_KEYWORD = "for";
    /// ```
    YulForKeyword,
    /// ```ebnf
    /// YUL_FUNCTION_KEYWORD = "function";
    /// ```
    YulFunctionKeyword,
    /// ```ebnf
    /// YUL_GAS_KEYWORD = "gas";
    /// ```
    YulGasKeyword,
    /// ```ebnf
    /// YUL_GAS_LIMIT_KEYWORD = "gaslimit";
    /// ```
    YulGasLimitKeyword,
    /// ```ebnf
    /// YUL_GAS_PRICE_KEYWORD = "gasprice";
    /// ```
    YulGasPriceKeyword,
    /// ```ebnf
    /// YUL_GT_KEYWORD = "gt";
    /// ```
    YulGtKeyword,
    /// ```ebnf
    /// (* Reserved from 0.7.0 until 0.7.1 *)
    /// YUL_GWEI_KEYWORD = "gwei";
    /// ```
    YulGweiKeyword,
    /// ```ebnf
    /// YUL_HEX_KEYWORD = "hex";
    /// ```
    YulHexKeyword,
    /// ```ebnf
    /// YUL_HEX_LITERAL = "0x" «HEX_CHARACTER»+;
    /// ```
    YulHexLiteral,
    /// ```ebnf
    /// (* Reserved until 0.7.1 *)
    /// YUL_HOURS_KEYWORD = "hours";
    /// ```
    YulHoursKeyword,
    /// ```ebnf
    /// (* Introduced in 0.5.8 and deprecated in 0.7.0. *)
    /// YUL_IDENTIFIER = «IDENTIFIER_START» («IDENTIFIER_PART» | ".")*;
    ///
    /// YUL_IDENTIFIER = «IDENTIFIER_START» «IDENTIFIER_PART»*;
    /// ```
    YulIdentifier,
    /// ```ebnf
    /// YUL_IF_KEYWORD = "if";
    /// ```
    YulIfKeyword,
    /// ```ebnf
    /// (* Reserved from 0.5.0 until 0.7.1 *)
    /// YUL_IMMUTABLE_KEYWORD = "immutable";
    /// ```
    YulImmutableKeyword,
    /// ```ebnf
    /// (* Reserved from 0.5.0 until 0.7.1 *)
    /// YUL_IMPLEMENTS_KEYWORD = "implements";
    /// ```
    YulImplementsKeyword,
    /// ```ebnf
    /// (* Reserved until 0.7.1 *)
    /// YUL_IMPORT_KEYWORD = "import";
    /// ```
    YulImportKeyword,
    /// ```ebnf
    /// (* Reserved until 0.6.8 *)
    /// YUL_IN_KEYWORD = "in";
    /// ```
    YulInKeyword,
    /// ```ebnf
    /// (* Reserved until 0.7.1 *)
    /// YUL_INDEXED_KEYWORD = "indexed";
    /// ```
    YulIndexedKeyword,
    /// ```ebnf
    /// (* Reserved until 0.7.1 *)
    /// YUL_INLINE_KEYWORD = "inline";
    /// ```
    YulInlineKeyword,
    /// ```ebnf
    /// (* Reserved until 0.7.1 *)
    /// YUL_INT_KEYWORD = "int" ("8" | "16" | "24" | "32" | "40" | "48" | "56" | "64" | "72" | "80" | "88" | "96" | "104" | "112" | "120" | "128" | "136" | "144" | "152" | "160" | "168" | "176" | "184" | "192" | "200" | "208" | "216" | "224" | "232" | "240" | "248" | "256")?;
    /// ```
    YulIntKeyword,
    /// ```ebnf
    /// (* Reserved until 0.7.1 *)
    /// YUL_INTERFACE_KEYWORD = "interface";
    /// ```
    YulInterfaceKeyword,
    /// ```ebnf
    /// (* Reserved until 0.7.1 *)
    /// YUL_INTERNAL_KEYWORD = "internal";
    /// ```
    YulInternalKeyword,
    /// ```ebnf
    /// YUL_INVALID_KEYWORD = "invalid";
    /// ```
    YulInvalidKeyword,
    /// ```ebnf
    /// (* Reserved until 0.7.1 *)
    /// YUL_IS_KEYWORD = "is";
    /// ```
    YulIsKeyword,
    /// ```ebnf
    /// YUL_IS_ZERO_KEYWORD = "iszero";
    /// ```
    YulIsZeroKeyword,
    /// ```ebnf
    /// (* Deprecated in 0.5.0 *)
    /// YUL_JUMP_KEYWORD = "jump";
    /// ```
    YulJumpKeyword,
    /// ```ebnf
    /// (* Deprecated in 0.5.0 *)
    /// YUL_JUMPI_KEYWORD = "jumpi";
    /// ```
    YulJumpiKeyword,
    /// ```ebnf
    /// (* Introduced in 0.4.12 *)
    /// (* Reserved in 0.4.12 *)
    /// YUL_KECCAK_256_KEYWORD = "keccak256";
    /// ```
    YulKeccak256Keyword,
    /// ```ebnf
    /// (* Introduced in 0.6.0 *)
    /// (* Reserved in 0.7.1 *)
    /// YUL_LEAVE_KEYWORD = "leave";
    /// ```
    YulLeaveKeyword,
    /// ```ebnf
    /// YUL_LET_KEYWORD = "let";
    /// ```
    YulLetKeyword,
    /// ```ebnf
    /// (* Reserved until 0.7.1 *)
    /// YUL_LIBRARY_KEYWORD = "library";
    /// ```
    YulLibraryKeyword,
    /// ```ebnf
    /// YUL_LOG_0_KEYWORD = "log0";
    /// ```
    YulLog0Keyword,
    /// ```ebnf
    /// YUL_LOG_1_KEYWORD = "log1";
    /// ```
    YulLog1Keyword,
    /// ```ebnf
    /// YUL_LOG_2_KEYWORD = "log2";
    /// ```
    YulLog2Keyword,
    /// ```ebnf
    /// YUL_LOG_3_KEYWORD = "log3";
    /// ```
    YulLog3Keyword,
    /// ```ebnf
    /// YUL_LOG_4_KEYWORD = "log4";
    /// ```
    YulLog4Keyword,
    /// ```ebnf
    /// YUL_LT_KEYWORD = "lt";
    /// ```
    YulLtKeyword,
    /// ```ebnf
    /// (* Introduced in 0.8.24 *)
    /// (* Reserved in 0.8.25 *)
    /// YUL_M_COPY_KEYWORD = "mcopy";
    /// ```
    YulMCopyKeyword,
    /// ```ebnf
    /// YUL_M_LOAD_KEYWORD = "mload";
    /// ```
    YulMLoadKeyword,
    /// ```ebnf
    /// YUL_M_SIZE_KEYWORD = "msize";
    /// ```
    YulMSizeKeyword,
    /// ```ebnf
    /// YUL_M_STORE_8_KEYWORD = "mstore8";
    /// ```
    YulMStore8Keyword,
    /// ```ebnf
    /// YUL_M_STORE_KEYWORD = "mstore";
    /// ```
    YulMStoreKeyword,
    /// ```ebnf
    /// (* Reserved from 0.5.0 until 0.7.1 *)
    /// YUL_MACRO_KEYWORD = "macro";
    /// ```
    YulMacroKeyword,
    /// ```ebnf
    /// (* Reserved until 0.7.1 *)
    /// YUL_MAPPING_KEYWORD = "mapping";
    /// ```
    YulMappingKeyword,
    /// ```ebnf
    /// (* Reserved until 0.7.1 *)
    /// YUL_MATCH_KEYWORD = "match";
    /// ```
    YulMatchKeyword,
    /// ```ebnf
    /// (* Reserved until 0.7.1 *)
    /// YUL_MEMORY_KEYWORD = "memory";
    /// ```
    YulMemoryKeyword,
    /// ```ebnf
    /// (* Reserved until 0.7.1 *)
    /// YUL_MINUTES_KEYWORD = "minutes";
    /// ```
    YulMinutesKeyword,
    /// ```ebnf
    /// YUL_MOD_KEYWORD = "mod";
    /// ```
    YulModKeyword,
    /// ```ebnf
    /// (* Reserved until 0.7.1 *)
    /// YUL_MODIFIER_KEYWORD = "modifier";
    /// ```
    YulModifierKeyword,
    /// ```ebnf
    /// YUL_MUL_KEYWORD = "mul";
    /// ```
    YulMulKeyword,
    /// ```ebnf
    /// YUL_MUL_MOD_KEYWORD = "mulmod";
    /// ```
    YulMulModKeyword,
    /// ```ebnf
    /// (* Reserved from 0.5.0 until 0.7.1 *)
    /// YUL_MUTABLE_KEYWORD = "mutable";
    /// ```
    YulMutableKeyword,
    /// ```ebnf
    /// (* Reserved until 0.7.1 *)
    /// YUL_NEW_KEYWORD = "new";
    /// ```
    YulNewKeyword,
    /// ```ebnf
    /// YUL_NOT_KEYWORD = "not";
    /// ```
    YulNotKeyword,
    /// ```ebnf
    /// (* Reserved until 0.7.1 *)
    /// YUL_NULL_KEYWORD = "null";
    /// ```
    YulNullKeyword,
    /// ```ebnf
    /// YUL_NUMBER_KEYWORD = "number";
    /// ```
    YulNumberKeyword,
    /// ```ebnf
    /// (* Reserved until 0.7.1 *)
    /// YUL_OF_KEYWORD = "of";
    /// ```
    YulOfKeyword,
    /// ```ebnf
    /// YUL_OR_KEYWORD = "or";
    /// ```
    YulOrKeyword,
    /// ```ebnf
    /// YUL_ORIGIN_KEYWORD = "origin";
    /// ```
    YulOriginKeyword,
    /// ```ebnf
    /// (* Reserved from 0.5.0 until 0.7.1 *)
    /// YUL_OVERRIDE_KEYWORD = "override";
    /// ```
    YulOverrideKeyword,
    /// ```ebnf
    /// (* Reserved from 0.5.0 until 0.7.1 *)
    /// YUL_PARTIAL_KEYWORD = "partial";
    /// ```
    YulPartialKeyword,
    /// ```ebnf
    /// (* Reserved until 0.7.1 *)
    /// YUL_PAYABLE_KEYWORD = "payable";
    /// ```
    YulPayableKeyword,
    /// ```ebnf
    /// YUL_POP_KEYWORD = "pop";
    /// ```
    YulPopKeyword,
    /// ```ebnf
    /// (* Reserved until 0.7.1 *)
    /// YUL_PRAGMA_KEYWORD = "pragma";
    /// ```
    YulPragmaKeyword,
    /// ```ebnf
    /// (* Introduced in 0.8.18 *)
    /// (* Reserved in 0.8.18 *)
    /// YUL_PREV_RANDAO_KEYWORD = "prevrandao";
    /// ```
    YulPrevRandaoKeyword,
    /// ```ebnf
    /// (* Reserved until 0.7.1 *)
    /// YUL_PRIVATE_KEYWORD = "private";
    /// ```
    YulPrivateKeyword,
    /// ```ebnf
    /// (* Reserved from 0.5.0 until 0.7.1 *)
    /// YUL_PROMISE_KEYWORD = "promise";
    /// ```
    YulPromiseKeyword,
    /// ```ebnf
    /// (* Reserved until 0.7.1 *)
    /// YUL_PUBLIC_KEYWORD = "public";
    /// ```
    YulPublicKeyword,
    /// ```ebnf
    /// (* Reserved until 0.7.1 *)
    /// YUL_PURE_KEYWORD = "pure";
    /// ```
    YulPureKeyword,
    /// ```ebnf
    /// (* Reserved from 0.6.0 until 0.7.1 *)
    /// YUL_RECEIVE_KEYWORD = "receive";
    /// ```
    YulReceiveKeyword,
    /// ```ebnf
    /// (* Reserved from 0.5.0 until 0.7.1 *)
    /// YUL_REFERENCE_KEYWORD = "reference";
    /// ```
    YulReferenceKeyword,
    /// ```ebnf
    /// (* Reserved until 0.7.1 *)
    /// YUL_RELOCATABLE_KEYWORD = "relocatable";
    /// ```
    YulRelocatableKeyword,
    /// ```ebnf
    /// (* Introduced in 0.4.12 *)
    /// (* Reserved in 0.4.12 *)
    /// YUL_RETURN_DATA_COPY_KEYWORD = "returndatacopy";
    /// ```
    YulReturnDataCopyKeyword,
    /// ```ebnf
    /// (* Introduced in 0.4.12 *)
    /// (* Reserved in 0.4.12 *)
    /// YUL_RETURN_DATA_SIZE_KEYWORD = "returndatasize";
    /// ```
    YulReturnDataSizeKeyword,
    /// ```ebnf
    /// YUL_RETURN_KEYWORD = "return";
    /// ```
    YulReturnKeyword,
    /// ```ebnf
    /// (* Reserved until 0.7.1 *)
    /// YUL_RETURNS_KEYWORD = "returns";
    /// ```
    YulReturnsKeyword,
    /// ```ebnf
    /// YUL_REVERT_KEYWORD = "revert";
    /// ```
    YulRevertKeyword,
    /// ```ebnf
    /// YUL_S_DIV_KEYWORD = "sdiv";
    /// ```
    YulSDivKeyword,
    /// ```ebnf
    /// YUL_S_LOAD_KEYWORD = "sload";
    /// ```
    YulSLoadKeyword,
    /// ```ebnf
    /// YUL_S_MOD_KEYWORD = "smod";
    /// ```
    YulSModKeyword,
    /// ```ebnf
    /// YUL_S_STORE_KEYWORD = "sstore";
    /// ```
    YulSStoreKeyword,
    /// ```ebnf
    /// (* Reserved in 0.4.21 *)
    /// YUL_SAR_KEYWORD = "sar";
    /// ```
    YulSarKeyword,
    /// ```ebnf
    /// (* Reserved from 0.5.0 until 0.7.1 *)
    /// YUL_SEALED_KEYWORD = "sealed";
    /// ```
    YulSealedKeyword,
    /// ```ebnf
    /// (* Reserved until 0.7.1 *)
    /// YUL_SECONDS_KEYWORD = "seconds";
    /// ```
    YulSecondsKeyword,
    /// ```ebnf
    /// (* Reserved in 0.5.12 *)
    /// YUL_SELF_BALANCE_KEYWORD = "selfbalance";
    /// ```
    YulSelfBalanceKeyword,
    /// ```ebnf
    /// YUL_SELF_DESTRUCT_KEYWORD = "selfdestruct";
    /// ```
    YulSelfDestructKeyword,
    /// ```ebnf
    /// YUL_SGT_KEYWORD = "sgt";
    /// ```
    YulSgtKeyword,
    /// ```ebnf
    /// (* Deprecated in 0.5.0 *)
    /// (* Reserved until 0.5.0 *)
    /// YUL_SHA_3_KEYWORD = "sha3";
    /// ```
    YulSha3Keyword,
    /// ```ebnf
    /// (* Reserved in 0.4.21 *)
    /// YUL_SHL_KEYWORD = "shl";
    /// ```
    YulShlKeyword,
    /// ```ebnf
    /// (* Reserved in 0.4.21 *)
    /// YUL_SHR_KEYWORD = "shr";
    /// ```
    YulShrKeyword,
    /// ```ebnf
    /// YUL_SIGN_EXTEND_KEYWORD = "signextend";
    /// ```
    YulSignExtendKeyword,
    /// ```ebnf
    /// (* Reserved from 0.5.0 until 0.7.1 *)
    /// YUL_SIZE_OF_KEYWORD = "sizeof";
    /// ```
    YulSizeOfKeyword,
    /// ```ebnf
    /// YUL_SLT_KEYWORD = "slt";
    /// ```
    YulSltKeyword,
    /// ```ebnf
    /// (* Introduced in 0.4.12 *)
    /// (* Reserved in 0.4.12 *)
    /// YUL_STATIC_CALL_KEYWORD = "staticcall";
    /// ```
    YulStaticCallKeyword,
    /// ```ebnf
    /// (* Reserved until 0.7.1 *)
    /// YUL_STATIC_KEYWORD = "static";
    /// ```
    YulStaticKeyword,
    /// ```ebnf
    /// YUL_STOP_KEYWORD = "stop";
    /// ```
    YulStopKeyword,
    /// ```ebnf
    /// (* Reserved until 0.7.1 *)
    /// YUL_STORAGE_KEYWORD = "storage";
    /// ```
    YulStorageKeyword,
    /// ```ebnf
    /// (* Reserved until 0.7.1 *)
    /// YUL_STRING_KEYWORD = "string";
    /// ```
    YulStringKeyword,
    /// ```ebnf
    /// (* Reserved until 0.7.1 *)
    /// YUL_STRUCT_KEYWORD = "struct";
    /// ```
    YulStructKeyword,
    /// ```ebnf
    /// YUL_SUB_KEYWORD = "sub";
    /// ```
    YulSubKeyword,
    /// ```ebnf
    /// (* Deprecated in 0.5.0 *)
    /// (* Reserved until 0.5.0 *)
    /// YUL_SUICIDE_KEYWORD = "suicide";
    /// ```
    YulSuicideKeyword,
    /// ```ebnf
    /// YUL_SUPER_KEYWORD = "super";
    /// ```
    YulSuperKeyword,
    /// ```ebnf
    /// (* Reserved from 0.5.0 until 0.7.1 *)
    /// YUL_SUPPORTS_KEYWORD = "supports";
    /// ```
    YulSupportsKeyword,
    /// ```ebnf
    /// YUL_SWITCH_KEYWORD = "switch";
    /// ```
    YulSwitchKeyword,
    /// ```ebnf
    /// (* Reserved until 0.7.0 *)
    /// YUL_SZABO_KEYWORD = "szabo";
    /// ```
    YulSzaboKeyword,
    /// ```ebnf
    /// (* Introduced in 0.8.24 *)
    /// (* Reserved in 0.8.25 *)
    /// YUL_T_LOAD_KEYWORD = "tload";
    /// ```
    YulTLoadKeyword,
    /// ```ebnf
    /// (* Introduced in 0.8.24 *)
    /// (* Reserved in 0.8.25 *)
    /// YUL_T_STORE_KEYWORD = "tstore";
    /// ```
    YulTStoreKeyword,
    /// ```ebnf
    /// YUL_THIS_KEYWORD = "this";
    /// ```
    YulThisKeyword,
    /// ```ebnf
    /// (* Reserved until 0.7.1 *)
    /// YUL_THROW_KEYWORD = "throw";
    /// ```
    YulThrowKeyword,
    /// ```ebnf
    /// YUL_TIMESTAMP_KEYWORD = "timestamp";
    /// ```
    YulTimestampKeyword,
    /// ```ebnf
    /// YUL_TRUE_KEYWORD = "true";
    /// ```
    YulTrueKeyword,
    /// ```ebnf
    /// (* Reserved until 0.7.1 *)
    /// YUL_TRY_KEYWORD = "try";
    /// ```
    YulTryKeyword,
    /// ```ebnf
    /// (* Reserved from 0.5.0 until 0.7.1 *)
    /// YUL_TYPE_DEF_KEYWORD = "typedef";
    /// ```
    YulTypeDefKeyword,
    /// ```ebnf
    /// (* Reserved until 0.7.1 *)
    /// YUL_TYPE_KEYWORD = "type";
    /// ```
    YulTypeKeyword,
    /// ```ebnf
    /// (* Reserved until 0.7.1 *)
    /// YUL_TYPE_OF_KEYWORD = "typeof";
    /// ```
    YulTypeOfKeyword,
    /// ```ebnf
    /// (* Reserved until 0.7.1 *)
    /// YUL_UFIXED_KEYWORD = "ufixed";
    ///
    /// (* Reserved until 0.7.1 *)
    /// YUL_UFIXED_KEYWORD = "ufixed" ("8" | "16" | "24" | "32" | "40" | "48" | "56" | "64" | "72" | "80" | "88" | "96" | "104" | "112" | "120" | "128" | "136" | "144" | "152" | "160" | "168" | "176") "x" ("8" | "16" | "24" | "32" | "40" | "48" | "56" | "64" | "72" | "80");
    ///
    /// (* Reserved until 0.7.1 *)
    /// YUL_UFIXED_KEYWORD = "ufixed" ("184x8" | "184x16" | "184x24" | "184x32" | "184x40" | "184x48" | "184x56" | "184x64" | "184x72" | "192x8" | "192x16" | "192x24" | "192x32" | "192x40" | "192x48" | "192x56" | "192x64" | "200x8" | "200x16" | "200x24" | "200x32" | "200x40" | "200x48" | "200x56" | "208x8" | "208x16" | "208x24" | "208x32" | "208x40" | "208x48" | "216x8" | "216x16" | "216x24" | "216x32" | "216x40" | "224x8" | "224x16" | "224x24" | "224x32" | "232x8" | "232x16" | "232x24" | "240x8" | "240x16" | "248x8");
    ///
    /// (* Reserved from 0.4.14 until 0.7.1 *)
    /// YUL_UFIXED_KEYWORD = "ufixed" ("184x80" | "192x72" | "192x80" | "200x64" | "200x72" | "200x80" | "208x56" | "208x64" | "208x72" | "208x80" | "216x48" | "216x56" | "216x64" | "216x72" | "216x80" | "224x40" | "224x48" | "224x56" | "224x64" | "224x72" | "224x80" | "232x32" | "232x40" | "232x48" | "232x56" | "232x64" | "232x72" | "232x80" | "240x24" | "240x32" | "240x40" | "240x48" | "240x56" | "240x64" | "240x72" | "240x80" | "248x16" | "248x24" | "248x32" | "248x40" | "248x48" | "248x56" | "248x64" | "248x72" | "248x80" | "256x8" | "256x16" | "256x24" | "256x32" | "256x40" | "256x48" | "256x56" | "256x64" | "256x72" | "256x80");
    ///
    /// (* Reserved from 0.4.14 until 0.7.1 *)
    /// YUL_UFIXED_KEYWORD = "ufixed" ("8" | "16" | "24" | "32" | "40" | "48" | "56" | "64" | "72" | "80" | "88" | "96" | "104" | "112" | "120" | "128" | "136" | "144" | "152" | "160" | "168" | "176" | "184" | "192" | "200" | "208" | "216" | "224" | "232" | "240" | "248" | "256") "x" ("0" | "1" | "2" | "3" | "4" | "5" | "6" | "7" | "9" | "10" | "11" | "12" | "13" | "14" | "15" | "17" | "18" | "19" | "20" | "21" | "22" | "23" | "25" | "26" | "27" | "28" | "29" | "30" | "31" | "33" | "34" | "35" | "36" | "37" | "38" | "39" | "41" | "42" | "43" | "44" | "45" | "46" | "47" | "49" | "50" | "51" | "52" | "53" | "54" | "55" | "57" | "58" | "59" | "60" | "61" | "62" | "63" | "65" | "66" | "67" | "68" | "69" | "70" | "71" | "73" | "74" | "75" | "76" | "77" | "78" | "79");
    /// ```
    YulUfixedKeyword,
    /// ```ebnf
    /// (* Reserved until 0.7.1 *)
    /// YUL_UINT_KEYWORD = "uint" ("8" | "16" | "24" | "32" | "40" | "48" | "56" | "64" | "72" | "80" | "88" | "96" | "104" | "112" | "120" | "128" | "136" | "144" | "152" | "160" | "168" | "176" | "184" | "192" | "200" | "208" | "216" | "224" | "232" | "240" | "248" | "256")?;
    /// ```
    YulUintKeyword,
    /// ```ebnf
    /// (* Reserved from 0.5.0 until 0.7.1 *)
    /// YUL_UNCHECKED_KEYWORD = "unchecked";
    /// ```
    YulUncheckedKeyword,
    /// ```ebnf
    /// (* Reserved until 0.7.1 *)
    /// YUL_USING_KEYWORD = "using";
    /// ```
    YulUsingKeyword,
    /// ```ebnf
    /// (* Reserved until 0.6.5 *)
    /// YUL_VAR_KEYWORD = "var";
    /// ```
    YulVarKeyword,
    /// ```ebnf
    /// (* Reserved until 0.7.1 *)
    /// YUL_VIEW_KEYWORD = "view";
    /// ```
    YulViewKeyword,
    /// ```ebnf
    /// (* Reserved from 0.6.0 until 0.7.1 *)
    /// YUL_VIRTUAL_KEYWORD = "virtual";
    /// ```
    YulVirtualKeyword,
    /// ```ebnf
    /// (* Reserved until 0.7.1 *)
    /// YUL_WEEKS_KEYWORD = "weeks";
    /// ```
    YulWeeksKeyword,
    /// ```ebnf
    /// (* Reserved until 0.7.1 *)
    /// YUL_WEI_KEYWORD = "wei";
    /// ```
    YulWeiKeyword,
    /// ```ebnf
    /// (* Reserved until 0.7.1 *)
    /// YUL_WHILE_KEYWORD = "while";
    /// ```
    YulWhileKeyword,
    /// ```ebnf
    /// YUL_XOR_KEYWORD = "xor";
    /// ```
    YulXorKeyword,
    /// ```ebnf
    /// (* Reserved until 0.7.1 *)
    /// YUL_YEARS_KEYWORD = "years";
    /// ```
    YulYearsKeyword,
}

impl crate::cst::TerminalKindExtensions for TerminalKind {
    fn is_trivia(&self) -> bool {
        matches!(self, |Self::EndOfLine| Self::MultiLineComment
            | Self::MultiLineNatSpecComment
            | Self::SingleLineComment
            | Self::SingleLineNatSpecComment
            | Self::Whitespace)
    }

    fn is_valid(&self) -> bool {
        !matches!(self, Self::UNRECOGNIZED | Self::MISSING)
    }
}

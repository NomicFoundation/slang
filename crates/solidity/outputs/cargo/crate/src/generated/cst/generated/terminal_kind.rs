// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

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
    /// (* Never reserved *)
    /// ABICODER_KEYWORD = "abicoder";
    AbicoderKeyword,
    /// (* Introduced in 0.6.0 *)
    /// ABSTRACT_KEYWORD = "abstract";
    AbstractKeyword,
    /// (* Never reserved *)
    /// ADDRESS_KEYWORD = "address";
    AddressKeyword,
    /// AFTER_KEYWORD = "after";
    AfterKeyword,
    /// (* Reserved in 0.5.0 *)
    /// ALIAS_KEYWORD = "alias";
    AliasKeyword,
    /// AMPERSAND = "&";
    Ampersand,
    /// AMPERSAND_AMPERSAND = "&&";
    AmpersandAmpersand,
    /// AMPERSAND_EQUAL = "&=";
    AmpersandEqual,
    /// ANONYMOUS_KEYWORD = "anonymous";
    AnonymousKeyword,
    /// (* Reserved in 0.5.0 *)
    /// APPLY_KEYWORD = "apply";
    ApplyKeyword,
    /// AS_KEYWORD = "as";
    AsKeyword,
    /// ASSEMBLY_KEYWORD = "assembly";
    AssemblyKeyword,
    /// ASTERISK = "*";
    Asterisk,
    /// ASTERISK_ASTERISK = "**";
    AsteriskAsterisk,
    /// ASTERISK_EQUAL = "*=";
    AsteriskEqual,
    /// (* Reserved in 0.5.0 *)
    /// AUTO_KEYWORD = "auto";
    AutoKeyword,
    /// BANG = "!";
    Bang,
    /// BANG_EQUAL = "!=";
    BangEqual,
    /// BAR = "|";
    Bar,
    /// BAR_BAR = "||";
    BarBar,
    /// BAR_EQUAL = "|=";
    BarEqual,
    /// BOOL_KEYWORD = "bool";
    BoolKeyword,
    /// BREAK_KEYWORD = "break";
    BreakKeyword,
    /// (* Deprecated in 0.8.0 *)
    /// BYTE_KEYWORD = "byte";
    ByteKeyword,
    /// BYTES_KEYWORD = "bytes" ("1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9" | "10" | "11" | "12" | "13" | "14" | "15" | "16" | "17" | "18" | "19" | "20" | "21" | "22" | "23" | "24" | "25" | "26" | "27" | "28" | "29" | "30" | "31" | "32")?;
    BytesKeyword,
    /// (* Introduced in 0.5.0 *)
    /// (* Reserved in 0.5.0 *)
    /// CALL_DATA_KEYWORD = "calldata";
    CallDataKeyword,
    /// CARET = "^";
    Caret,
    /// CARET_EQUAL = "^=";
    CaretEqual,
    /// CASE_KEYWORD = "case";
    CaseKeyword,
    /// (* Introduced in 0.6.0 *)
    /// CATCH_KEYWORD = "catch";
    CatchKeyword,
    /// CLOSE_BRACE = "}";
    CloseBrace,
    /// CLOSE_BRACKET = "]";
    CloseBracket,
    /// CLOSE_PAREN = ")";
    CloseParen,
    /// COLON = ":";
    Colon,
    /// COLON_EQUAL = ":=";
    ColonEqual,
    /// COMMA = ",";
    Comma,
    /// CONSTANT_KEYWORD = "constant";
    ConstantKeyword,
    /// (* Introduced in 0.4.22 *)
    /// (* Reserved in 0.5.0 *)
    /// CONSTRUCTOR_KEYWORD = "constructor";
    ConstructorKeyword,
    /// CONTINUE_KEYWORD = "continue";
    ContinueKeyword,
    /// CONTRACT_KEYWORD = "contract";
    ContractKeyword,
    /// (* Reserved in 0.5.0 *)
    /// COPY_OF_KEYWORD = "copyof";
    CopyOfKeyword,
    /// DAYS_KEYWORD = "days";
    DaysKeyword,
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
    DecimalLiteral,
    /// DEFAULT_KEYWORD = "default";
    DefaultKeyword,
    /// (* Reserved in 0.5.0 *)
    /// DEFINE_KEYWORD = "define";
    DefineKeyword,
    /// DELETE_KEYWORD = "delete";
    DeleteKeyword,
    /// DO_KEYWORD = "do";
    DoKeyword,
    /// DOUBLE_QUOTED_HEX_STRING_LITERAL = 'hex"' «HEX_STRING_CONTENTS»? '"';
    DoubleQuotedHexStringLiteral,
    /// (* Deprecated in 0.4.25 *)
    /// DOUBLE_QUOTED_STRING_LITERAL = '"' («ESCAPE_SEQUENCE_ARBITRARY» | !('"' "\\" "\r" "\n"))* '"';
    ///
    /// (* Introduced in 0.4.25 and deprecated in 0.7.0. *)
    /// DOUBLE_QUOTED_STRING_LITERAL = '"' («ESCAPE_SEQUENCE» | !('"' "\\" "\r" "\n"))* '"';
    ///
    /// DOUBLE_QUOTED_STRING_LITERAL = '"' («ESCAPE_SEQUENCE» | (" "…"!") | ("#"…"[") | ("]"…"~"))* '"';
    DoubleQuotedStringLiteral,
    /// (* Introduced in 0.7.0 *)
    /// DOUBLE_QUOTED_UNICODE_STRING_LITERAL = 'unicode"' («ESCAPE_SEQUENCE» | !('"' "\\" "\r" "\n"))* '"';
    DoubleQuotedUnicodeStringLiteral,
    /// DOUBLE_QUOTED_VERSION_LITERAL = '"' «VERSION_SPECIFIER_FRAGMENT» ("." «VERSION_SPECIFIER_FRAGMENT»)* '"';
    DoubleQuotedVersionLiteral,
    /// ELSE_KEYWORD = "else";
    ElseKeyword,
    /// (* Introduced in 0.4.21 *)
    /// (* Reserved in 0.5.0 *)
    /// EMIT_KEYWORD = "emit";
    EmitKeyword,
    /// END_OF_LINE = "\n" | ("\r" "\n"?);
    EndOfLine,
    /// ENUM_KEYWORD = "enum";
    EnumKeyword,
    /// EQUAL = "=";
    Equal,
    /// (* Deprecated in 0.5.0 *)
    /// EQUAL_COLON = "=:";
    EqualColon,
    /// EQUAL_EQUAL = "==";
    EqualEqual,
    /// EQUAL_GREATER_THAN = "=>";
    EqualGreaterThan,
    /// (* Introduced in 0.8.4 *)
    /// (* Never reserved *)
    /// ERROR_KEYWORD = "error";
    ErrorKeyword,
    /// ETHER_KEYWORD = "ether";
    EtherKeyword,
    /// EVENT_KEYWORD = "event";
    EventKeyword,
    /// (* Never reserved *)
    /// EXPERIMENTAL_KEYWORD = "experimental";
    ExperimentalKeyword,
    /// EXTERNAL_KEYWORD = "external";
    ExternalKeyword,
    /// (* Reserved in 0.6.0 *)
    /// FALLBACK_KEYWORD = "fallback";
    FallbackKeyword,
    /// FALSE_KEYWORD = "false";
    FalseKeyword,
    /// FINAL_KEYWORD = "final";
    FinalKeyword,
    /// (* Deprecated in 0.7.0 *)
    /// (* Reserved until 0.7.0 *)
    /// FINNEY_KEYWORD = "finney";
    FinneyKeyword,
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
    FixedKeyword,
    /// FOR_KEYWORD = "for";
    ForKeyword,
    /// (* Never reserved *)
    /// FROM_KEYWORD = "from";
    FromKeyword,
    /// FUNCTION_KEYWORD = "function";
    FunctionKeyword,
    /// (* Introduced in 0.8.13 *)
    /// (* Never reserved *)
    /// GLOBAL_KEYWORD = "global";
    GlobalKeyword,
    /// GREATER_THAN = ">";
    GreaterThan,
    /// GREATER_THAN_EQUAL = ">=";
    GreaterThanEqual,
    /// GREATER_THAN_GREATER_THAN = ">>";
    GreaterThanGreaterThan,
    /// GREATER_THAN_GREATER_THAN_EQUAL = ">>=";
    GreaterThanGreaterThanEqual,
    /// GREATER_THAN_GREATER_THAN_GREATER_THAN = ">>>";
    GreaterThanGreaterThanGreaterThan,
    /// GREATER_THAN_GREATER_THAN_GREATER_THAN_EQUAL = ">>>=";
    GreaterThanGreaterThanGreaterThanEqual,
    /// (* Introduced in 0.6.11 *)
    /// (* Reserved in 0.7.0 *)
    /// GWEI_KEYWORD = "gwei";
    GweiKeyword,
    /// HEX_KEYWORD = "hex";
    HexKeyword,
    /// HEX_LITERAL = "0x" «HEX_CHARACTER»+ ("_" «HEX_CHARACTER»+)*;
    ///
    /// (* Deprecated in 0.5.0 *)
    /// HEX_LITERAL = "0X" «HEX_CHARACTER»+ ("_" «HEX_CHARACTER»+)*;
    HexLiteral,
    /// HOURS_KEYWORD = "hours";
    HoursKeyword,
    /// IDENTIFIER = «IDENTIFIER_START» «IDENTIFIER_PART»*;
    Identifier,
    /// IF_KEYWORD = "if";
    IfKeyword,
    /// (* Introduced in 0.6.5 *)
    /// (* Reserved in 0.5.0 *)
    /// IMMUTABLE_KEYWORD = "immutable";
    ImmutableKeyword,
    /// (* Reserved in 0.5.0 *)
    /// IMPLEMENTS_KEYWORD = "implements";
    ImplementsKeyword,
    /// IMPORT_KEYWORD = "import";
    ImportKeyword,
    /// IN_KEYWORD = "in";
    InKeyword,
    /// INDEXED_KEYWORD = "indexed";
    IndexedKeyword,
    /// INLINE_KEYWORD = "inline";
    InlineKeyword,
    /// INT_KEYWORD = "int" ("8" | "16" | "24" | "32" | "40" | "48" | "56" | "64" | "72" | "80" | "88" | "96" | "104" | "112" | "120" | "128" | "136" | "144" | "152" | "160" | "168" | "176" | "184" | "192" | "200" | "208" | "216" | "224" | "232" | "240" | "248" | "256")?;
    IntKeyword,
    /// INTERFACE_KEYWORD = "interface";
    InterfaceKeyword,
    /// INTERNAL_KEYWORD = "internal";
    InternalKeyword,
    /// IS_KEYWORD = "is";
    IsKeyword,
    /// LESS_THAN = "<";
    LessThan,
    /// LESS_THAN_EQUAL = "<=";
    LessThanEqual,
    /// LESS_THAN_LESS_THAN = "<<";
    LessThanLessThan,
    /// LESS_THAN_LESS_THAN_EQUAL = "<<=";
    LessThanLessThanEqual,
    /// LET_KEYWORD = "let";
    LetKeyword,
    /// LIBRARY_KEYWORD = "library";
    LibraryKeyword,
    /// (* Reserved in 0.5.0 *)
    /// MACRO_KEYWORD = "macro";
    MacroKeyword,
    /// MAPPING_KEYWORD = "mapping";
    MappingKeyword,
    /// MATCH_KEYWORD = "match";
    MatchKeyword,
    /// MEMORY_KEYWORD = "memory";
    MemoryKeyword,
    /// MINUS = "-";
    Minus,
    /// MINUS_EQUAL = "-=";
    MinusEqual,
    /// MINUS_GREATER_THAN = "->";
    MinusGreaterThan,
    /// MINUS_MINUS = "--";
    MinusMinus,
    /// MINUTES_KEYWORD = "minutes";
    MinutesKeyword,
    /// MODIFIER_KEYWORD = "modifier";
    ModifierKeyword,
    /// MULTI_LINE_COMMENT = "/*" (!"*" | "*")* "*/";
    MultiLineComment,
    /// MULTI_LINE_NAT_SPEC_COMMENT = "/**" (!"*" | "*")* "*/";
    MultiLineNatSpecComment,
    /// (* Reserved in 0.5.0 *)
    /// MUTABLE_KEYWORD = "mutable";
    MutableKeyword,
    /// NEW_KEYWORD = "new";
    NewKeyword,
    /// NULL_KEYWORD = "null";
    NullKeyword,
    /// OF_KEYWORD = "of";
    OfKeyword,
    /// OPEN_BRACE = "{";
    OpenBrace,
    /// OPEN_BRACKET = "[";
    OpenBracket,
    /// OPEN_PAREN = "(";
    OpenParen,
    /// (* Introduced in 0.6.0 *)
    /// (* Reserved in 0.5.0 *)
    /// OVERRIDE_KEYWORD = "override";
    OverrideKeyword,
    /// (* Reserved in 0.5.0 *)
    /// PARTIAL_KEYWORD = "partial";
    PartialKeyword,
    /// PAYABLE_KEYWORD = "payable";
    PayableKeyword,
    /// PERCENT = "%";
    Percent,
    /// PERCENT_EQUAL = "%=";
    PercentEqual,
    /// PERIOD = ".";
    Period,
    /// PLUS = "+";
    Plus,
    /// PLUS_EQUAL = "+=";
    PlusEqual,
    /// PLUS_PLUS = "++";
    PlusPlus,
    /// PRAGMA_KEYWORD = "pragma";
    PragmaKeyword,
    /// PRIVATE_KEYWORD = "private";
    PrivateKeyword,
    /// (* Reserved in 0.5.0 *)
    /// PROMISE_KEYWORD = "promise";
    PromiseKeyword,
    /// PUBLIC_KEYWORD = "public";
    PublicKeyword,
    /// (* Introduced in 0.4.16 *)
    /// PURE_KEYWORD = "pure";
    PureKeyword,
    /// QUESTION_MARK = "?";
    QuestionMark,
    /// (* Reserved in 0.6.0 *)
    /// RECEIVE_KEYWORD = "receive";
    ReceiveKeyword,
    /// (* Reserved in 0.5.0 *)
    /// REFERENCE_KEYWORD = "reference";
    ReferenceKeyword,
    /// RELOCATABLE_KEYWORD = "relocatable";
    RelocatableKeyword,
    /// RETURN_KEYWORD = "return";
    ReturnKeyword,
    /// RETURNS_KEYWORD = "returns";
    ReturnsKeyword,
    /// (* Introduced in 0.8.4 *)
    /// (* Never reserved *)
    /// REVERT_KEYWORD = "revert";
    RevertKeyword,
    /// (* Reserved in 0.5.0 *)
    /// SEALED_KEYWORD = "sealed";
    SealedKeyword,
    /// SECONDS_KEYWORD = "seconds";
    SecondsKeyword,
    /// SEMICOLON = ";";
    Semicolon,
    /// SINGLE_LINE_COMMENT = "//" (!("\r" "\n"))*;
    SingleLineComment,
    /// SINGLE_LINE_NAT_SPEC_COMMENT = "///" (!("\r" "\n"))*;
    SingleLineNatSpecComment,
    /// SINGLE_QUOTED_HEX_STRING_LITERAL = "hex'" «HEX_STRING_CONTENTS»? "'";
    SingleQuotedHexStringLiteral,
    /// (* Deprecated in 0.4.25 *)
    /// SINGLE_QUOTED_STRING_LITERAL = "'" («ESCAPE_SEQUENCE_ARBITRARY» | !("'" "\\" "\r" "\n"))* "'";
    ///
    /// (* Introduced in 0.4.25 and deprecated in 0.7.0. *)
    /// SINGLE_QUOTED_STRING_LITERAL = "'" («ESCAPE_SEQUENCE» | !("'" "\\" "\r" "\n"))* "'";
    ///
    /// SINGLE_QUOTED_STRING_LITERAL = "'" («ESCAPE_SEQUENCE» | (" "…"&") | ("("…"[") | ("]"…"~"))* "'";
    SingleQuotedStringLiteral,
    /// (* Introduced in 0.7.0 *)
    /// SINGLE_QUOTED_UNICODE_STRING_LITERAL = "unicode'" («ESCAPE_SEQUENCE» | !("'" "\\" "\r" "\n"))* "'";
    SingleQuotedUnicodeStringLiteral,
    /// SINGLE_QUOTED_VERSION_LITERAL = "'" «VERSION_SPECIFIER_FRAGMENT» ("." «VERSION_SPECIFIER_FRAGMENT»)* "'";
    SingleQuotedVersionLiteral,
    /// (* Reserved in 0.5.0 *)
    /// SIZE_OF_KEYWORD = "sizeof";
    SizeOfKeyword,
    /// SLASH = "/";
    Slash,
    /// SLASH_EQUAL = "/=";
    SlashEqual,
    /// (* Never reserved *)
    /// SOLIDITY_KEYWORD = "solidity";
    SolidityKeyword,
    /// STATIC_KEYWORD = "static";
    StaticKeyword,
    /// STORAGE_KEYWORD = "storage";
    StorageKeyword,
    /// STRING_KEYWORD = "string";
    StringKeyword,
    /// STRUCT_KEYWORD = "struct";
    StructKeyword,
    /// SUPER_KEYWORD = "super";
    SuperKeyword,
    /// (* Reserved in 0.5.0 *)
    /// SUPPORTS_KEYWORD = "supports";
    SupportsKeyword,
    /// SWITCH_KEYWORD = "switch";
    SwitchKeyword,
    /// (* Deprecated in 0.7.0 *)
    /// (* Reserved until 0.7.0 *)
    /// SZABO_KEYWORD = "szabo";
    SzaboKeyword,
    /// THIS_KEYWORD = "this";
    ThisKeyword,
    /// (* Deprecated in 0.5.0 *)
    /// THROW_KEYWORD = "throw";
    ThrowKeyword,
    /// TILDE = "~";
    Tilde,
    /// (* Introduced in 0.8.27 *)
    /// (* Never reserved *)
    /// TRANSIENT_KEYWORD = "transient";
    TransientKeyword,
    /// TRUE_KEYWORD = "true";
    TrueKeyword,
    /// (* Introduced in 0.6.0 *)
    /// TRY_KEYWORD = "try";
    TryKeyword,
    /// (* Reserved in 0.5.0 *)
    /// TYPE_DEF_KEYWORD = "typedef";
    TypeDefKeyword,
    /// (* Introduced in 0.5.3 *)
    /// TYPE_KEYWORD = "type";
    TypeKeyword,
    /// TYPE_OF_KEYWORD = "typeof";
    TypeOfKeyword,
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
    UfixedKeyword,
    /// UINT_KEYWORD = "uint" ("8" | "16" | "24" | "32" | "40" | "48" | "56" | "64" | "72" | "80" | "88" | "96" | "104" | "112" | "120" | "128" | "136" | "144" | "152" | "160" | "168" | "176" | "184" | "192" | "200" | "208" | "216" | "224" | "232" | "240" | "248" | "256")?;
    UintKeyword,
    /// (* Introduced in 0.8.0 *)
    /// (* Reserved in 0.5.0 *)
    /// UNCHECKED_KEYWORD = "unchecked";
    UncheckedKeyword,
    /// USING_KEYWORD = "using";
    UsingKeyword,
    /// (* Deprecated in 0.5.0 *)
    /// VAR_KEYWORD = "var";
    VarKeyword,
    /// VERSION_SPECIFIER = «VERSION_SPECIFIER_FRAGMENT»;
    VersionSpecifier,
    /// (* Introduced in 0.4.16 *)
    /// VIEW_KEYWORD = "view";
    ViewKeyword,
    /// (* Introduced in 0.6.0 *)
    /// (* Reserved in 0.6.0 *)
    /// VIRTUAL_KEYWORD = "virtual";
    VirtualKeyword,
    /// WEEKS_KEYWORD = "weeks";
    WeeksKeyword,
    /// WEI_KEYWORD = "wei";
    WeiKeyword,
    /// WHILE_KEYWORD = "while";
    WhileKeyword,
    /// WHITESPACE = (" " | "\t")+;
    Whitespace,
    /// (* Deprecated in 0.5.0 *)
    /// YEARS_KEYWORD = "years";
    YearsKeyword,
    /// (* Reserved until 0.7.1 *)
    /// YUL_ABSTRACT_KEYWORD = "abstract";
    YulAbstractKeyword,
    /// YUL_ADD_KEYWORD = "add";
    YulAddKeyword,
    /// YUL_ADD_MOD_KEYWORD = "addmod";
    YulAddModKeyword,
    /// (* Never reserved *)
    /// YUL_ADDRESS_KEYWORD = "address";
    YulAddressKeyword,
    /// (* Reserved until 0.7.1 *)
    /// YUL_AFTER_KEYWORD = "after";
    YulAfterKeyword,
    /// (* Reserved from 0.5.0 until 0.7.1 *)
    /// YUL_ALIAS_KEYWORD = "alias";
    YulAliasKeyword,
    /// YUL_AND_KEYWORD = "and";
    YulAndKeyword,
    /// (* Reserved until 0.7.1 *)
    /// YUL_ANONYMOUS_KEYWORD = "anonymous";
    YulAnonymousKeyword,
    /// (* Reserved from 0.5.0 until 0.7.1 *)
    /// YUL_APPLY_KEYWORD = "apply";
    YulApplyKeyword,
    /// (* Reserved until 0.7.1 *)
    /// YUL_AS_KEYWORD = "as";
    YulAsKeyword,
    /// (* Reserved until 0.7.1 *)
    /// YUL_ASSEMBLY_KEYWORD = "assembly";
    YulAssemblyKeyword,
    /// (* Reserved from 0.5.0 until 0.7.1 *)
    /// YUL_AUTO_KEYWORD = "auto";
    YulAutoKeyword,
    /// YUL_BALANCE_KEYWORD = "balance";
    YulBalanceKeyword,
    /// (* Introduced in 0.8.7 *)
    /// (* Reserved in 0.8.7 *)
    /// YUL_BASE_FEE_KEYWORD = "basefee";
    YulBaseFeeKeyword,
    /// (* Introduced in 0.8.24 *)
    /// (* Reserved in 0.8.25 *)
    /// YUL_BLOB_BASE_FEE_KEYWORD = "blobbasefee";
    YulBlobBaseFeeKeyword,
    /// (* Introduced in 0.8.24 *)
    /// (* Reserved in 0.8.25 *)
    /// YUL_BLOB_HASH_KEYWORD = "blobhash";
    YulBlobHashKeyword,
    /// YUL_BLOCK_HASH_KEYWORD = "blockhash";
    YulBlockHashKeyword,
    /// (* Reserved until 0.5.10 *)
    /// YUL_BOOL_KEYWORD = "bool";
    YulBoolKeyword,
    /// YUL_BREAK_KEYWORD = "break";
    YulBreakKeyword,
    /// YUL_BYTE_KEYWORD = "byte";
    YulByteKeyword,
    /// (* Reserved until 0.7.1 *)
    /// YUL_BYTES_KEYWORD = "bytes" ("1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9" | "10" | "11" | "12" | "13" | "14" | "15" | "16" | "17" | "18" | "19" | "20" | "21" | "22" | "23" | "24" | "25" | "26" | "27" | "28" | "29" | "30" | "31" | "32")?;
    YulBytesKeyword,
    /// YUL_CALL_CODE_KEYWORD = "callcode";
    YulCallCodeKeyword,
    /// YUL_CALL_DATA_COPY_KEYWORD = "calldatacopy";
    YulCallDataCopyKeyword,
    /// (* Reserved from 0.5.0 until 0.7.1 *)
    /// YUL_CALL_DATA_KEYWORD = "calldata";
    YulCallDataKeyword,
    /// YUL_CALL_DATA_LOAD_KEYWORD = "calldataload";
    YulCallDataLoadKeyword,
    /// YUL_CALL_DATA_SIZE_KEYWORD = "calldatasize";
    YulCallDataSizeKeyword,
    /// YUL_CALL_KEYWORD = "call";
    YulCallKeyword,
    /// YUL_CALL_VALUE_KEYWORD = "callvalue";
    YulCallValueKeyword,
    /// YUL_CALLER_KEYWORD = "caller";
    YulCallerKeyword,
    /// YUL_CASE_KEYWORD = "case";
    YulCaseKeyword,
    /// (* Reserved until 0.7.1 *)
    /// YUL_CATCH_KEYWORD = "catch";
    YulCatchKeyword,
    /// (* Reserved in 0.5.12 *)
    /// YUL_CHAIN_ID_KEYWORD = "chainid";
    YulChainIdKeyword,
    /// YUL_COIN_BASE_KEYWORD = "coinbase";
    YulCoinBaseKeyword,
    /// (* Reserved until 0.7.1 *)
    /// YUL_CONSTANT_KEYWORD = "constant";
    YulConstantKeyword,
    /// (* Reserved from 0.5.0 until 0.7.1 *)
    /// YUL_CONSTRUCTOR_KEYWORD = "constructor";
    YulConstructorKeyword,
    /// YUL_CONTINUE_KEYWORD = "continue";
    YulContinueKeyword,
    /// (* Reserved until 0.7.1 *)
    /// YUL_CONTRACT_KEYWORD = "contract";
    YulContractKeyword,
    /// (* Reserved from 0.5.0 until 0.7.1 *)
    /// YUL_COPY_OF_KEYWORD = "copyof";
    YulCopyOfKeyword,
    /// (* Introduced in 0.4.12 *)
    /// (* Reserved in 0.4.12 *)
    /// YUL_CREATE_2_KEYWORD = "create2";
    YulCreate2Keyword,
    /// YUL_CREATE_KEYWORD = "create";
    YulCreateKeyword,
    /// (* Reserved until 0.7.1 *)
    /// YUL_DAYS_KEYWORD = "days";
    YulDaysKeyword,
    /// YUL_DECIMAL_LITERAL = "0" | (("1"…"9") ("0"…"9")*);
    YulDecimalLiteral,
    /// YUL_DEFAULT_KEYWORD = "default";
    YulDefaultKeyword,
    /// (* Reserved from 0.5.0 until 0.7.1 *)
    /// YUL_DEFINE_KEYWORD = "define";
    YulDefineKeyword,
    /// YUL_DELEGATE_CALL_KEYWORD = "delegatecall";
    YulDelegateCallKeyword,
    /// (* Reserved until 0.7.1 *)
    /// YUL_DELETE_KEYWORD = "delete";
    YulDeleteKeyword,
    /// (* Deprecated in 0.8.18 *)
    /// YUL_DIFFICULTY_KEYWORD = "difficulty";
    YulDifficultyKeyword,
    /// YUL_DIV_KEYWORD = "div";
    YulDivKeyword,
    /// (* Reserved until 0.7.1 *)
    /// YUL_DO_KEYWORD = "do";
    YulDoKeyword,
    /// (* Reserved until 0.7.1 *)
    /// YUL_ELSE_KEYWORD = "else";
    YulElseKeyword,
    /// (* Reserved from 0.5.0 until 0.7.1 *)
    /// YUL_EMIT_KEYWORD = "emit";
    YulEmitKeyword,
    /// (* Reserved until 0.7.1 *)
    /// YUL_ENUM_KEYWORD = "enum";
    YulEnumKeyword,
    /// YUL_EQ_KEYWORD = "eq";
    YulEqKeyword,
    /// (* Reserved until 0.7.1 *)
    /// YUL_ETHER_KEYWORD = "ether";
    YulEtherKeyword,
    /// (* Reserved until 0.7.1 *)
    /// YUL_EVENT_KEYWORD = "event";
    YulEventKeyword,
    /// YUL_EXP_KEYWORD = "exp";
    YulExpKeyword,
    /// YUL_EXT_CODE_COPY_KEYWORD = "extcodecopy";
    YulExtCodeCopyKeyword,
    /// (* Introduced in 0.5.0 *)
    /// (* Reserved in 0.5.0 *)
    /// YUL_EXT_CODE_HASH_KEYWORD = "extcodehash";
    YulExtCodeHashKeyword,
    /// YUL_EXT_CODE_SIZE_KEYWORD = "extcodesize";
    YulExtCodeSizeKeyword,
    /// (* Reserved until 0.7.1 *)
    /// YUL_EXTERNAL_KEYWORD = "external";
    YulExternalKeyword,
    /// (* Reserved from 0.6.0 until 0.7.1 *)
    /// YUL_FALLBACK_KEYWORD = "fallback";
    YulFallbackKeyword,
    /// YUL_FALSE_KEYWORD = "false";
    YulFalseKeyword,
    /// (* Reserved until 0.7.1 *)
    /// YUL_FINAL_KEYWORD = "final";
    YulFinalKeyword,
    /// (* Reserved until 0.7.0 *)
    /// YUL_FINNEY_KEYWORD = "finney";
    YulFinneyKeyword,
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
    YulFixedKeyword,
    /// YUL_FOR_KEYWORD = "for";
    YulForKeyword,
    /// YUL_FUNCTION_KEYWORD = "function";
    YulFunctionKeyword,
    /// YUL_GAS_KEYWORD = "gas";
    YulGasKeyword,
    /// YUL_GAS_LIMIT_KEYWORD = "gaslimit";
    YulGasLimitKeyword,
    /// YUL_GAS_PRICE_KEYWORD = "gasprice";
    YulGasPriceKeyword,
    /// YUL_GT_KEYWORD = "gt";
    YulGtKeyword,
    /// (* Reserved from 0.7.0 until 0.7.1 *)
    /// YUL_GWEI_KEYWORD = "gwei";
    YulGweiKeyword,
    /// YUL_HEX_KEYWORD = "hex";
    YulHexKeyword,
    /// YUL_HEX_LITERAL = "0x" «HEX_CHARACTER»+;
    YulHexLiteral,
    /// (* Reserved until 0.7.1 *)
    /// YUL_HOURS_KEYWORD = "hours";
    YulHoursKeyword,
    /// (* Introduced in 0.5.8 and deprecated in 0.7.0. *)
    /// YUL_IDENTIFIER = «IDENTIFIER_START» («IDENTIFIER_PART» | ".")*;
    ///
    /// YUL_IDENTIFIER = «IDENTIFIER_START» «IDENTIFIER_PART»*;
    YulIdentifier,
    /// YUL_IF_KEYWORD = "if";
    YulIfKeyword,
    /// (* Reserved from 0.5.0 until 0.7.1 *)
    /// YUL_IMMUTABLE_KEYWORD = "immutable";
    YulImmutableKeyword,
    /// (* Reserved from 0.5.0 until 0.7.1 *)
    /// YUL_IMPLEMENTS_KEYWORD = "implements";
    YulImplementsKeyword,
    /// (* Reserved until 0.7.1 *)
    /// YUL_IMPORT_KEYWORD = "import";
    YulImportKeyword,
    /// (* Reserved until 0.6.8 *)
    /// YUL_IN_KEYWORD = "in";
    YulInKeyword,
    /// (* Reserved until 0.7.1 *)
    /// YUL_INDEXED_KEYWORD = "indexed";
    YulIndexedKeyword,
    /// (* Reserved until 0.7.1 *)
    /// YUL_INLINE_KEYWORD = "inline";
    YulInlineKeyword,
    /// (* Reserved until 0.7.1 *)
    /// YUL_INT_KEYWORD = "int" ("8" | "16" | "24" | "32" | "40" | "48" | "56" | "64" | "72" | "80" | "88" | "96" | "104" | "112" | "120" | "128" | "136" | "144" | "152" | "160" | "168" | "176" | "184" | "192" | "200" | "208" | "216" | "224" | "232" | "240" | "248" | "256")?;
    YulIntKeyword,
    /// (* Reserved until 0.7.1 *)
    /// YUL_INTERFACE_KEYWORD = "interface";
    YulInterfaceKeyword,
    /// (* Reserved until 0.7.1 *)
    /// YUL_INTERNAL_KEYWORD = "internal";
    YulInternalKeyword,
    /// YUL_INVALID_KEYWORD = "invalid";
    YulInvalidKeyword,
    /// (* Reserved until 0.7.1 *)
    /// YUL_IS_KEYWORD = "is";
    YulIsKeyword,
    /// YUL_IS_ZERO_KEYWORD = "iszero";
    YulIsZeroKeyword,
    /// (* Deprecated in 0.5.0 *)
    /// YUL_JUMP_KEYWORD = "jump";
    YulJumpKeyword,
    /// (* Deprecated in 0.5.0 *)
    /// YUL_JUMPI_KEYWORD = "jumpi";
    YulJumpiKeyword,
    /// (* Introduced in 0.4.12 *)
    /// (* Reserved in 0.4.12 *)
    /// YUL_KECCAK_256_KEYWORD = "keccak256";
    YulKeccak256Keyword,
    /// (* Introduced in 0.6.0 *)
    /// (* Reserved in 0.7.1 *)
    /// YUL_LEAVE_KEYWORD = "leave";
    YulLeaveKeyword,
    /// YUL_LET_KEYWORD = "let";
    YulLetKeyword,
    /// (* Reserved until 0.7.1 *)
    /// YUL_LIBRARY_KEYWORD = "library";
    YulLibraryKeyword,
    /// YUL_LOG_0_KEYWORD = "log0";
    YulLog0Keyword,
    /// YUL_LOG_1_KEYWORD = "log1";
    YulLog1Keyword,
    /// YUL_LOG_2_KEYWORD = "log2";
    YulLog2Keyword,
    /// YUL_LOG_3_KEYWORD = "log3";
    YulLog3Keyword,
    /// YUL_LOG_4_KEYWORD = "log4";
    YulLog4Keyword,
    /// YUL_LT_KEYWORD = "lt";
    YulLtKeyword,
    /// (* Introduced in 0.8.24 *)
    /// (* Reserved in 0.8.25 *)
    /// YUL_M_COPY_KEYWORD = "mcopy";
    YulMCopyKeyword,
    /// YUL_M_LOAD_KEYWORD = "mload";
    YulMLoadKeyword,
    /// YUL_M_SIZE_KEYWORD = "msize";
    YulMSizeKeyword,
    /// YUL_M_STORE_8_KEYWORD = "mstore8";
    YulMStore8Keyword,
    /// YUL_M_STORE_KEYWORD = "mstore";
    YulMStoreKeyword,
    /// (* Reserved from 0.5.0 until 0.7.1 *)
    /// YUL_MACRO_KEYWORD = "macro";
    YulMacroKeyword,
    /// (* Reserved until 0.7.1 *)
    /// YUL_MAPPING_KEYWORD = "mapping";
    YulMappingKeyword,
    /// (* Reserved until 0.7.1 *)
    /// YUL_MATCH_KEYWORD = "match";
    YulMatchKeyword,
    /// (* Reserved until 0.7.1 *)
    /// YUL_MEMORY_KEYWORD = "memory";
    YulMemoryKeyword,
    /// (* Reserved until 0.7.1 *)
    /// YUL_MINUTES_KEYWORD = "minutes";
    YulMinutesKeyword,
    /// YUL_MOD_KEYWORD = "mod";
    YulModKeyword,
    /// (* Reserved until 0.7.1 *)
    /// YUL_MODIFIER_KEYWORD = "modifier";
    YulModifierKeyword,
    /// YUL_MUL_KEYWORD = "mul";
    YulMulKeyword,
    /// YUL_MUL_MOD_KEYWORD = "mulmod";
    YulMulModKeyword,
    /// (* Reserved from 0.5.0 until 0.7.1 *)
    /// YUL_MUTABLE_KEYWORD = "mutable";
    YulMutableKeyword,
    /// (* Reserved until 0.7.1 *)
    /// YUL_NEW_KEYWORD = "new";
    YulNewKeyword,
    /// YUL_NOT_KEYWORD = "not";
    YulNotKeyword,
    /// (* Reserved until 0.7.1 *)
    /// YUL_NULL_KEYWORD = "null";
    YulNullKeyword,
    /// YUL_NUMBER_KEYWORD = "number";
    YulNumberKeyword,
    /// (* Reserved until 0.7.1 *)
    /// YUL_OF_KEYWORD = "of";
    YulOfKeyword,
    /// YUL_OR_KEYWORD = "or";
    YulOrKeyword,
    /// YUL_ORIGIN_KEYWORD = "origin";
    YulOriginKeyword,
    /// (* Reserved from 0.5.0 until 0.7.1 *)
    /// YUL_OVERRIDE_KEYWORD = "override";
    YulOverrideKeyword,
    /// (* Reserved from 0.5.0 until 0.7.1 *)
    /// YUL_PARTIAL_KEYWORD = "partial";
    YulPartialKeyword,
    /// (* Reserved until 0.7.1 *)
    /// YUL_PAYABLE_KEYWORD = "payable";
    YulPayableKeyword,
    /// YUL_POP_KEYWORD = "pop";
    YulPopKeyword,
    /// (* Reserved until 0.7.1 *)
    /// YUL_PRAGMA_KEYWORD = "pragma";
    YulPragmaKeyword,
    /// (* Introduced in 0.8.18 *)
    /// (* Reserved in 0.8.18 *)
    /// YUL_PREV_RANDAO_KEYWORD = "prevrandao";
    YulPrevRandaoKeyword,
    /// (* Reserved until 0.7.1 *)
    /// YUL_PRIVATE_KEYWORD = "private";
    YulPrivateKeyword,
    /// (* Reserved from 0.5.0 until 0.7.1 *)
    /// YUL_PROMISE_KEYWORD = "promise";
    YulPromiseKeyword,
    /// (* Reserved until 0.7.1 *)
    /// YUL_PUBLIC_KEYWORD = "public";
    YulPublicKeyword,
    /// (* Reserved until 0.7.1 *)
    /// YUL_PURE_KEYWORD = "pure";
    YulPureKeyword,
    /// (* Reserved from 0.6.0 until 0.7.1 *)
    /// YUL_RECEIVE_KEYWORD = "receive";
    YulReceiveKeyword,
    /// (* Reserved from 0.5.0 until 0.7.1 *)
    /// YUL_REFERENCE_KEYWORD = "reference";
    YulReferenceKeyword,
    /// (* Reserved until 0.7.1 *)
    /// YUL_RELOCATABLE_KEYWORD = "relocatable";
    YulRelocatableKeyword,
    /// (* Introduced in 0.4.12 *)
    /// (* Reserved in 0.4.12 *)
    /// YUL_RETURN_DATA_COPY_KEYWORD = "returndatacopy";
    YulReturnDataCopyKeyword,
    /// (* Introduced in 0.4.12 *)
    /// (* Reserved in 0.4.12 *)
    /// YUL_RETURN_DATA_SIZE_KEYWORD = "returndatasize";
    YulReturnDataSizeKeyword,
    /// YUL_RETURN_KEYWORD = "return";
    YulReturnKeyword,
    /// (* Reserved until 0.7.1 *)
    /// YUL_RETURNS_KEYWORD = "returns";
    YulReturnsKeyword,
    /// YUL_REVERT_KEYWORD = "revert";
    YulRevertKeyword,
    /// YUL_S_DIV_KEYWORD = "sdiv";
    YulSDivKeyword,
    /// YUL_S_LOAD_KEYWORD = "sload";
    YulSLoadKeyword,
    /// YUL_S_MOD_KEYWORD = "smod";
    YulSModKeyword,
    /// YUL_S_STORE_KEYWORD = "sstore";
    YulSStoreKeyword,
    /// (* Reserved in 0.4.21 *)
    /// YUL_SAR_KEYWORD = "sar";
    YulSarKeyword,
    /// (* Reserved from 0.5.0 until 0.7.1 *)
    /// YUL_SEALED_KEYWORD = "sealed";
    YulSealedKeyword,
    /// (* Reserved until 0.7.1 *)
    /// YUL_SECONDS_KEYWORD = "seconds";
    YulSecondsKeyword,
    /// (* Reserved in 0.5.12 *)
    /// YUL_SELF_BALANCE_KEYWORD = "selfbalance";
    YulSelfBalanceKeyword,
    /// YUL_SELF_DESTRUCT_KEYWORD = "selfdestruct";
    YulSelfDestructKeyword,
    /// YUL_SGT_KEYWORD = "sgt";
    YulSgtKeyword,
    /// (* Deprecated in 0.5.0 *)
    /// (* Reserved until 0.5.0 *)
    /// YUL_SHA_3_KEYWORD = "sha3";
    YulSha3Keyword,
    /// (* Reserved in 0.4.21 *)
    /// YUL_SHL_KEYWORD = "shl";
    YulShlKeyword,
    /// (* Reserved in 0.4.21 *)
    /// YUL_SHR_KEYWORD = "shr";
    YulShrKeyword,
    /// YUL_SIGN_EXTEND_KEYWORD = "signextend";
    YulSignExtendKeyword,
    /// (* Reserved from 0.5.0 until 0.7.1 *)
    /// YUL_SIZE_OF_KEYWORD = "sizeof";
    YulSizeOfKeyword,
    /// YUL_SLT_KEYWORD = "slt";
    YulSltKeyword,
    /// (* Introduced in 0.4.12 *)
    /// (* Reserved in 0.4.12 *)
    /// YUL_STATIC_CALL_KEYWORD = "staticcall";
    YulStaticCallKeyword,
    /// (* Reserved until 0.7.1 *)
    /// YUL_STATIC_KEYWORD = "static";
    YulStaticKeyword,
    /// YUL_STOP_KEYWORD = "stop";
    YulStopKeyword,
    /// (* Reserved until 0.7.1 *)
    /// YUL_STORAGE_KEYWORD = "storage";
    YulStorageKeyword,
    /// (* Reserved until 0.7.1 *)
    /// YUL_STRING_KEYWORD = "string";
    YulStringKeyword,
    /// (* Reserved until 0.7.1 *)
    /// YUL_STRUCT_KEYWORD = "struct";
    YulStructKeyword,
    /// YUL_SUB_KEYWORD = "sub";
    YulSubKeyword,
    /// (* Deprecated in 0.5.0 *)
    /// (* Reserved until 0.5.0 *)
    /// YUL_SUICIDE_KEYWORD = "suicide";
    YulSuicideKeyword,
    /// YUL_SUPER_KEYWORD = "super";
    YulSuperKeyword,
    /// (* Reserved from 0.5.0 until 0.7.1 *)
    /// YUL_SUPPORTS_KEYWORD = "supports";
    YulSupportsKeyword,
    /// YUL_SWITCH_KEYWORD = "switch";
    YulSwitchKeyword,
    /// (* Reserved until 0.7.0 *)
    /// YUL_SZABO_KEYWORD = "szabo";
    YulSzaboKeyword,
    /// (* Introduced in 0.8.24 *)
    /// (* Reserved in 0.8.25 *)
    /// YUL_T_LOAD_KEYWORD = "tload";
    YulTLoadKeyword,
    /// (* Introduced in 0.8.24 *)
    /// (* Reserved in 0.8.25 *)
    /// YUL_T_STORE_KEYWORD = "tstore";
    YulTStoreKeyword,
    /// YUL_THIS_KEYWORD = "this";
    YulThisKeyword,
    /// (* Reserved until 0.7.1 *)
    /// YUL_THROW_KEYWORD = "throw";
    YulThrowKeyword,
    /// YUL_TIMESTAMP_KEYWORD = "timestamp";
    YulTimestampKeyword,
    /// YUL_TRUE_KEYWORD = "true";
    YulTrueKeyword,
    /// (* Reserved until 0.7.1 *)
    /// YUL_TRY_KEYWORD = "try";
    YulTryKeyword,
    /// (* Reserved from 0.5.0 until 0.7.1 *)
    /// YUL_TYPE_DEF_KEYWORD = "typedef";
    YulTypeDefKeyword,
    /// (* Reserved until 0.7.1 *)
    /// YUL_TYPE_KEYWORD = "type";
    YulTypeKeyword,
    /// (* Reserved until 0.7.1 *)
    /// YUL_TYPE_OF_KEYWORD = "typeof";
    YulTypeOfKeyword,
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
    YulUfixedKeyword,
    /// (* Reserved until 0.7.1 *)
    /// YUL_UINT_KEYWORD = "uint" ("8" | "16" | "24" | "32" | "40" | "48" | "56" | "64" | "72" | "80" | "88" | "96" | "104" | "112" | "120" | "128" | "136" | "144" | "152" | "160" | "168" | "176" | "184" | "192" | "200" | "208" | "216" | "224" | "232" | "240" | "248" | "256")?;
    YulUintKeyword,
    /// (* Reserved from 0.5.0 until 0.7.1 *)
    /// YUL_UNCHECKED_KEYWORD = "unchecked";
    YulUncheckedKeyword,
    /// (* Reserved until 0.7.1 *)
    /// YUL_USING_KEYWORD = "using";
    YulUsingKeyword,
    /// (* Reserved until 0.6.5 *)
    /// YUL_VAR_KEYWORD = "var";
    YulVarKeyword,
    /// (* Reserved until 0.7.1 *)
    /// YUL_VIEW_KEYWORD = "view";
    YulViewKeyword,
    /// (* Reserved from 0.6.0 until 0.7.1 *)
    /// YUL_VIRTUAL_KEYWORD = "virtual";
    YulVirtualKeyword,
    /// (* Reserved until 0.7.1 *)
    /// YUL_WEEKS_KEYWORD = "weeks";
    YulWeeksKeyword,
    /// (* Reserved until 0.7.1 *)
    /// YUL_WEI_KEYWORD = "wei";
    YulWeiKeyword,
    /// (* Reserved until 0.7.1 *)
    /// YUL_WHILE_KEYWORD = "while";
    YulWhileKeyword,
    /// YUL_XOR_KEYWORD = "xor";
    YulXorKeyword,
    /// (* Reserved until 0.7.1 *)
    /// YUL_YEARS_KEYWORD = "years";
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

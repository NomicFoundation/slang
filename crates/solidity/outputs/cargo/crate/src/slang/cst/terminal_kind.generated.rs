// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

/// Represents different kinds of terminal nodes in the syntax tree.
/// These are leaf nodes that represent actual tokens in the source code.
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
#[allow(clippy::doc_markdown)]
#[allow(clippy::doc_link_with_quotes)]
#[repr(u16)]
pub enum TerminalKind {
    /// This terminal is created when the parser encounters an unexpected part of the input,
    /// and it cannot recognize it as any valid syntax in this position in the grammar.
    UNRECOGNIZED,
    /// This terminal is created when the parser is expecting a certain terminal but does not find it.
    /// Adding the missing input in this position may allow the parser to produce a valid tree there.
    MISSING,

    /// Represents a node with kind `ABIEncoderV2Keyword`, having the following structure:
    ///
    /// ```ebnf
    /// (* Introduced in 0.4.16 *)
    /// (* Never reserved *)
    /// ABI_ENCODER_V2_KEYWORD = "ABIEncoderV2";
    /// ```
    ABIEncoderV2Keyword,
    /// Represents a node with kind `AbicoderKeyword`, having the following structure:
    ///
    /// ```ebnf
    /// (* Introduced in 0.7.5 *)
    /// (* Never reserved *)
    /// ABICODER_KEYWORD = "abicoder";
    /// ```
    AbicoderKeyword,
    /// Represents a node with kind `AbicoderV1Keyword`, having the following structure:
    ///
    /// ```ebnf
    /// (* Introduced in 0.7.5 *)
    /// (* Never reserved *)
    /// ABICODER_V1_KEYWORD = "v1";
    /// ```
    AbicoderV1Keyword,
    /// Represents a node with kind `AbicoderV2Keyword`, having the following structure:
    ///
    /// ```ebnf
    /// (* Introduced in 0.7.5 *)
    /// (* Never reserved *)
    /// ABICODER_V2_KEYWORD = "v2";
    /// ```
    AbicoderV2Keyword,
    /// Represents a node with kind `AbstractKeyword`, having the following structure:
    ///
    /// ```ebnf
    /// (* Introduced in 0.6.0 *)
    /// ABSTRACT_KEYWORD = "abstract";
    /// ```
    AbstractKeyword,
    /// Represents a node with kind `AddressKeyword`, having the following structure:
    ///
    /// ```ebnf
    /// (* Never reserved *)
    /// ADDRESS_KEYWORD = "address";
    /// ```
    AddressKeyword,
    /// Represents a node with kind `AfterKeyword`, having the following structure:
    ///
    /// ```ebnf
    /// AFTER_KEYWORD = "after";
    /// ```
    AfterKeyword,
    /// Represents a node with kind `AliasKeyword`, having the following structure:
    ///
    /// ```ebnf
    /// (* Reserved in 0.5.0 *)
    /// ALIAS_KEYWORD = "alias";
    /// ```
    AliasKeyword,
    /// Represents a node with kind `Ampersand`, having the following structure:
    ///
    /// ```ebnf
    /// AMPERSAND = "&";
    /// ```
    Ampersand,
    /// Represents a node with kind `AmpersandAmpersand`, having the following structure:
    ///
    /// ```ebnf
    /// AMPERSAND_AMPERSAND = "&&";
    /// ```
    AmpersandAmpersand,
    /// Represents a node with kind `AmpersandEqual`, having the following structure:
    ///
    /// ```ebnf
    /// AMPERSAND_EQUAL = "&=";
    /// ```
    AmpersandEqual,
    /// Represents a node with kind `AnonymousKeyword`, having the following structure:
    ///
    /// ```ebnf
    /// ANONYMOUS_KEYWORD = "anonymous";
    /// ```
    AnonymousKeyword,
    /// Represents a node with kind `ApplyKeyword`, having the following structure:
    ///
    /// ```ebnf
    /// (* Reserved in 0.5.0 *)
    /// APPLY_KEYWORD = "apply";
    /// ```
    ApplyKeyword,
    /// Represents a node with kind `AsKeyword`, having the following structure:
    ///
    /// ```ebnf
    /// AS_KEYWORD = "as";
    /// ```
    AsKeyword,
    /// Represents a node with kind `AssemblyKeyword`, having the following structure:
    ///
    /// ```ebnf
    /// ASSEMBLY_KEYWORD = "assembly";
    /// ```
    AssemblyKeyword,
    /// Represents a node with kind `Asterisk`, having the following structure:
    ///
    /// ```ebnf
    /// ASTERISK = "*";
    /// ```
    Asterisk,
    /// Represents a node with kind `AsteriskAsterisk`, having the following structure:
    ///
    /// ```ebnf
    /// ASTERISK_ASTERISK = "**";
    /// ```
    AsteriskAsterisk,
    /// Represents a node with kind `AsteriskEqual`, having the following structure:
    ///
    /// ```ebnf
    /// ASTERISK_EQUAL = "*=";
    /// ```
    AsteriskEqual,
    /// Represents a node with kind `AtKeyword`, having the following structure:
    ///
    /// ```ebnf
    /// (* Introduced in 0.8.29 *)
    /// (* Never reserved *)
    /// AT_KEYWORD = "at";
    /// ```
    AtKeyword,
    /// Represents a node with kind `AutoKeyword`, having the following structure:
    ///
    /// ```ebnf
    /// (* Reserved in 0.5.0 *)
    /// AUTO_KEYWORD = "auto";
    /// ```
    AutoKeyword,
    /// Represents a node with kind `Bang`, having the following structure:
    ///
    /// ```ebnf
    /// BANG = "!";
    /// ```
    Bang,
    /// Represents a node with kind `BangEqual`, having the following structure:
    ///
    /// ```ebnf
    /// BANG_EQUAL = "!=";
    /// ```
    BangEqual,
    /// Represents a node with kind `Bar`, having the following structure:
    ///
    /// ```ebnf
    /// BAR = "|";
    /// ```
    Bar,
    /// Represents a node with kind `BarBar`, having the following structure:
    ///
    /// ```ebnf
    /// BAR_BAR = "||";
    /// ```
    BarBar,
    /// Represents a node with kind `BarEqual`, having the following structure:
    ///
    /// ```ebnf
    /// BAR_EQUAL = "|=";
    /// ```
    BarEqual,
    /// Represents a node with kind `BoolKeyword`, having the following structure:
    ///
    /// ```ebnf
    /// BOOL_KEYWORD = "bool";
    /// ```
    BoolKeyword,
    /// Represents a node with kind `BreakKeyword`, having the following structure:
    ///
    /// ```ebnf
    /// BREAK_KEYWORD = "break";
    /// ```
    BreakKeyword,
    /// Represents a node with kind `ByteKeyword`, having the following structure:
    ///
    /// ```ebnf
    /// (* Deprecated in 0.8.0 *)
    /// BYTE_KEYWORD = "byte";
    /// ```
    ByteKeyword,
    /// Represents a node with kind `BytesKeyword`, having the following structure:
    ///
    /// ```ebnf
    /// BYTES_KEYWORD = "bytes" ("1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9" | "10" | "11" | "12" | "13" | "14" | "15" | "16" | "17" | "18" | "19" | "20" | "21" | "22" | "23" | "24" | "25" | "26" | "27" | "28" | "29" | "30" | "31" | "32")?;
    /// ```
    BytesKeyword,
    /// Represents a node with kind `CallDataKeyword`, having the following structure:
    ///
    /// ```ebnf
    /// (* Introduced in 0.5.0 *)
    /// (* Reserved in 0.5.0 *)
    /// CALL_DATA_KEYWORD = "calldata";
    /// ```
    CallDataKeyword,
    /// Represents a node with kind `Caret`, having the following structure:
    ///
    /// ```ebnf
    /// CARET = "^";
    /// ```
    Caret,
    /// Represents a node with kind `CaretEqual`, having the following structure:
    ///
    /// ```ebnf
    /// CARET_EQUAL = "^=";
    /// ```
    CaretEqual,
    /// Represents a node with kind `CaseKeyword`, having the following structure:
    ///
    /// ```ebnf
    /// CASE_KEYWORD = "case";
    /// ```
    CaseKeyword,
    /// Represents a node with kind `CatchKeyword`, having the following structure:
    ///
    /// ```ebnf
    /// (* Introduced in 0.6.0 *)
    /// CATCH_KEYWORD = "catch";
    /// ```
    CatchKeyword,
    /// Represents a node with kind `CloseBrace`, having the following structure:
    ///
    /// ```ebnf
    /// CLOSE_BRACE = "}";
    /// ```
    CloseBrace,
    /// Represents a node with kind `CloseBracket`, having the following structure:
    ///
    /// ```ebnf
    /// CLOSE_BRACKET = "]";
    /// ```
    CloseBracket,
    /// Represents a node with kind `CloseParen`, having the following structure:
    ///
    /// ```ebnf
    /// CLOSE_PAREN = ")";
    /// ```
    CloseParen,
    /// Represents a node with kind `Colon`, having the following structure:
    ///
    /// ```ebnf
    /// COLON = ":";
    /// ```
    Colon,
    /// Represents a node with kind `ColonEqual`, having the following structure:
    ///
    /// ```ebnf
    /// COLON_EQUAL = ":=";
    /// ```
    ColonEqual,
    /// Represents a node with kind `Comma`, having the following structure:
    ///
    /// ```ebnf
    /// COMMA = ",";
    /// ```
    Comma,
    /// Represents a node with kind `ConstantKeyword`, having the following structure:
    ///
    /// ```ebnf
    /// CONSTANT_KEYWORD = "constant";
    /// ```
    ConstantKeyword,
    /// Represents a node with kind `ConstructorKeyword`, having the following structure:
    ///
    /// ```ebnf
    /// (* Introduced in 0.4.22 *)
    /// (* Reserved in 0.5.0 *)
    /// CONSTRUCTOR_KEYWORD = "constructor";
    /// ```
    ConstructorKeyword,
    /// Represents a node with kind `ContinueKeyword`, having the following structure:
    ///
    /// ```ebnf
    /// CONTINUE_KEYWORD = "continue";
    /// ```
    ContinueKeyword,
    /// Represents a node with kind `ContractKeyword`, having the following structure:
    ///
    /// ```ebnf
    /// CONTRACT_KEYWORD = "contract";
    /// ```
    ContractKeyword,
    /// Represents a node with kind `CopyOfKeyword`, having the following structure:
    ///
    /// ```ebnf
    /// (* Reserved in 0.5.0 *)
    /// COPY_OF_KEYWORD = "copyof";
    /// ```
    CopyOfKeyword,
    /// Represents a node with kind `DaysKeyword`, having the following structure:
    ///
    /// ```ebnf
    /// DAYS_KEYWORD = "days";
    /// ```
    DaysKeyword,
    /// Represents a node with kind `DecimalLiteral`, having the following structure:
    ///
    /// ```ebnf
    /// DECIMAL_LITERAL = "." «DECIMAL_DIGITS» «DECIMAL_EXPONENT»? (?!«IDENTIFIER_START»);
    ///
    /// DECIMAL_LITERAL = «DECIMAL_DIGITS» (?!".") «DECIMAL_EXPONENT»? (?!«IDENTIFIER_START»);
    ///
    /// (* Deprecated in 0.5.0 *)
    /// DECIMAL_LITERAL = «DECIMAL_DIGITS» "." (?!«DECIMAL_DIGITS») «DECIMAL_EXPONENT»? (?!«IDENTIFIER_START»);
    ///
    /// (* Deprecated in 0.5.0 *)
    /// DECIMAL_LITERAL = «DECIMAL_DIGITS» "." «DECIMAL_DIGITS» «DECIMAL_EXPONENT»? (?!«IDENTIFIER_START»);
    ///
    /// (* Introduced in 0.5.0 *)
    /// DECIMAL_LITERAL = «DECIMAL_DIGITS» ("." «DECIMAL_DIGITS»)? «DECIMAL_EXPONENT»? (?!«IDENTIFIER_START»);
    /// ```
    DecimalLiteral,
    /// Represents a node with kind `DefaultKeyword`, having the following structure:
    ///
    /// ```ebnf
    /// DEFAULT_KEYWORD = "default";
    /// ```
    DefaultKeyword,
    /// Represents a node with kind `DefineKeyword`, having the following structure:
    ///
    /// ```ebnf
    /// (* Reserved in 0.5.0 *)
    /// DEFINE_KEYWORD = "define";
    /// ```
    DefineKeyword,
    /// Represents a node with kind `DeleteKeyword`, having the following structure:
    ///
    /// ```ebnf
    /// DELETE_KEYWORD = "delete";
    /// ```
    DeleteKeyword,
    /// Represents a node with kind `DoKeyword`, having the following structure:
    ///
    /// ```ebnf
    /// DO_KEYWORD = "do";
    /// ```
    DoKeyword,
    /// Represents a node with kind `DoubleQuotedHexStringLiteral`, having the following structure:
    ///
    /// ```ebnf
    /// DOUBLE_QUOTED_HEX_STRING_LITERAL = 'hex"' «HEX_STRING_CONTENTS»? '"';
    /// ```
    DoubleQuotedHexStringLiteral,
    /// Represents a node with kind `DoubleQuotedStringLiteral`, having the following structure:
    ///
    /// ```ebnf
    /// (* Deprecated in 0.4.25 *)
    /// DOUBLE_QUOTED_STRING_LITERAL = '"' («ESCAPE_SEQUENCE_ARBITRARY» | !('"' | "\\" | "\r" | "\n"))* '"';
    ///
    /// (* Introduced in 0.4.25 and deprecated in 0.7.0. *)
    /// DOUBLE_QUOTED_STRING_LITERAL = '"' («ESCAPE_SEQUENCE» | !('"' | "\\" | "\r" | "\n"))* '"';
    ///
    /// DOUBLE_QUOTED_STRING_LITERAL = '"' («ESCAPE_SEQUENCE» | " "…"!" | "#"…"[" | "]"…"~")* '"';
    /// ```
    DoubleQuotedStringLiteral,
    /// Represents a node with kind `DoubleQuotedUnicodeStringLiteral`, having the following structure:
    ///
    /// ```ebnf
    /// (* Introduced in 0.7.0 *)
    /// DOUBLE_QUOTED_UNICODE_STRING_LITERAL = 'unicode"' («ESCAPE_SEQUENCE» | !('"' | "\\" | "\r" | "\n"))* '"';
    /// ```
    DoubleQuotedUnicodeStringLiteral,
    /// Represents a node with kind `DoubleQuotedVersionLiteral`, having the following structure:
    ///
    /// ```ebnf
    /// DOUBLE_QUOTED_VERSION_LITERAL = '"' «VERSION_SPECIFIER_FRAGMENT» ("." «VERSION_SPECIFIER_FRAGMENT»)* '"';
    /// ```
    DoubleQuotedVersionLiteral,
    /// Represents a node with kind `ElseKeyword`, having the following structure:
    ///
    /// ```ebnf
    /// ELSE_KEYWORD = "else";
    /// ```
    ElseKeyword,
    /// Represents a node with kind `EmitKeyword`, having the following structure:
    ///
    /// ```ebnf
    /// (* Introduced in 0.4.21 *)
    /// (* Reserved in 0.5.0 *)
    /// EMIT_KEYWORD = "emit";
    /// ```
    EmitKeyword,
    /// Represents a node with kind `EndOfLine`, having the following structure:
    ///
    /// ```ebnf
    /// END_OF_LINE = "\n" | ("\r" "\n"?);
    /// ```
    EndOfLine,
    /// Represents a node with kind `EnumKeyword`, having the following structure:
    ///
    /// ```ebnf
    /// ENUM_KEYWORD = "enum";
    /// ```
    EnumKeyword,
    /// Represents a node with kind `Equal`, having the following structure:
    ///
    /// ```ebnf
    /// EQUAL = "=";
    /// ```
    Equal,
    /// Represents a node with kind `EqualColon`, having the following structure:
    ///
    /// ```ebnf
    /// (* Deprecated in 0.5.0 *)
    /// EQUAL_COLON = "=:";
    /// ```
    EqualColon,
    /// Represents a node with kind `EqualEqual`, having the following structure:
    ///
    /// ```ebnf
    /// EQUAL_EQUAL = "==";
    /// ```
    EqualEqual,
    /// Represents a node with kind `EqualGreaterThan`, having the following structure:
    ///
    /// ```ebnf
    /// EQUAL_GREATER_THAN = "=>";
    /// ```
    EqualGreaterThan,
    /// Represents a node with kind `ErrorKeyword`, having the following structure:
    ///
    /// ```ebnf
    /// (* Introduced in 0.8.4 *)
    /// (* Never reserved *)
    /// ERROR_KEYWORD = "error";
    /// ```
    ErrorKeyword,
    /// Represents a node with kind `EtherKeyword`, having the following structure:
    ///
    /// ```ebnf
    /// ETHER_KEYWORD = "ether";
    /// ```
    EtherKeyword,
    /// Represents a node with kind `EventKeyword`, having the following structure:
    ///
    /// ```ebnf
    /// EVENT_KEYWORD = "event";
    /// ```
    EventKeyword,
    /// Represents a node with kind `ExperimentalKeyword`, having the following structure:
    ///
    /// ```ebnf
    /// (* Introduced in 0.4.16 *)
    /// (* Never reserved *)
    /// EXPERIMENTAL_KEYWORD = "experimental";
    /// ```
    ExperimentalKeyword,
    /// Represents a node with kind `ExternalKeyword`, having the following structure:
    ///
    /// ```ebnf
    /// EXTERNAL_KEYWORD = "external";
    /// ```
    ExternalKeyword,
    /// Represents a node with kind `FallbackKeyword`, having the following structure:
    ///
    /// ```ebnf
    /// (* Reserved in 0.6.0 *)
    /// FALLBACK_KEYWORD = "fallback";
    /// ```
    FallbackKeyword,
    /// Represents a node with kind `FalseKeyword`, having the following structure:
    ///
    /// ```ebnf
    /// FALSE_KEYWORD = "false";
    /// ```
    FalseKeyword,
    /// Represents a node with kind `FinalKeyword`, having the following structure:
    ///
    /// ```ebnf
    /// FINAL_KEYWORD = "final";
    /// ```
    FinalKeyword,
    /// Represents a node with kind `FinneyKeyword`, having the following structure:
    ///
    /// ```ebnf
    /// (* Deprecated in 0.7.0 *)
    /// (* Reserved until 0.7.0 *)
    /// FINNEY_KEYWORD = "finney";
    /// ```
    FinneyKeyword,
    /// Represents a node with kind `FixedKeyword`, having the following structure:
    ///
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
    /// Represents a node with kind `ForKeyword`, having the following structure:
    ///
    /// ```ebnf
    /// FOR_KEYWORD = "for";
    /// ```
    ForKeyword,
    /// Represents a node with kind `FromKeyword`, having the following structure:
    ///
    /// ```ebnf
    /// (* Never reserved *)
    /// FROM_KEYWORD = "from";
    /// ```
    FromKeyword,
    /// Represents a node with kind `FunctionKeyword`, having the following structure:
    ///
    /// ```ebnf
    /// FUNCTION_KEYWORD = "function";
    /// ```
    FunctionKeyword,
    /// Represents a node with kind `GlobalKeyword`, having the following structure:
    ///
    /// ```ebnf
    /// (* Introduced in 0.8.13 *)
    /// (* Never reserved *)
    /// GLOBAL_KEYWORD = "global";
    /// ```
    GlobalKeyword,
    /// Represents a node with kind `GreaterThan`, having the following structure:
    ///
    /// ```ebnf
    /// GREATER_THAN = ">";
    /// ```
    GreaterThan,
    /// Represents a node with kind `GreaterThanEqual`, having the following structure:
    ///
    /// ```ebnf
    /// GREATER_THAN_EQUAL = ">=";
    /// ```
    GreaterThanEqual,
    /// Represents a node with kind `GreaterThanGreaterThan`, having the following structure:
    ///
    /// ```ebnf
    /// GREATER_THAN_GREATER_THAN = ">>";
    /// ```
    GreaterThanGreaterThan,
    /// Represents a node with kind `GreaterThanGreaterThanEqual`, having the following structure:
    ///
    /// ```ebnf
    /// GREATER_THAN_GREATER_THAN_EQUAL = ">>=";
    /// ```
    GreaterThanGreaterThanEqual,
    /// Represents a node with kind `GreaterThanGreaterThanGreaterThan`, having the following structure:
    ///
    /// ```ebnf
    /// GREATER_THAN_GREATER_THAN_GREATER_THAN = ">>>";
    /// ```
    GreaterThanGreaterThanGreaterThan,
    /// Represents a node with kind `GreaterThanGreaterThanGreaterThanEqual`, having the following structure:
    ///
    /// ```ebnf
    /// GREATER_THAN_GREATER_THAN_GREATER_THAN_EQUAL = ">>>=";
    /// ```
    GreaterThanGreaterThanGreaterThanEqual,
    /// Represents a node with kind `GweiKeyword`, having the following structure:
    ///
    /// ```ebnf
    /// (* Introduced in 0.6.11 *)
    /// (* Reserved in 0.7.0 *)
    /// GWEI_KEYWORD = "gwei";
    /// ```
    GweiKeyword,
    /// Represents a node with kind `HexKeyword`, having the following structure:
    ///
    /// ```ebnf
    /// HEX_KEYWORD = "hex";
    /// ```
    HexKeyword,
    /// Represents a node with kind `HexLiteral`, having the following structure:
    ///
    /// ```ebnf
    /// HEX_LITERAL = "0x" «HEX_CHARACTER»+ ("_" «HEX_CHARACTER»+)* (?!«IDENTIFIER_START»);
    ///
    /// (* Deprecated in 0.5.0 *)
    /// HEX_LITERAL = "0X" «HEX_CHARACTER»+ ("_" «HEX_CHARACTER»+)* (?!«IDENTIFIER_START»);
    /// ```
    HexLiteral,
    /// Represents a node with kind `HoursKeyword`, having the following structure:
    ///
    /// ```ebnf
    /// HOURS_KEYWORD = "hours";
    /// ```
    HoursKeyword,
    /// Represents a node with kind `Identifier`, having the following structure:
    ///
    /// ```ebnf
    /// IDENTIFIER = «IDENTIFIER_START» «IDENTIFIER_PART»*;
    /// ```
    Identifier,
    /// Represents a node with kind `IfKeyword`, having the following structure:
    ///
    /// ```ebnf
    /// IF_KEYWORD = "if";
    /// ```
    IfKeyword,
    /// Represents a node with kind `ImmutableKeyword`, having the following structure:
    ///
    /// ```ebnf
    /// (* Introduced in 0.6.5 *)
    /// (* Reserved in 0.5.0 *)
    /// IMMUTABLE_KEYWORD = "immutable";
    /// ```
    ImmutableKeyword,
    /// Represents a node with kind `ImplementsKeyword`, having the following structure:
    ///
    /// ```ebnf
    /// (* Reserved in 0.5.0 *)
    /// IMPLEMENTS_KEYWORD = "implements";
    /// ```
    ImplementsKeyword,
    /// Represents a node with kind `ImportKeyword`, having the following structure:
    ///
    /// ```ebnf
    /// IMPORT_KEYWORD = "import";
    /// ```
    ImportKeyword,
    /// Represents a node with kind `InKeyword`, having the following structure:
    ///
    /// ```ebnf
    /// IN_KEYWORD = "in";
    /// ```
    InKeyword,
    /// Represents a node with kind `IndexedKeyword`, having the following structure:
    ///
    /// ```ebnf
    /// INDEXED_KEYWORD = "indexed";
    /// ```
    IndexedKeyword,
    /// Represents a node with kind `InlineKeyword`, having the following structure:
    ///
    /// ```ebnf
    /// INLINE_KEYWORD = "inline";
    /// ```
    InlineKeyword,
    /// Represents a node with kind `IntKeyword`, having the following structure:
    ///
    /// ```ebnf
    /// INT_KEYWORD = "int" ("8" | "16" | "24" | "32" | "40" | "48" | "56" | "64" | "72" | "80" | "88" | "96" | "104" | "112" | "120" | "128" | "136" | "144" | "152" | "160" | "168" | "176" | "184" | "192" | "200" | "208" | "216" | "224" | "232" | "240" | "248" | "256")?;
    /// ```
    IntKeyword,
    /// Represents a node with kind `InterfaceKeyword`, having the following structure:
    ///
    /// ```ebnf
    /// INTERFACE_KEYWORD = "interface";
    /// ```
    InterfaceKeyword,
    /// Represents a node with kind `InternalKeyword`, having the following structure:
    ///
    /// ```ebnf
    /// INTERNAL_KEYWORD = "internal";
    /// ```
    InternalKeyword,
    /// Represents a node with kind `IsKeyword`, having the following structure:
    ///
    /// ```ebnf
    /// IS_KEYWORD = "is";
    /// ```
    IsKeyword,
    /// Represents a node with kind `LayoutKeyword`, having the following structure:
    ///
    /// ```ebnf
    /// (* Introduced in 0.8.29 *)
    /// (* Never reserved *)
    /// LAYOUT_KEYWORD = "layout";
    /// ```
    LayoutKeyword,
    /// Represents a node with kind `LessThan`, having the following structure:
    ///
    /// ```ebnf
    /// LESS_THAN = "<";
    /// ```
    LessThan,
    /// Represents a node with kind `LessThanEqual`, having the following structure:
    ///
    /// ```ebnf
    /// LESS_THAN_EQUAL = "<=";
    /// ```
    LessThanEqual,
    /// Represents a node with kind `LessThanLessThan`, having the following structure:
    ///
    /// ```ebnf
    /// LESS_THAN_LESS_THAN = "<<";
    /// ```
    LessThanLessThan,
    /// Represents a node with kind `LessThanLessThanEqual`, having the following structure:
    ///
    /// ```ebnf
    /// LESS_THAN_LESS_THAN_EQUAL = "<<=";
    /// ```
    LessThanLessThanEqual,
    /// Represents a node with kind `LetKeyword`, having the following structure:
    ///
    /// ```ebnf
    /// LET_KEYWORD = "let";
    /// ```
    LetKeyword,
    /// Represents a node with kind `LibraryKeyword`, having the following structure:
    ///
    /// ```ebnf
    /// LIBRARY_KEYWORD = "library";
    /// ```
    LibraryKeyword,
    /// Represents a node with kind `MacroKeyword`, having the following structure:
    ///
    /// ```ebnf
    /// (* Reserved in 0.5.0 *)
    /// MACRO_KEYWORD = "macro";
    /// ```
    MacroKeyword,
    /// Represents a node with kind `MappingKeyword`, having the following structure:
    ///
    /// ```ebnf
    /// MAPPING_KEYWORD = "mapping";
    /// ```
    MappingKeyword,
    /// Represents a node with kind `MatchKeyword`, having the following structure:
    ///
    /// ```ebnf
    /// MATCH_KEYWORD = "match";
    /// ```
    MatchKeyword,
    /// Represents a node with kind `MemoryKeyword`, having the following structure:
    ///
    /// ```ebnf
    /// MEMORY_KEYWORD = "memory";
    /// ```
    MemoryKeyword,
    /// Represents a node with kind `Minus`, having the following structure:
    ///
    /// ```ebnf
    /// MINUS = "-";
    /// ```
    Minus,
    /// Represents a node with kind `MinusEqual`, having the following structure:
    ///
    /// ```ebnf
    /// MINUS_EQUAL = "-=";
    /// ```
    MinusEqual,
    /// Represents a node with kind `MinusGreaterThan`, having the following structure:
    ///
    /// ```ebnf
    /// MINUS_GREATER_THAN = "->";
    /// ```
    MinusGreaterThan,
    /// Represents a node with kind `MinusMinus`, having the following structure:
    ///
    /// ```ebnf
    /// MINUS_MINUS = "--";
    /// ```
    MinusMinus,
    /// Represents a node with kind `MinutesKeyword`, having the following structure:
    ///
    /// ```ebnf
    /// MINUTES_KEYWORD = "minutes";
    /// ```
    MinutesKeyword,
    /// Represents a node with kind `ModifierKeyword`, having the following structure:
    ///
    /// ```ebnf
    /// MODIFIER_KEYWORD = "modifier";
    /// ```
    ModifierKeyword,
    /// Represents a node with kind `MultiLineComment`, having the following structure:
    ///
    /// ```ebnf
    /// MULTI_LINE_COMMENT = "/*" (?!"*" !"/") (!"*" | ("*" (?!"/")))* "*/";
    /// ```
    MultiLineComment,
    /// Represents a node with kind `MultiLineNatSpecComment`, having the following structure:
    ///
    /// ```ebnf
    /// MULTI_LINE_NAT_SPEC_COMMENT = "/**" (?!"/") (!"*" | ("*" (?!"/")))* "*/";
    /// ```
    MultiLineNatSpecComment,
    /// Represents a node with kind `MutableKeyword`, having the following structure:
    ///
    /// ```ebnf
    /// (* Reserved in 0.5.0 *)
    /// MUTABLE_KEYWORD = "mutable";
    /// ```
    MutableKeyword,
    /// Represents a node with kind `NewKeyword`, having the following structure:
    ///
    /// ```ebnf
    /// NEW_KEYWORD = "new";
    /// ```
    NewKeyword,
    /// Represents a node with kind `NullKeyword`, having the following structure:
    ///
    /// ```ebnf
    /// NULL_KEYWORD = "null";
    /// ```
    NullKeyword,
    /// Represents a node with kind `OfKeyword`, having the following structure:
    ///
    /// ```ebnf
    /// OF_KEYWORD = "of";
    /// ```
    OfKeyword,
    /// Represents a node with kind `OpenBrace`, having the following structure:
    ///
    /// ```ebnf
    /// OPEN_BRACE = "{";
    /// ```
    OpenBrace,
    /// Represents a node with kind `OpenBracket`, having the following structure:
    ///
    /// ```ebnf
    /// OPEN_BRACKET = "[";
    /// ```
    OpenBracket,
    /// Represents a node with kind `OpenParen`, having the following structure:
    ///
    /// ```ebnf
    /// OPEN_PAREN = "(";
    /// ```
    OpenParen,
    /// Represents a node with kind `OverrideKeyword`, having the following structure:
    ///
    /// ```ebnf
    /// (* Introduced in 0.6.0 *)
    /// (* Reserved in 0.5.0 *)
    /// OVERRIDE_KEYWORD = "override";
    /// ```
    OverrideKeyword,
    /// Represents a node with kind `PartialKeyword`, having the following structure:
    ///
    /// ```ebnf
    /// (* Reserved in 0.5.0 *)
    /// PARTIAL_KEYWORD = "partial";
    /// ```
    PartialKeyword,
    /// Represents a node with kind `PayableKeyword`, having the following structure:
    ///
    /// ```ebnf
    /// PAYABLE_KEYWORD = "payable";
    /// ```
    PayableKeyword,
    /// Represents a node with kind `Percent`, having the following structure:
    ///
    /// ```ebnf
    /// PERCENT = "%";
    /// ```
    Percent,
    /// Represents a node with kind `PercentEqual`, having the following structure:
    ///
    /// ```ebnf
    /// PERCENT_EQUAL = "%=";
    /// ```
    PercentEqual,
    /// Represents a node with kind `Period`, having the following structure:
    ///
    /// ```ebnf
    /// PERIOD = ".";
    /// ```
    Period,
    /// Represents a node with kind `Plus`, having the following structure:
    ///
    /// ```ebnf
    /// PLUS = "+";
    /// ```
    Plus,
    /// Represents a node with kind `PlusEqual`, having the following structure:
    ///
    /// ```ebnf
    /// PLUS_EQUAL = "+=";
    /// ```
    PlusEqual,
    /// Represents a node with kind `PlusPlus`, having the following structure:
    ///
    /// ```ebnf
    /// PLUS_PLUS = "++";
    /// ```
    PlusPlus,
    /// Represents a node with kind `PragmaKeyword`, having the following structure:
    ///
    /// ```ebnf
    /// PRAGMA_KEYWORD = "pragma";
    /// ```
    PragmaKeyword,
    /// Represents a node with kind `PrivateKeyword`, having the following structure:
    ///
    /// ```ebnf
    /// PRIVATE_KEYWORD = "private";
    /// ```
    PrivateKeyword,
    /// Represents a node with kind `PromiseKeyword`, having the following structure:
    ///
    /// ```ebnf
    /// (* Reserved in 0.5.0 *)
    /// PROMISE_KEYWORD = "promise";
    /// ```
    PromiseKeyword,
    /// Represents a node with kind `PublicKeyword`, having the following structure:
    ///
    /// ```ebnf
    /// PUBLIC_KEYWORD = "public";
    /// ```
    PublicKeyword,
    /// Represents a node with kind `PureKeyword`, having the following structure:
    ///
    /// ```ebnf
    /// (* Introduced in 0.4.16 *)
    /// PURE_KEYWORD = "pure";
    /// ```
    PureKeyword,
    /// Represents a node with kind `QuestionMark`, having the following structure:
    ///
    /// ```ebnf
    /// QUESTION_MARK = "?";
    /// ```
    QuestionMark,
    /// Represents a node with kind `ReceiveKeyword`, having the following structure:
    ///
    /// ```ebnf
    /// (* Reserved in 0.6.0 *)
    /// RECEIVE_KEYWORD = "receive";
    /// ```
    ReceiveKeyword,
    /// Represents a node with kind `ReferenceKeyword`, having the following structure:
    ///
    /// ```ebnf
    /// (* Reserved in 0.5.0 *)
    /// REFERENCE_KEYWORD = "reference";
    /// ```
    ReferenceKeyword,
    /// Represents a node with kind `RelocatableKeyword`, having the following structure:
    ///
    /// ```ebnf
    /// RELOCATABLE_KEYWORD = "relocatable";
    /// ```
    RelocatableKeyword,
    /// Represents a node with kind `ReturnKeyword`, having the following structure:
    ///
    /// ```ebnf
    /// RETURN_KEYWORD = "return";
    /// ```
    ReturnKeyword,
    /// Represents a node with kind `ReturnsKeyword`, having the following structure:
    ///
    /// ```ebnf
    /// RETURNS_KEYWORD = "returns";
    /// ```
    ReturnsKeyword,
    /// Represents a node with kind `RevertKeyword`, having the following structure:
    ///
    /// ```ebnf
    /// (* Introduced in 0.8.4 *)
    /// (* Never reserved *)
    /// REVERT_KEYWORD = "revert";
    /// ```
    RevertKeyword,
    /// Represents a node with kind `SMTCheckerKeyword`, having the following structure:
    ///
    /// ```ebnf
    /// (* Introduced in 0.4.16 *)
    /// (* Never reserved *)
    /// SMT_CHECKER_KEYWORD = "SMTChecker";
    /// ```
    SMTCheckerKeyword,
    /// Represents a node with kind `SealedKeyword`, having the following structure:
    ///
    /// ```ebnf
    /// (* Reserved in 0.5.0 *)
    /// SEALED_KEYWORD = "sealed";
    /// ```
    SealedKeyword,
    /// Represents a node with kind `SecondsKeyword`, having the following structure:
    ///
    /// ```ebnf
    /// SECONDS_KEYWORD = "seconds";
    /// ```
    SecondsKeyword,
    /// Represents a node with kind `Semicolon`, having the following structure:
    ///
    /// ```ebnf
    /// SEMICOLON = ";";
    /// ```
    Semicolon,
    /// Represents a node with kind `SingleLineComment`, having the following structure:
    ///
    /// ```ebnf
    /// SINGLE_LINE_COMMENT = "//" (?!"/") (!("\r" | "\n"))*;
    /// ```
    SingleLineComment,
    /// Represents a node with kind `SingleLineNatSpecComment`, having the following structure:
    ///
    /// ```ebnf
    /// SINGLE_LINE_NAT_SPEC_COMMENT = "///" (!("\r" | "\n"))*;
    /// ```
    SingleLineNatSpecComment,
    /// Represents a node with kind `SingleQuotedHexStringLiteral`, having the following structure:
    ///
    /// ```ebnf
    /// SINGLE_QUOTED_HEX_STRING_LITERAL = "hex'" «HEX_STRING_CONTENTS»? "'";
    /// ```
    SingleQuotedHexStringLiteral,
    /// Represents a node with kind `SingleQuotedStringLiteral`, having the following structure:
    ///
    /// ```ebnf
    /// (* Deprecated in 0.4.25 *)
    /// SINGLE_QUOTED_STRING_LITERAL = "'" («ESCAPE_SEQUENCE_ARBITRARY» | !("'" | "\\" | "\r" | "\n"))* "'";
    ///
    /// (* Introduced in 0.4.25 and deprecated in 0.7.0. *)
    /// SINGLE_QUOTED_STRING_LITERAL = "'" («ESCAPE_SEQUENCE» | !("'" | "\\" | "\r" | "\n"))* "'";
    ///
    /// SINGLE_QUOTED_STRING_LITERAL = "'" («ESCAPE_SEQUENCE» | " "…"&" | "("…"[" | "]"…"~")* "'";
    /// ```
    SingleQuotedStringLiteral,
    /// Represents a node with kind `SingleQuotedUnicodeStringLiteral`, having the following structure:
    ///
    /// ```ebnf
    /// (* Introduced in 0.7.0 *)
    /// SINGLE_QUOTED_UNICODE_STRING_LITERAL = "unicode'" («ESCAPE_SEQUENCE» | !("'" | "\\" | "\r" | "\n"))* "'";
    /// ```
    SingleQuotedUnicodeStringLiteral,
    /// Represents a node with kind `SingleQuotedVersionLiteral`, having the following structure:
    ///
    /// ```ebnf
    /// SINGLE_QUOTED_VERSION_LITERAL = "'" «VERSION_SPECIFIER_FRAGMENT» ("." «VERSION_SPECIFIER_FRAGMENT»)* "'";
    /// ```
    SingleQuotedVersionLiteral,
    /// Represents a node with kind `SizeOfKeyword`, having the following structure:
    ///
    /// ```ebnf
    /// (* Reserved in 0.5.0 *)
    /// SIZE_OF_KEYWORD = "sizeof";
    /// ```
    SizeOfKeyword,
    /// Represents a node with kind `Slash`, having the following structure:
    ///
    /// ```ebnf
    /// SLASH = "/" (?!"*" | "/" | "=");
    /// ```
    Slash,
    /// Represents a node with kind `SlashEqual`, having the following structure:
    ///
    /// ```ebnf
    /// SLASH_EQUAL = "/=";
    /// ```
    SlashEqual,
    /// Represents a node with kind `SolidityKeyword`, having the following structure:
    ///
    /// ```ebnf
    /// (* Never reserved *)
    /// SOLIDITY_KEYWORD = "solidity";
    /// ```
    SolidityKeyword,
    /// Represents a node with kind `StaticKeyword`, having the following structure:
    ///
    /// ```ebnf
    /// STATIC_KEYWORD = "static";
    /// ```
    StaticKeyword,
    /// Represents a node with kind `StorageKeyword`, having the following structure:
    ///
    /// ```ebnf
    /// STORAGE_KEYWORD = "storage";
    /// ```
    StorageKeyword,
    /// Represents a node with kind `StringKeyword`, having the following structure:
    ///
    /// ```ebnf
    /// STRING_KEYWORD = "string";
    /// ```
    StringKeyword,
    /// Represents a node with kind `StructKeyword`, having the following structure:
    ///
    /// ```ebnf
    /// STRUCT_KEYWORD = "struct";
    /// ```
    StructKeyword,
    /// Represents a node with kind `SuperKeyword`, having the following structure:
    ///
    /// ```ebnf
    /// (* Reserved in 0.8.0 *)
    /// SUPER_KEYWORD = "super";
    /// ```
    SuperKeyword,
    /// Represents a node with kind `SupportsKeyword`, having the following structure:
    ///
    /// ```ebnf
    /// (* Reserved in 0.5.0 *)
    /// SUPPORTS_KEYWORD = "supports";
    /// ```
    SupportsKeyword,
    /// Represents a node with kind `SwitchKeyword`, having the following structure:
    ///
    /// ```ebnf
    /// SWITCH_KEYWORD = "switch";
    /// ```
    SwitchKeyword,
    /// Represents a node with kind `SzaboKeyword`, having the following structure:
    ///
    /// ```ebnf
    /// (* Deprecated in 0.7.0 *)
    /// (* Reserved until 0.7.0 *)
    /// SZABO_KEYWORD = "szabo";
    /// ```
    SzaboKeyword,
    /// Represents a node with kind `ThisKeyword`, having the following structure:
    ///
    /// ```ebnf
    /// (* Reserved in 0.8.0 *)
    /// THIS_KEYWORD = "this";
    /// ```
    ThisKeyword,
    /// Represents a node with kind `ThrowKeyword`, having the following structure:
    ///
    /// ```ebnf
    /// (* Deprecated in 0.5.0 *)
    /// THROW_KEYWORD = "throw";
    /// ```
    ThrowKeyword,
    /// Represents a node with kind `Tilde`, having the following structure:
    ///
    /// ```ebnf
    /// TILDE = "~";
    /// ```
    Tilde,
    /// Represents a node with kind `TransientKeyword`, having the following structure:
    ///
    /// ```ebnf
    /// (* Introduced in 0.8.27 *)
    /// (* Never reserved *)
    /// TRANSIENT_KEYWORD = "transient";
    /// ```
    TransientKeyword,
    /// Represents a node with kind `TrueKeyword`, having the following structure:
    ///
    /// ```ebnf
    /// TRUE_KEYWORD = "true";
    /// ```
    TrueKeyword,
    /// Represents a node with kind `TryKeyword`, having the following structure:
    ///
    /// ```ebnf
    /// (* Introduced in 0.6.0 *)
    /// TRY_KEYWORD = "try";
    /// ```
    TryKeyword,
    /// Represents a node with kind `TypeDefKeyword`, having the following structure:
    ///
    /// ```ebnf
    /// (* Reserved in 0.5.0 *)
    /// TYPE_DEF_KEYWORD = "typedef";
    /// ```
    TypeDefKeyword,
    /// Represents a node with kind `TypeKeyword`, having the following structure:
    ///
    /// ```ebnf
    /// (* Introduced in 0.5.3 *)
    /// TYPE_KEYWORD = "type";
    /// ```
    TypeKeyword,
    /// Represents a node with kind `TypeOfKeyword`, having the following structure:
    ///
    /// ```ebnf
    /// TYPE_OF_KEYWORD = "typeof";
    /// ```
    TypeOfKeyword,
    /// Represents a node with kind `UfixedKeyword`, having the following structure:
    ///
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
    /// Represents a node with kind `UintKeyword`, having the following structure:
    ///
    /// ```ebnf
    /// UINT_KEYWORD = "uint" ("8" | "16" | "24" | "32" | "40" | "48" | "56" | "64" | "72" | "80" | "88" | "96" | "104" | "112" | "120" | "128" | "136" | "144" | "152" | "160" | "168" | "176" | "184" | "192" | "200" | "208" | "216" | "224" | "232" | "240" | "248" | "256")?;
    /// ```
    UintKeyword,
    /// Represents a node with kind `UncheckedKeyword`, having the following structure:
    ///
    /// ```ebnf
    /// (* Introduced in 0.8.0 *)
    /// (* Reserved in 0.5.0 *)
    /// UNCHECKED_KEYWORD = "unchecked";
    /// ```
    UncheckedKeyword,
    /// Represents a node with kind `UsingKeyword`, having the following structure:
    ///
    /// ```ebnf
    /// USING_KEYWORD = "using";
    /// ```
    UsingKeyword,
    /// Represents a node with kind `VarKeyword`, having the following structure:
    ///
    /// ```ebnf
    /// (* Deprecated in 0.5.0 *)
    /// VAR_KEYWORD = "var";
    /// ```
    VarKeyword,
    /// Represents a node with kind `VersionSpecifier`, having the following structure:
    ///
    /// ```ebnf
    /// VERSION_SPECIFIER = «VERSION_SPECIFIER_FRAGMENT»;
    /// ```
    VersionSpecifier,
    /// Represents a node with kind `ViewKeyword`, having the following structure:
    ///
    /// ```ebnf
    /// (* Introduced in 0.4.16 *)
    /// VIEW_KEYWORD = "view";
    /// ```
    ViewKeyword,
    /// Represents a node with kind `VirtualKeyword`, having the following structure:
    ///
    /// ```ebnf
    /// (* Introduced in 0.6.0 *)
    /// (* Reserved in 0.6.0 *)
    /// VIRTUAL_KEYWORD = "virtual";
    /// ```
    VirtualKeyword,
    /// Represents a node with kind `WeeksKeyword`, having the following structure:
    ///
    /// ```ebnf
    /// WEEKS_KEYWORD = "weeks";
    /// ```
    WeeksKeyword,
    /// Represents a node with kind `WeiKeyword`, having the following structure:
    ///
    /// ```ebnf
    /// WEI_KEYWORD = "wei";
    /// ```
    WeiKeyword,
    /// Represents a node with kind `WhileKeyword`, having the following structure:
    ///
    /// ```ebnf
    /// WHILE_KEYWORD = "while";
    /// ```
    WhileKeyword,
    /// Represents a node with kind `Whitespace`, having the following structure:
    ///
    /// ```ebnf
    /// WHITESPACE = (" " | "\t")+;
    /// ```
    Whitespace,
    /// Represents a node with kind `YearsKeyword`, having the following structure:
    ///
    /// ```ebnf
    /// (* Deprecated in 0.5.0 *)
    /// YEARS_KEYWORD = "years";
    /// ```
    YearsKeyword,
    /// Represents a node with kind `YulAbstractKeyword`, having the following structure:
    ///
    /// ```ebnf
    /// (* Reserved until 0.7.1 *)
    /// YUL_ABSTRACT_KEYWORD = "abstract";
    /// ```
    YulAbstractKeyword,
    /// Represents a node with kind `YulAfterKeyword`, having the following structure:
    ///
    /// ```ebnf
    /// (* Reserved until 0.7.1 *)
    /// YUL_AFTER_KEYWORD = "after";
    /// ```
    YulAfterKeyword,
    /// Represents a node with kind `YulAliasKeyword`, having the following structure:
    ///
    /// ```ebnf
    /// (* Reserved from 0.5.0 until 0.7.1 *)
    /// YUL_ALIAS_KEYWORD = "alias";
    /// ```
    YulAliasKeyword,
    /// Represents a node with kind `YulAnonymousKeyword`, having the following structure:
    ///
    /// ```ebnf
    /// (* Reserved until 0.7.1 *)
    /// YUL_ANONYMOUS_KEYWORD = "anonymous";
    /// ```
    YulAnonymousKeyword,
    /// Represents a node with kind `YulApplyKeyword`, having the following structure:
    ///
    /// ```ebnf
    /// (* Reserved from 0.5.0 until 0.7.1 *)
    /// YUL_APPLY_KEYWORD = "apply";
    /// ```
    YulApplyKeyword,
    /// Represents a node with kind `YulAsKeyword`, having the following structure:
    ///
    /// ```ebnf
    /// (* Reserved until 0.7.1 *)
    /// YUL_AS_KEYWORD = "as";
    /// ```
    YulAsKeyword,
    /// Represents a node with kind `YulAssemblyKeyword`, having the following structure:
    ///
    /// ```ebnf
    /// (* Reserved until 0.7.1 *)
    /// YUL_ASSEMBLY_KEYWORD = "assembly";
    /// ```
    YulAssemblyKeyword,
    /// Represents a node with kind `YulAutoKeyword`, having the following structure:
    ///
    /// ```ebnf
    /// (* Reserved from 0.5.0 until 0.7.1 *)
    /// YUL_AUTO_KEYWORD = "auto";
    /// ```
    YulAutoKeyword,
    /// Represents a node with kind `YulBoolKeyword`, having the following structure:
    ///
    /// ```ebnf
    /// (* Reserved until 0.5.10 *)
    /// YUL_BOOL_KEYWORD = "bool";
    /// ```
    YulBoolKeyword,
    /// Represents a node with kind `YulBreakKeyword`, having the following structure:
    ///
    /// ```ebnf
    /// YUL_BREAK_KEYWORD = "break";
    /// ```
    YulBreakKeyword,
    /// Represents a node with kind `YulBytesKeyword`, having the following structure:
    ///
    /// ```ebnf
    /// (* Reserved until 0.7.1 *)
    /// YUL_BYTES_KEYWORD = "bytes" ("1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9" | "10" | "11" | "12" | "13" | "14" | "15" | "16" | "17" | "18" | "19" | "20" | "21" | "22" | "23" | "24" | "25" | "26" | "27" | "28" | "29" | "30" | "31" | "32")?;
    /// ```
    YulBytesKeyword,
    /// Represents a node with kind `YulCallDataKeyword`, having the following structure:
    ///
    /// ```ebnf
    /// (* Reserved from 0.5.0 until 0.7.1 *)
    /// YUL_CALL_DATA_KEYWORD = "calldata";
    /// ```
    YulCallDataKeyword,
    /// Represents a node with kind `YulCaseKeyword`, having the following structure:
    ///
    /// ```ebnf
    /// YUL_CASE_KEYWORD = "case";
    /// ```
    YulCaseKeyword,
    /// Represents a node with kind `YulCatchKeyword`, having the following structure:
    ///
    /// ```ebnf
    /// (* Reserved until 0.7.1 *)
    /// YUL_CATCH_KEYWORD = "catch";
    /// ```
    YulCatchKeyword,
    /// Represents a node with kind `YulConstantKeyword`, having the following structure:
    ///
    /// ```ebnf
    /// (* Reserved until 0.7.1 *)
    /// YUL_CONSTANT_KEYWORD = "constant";
    /// ```
    YulConstantKeyword,
    /// Represents a node with kind `YulConstructorKeyword`, having the following structure:
    ///
    /// ```ebnf
    /// (* Reserved from 0.5.0 until 0.7.1 *)
    /// YUL_CONSTRUCTOR_KEYWORD = "constructor";
    /// ```
    YulConstructorKeyword,
    /// Represents a node with kind `YulContinueKeyword`, having the following structure:
    ///
    /// ```ebnf
    /// YUL_CONTINUE_KEYWORD = "continue";
    /// ```
    YulContinueKeyword,
    /// Represents a node with kind `YulContractKeyword`, having the following structure:
    ///
    /// ```ebnf
    /// (* Reserved until 0.7.1 *)
    /// YUL_CONTRACT_KEYWORD = "contract";
    /// ```
    YulContractKeyword,
    /// Represents a node with kind `YulCopyOfKeyword`, having the following structure:
    ///
    /// ```ebnf
    /// (* Reserved from 0.5.0 until 0.7.1 *)
    /// YUL_COPY_OF_KEYWORD = "copyof";
    /// ```
    YulCopyOfKeyword,
    /// Represents a node with kind `YulDaysKeyword`, having the following structure:
    ///
    /// ```ebnf
    /// (* Reserved until 0.7.1 *)
    /// YUL_DAYS_KEYWORD = "days";
    /// ```
    YulDaysKeyword,
    /// Represents a node with kind `YulDecimalLiteral`, having the following structure:
    ///
    /// ```ebnf
    /// YUL_DECIMAL_LITERAL = ("0" | ("1"…"9" "0"…"9"*)) (?!«IDENTIFIER_START»);
    /// ```
    YulDecimalLiteral,
    /// Represents a node with kind `YulDefaultKeyword`, having the following structure:
    ///
    /// ```ebnf
    /// YUL_DEFAULT_KEYWORD = "default";
    /// ```
    YulDefaultKeyword,
    /// Represents a node with kind `YulDefineKeyword`, having the following structure:
    ///
    /// ```ebnf
    /// (* Reserved from 0.5.0 until 0.7.1 *)
    /// YUL_DEFINE_KEYWORD = "define";
    /// ```
    YulDefineKeyword,
    /// Represents a node with kind `YulDeleteKeyword`, having the following structure:
    ///
    /// ```ebnf
    /// (* Reserved until 0.7.1 *)
    /// YUL_DELETE_KEYWORD = "delete";
    /// ```
    YulDeleteKeyword,
    /// Represents a node with kind `YulDoKeyword`, having the following structure:
    ///
    /// ```ebnf
    /// (* Reserved until 0.7.1 *)
    /// YUL_DO_KEYWORD = "do";
    /// ```
    YulDoKeyword,
    /// Represents a node with kind `YulElseKeyword`, having the following structure:
    ///
    /// ```ebnf
    /// (* Reserved until 0.7.1 *)
    /// YUL_ELSE_KEYWORD = "else";
    /// ```
    YulElseKeyword,
    /// Represents a node with kind `YulEmitKeyword`, having the following structure:
    ///
    /// ```ebnf
    /// (* Reserved from 0.5.0 until 0.7.1 *)
    /// YUL_EMIT_KEYWORD = "emit";
    /// ```
    YulEmitKeyword,
    /// Represents a node with kind `YulEnumKeyword`, having the following structure:
    ///
    /// ```ebnf
    /// (* Reserved until 0.7.1 *)
    /// YUL_ENUM_KEYWORD = "enum";
    /// ```
    YulEnumKeyword,
    /// Represents a node with kind `YulEtherKeyword`, having the following structure:
    ///
    /// ```ebnf
    /// (* Reserved until 0.7.1 *)
    /// YUL_ETHER_KEYWORD = "ether";
    /// ```
    YulEtherKeyword,
    /// Represents a node with kind `YulEventKeyword`, having the following structure:
    ///
    /// ```ebnf
    /// (* Reserved until 0.7.1 *)
    /// YUL_EVENT_KEYWORD = "event";
    /// ```
    YulEventKeyword,
    /// Represents a node with kind `YulExternalKeyword`, having the following structure:
    ///
    /// ```ebnf
    /// (* Reserved until 0.7.1 *)
    /// YUL_EXTERNAL_KEYWORD = "external";
    /// ```
    YulExternalKeyword,
    /// Represents a node with kind `YulFallbackKeyword`, having the following structure:
    ///
    /// ```ebnf
    /// (* Reserved from 0.6.0 until 0.7.1 *)
    /// YUL_FALLBACK_KEYWORD = "fallback";
    /// ```
    YulFallbackKeyword,
    /// Represents a node with kind `YulFalseKeyword`, having the following structure:
    ///
    /// ```ebnf
    /// (* Introduced in 0.6.2 *)
    /// YUL_FALSE_KEYWORD = "false";
    /// ```
    YulFalseKeyword,
    /// Represents a node with kind `YulFinalKeyword`, having the following structure:
    ///
    /// ```ebnf
    /// (* Reserved until 0.7.1 *)
    /// YUL_FINAL_KEYWORD = "final";
    /// ```
    YulFinalKeyword,
    /// Represents a node with kind `YulFinneyKeyword`, having the following structure:
    ///
    /// ```ebnf
    /// (* Reserved until 0.7.0 *)
    /// YUL_FINNEY_KEYWORD = "finney";
    /// ```
    YulFinneyKeyword,
    /// Represents a node with kind `YulFixedKeyword`, having the following structure:
    ///
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
    /// Represents a node with kind `YulForKeyword`, having the following structure:
    ///
    /// ```ebnf
    /// YUL_FOR_KEYWORD = "for";
    /// ```
    YulForKeyword,
    /// Represents a node with kind `YulFunctionKeyword`, having the following structure:
    ///
    /// ```ebnf
    /// YUL_FUNCTION_KEYWORD = "function";
    /// ```
    YulFunctionKeyword,
    /// Represents a node with kind `YulGweiKeyword`, having the following structure:
    ///
    /// ```ebnf
    /// (* Reserved from 0.7.0 until 0.7.1 *)
    /// YUL_GWEI_KEYWORD = "gwei";
    /// ```
    YulGweiKeyword,
    /// Represents a node with kind `YulHexKeyword`, having the following structure:
    ///
    /// ```ebnf
    /// YUL_HEX_KEYWORD = "hex";
    /// ```
    YulHexKeyword,
    /// Represents a node with kind `YulHexLiteral`, having the following structure:
    ///
    /// ```ebnf
    /// YUL_HEX_LITERAL = "0x" «HEX_CHARACTER»+ (?!«IDENTIFIER_START»);
    /// ```
    YulHexLiteral,
    /// Represents a node with kind `YulHoursKeyword`, having the following structure:
    ///
    /// ```ebnf
    /// (* Reserved until 0.7.1 *)
    /// YUL_HOURS_KEYWORD = "hours";
    /// ```
    YulHoursKeyword,
    /// Represents a node with kind `YulIdentifier`, having the following structure:
    ///
    /// ```ebnf
    /// (* Introduced in 0.5.8 and deprecated in 0.7.0. *)
    /// YUL_IDENTIFIER = «IDENTIFIER_START» («IDENTIFIER_PART» | ".")*;
    ///
    /// YUL_IDENTIFIER = «IDENTIFIER_START» «IDENTIFIER_PART»*;
    /// ```
    YulIdentifier,
    /// Represents a node with kind `YulIfKeyword`, having the following structure:
    ///
    /// ```ebnf
    /// YUL_IF_KEYWORD = "if";
    /// ```
    YulIfKeyword,
    /// Represents a node with kind `YulImmutableKeyword`, having the following structure:
    ///
    /// ```ebnf
    /// (* Reserved from 0.5.0 until 0.7.1 *)
    /// YUL_IMMUTABLE_KEYWORD = "immutable";
    /// ```
    YulImmutableKeyword,
    /// Represents a node with kind `YulImplementsKeyword`, having the following structure:
    ///
    /// ```ebnf
    /// (* Reserved from 0.5.0 until 0.7.1 *)
    /// YUL_IMPLEMENTS_KEYWORD = "implements";
    /// ```
    YulImplementsKeyword,
    /// Represents a node with kind `YulImportKeyword`, having the following structure:
    ///
    /// ```ebnf
    /// (* Reserved until 0.7.1 *)
    /// YUL_IMPORT_KEYWORD = "import";
    /// ```
    YulImportKeyword,
    /// Represents a node with kind `YulInKeyword`, having the following structure:
    ///
    /// ```ebnf
    /// (* Reserved until 0.6.8 *)
    /// YUL_IN_KEYWORD = "in";
    /// ```
    YulInKeyword,
    /// Represents a node with kind `YulIndexedKeyword`, having the following structure:
    ///
    /// ```ebnf
    /// (* Reserved until 0.7.1 *)
    /// YUL_INDEXED_KEYWORD = "indexed";
    /// ```
    YulIndexedKeyword,
    /// Represents a node with kind `YulInlineKeyword`, having the following structure:
    ///
    /// ```ebnf
    /// (* Reserved until 0.7.1 *)
    /// YUL_INLINE_KEYWORD = "inline";
    /// ```
    YulInlineKeyword,
    /// Represents a node with kind `YulIntKeyword`, having the following structure:
    ///
    /// ```ebnf
    /// (* Reserved until 0.7.1 *)
    /// YUL_INT_KEYWORD = "int" ("8" | "16" | "24" | "32" | "40" | "48" | "56" | "64" | "72" | "80" | "88" | "96" | "104" | "112" | "120" | "128" | "136" | "144" | "152" | "160" | "168" | "176" | "184" | "192" | "200" | "208" | "216" | "224" | "232" | "240" | "248" | "256")?;
    /// ```
    YulIntKeyword,
    /// Represents a node with kind `YulInterfaceKeyword`, having the following structure:
    ///
    /// ```ebnf
    /// (* Reserved until 0.7.1 *)
    /// YUL_INTERFACE_KEYWORD = "interface";
    /// ```
    YulInterfaceKeyword,
    /// Represents a node with kind `YulInternalKeyword`, having the following structure:
    ///
    /// ```ebnf
    /// (* Reserved until 0.7.1 *)
    /// YUL_INTERNAL_KEYWORD = "internal";
    /// ```
    YulInternalKeyword,
    /// Represents a node with kind `YulIsKeyword`, having the following structure:
    ///
    /// ```ebnf
    /// (* Reserved until 0.7.1 *)
    /// YUL_IS_KEYWORD = "is";
    /// ```
    YulIsKeyword,
    /// Represents a node with kind `YulLeaveKeyword`, having the following structure:
    ///
    /// ```ebnf
    /// (* Introduced in 0.6.0 *)
    /// (* Reserved in 0.7.1 *)
    /// YUL_LEAVE_KEYWORD = "leave";
    /// ```
    YulLeaveKeyword,
    /// Represents a node with kind `YulLetKeyword`, having the following structure:
    ///
    /// ```ebnf
    /// YUL_LET_KEYWORD = "let";
    /// ```
    YulLetKeyword,
    /// Represents a node with kind `YulLibraryKeyword`, having the following structure:
    ///
    /// ```ebnf
    /// (* Reserved until 0.7.1 *)
    /// YUL_LIBRARY_KEYWORD = "library";
    /// ```
    YulLibraryKeyword,
    /// Represents a node with kind `YulMacroKeyword`, having the following structure:
    ///
    /// ```ebnf
    /// (* Reserved from 0.5.0 until 0.7.1 *)
    /// YUL_MACRO_KEYWORD = "macro";
    /// ```
    YulMacroKeyword,
    /// Represents a node with kind `YulMappingKeyword`, having the following structure:
    ///
    /// ```ebnf
    /// (* Reserved until 0.7.1 *)
    /// YUL_MAPPING_KEYWORD = "mapping";
    /// ```
    YulMappingKeyword,
    /// Represents a node with kind `YulMatchKeyword`, having the following structure:
    ///
    /// ```ebnf
    /// (* Reserved until 0.7.1 *)
    /// YUL_MATCH_KEYWORD = "match";
    /// ```
    YulMatchKeyword,
    /// Represents a node with kind `YulMemoryKeyword`, having the following structure:
    ///
    /// ```ebnf
    /// (* Reserved until 0.7.1 *)
    /// YUL_MEMORY_KEYWORD = "memory";
    /// ```
    YulMemoryKeyword,
    /// Represents a node with kind `YulMinutesKeyword`, having the following structure:
    ///
    /// ```ebnf
    /// (* Reserved until 0.7.1 *)
    /// YUL_MINUTES_KEYWORD = "minutes";
    /// ```
    YulMinutesKeyword,
    /// Represents a node with kind `YulModifierKeyword`, having the following structure:
    ///
    /// ```ebnf
    /// (* Reserved until 0.7.1 *)
    /// YUL_MODIFIER_KEYWORD = "modifier";
    /// ```
    YulModifierKeyword,
    /// Represents a node with kind `YulMutableKeyword`, having the following structure:
    ///
    /// ```ebnf
    /// (* Reserved from 0.5.0 until 0.7.1 *)
    /// YUL_MUTABLE_KEYWORD = "mutable";
    /// ```
    YulMutableKeyword,
    /// Represents a node with kind `YulNewKeyword`, having the following structure:
    ///
    /// ```ebnf
    /// (* Reserved until 0.7.1 *)
    /// YUL_NEW_KEYWORD = "new";
    /// ```
    YulNewKeyword,
    /// Represents a node with kind `YulNullKeyword`, having the following structure:
    ///
    /// ```ebnf
    /// (* Reserved until 0.7.1 *)
    /// YUL_NULL_KEYWORD = "null";
    /// ```
    YulNullKeyword,
    /// Represents a node with kind `YulOfKeyword`, having the following structure:
    ///
    /// ```ebnf
    /// (* Reserved until 0.7.1 *)
    /// YUL_OF_KEYWORD = "of";
    /// ```
    YulOfKeyword,
    /// Represents a node with kind `YulOverrideKeyword`, having the following structure:
    ///
    /// ```ebnf
    /// (* Reserved from 0.5.0 until 0.7.1 *)
    /// YUL_OVERRIDE_KEYWORD = "override";
    /// ```
    YulOverrideKeyword,
    /// Represents a node with kind `YulPartialKeyword`, having the following structure:
    ///
    /// ```ebnf
    /// (* Reserved from 0.5.0 until 0.7.1 *)
    /// YUL_PARTIAL_KEYWORD = "partial";
    /// ```
    YulPartialKeyword,
    /// Represents a node with kind `YulPayableKeyword`, having the following structure:
    ///
    /// ```ebnf
    /// (* Reserved until 0.7.1 *)
    /// YUL_PAYABLE_KEYWORD = "payable";
    /// ```
    YulPayableKeyword,
    /// Represents a node with kind `YulPragmaKeyword`, having the following structure:
    ///
    /// ```ebnf
    /// (* Reserved until 0.7.1 *)
    /// YUL_PRAGMA_KEYWORD = "pragma";
    /// ```
    YulPragmaKeyword,
    /// Represents a node with kind `YulPrivateKeyword`, having the following structure:
    ///
    /// ```ebnf
    /// (* Reserved until 0.7.1 *)
    /// YUL_PRIVATE_KEYWORD = "private";
    /// ```
    YulPrivateKeyword,
    /// Represents a node with kind `YulPromiseKeyword`, having the following structure:
    ///
    /// ```ebnf
    /// (* Reserved from 0.5.0 until 0.7.1 *)
    /// YUL_PROMISE_KEYWORD = "promise";
    /// ```
    YulPromiseKeyword,
    /// Represents a node with kind `YulPublicKeyword`, having the following structure:
    ///
    /// ```ebnf
    /// (* Reserved until 0.7.1 *)
    /// YUL_PUBLIC_KEYWORD = "public";
    /// ```
    YulPublicKeyword,
    /// Represents a node with kind `YulPureKeyword`, having the following structure:
    ///
    /// ```ebnf
    /// (* Reserved until 0.7.1 *)
    /// YUL_PURE_KEYWORD = "pure";
    /// ```
    YulPureKeyword,
    /// Represents a node with kind `YulReceiveKeyword`, having the following structure:
    ///
    /// ```ebnf
    /// (* Reserved from 0.6.0 until 0.7.1 *)
    /// YUL_RECEIVE_KEYWORD = "receive";
    /// ```
    YulReceiveKeyword,
    /// Represents a node with kind `YulReferenceKeyword`, having the following structure:
    ///
    /// ```ebnf
    /// (* Reserved from 0.5.0 until 0.7.1 *)
    /// YUL_REFERENCE_KEYWORD = "reference";
    /// ```
    YulReferenceKeyword,
    /// Represents a node with kind `YulRelocatableKeyword`, having the following structure:
    ///
    /// ```ebnf
    /// (* Reserved until 0.7.1 *)
    /// YUL_RELOCATABLE_KEYWORD = "relocatable";
    /// ```
    YulRelocatableKeyword,
    /// Represents a node with kind `YulReturnsKeyword`, having the following structure:
    ///
    /// ```ebnf
    /// (* Reserved until 0.7.1 *)
    /// YUL_RETURNS_KEYWORD = "returns";
    /// ```
    YulReturnsKeyword,
    /// Represents a node with kind `YulSealedKeyword`, having the following structure:
    ///
    /// ```ebnf
    /// (* Reserved from 0.5.0 until 0.7.1 *)
    /// YUL_SEALED_KEYWORD = "sealed";
    /// ```
    YulSealedKeyword,
    /// Represents a node with kind `YulSecondsKeyword`, having the following structure:
    ///
    /// ```ebnf
    /// (* Reserved until 0.7.1 *)
    /// YUL_SECONDS_KEYWORD = "seconds";
    /// ```
    YulSecondsKeyword,
    /// Represents a node with kind `YulSizeOfKeyword`, having the following structure:
    ///
    /// ```ebnf
    /// (* Reserved from 0.5.0 until 0.7.1 *)
    /// YUL_SIZE_OF_KEYWORD = "sizeof";
    /// ```
    YulSizeOfKeyword,
    /// Represents a node with kind `YulStaticKeyword`, having the following structure:
    ///
    /// ```ebnf
    /// (* Reserved until 0.7.1 *)
    /// YUL_STATIC_KEYWORD = "static";
    /// ```
    YulStaticKeyword,
    /// Represents a node with kind `YulStorageKeyword`, having the following structure:
    ///
    /// ```ebnf
    /// (* Reserved until 0.7.1 *)
    /// YUL_STORAGE_KEYWORD = "storage";
    /// ```
    YulStorageKeyword,
    /// Represents a node with kind `YulStringKeyword`, having the following structure:
    ///
    /// ```ebnf
    /// (* Reserved until 0.7.1 *)
    /// YUL_STRING_KEYWORD = "string";
    /// ```
    YulStringKeyword,
    /// Represents a node with kind `YulStructKeyword`, having the following structure:
    ///
    /// ```ebnf
    /// (* Reserved until 0.7.1 *)
    /// YUL_STRUCT_KEYWORD = "struct";
    /// ```
    YulStructKeyword,
    /// Represents a node with kind `YulSuperKeyword`, having the following structure:
    ///
    /// ```ebnf
    /// (* Reserved in 0.8.0 *)
    /// YUL_SUPER_KEYWORD = "super";
    /// ```
    YulSuperKeyword,
    /// Represents a node with kind `YulSupportsKeyword`, having the following structure:
    ///
    /// ```ebnf
    /// (* Reserved from 0.5.0 until 0.7.1 *)
    /// YUL_SUPPORTS_KEYWORD = "supports";
    /// ```
    YulSupportsKeyword,
    /// Represents a node with kind `YulSwitchKeyword`, having the following structure:
    ///
    /// ```ebnf
    /// YUL_SWITCH_KEYWORD = "switch";
    /// ```
    YulSwitchKeyword,
    /// Represents a node with kind `YulSzaboKeyword`, having the following structure:
    ///
    /// ```ebnf
    /// (* Reserved until 0.7.0 *)
    /// YUL_SZABO_KEYWORD = "szabo";
    /// ```
    YulSzaboKeyword,
    /// Represents a node with kind `YulThisKeyword`, having the following structure:
    ///
    /// ```ebnf
    /// (* Reserved in 0.8.0 *)
    /// YUL_THIS_KEYWORD = "this";
    /// ```
    YulThisKeyword,
    /// Represents a node with kind `YulThrowKeyword`, having the following structure:
    ///
    /// ```ebnf
    /// (* Reserved until 0.7.1 *)
    /// YUL_THROW_KEYWORD = "throw";
    /// ```
    YulThrowKeyword,
    /// Represents a node with kind `YulTrueKeyword`, having the following structure:
    ///
    /// ```ebnf
    /// (* Introduced in 0.6.2 *)
    /// YUL_TRUE_KEYWORD = "true";
    /// ```
    YulTrueKeyword,
    /// Represents a node with kind `YulTryKeyword`, having the following structure:
    ///
    /// ```ebnf
    /// (* Reserved until 0.7.1 *)
    /// YUL_TRY_KEYWORD = "try";
    /// ```
    YulTryKeyword,
    /// Represents a node with kind `YulTypeDefKeyword`, having the following structure:
    ///
    /// ```ebnf
    /// (* Reserved from 0.5.0 until 0.7.1 *)
    /// YUL_TYPE_DEF_KEYWORD = "typedef";
    /// ```
    YulTypeDefKeyword,
    /// Represents a node with kind `YulTypeKeyword`, having the following structure:
    ///
    /// ```ebnf
    /// (* Reserved until 0.7.1 *)
    /// YUL_TYPE_KEYWORD = "type";
    /// ```
    YulTypeKeyword,
    /// Represents a node with kind `YulTypeOfKeyword`, having the following structure:
    ///
    /// ```ebnf
    /// (* Reserved until 0.7.1 *)
    /// YUL_TYPE_OF_KEYWORD = "typeof";
    /// ```
    YulTypeOfKeyword,
    /// Represents a node with kind `YulUfixedKeyword`, having the following structure:
    ///
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
    /// Represents a node with kind `YulUintKeyword`, having the following structure:
    ///
    /// ```ebnf
    /// (* Reserved until 0.7.1 *)
    /// YUL_UINT_KEYWORD = "uint" ("8" | "16" | "24" | "32" | "40" | "48" | "56" | "64" | "72" | "80" | "88" | "96" | "104" | "112" | "120" | "128" | "136" | "144" | "152" | "160" | "168" | "176" | "184" | "192" | "200" | "208" | "216" | "224" | "232" | "240" | "248" | "256")?;
    /// ```
    YulUintKeyword,
    /// Represents a node with kind `YulUncheckedKeyword`, having the following structure:
    ///
    /// ```ebnf
    /// (* Reserved from 0.5.0 until 0.7.1 *)
    /// YUL_UNCHECKED_KEYWORD = "unchecked";
    /// ```
    YulUncheckedKeyword,
    /// Represents a node with kind `YulUsingKeyword`, having the following structure:
    ///
    /// ```ebnf
    /// (* Reserved until 0.7.1 *)
    /// YUL_USING_KEYWORD = "using";
    /// ```
    YulUsingKeyword,
    /// Represents a node with kind `YulVarKeyword`, having the following structure:
    ///
    /// ```ebnf
    /// (* Reserved until 0.6.5 *)
    /// YUL_VAR_KEYWORD = "var";
    /// ```
    YulVarKeyword,
    /// Represents a node with kind `YulViewKeyword`, having the following structure:
    ///
    /// ```ebnf
    /// (* Reserved until 0.7.1 *)
    /// YUL_VIEW_KEYWORD = "view";
    /// ```
    YulViewKeyword,
    /// Represents a node with kind `YulVirtualKeyword`, having the following structure:
    ///
    /// ```ebnf
    /// (* Reserved from 0.6.0 until 0.7.1 *)
    /// YUL_VIRTUAL_KEYWORD = "virtual";
    /// ```
    YulVirtualKeyword,
    /// Represents a node with kind `YulWeeksKeyword`, having the following structure:
    ///
    /// ```ebnf
    /// (* Reserved until 0.7.1 *)
    /// YUL_WEEKS_KEYWORD = "weeks";
    /// ```
    YulWeeksKeyword,
    /// Represents a node with kind `YulWeiKeyword`, having the following structure:
    ///
    /// ```ebnf
    /// (* Reserved until 0.7.1 *)
    /// YUL_WEI_KEYWORD = "wei";
    /// ```
    YulWeiKeyword,
    /// Represents a node with kind `YulWhileKeyword`, having the following structure:
    ///
    /// ```ebnf
    /// (* Reserved until 0.7.1 *)
    /// YUL_WHILE_KEYWORD = "while";
    /// ```
    YulWhileKeyword,
    /// Represents a node with kind `YulYearsKeyword`, having the following structure:
    ///
    /// ```ebnf
    /// (* Reserved until 0.7.1 *)
    /// YUL_YEARS_KEYWORD = "years";
    /// ```
    YulYearsKeyword,
}

impl crate::cst::TerminalKindExtensions for TerminalKind {
    fn is_identifier(self) -> bool {
        matches!(self, |Self::Identifier| Self::YulIdentifier)
    }

    fn is_trivia(self) -> bool {
        matches!(self, |Self::EndOfLine| Self::MultiLineComment
            | Self::MultiLineNatSpecComment
            | Self::SingleLineComment
            | Self::SingleLineNatSpecComment
            | Self::Whitespace)
    }

    fn is_valid(self) -> bool {
        !matches!(self, Self::UNRECOGNIZED | Self::MISSING)
    }
}

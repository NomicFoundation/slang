// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

pub use std::{collections::BTreeSet, ops::Range, rc::Rc};

#[allow(deprecated, unused_imports)]
use semver::Version;

pub use super::{cst, kinds::*, parser_output::ParserOutput};

const DEBUG_ERROR_MERGING: bool = false;

#[derive(PartialEq)]
pub struct ParseError {
    position: usize,
    expected: BTreeSet<String>,
}

impl ParseError {
    pub fn new<T: Into<String>>(position: usize, expected: T) -> Self {
        Self {
            position,
            expected: BTreeSet::from([expected.into()]),
        }
    }

    pub fn merge_with(&mut self, other: Self) {
        if DEBUG_ERROR_MERGING {
            if self.position < other.position {
                self.expected = BTreeSet::from([format!(
                    "O={other_expected}\nNOT {position}@[{expected}]",
                    other_expected = other.expected.iter().next().unwrap(),
                    position = self.position,
                    expected = self.expected.iter().next().unwrap(),
                )]);
                self.position = other.position;
            } else if self.position == other.position {
                self.expected = BTreeSet::from([format!(
                    "{other_expected}, or {expected}",
                    other_expected = other.expected.iter().next().unwrap(),
                    expected = self.expected.iter().next().unwrap(),
                )]);
            } else {
                self.expected = BTreeSet::from([format!(
                    "S={expected}\nNOT {other_position}@[{other_expected}]",
                    expected = self.expected.iter().next().unwrap(),
                    other_position = other.position,
                    other_expected = other.expected.iter().next().unwrap(),
                )]);
            }
        } else {
            if self.position < other.position {
                *self = other;
            } else if self.position == other.position {
                self.expected.extend(other.expected);
            }
        }
    }

    pub fn maybe_merge_with(mut self, other: Option<Self>) -> Self {
        if let Some(other) = other {
            self.merge_with(other)
        }
        self
    }
}

pub enum ParseResult {
    Pass {
        node: Rc<cst::Node>,
        error: Option<ParseError>,
    },
    Fail {
        error: ParseError,
    },
}

pub struct Stream<'s> {
    source: &'s str,
    position: usize,
    undo_position: usize,
    has_undo: bool,
}

impl<'s> Stream<'s> {
    pub fn new(source: &'s str) -> Self {
        Self {
            source,
            position: 0,
            undo_position: 0,
            has_undo: false,
        }
    }

    pub fn source(&self) -> &'s str {
        self.source
    }

    pub fn position(&self) -> usize {
        self.position
    }

    pub fn set_position(&mut self, position: usize) {
        self.position = position;
    }

    pub fn peek(&self) -> Option<char> {
        self.source[self.position..].chars().next()
    }

    pub fn next(&mut self) -> Option<char> {
        self.has_undo = true;
        self.undo_position = self.position;
        let mut chars = self.source[self.position..].chars();
        if let Some(c) = chars.next() {
            self.position += c.len_utf8();
            Some(c)
        } else {
            None
        }
    }

    pub fn undo(&mut self) {
        if !self.has_undo {
            panic!("No undo position");
        }
        self.position = self.undo_position;
        self.has_undo = false;
    }
}

pub(crate) fn render_error_report(
    error: &ParseError,
    source_id: &str,
    source: &str,
    with_color: bool,
) -> String {
    use ariadne::{Color, Config, Label, Report, ReportKind, Source};

    let kind = ReportKind::Error;
    let color = if with_color { Color::Red } else { Color::Unset };
    let source_start = error.position;
    let source_end = error.position;

    let message = {
        let message = format!(
            "Expected {expectations}.",
            expectations = error
                .expected
                .iter()
                .cloned()
                .collect::<Vec<String>>()
                .join(" or ")
        );

        if DEBUG_ERROR_MERGING {
            format!("{position}: {message}", position = error.position)
        } else {
            message
        }
    };

    if source.is_empty() {
        return format!("{kind}: {message}\n   ─[{source_id}:{source_start}:{source_end}]");
    }

    let mut builder = Report::build(kind, source_id, source_start)
        .with_config(Config::default().with_color(with_color))
        .with_message(message);

    builder.add_label(
        Label::new((source_id, source_start..source_end))
            .with_color(color)
            .with_message("Error occurred here.".to_string()),
    );

    let mut result = vec![];
    builder
        .finish()
        .write((source_id, Source::from(&source)), &mut result)
        .expect("Failed to write report");

    return String::from_utf8(result)
        .expect("Failed to convert report to utf8")
        .trim()
        .to_string();
}

#[allow(dead_code)]
fn call_scanner<L, F>(
    language: &L,
    input: &str,
    scanner: F,
    kind: TokenKind,
    error_message: &str,
) -> Option<ParserOutput>
where
    F: Fn(&L, &mut Stream) -> bool,
{
    let mut stream = Stream::new(input);
    Some(
        if scanner(language, &mut stream) && stream.peek().is_none() {
            ParserOutput {
                parse_tree: Some(cst::Node::token(
                    kind,
                    Range {
                        start: 0,
                        end: stream.position(),
                    },
                    None,
                    None,
                )),
                errors: vec![],
            }
        } else {
            ParserOutput {
                parse_tree: None,
                errors: vec![ParseError::new(stream.position(), error_message)],
            }
        },
    )
}

#[allow(dead_code)]
fn try_call_scanner<L, F>(
    language: &L,
    input: &str,
    scanner: F,
    kind: TokenKind,
    error_message: &str,
) -> Option<ParserOutput>
where
    F: Fn(&L, &mut Stream) -> Option<bool>,
{
    let mut stream = Stream::new(input);
    scanner(language, &mut stream).map(|result| {
        if result && stream.peek().is_none() {
            ParserOutput {
                parse_tree: Some(cst::Node::token(
                    kind,
                    Range {
                        start: 0,
                        end: stream.position(),
                    },
                    None,
                    None,
                )),
                errors: vec![],
            }
        } else {
            ParserOutput {
                parse_tree: None,
                errors: vec![ParseError::new(stream.position(), error_message)],
            }
        }
    })
}

#[allow(dead_code)]
fn call_parser<L, F>(language: &L, input: &str, parser: F) -> Option<ParserOutput>
where
    F: Fn(&L, &mut Stream) -> ParseResult,
{
    let mut stream = Stream::new(input);
    Some(match parser(language, &mut stream) {
        ParseResult::Pass { node, .. } if stream.peek().is_none() => ParserOutput {
            parse_tree: Some(node),
            errors: vec![],
        },
        ParseResult::Pass { .. } => ParserOutput {
            parse_tree: None,
            errors: vec![ParseError::new(stream.position(), "end of input")],
        },
        ParseResult::Fail { error } => ParserOutput {
            parse_tree: None,
            errors: vec![error],
        },
    })
}

#[allow(dead_code)]
fn try_call_parser<L, F>(language: &L, input: &str, parser: F) -> Option<ParserOutput>
where
    F: Fn(&L, &mut Stream) -> Option<ParseResult>,
{
    let mut stream = Stream::new(input);
    parser(language, &mut stream).map(|result| match result {
        ParseResult::Pass { node, .. } if stream.peek().is_none() => ParserOutput {
            parse_tree: Some(node),
            errors: vec![],
        },
        ParseResult::Pass { .. } => ParserOutput {
            parse_tree: None,
            errors: vec![ParseError::new(stream.position(), "end of input")],
        },
        ParseResult::Fail { error } => ParserOutput {
            parse_tree: None,
            errors: vec![error],
        },
    })
}

#[napi]
pub struct Language {
    pub(crate) version: Version,
    pub(crate) version_is_equal_to_or_greater_than_0_4_22: bool,
    pub(crate) version_is_equal_to_or_greater_than_0_6_0: bool,
    pub(crate) version_is_equal_to_or_greater_than_0_8_18: bool,
}

#[napi]
impl Language {
    #[napi(constructor)]
    pub fn new(version: String) -> Self {
        static VERSIONS: &'static [&'static str] = &[
            "0.4.11", "0.4.12", "0.4.13", "0.4.14", "0.4.15", "0.4.16", "0.4.17", "0.4.18",
            "0.4.19", "0.4.20", "0.4.21", "0.4.22", "0.4.23", "0.4.24", "0.4.25", "0.4.26",
            "0.5.0", "0.5.1", "0.5.2", "0.5.3", "0.5.4", "0.5.5", "0.5.6", "0.5.7", "0.5.8",
            "0.5.9", "0.5.10", "0.5.11", "0.5.12", "0.5.13", "0.5.14", "0.5.15", "0.5.16",
            "0.5.17", "0.6.0", "0.6.1", "0.6.2", "0.6.3", "0.6.4", "0.6.5", "0.6.6", "0.6.7",
            "0.6.8", "0.6.9", "0.6.10", "0.6.11", "0.6.12", "0.7.0", "0.7.1", "0.7.2", "0.7.3",
            "0.7.4", "0.7.5", "0.7.6", "0.8.0", "0.8.1", "0.8.2", "0.8.3", "0.8.4", "0.8.5",
            "0.8.6", "0.8.7", "0.8.8", "0.8.9", "0.8.10", "0.8.11", "0.8.12", "0.8.13", "0.8.14",
            "0.8.15", "0.8.16", "0.8.17", "0.8.18",
        ];
        let version = Version::parse(&version).unwrap();
        if VERSIONS.contains(&version.to_string().as_str()) {
            Self {
                version_is_equal_to_or_greater_than_0_4_22: Version::parse("0.4.22").unwrap()
                    <= version,
                version_is_equal_to_or_greater_than_0_6_0: Version::parse("0.6.0").unwrap()
                    <= version,
                version_is_equal_to_or_greater_than_0_8_18: Version::parse("0.8.18").unwrap()
                    <= version,
                version,
            }
        } else {
            panic!("Invalid Solidity language version: {version}");
        }
    }

    #[napi]
    pub fn version(&self) -> String {
        self.version.to_string()
    }

    #[napi]
    pub fn parse(&self, production_kind: ProductionKind, input: String) -> ParserOutput {
        let input = input.as_str();
        match production_kind {
            ProductionKind::AbicoderKeyword => call_scanner(
                self,
                input,
                Language::scan_abicoder_keyword,
                TokenKind::AbicoderKeyword,
                "AbicoderKeyword",
            ),
            ProductionKind::AbstractKeyword => call_scanner(
                self,
                input,
                Language::scan_abstract_keyword,
                TokenKind::AbstractKeyword,
                "AbstractKeyword",
            ),
            ProductionKind::AddressKeyword => call_scanner(
                self,
                input,
                Language::scan_address_keyword,
                TokenKind::AddressKeyword,
                "AddressKeyword",
            ),
            ProductionKind::Ampersand => call_scanner(
                self,
                input,
                Language::scan_ampersand,
                TokenKind::Ampersand,
                "Ampersand",
            ),
            ProductionKind::AmpersandAmpersand => call_scanner(
                self,
                input,
                Language::scan_ampersand_ampersand,
                TokenKind::AmpersandAmpersand,
                "AmpersandAmpersand",
            ),
            ProductionKind::AmpersandEqual => call_scanner(
                self,
                input,
                Language::scan_ampersand_equal,
                TokenKind::AmpersandEqual,
                "AmpersandEqual",
            ),
            ProductionKind::AnonymousKeyword => call_scanner(
                self,
                input,
                Language::scan_anonymous_keyword,
                TokenKind::AnonymousKeyword,
                "AnonymousKeyword",
            ),
            ProductionKind::AsKeyword => call_scanner(
                self,
                input,
                Language::scan_as_keyword,
                TokenKind::AsKeyword,
                "AsKeyword",
            ),
            ProductionKind::AsciiEscape => call_scanner(
                self,
                input,
                Language::scan_ascii_escape,
                TokenKind::AsciiEscape,
                "AsciiEscape",
            ),
            ProductionKind::AsciiStringLiteral => call_scanner(
                self,
                input,
                Language::scan_ascii_string_literal,
                TokenKind::AsciiStringLiteral,
                "AsciiStringLiteral",
            ),
            ProductionKind::AssemblyKeyword => call_scanner(
                self,
                input,
                Language::scan_assembly_keyword,
                TokenKind::AssemblyKeyword,
                "AssemblyKeyword",
            ),
            ProductionKind::Bang => {
                call_scanner(self, input, Language::scan_bang, TokenKind::Bang, "Bang")
            }
            ProductionKind::BangEqual => call_scanner(
                self,
                input,
                Language::scan_bang_equal,
                TokenKind::BangEqual,
                "BangEqual",
            ),
            ProductionKind::Bar => {
                call_scanner(self, input, Language::scan_bar, TokenKind::Bar, "Bar")
            }
            ProductionKind::BarBar => call_scanner(
                self,
                input,
                Language::scan_bar_bar,
                TokenKind::BarBar,
                "BarBar",
            ),
            ProductionKind::BarEqual => call_scanner(
                self,
                input,
                Language::scan_bar_equal,
                TokenKind::BarEqual,
                "BarEqual",
            ),
            ProductionKind::BoolKeyword => call_scanner(
                self,
                input,
                Language::scan_bool_keyword,
                TokenKind::BoolKeyword,
                "BoolKeyword",
            ),
            ProductionKind::BreakKeyword => call_scanner(
                self,
                input,
                Language::scan_break_keyword,
                TokenKind::BreakKeyword,
                "BreakKeyword",
            ),
            ProductionKind::CalldataKeyword => call_scanner(
                self,
                input,
                Language::scan_calldata_keyword,
                TokenKind::CalldataKeyword,
                "CalldataKeyword",
            ),
            ProductionKind::Caret => {
                call_scanner(self, input, Language::scan_caret, TokenKind::Caret, "Caret")
            }
            ProductionKind::CaretEqual => call_scanner(
                self,
                input,
                Language::scan_caret_equal,
                TokenKind::CaretEqual,
                "CaretEqual",
            ),
            ProductionKind::CaseKeyword => call_scanner(
                self,
                input,
                Language::scan_case_keyword,
                TokenKind::CaseKeyword,
                "CaseKeyword",
            ),
            ProductionKind::CatchKeyword => call_scanner(
                self,
                input,
                Language::scan_catch_keyword,
                TokenKind::CatchKeyword,
                "CatchKeyword",
            ),
            ProductionKind::CloseBrace => call_scanner(
                self,
                input,
                Language::scan_close_brace,
                TokenKind::CloseBrace,
                "CloseBrace",
            ),
            ProductionKind::CloseBracket => call_scanner(
                self,
                input,
                Language::scan_close_bracket,
                TokenKind::CloseBracket,
                "CloseBracket",
            ),
            ProductionKind::CloseParen => call_scanner(
                self,
                input,
                Language::scan_close_paren,
                TokenKind::CloseParen,
                "CloseParen",
            ),
            ProductionKind::Colon => {
                call_scanner(self, input, Language::scan_colon, TokenKind::Colon, "Colon")
            }
            ProductionKind::ColonEqual => call_scanner(
                self,
                input,
                Language::scan_colon_equal,
                TokenKind::ColonEqual,
                "ColonEqual",
            ),
            ProductionKind::Comma => {
                call_scanner(self, input, Language::scan_comma, TokenKind::Comma, "Comma")
            }
            ProductionKind::ConstantKeyword => call_scanner(
                self,
                input,
                Language::scan_constant_keyword,
                TokenKind::ConstantKeyword,
                "ConstantKeyword",
            ),
            ProductionKind::ConstructorKeyword => call_scanner(
                self,
                input,
                Language::scan_constructor_keyword,
                TokenKind::ConstructorKeyword,
                "ConstructorKeyword",
            ),
            ProductionKind::ContinueKeyword => call_scanner(
                self,
                input,
                Language::scan_continue_keyword,
                TokenKind::ContinueKeyword,
                "ContinueKeyword",
            ),
            ProductionKind::ContractKeyword => call_scanner(
                self,
                input,
                Language::scan_contract_keyword,
                TokenKind::ContractKeyword,
                "ContractKeyword",
            ),
            ProductionKind::DaysKeyword => call_scanner(
                self,
                input,
                Language::scan_days_keyword,
                TokenKind::DaysKeyword,
                "DaysKeyword",
            ),
            ProductionKind::DecimalExponent => call_scanner(
                self,
                input,
                Language::scan_decimal_exponent,
                TokenKind::DecimalExponent,
                "DecimalExponent",
            ),
            ProductionKind::DecimalLiteral => call_scanner(
                self,
                input,
                Language::scan_decimal_literal,
                TokenKind::DecimalLiteral,
                "DecimalLiteral",
            ),
            ProductionKind::DecimalNumber => call_scanner(
                self,
                input,
                Language::scan_decimal_number,
                TokenKind::DecimalNumber,
                "DecimalNumber",
            ),
            ProductionKind::DefaultKeyword => call_scanner(
                self,
                input,
                Language::scan_default_keyword,
                TokenKind::DefaultKeyword,
                "DefaultKeyword",
            ),
            ProductionKind::DeleteKeyword => call_scanner(
                self,
                input,
                Language::scan_delete_keyword,
                TokenKind::DeleteKeyword,
                "DeleteKeyword",
            ),
            ProductionKind::DoKeyword => call_scanner(
                self,
                input,
                Language::scan_do_keyword,
                TokenKind::DoKeyword,
                "DoKeyword",
            ),
            ProductionKind::DoubleQuotedAsciiStringLiteral => call_scanner(
                self,
                input,
                Language::scan_double_quoted_ascii_string_literal,
                TokenKind::DoubleQuotedAsciiStringLiteral,
                "DoubleQuotedAsciiStringLiteral",
            ),
            ProductionKind::DoubleQuotedUnicodeStringLiteral => call_scanner(
                self,
                input,
                Language::scan_double_quoted_unicode_string_literal,
                TokenKind::DoubleQuotedUnicodeStringLiteral,
                "DoubleQuotedUnicodeStringLiteral",
            ),
            ProductionKind::ElseKeyword => call_scanner(
                self,
                input,
                Language::scan_else_keyword,
                TokenKind::ElseKeyword,
                "ElseKeyword",
            ),
            ProductionKind::EmitKeyword => call_scanner(
                self,
                input,
                Language::scan_emit_keyword,
                TokenKind::EmitKeyword,
                "EmitKeyword",
            ),
            ProductionKind::EndOfLine => call_scanner(
                self,
                input,
                Language::scan_end_of_line,
                TokenKind::EndOfLine,
                "EndOfLine",
            ),
            ProductionKind::EnumKeyword => call_scanner(
                self,
                input,
                Language::scan_enum_keyword,
                TokenKind::EnumKeyword,
                "EnumKeyword",
            ),
            ProductionKind::Equal => {
                call_scanner(self, input, Language::scan_equal, TokenKind::Equal, "Equal")
            }
            ProductionKind::EqualEqual => call_scanner(
                self,
                input,
                Language::scan_equal_equal,
                TokenKind::EqualEqual,
                "EqualEqual",
            ),
            ProductionKind::EqualGreaterThan => call_scanner(
                self,
                input,
                Language::scan_equal_greater_than,
                TokenKind::EqualGreaterThan,
                "EqualGreaterThan",
            ),
            ProductionKind::ErrorKeyword => call_scanner(
                self,
                input,
                Language::scan_error_keyword,
                TokenKind::ErrorKeyword,
                "ErrorKeyword",
            ),
            ProductionKind::EscapeSequence => call_scanner(
                self,
                input,
                Language::scan_escape_sequence,
                TokenKind::EscapeSequence,
                "EscapeSequence",
            ),
            ProductionKind::EtherKeyword => call_scanner(
                self,
                input,
                Language::scan_ether_keyword,
                TokenKind::EtherKeyword,
                "EtherKeyword",
            ),
            ProductionKind::EventKeyword => call_scanner(
                self,
                input,
                Language::scan_event_keyword,
                TokenKind::EventKeyword,
                "EventKeyword",
            ),
            ProductionKind::Evmasm => call_scanner(
                self,
                input,
                Language::scan_evmasm,
                TokenKind::Evmasm,
                "Evmasm",
            ),
            ProductionKind::ExperimentalKeyword => call_scanner(
                self,
                input,
                Language::scan_experimental_keyword,
                TokenKind::ExperimentalKeyword,
                "ExperimentalKeyword",
            ),
            ProductionKind::ExternalKeyword => call_scanner(
                self,
                input,
                Language::scan_external_keyword,
                TokenKind::ExternalKeyword,
                "ExternalKeyword",
            ),
            ProductionKind::FallbackKeyword => call_scanner(
                self,
                input,
                Language::scan_fallback_keyword,
                TokenKind::FallbackKeyword,
                "FallbackKeyword",
            ),
            ProductionKind::FalseKeyword => call_scanner(
                self,
                input,
                Language::scan_false_keyword,
                TokenKind::FalseKeyword,
                "FalseKeyword",
            ),
            ProductionKind::FinneyKeyword => call_scanner(
                self,
                input,
                Language::scan_finney_keyword,
                TokenKind::FinneyKeyword,
                "FinneyKeyword",
            ),
            ProductionKind::FixedBytesType => call_scanner(
                self,
                input,
                Language::scan_fixed_bytes_type,
                TokenKind::FixedBytesType,
                "FixedBytesType",
            ),
            ProductionKind::ForKeyword => call_scanner(
                self,
                input,
                Language::scan_for_keyword,
                TokenKind::ForKeyword,
                "ForKeyword",
            ),
            ProductionKind::FromKeyword => call_scanner(
                self,
                input,
                Language::scan_from_keyword,
                TokenKind::FromKeyword,
                "FromKeyword",
            ),
            ProductionKind::FunctionKeyword => call_scanner(
                self,
                input,
                Language::scan_function_keyword,
                TokenKind::FunctionKeyword,
                "FunctionKeyword",
            ),
            ProductionKind::GlobalKeyword => call_scanner(
                self,
                input,
                Language::scan_global_keyword,
                TokenKind::GlobalKeyword,
                "GlobalKeyword",
            ),
            ProductionKind::GreaterThan => call_scanner(
                self,
                input,
                Language::scan_greater_than,
                TokenKind::GreaterThan,
                "GreaterThan",
            ),
            ProductionKind::GreaterThanEqual => call_scanner(
                self,
                input,
                Language::scan_greater_than_equal,
                TokenKind::GreaterThanEqual,
                "GreaterThanEqual",
            ),
            ProductionKind::GreaterThanGreaterThan => call_scanner(
                self,
                input,
                Language::scan_greater_than_greater_than,
                TokenKind::GreaterThanGreaterThan,
                "GreaterThanGreaterThan",
            ),
            ProductionKind::GreaterThanGreaterThanEqual => call_scanner(
                self,
                input,
                Language::scan_greater_than_greater_than_equal,
                TokenKind::GreaterThanGreaterThanEqual,
                "GreaterThanGreaterThanEqual",
            ),
            ProductionKind::GreaterThanGreaterThanGreaterThan => call_scanner(
                self,
                input,
                Language::scan_greater_than_greater_than_greater_than,
                TokenKind::GreaterThanGreaterThanGreaterThan,
                "GreaterThanGreaterThanGreaterThan",
            ),
            ProductionKind::GreaterThanGreaterThanGreaterThanEqual => call_scanner(
                self,
                input,
                Language::scan_greater_than_greater_than_greater_than_equal,
                TokenKind::GreaterThanGreaterThanGreaterThanEqual,
                "GreaterThanGreaterThanGreaterThanEqual",
            ),
            ProductionKind::GweiKeyword => call_scanner(
                self,
                input,
                Language::scan_gwei_keyword,
                TokenKind::GweiKeyword,
                "GweiKeyword",
            ),
            ProductionKind::HexByteEscape => call_scanner(
                self,
                input,
                Language::scan_hex_byte_escape,
                TokenKind::HexByteEscape,
                "HexByteEscape",
            ),
            ProductionKind::HexCharacter => call_scanner(
                self,
                input,
                Language::scan_hex_character,
                TokenKind::HexCharacter,
                "HexCharacter",
            ),
            ProductionKind::HexLiteral => call_scanner(
                self,
                input,
                Language::scan_hex_literal,
                TokenKind::HexLiteral,
                "HexLiteral",
            ),
            ProductionKind::HexStringLiteral => call_scanner(
                self,
                input,
                Language::scan_hex_string_literal,
                TokenKind::HexStringLiteral,
                "HexStringLiteral",
            ),
            ProductionKind::HoursKeyword => call_scanner(
                self,
                input,
                Language::scan_hours_keyword,
                TokenKind::HoursKeyword,
                "HoursKeyword",
            ),
            ProductionKind::Identifier => call_scanner(
                self,
                input,
                Language::scan_identifier,
                TokenKind::Identifier,
                "Identifier",
            ),
            ProductionKind::IdentifierPart => call_scanner(
                self,
                input,
                Language::scan_identifier_part,
                TokenKind::IdentifierPart,
                "IdentifierPart",
            ),
            ProductionKind::IdentifierStart => call_scanner(
                self,
                input,
                Language::scan_identifier_start,
                TokenKind::IdentifierStart,
                "IdentifierStart",
            ),
            ProductionKind::IfKeyword => call_scanner(
                self,
                input,
                Language::scan_if_keyword,
                TokenKind::IfKeyword,
                "IfKeyword",
            ),
            ProductionKind::ImmutableKeyword => call_scanner(
                self,
                input,
                Language::scan_immutable_keyword,
                TokenKind::ImmutableKeyword,
                "ImmutableKeyword",
            ),
            ProductionKind::ImportKeyword => call_scanner(
                self,
                input,
                Language::scan_import_keyword,
                TokenKind::ImportKeyword,
                "ImportKeyword",
            ),
            ProductionKind::IndexedKeyword => call_scanner(
                self,
                input,
                Language::scan_indexed_keyword,
                TokenKind::IndexedKeyword,
                "IndexedKeyword",
            ),
            ProductionKind::InterfaceKeyword => call_scanner(
                self,
                input,
                Language::scan_interface_keyword,
                TokenKind::InterfaceKeyword,
                "InterfaceKeyword",
            ),
            ProductionKind::InternalKeyword => call_scanner(
                self,
                input,
                Language::scan_internal_keyword,
                TokenKind::InternalKeyword,
                "InternalKeyword",
            ),
            ProductionKind::IsKeyword => call_scanner(
                self,
                input,
                Language::scan_is_keyword,
                TokenKind::IsKeyword,
                "IsKeyword",
            ),
            ProductionKind::Keyword => call_scanner(
                self,
                input,
                Language::scan_keyword,
                TokenKind::Keyword,
                "Keyword",
            ),
            ProductionKind::LeaveKeyword => call_scanner(
                self,
                input,
                Language::scan_leave_keyword,
                TokenKind::LeaveKeyword,
                "LeaveKeyword",
            ),
            ProductionKind::LessThan => call_scanner(
                self,
                input,
                Language::scan_less_than,
                TokenKind::LessThan,
                "LessThan",
            ),
            ProductionKind::LessThanEqual => call_scanner(
                self,
                input,
                Language::scan_less_than_equal,
                TokenKind::LessThanEqual,
                "LessThanEqual",
            ),
            ProductionKind::LessThanLessThan => call_scanner(
                self,
                input,
                Language::scan_less_than_less_than,
                TokenKind::LessThanLessThan,
                "LessThanLessThan",
            ),
            ProductionKind::LessThanLessThanEqual => call_scanner(
                self,
                input,
                Language::scan_less_than_less_than_equal,
                TokenKind::LessThanLessThanEqual,
                "LessThanLessThanEqual",
            ),
            ProductionKind::LetKeyword => call_scanner(
                self,
                input,
                Language::scan_let_keyword,
                TokenKind::LetKeyword,
                "LetKeyword",
            ),
            ProductionKind::LibraryKeyword => call_scanner(
                self,
                input,
                Language::scan_library_keyword,
                TokenKind::LibraryKeyword,
                "LibraryKeyword",
            ),
            ProductionKind::MappingKeyword => call_scanner(
                self,
                input,
                Language::scan_mapping_keyword,
                TokenKind::MappingKeyword,
                "MappingKeyword",
            ),
            ProductionKind::MemoryKeyword => call_scanner(
                self,
                input,
                Language::scan_memory_keyword,
                TokenKind::MemoryKeyword,
                "MemoryKeyword",
            ),
            ProductionKind::Minus => {
                call_scanner(self, input, Language::scan_minus, TokenKind::Minus, "Minus")
            }
            ProductionKind::MinusEqual => call_scanner(
                self,
                input,
                Language::scan_minus_equal,
                TokenKind::MinusEqual,
                "MinusEqual",
            ),
            ProductionKind::MinusGreaterThan => call_scanner(
                self,
                input,
                Language::scan_minus_greater_than,
                TokenKind::MinusGreaterThan,
                "MinusGreaterThan",
            ),
            ProductionKind::MinusMinus => call_scanner(
                self,
                input,
                Language::scan_minus_minus,
                TokenKind::MinusMinus,
                "MinusMinus",
            ),
            ProductionKind::MinutesKeyword => call_scanner(
                self,
                input,
                Language::scan_minutes_keyword,
                TokenKind::MinutesKeyword,
                "MinutesKeyword",
            ),
            ProductionKind::ModifierKeyword => call_scanner(
                self,
                input,
                Language::scan_modifier_keyword,
                TokenKind::ModifierKeyword,
                "ModifierKeyword",
            ),
            ProductionKind::MultilineComment => call_scanner(
                self,
                input,
                Language::scan_multiline_comment,
                TokenKind::MultilineComment,
                "MultilineComment",
            ),
            ProductionKind::NewKeyword => call_scanner(
                self,
                input,
                Language::scan_new_keyword,
                TokenKind::NewKeyword,
                "NewKeyword",
            ),
            ProductionKind::OpenBrace => call_scanner(
                self,
                input,
                Language::scan_open_brace,
                TokenKind::OpenBrace,
                "OpenBrace",
            ),
            ProductionKind::OpenBracket => call_scanner(
                self,
                input,
                Language::scan_open_bracket,
                TokenKind::OpenBracket,
                "OpenBracket",
            ),
            ProductionKind::OpenParen => call_scanner(
                self,
                input,
                Language::scan_open_paren,
                TokenKind::OpenParen,
                "OpenParen",
            ),
            ProductionKind::OverrideKeyword => call_scanner(
                self,
                input,
                Language::scan_override_keyword,
                TokenKind::OverrideKeyword,
                "OverrideKeyword",
            ),
            ProductionKind::PayableKeyword => call_scanner(
                self,
                input,
                Language::scan_payable_keyword,
                TokenKind::PayableKeyword,
                "PayableKeyword",
            ),
            ProductionKind::Percent => call_scanner(
                self,
                input,
                Language::scan_percent,
                TokenKind::Percent,
                "Percent",
            ),
            ProductionKind::PercentEqual => call_scanner(
                self,
                input,
                Language::scan_percent_equal,
                TokenKind::PercentEqual,
                "PercentEqual",
            ),
            ProductionKind::Period => call_scanner(
                self,
                input,
                Language::scan_period,
                TokenKind::Period,
                "Period",
            ),
            ProductionKind::Plus => {
                call_scanner(self, input, Language::scan_plus, TokenKind::Plus, "Plus")
            }
            ProductionKind::PlusEqual => call_scanner(
                self,
                input,
                Language::scan_plus_equal,
                TokenKind::PlusEqual,
                "PlusEqual",
            ),
            ProductionKind::PlusPlus => call_scanner(
                self,
                input,
                Language::scan_plus_plus,
                TokenKind::PlusPlus,
                "PlusPlus",
            ),
            ProductionKind::PossiblySeparatedPairsOfHexDigits => call_scanner(
                self,
                input,
                Language::scan_possibly_separated_pairs_of_hex_digits,
                TokenKind::PossiblySeparatedPairsOfHexDigits,
                "PossiblySeparatedPairsOfHexDigits",
            ),
            ProductionKind::PragmaKeyword => call_scanner(
                self,
                input,
                Language::scan_pragma_keyword,
                TokenKind::PragmaKeyword,
                "PragmaKeyword",
            ),
            ProductionKind::PrivateKeyword => call_scanner(
                self,
                input,
                Language::scan_private_keyword,
                TokenKind::PrivateKeyword,
                "PrivateKeyword",
            ),
            ProductionKind::PublicKeyword => call_scanner(
                self,
                input,
                Language::scan_public_keyword,
                TokenKind::PublicKeyword,
                "PublicKeyword",
            ),
            ProductionKind::PureKeyword => call_scanner(
                self,
                input,
                Language::scan_pure_keyword,
                TokenKind::PureKeyword,
                "PureKeyword",
            ),
            ProductionKind::QuestionMark => call_scanner(
                self,
                input,
                Language::scan_question_mark,
                TokenKind::QuestionMark,
                "QuestionMark",
            ),
            ProductionKind::RawIdentifier => call_scanner(
                self,
                input,
                Language::scan_raw_identifier,
                TokenKind::RawIdentifier,
                "RawIdentifier",
            ),
            ProductionKind::ReceiveKeyword => call_scanner(
                self,
                input,
                Language::scan_receive_keyword,
                TokenKind::ReceiveKeyword,
                "ReceiveKeyword",
            ),
            ProductionKind::ReservedKeyword => call_scanner(
                self,
                input,
                Language::scan_reserved_keyword,
                TokenKind::ReservedKeyword,
                "ReservedKeyword",
            ),
            ProductionKind::ReturnKeyword => call_scanner(
                self,
                input,
                Language::scan_return_keyword,
                TokenKind::ReturnKeyword,
                "ReturnKeyword",
            ),
            ProductionKind::ReturnsKeyword => call_scanner(
                self,
                input,
                Language::scan_returns_keyword,
                TokenKind::ReturnsKeyword,
                "ReturnsKeyword",
            ),
            ProductionKind::RevertKeyword => call_scanner(
                self,
                input,
                Language::scan_revert_keyword,
                TokenKind::RevertKeyword,
                "RevertKeyword",
            ),
            ProductionKind::SecondsKeyword => call_scanner(
                self,
                input,
                Language::scan_seconds_keyword,
                TokenKind::SecondsKeyword,
                "SecondsKeyword",
            ),
            ProductionKind::Semicolon => call_scanner(
                self,
                input,
                Language::scan_semicolon,
                TokenKind::Semicolon,
                "Semicolon",
            ),
            ProductionKind::SignedFixedType => call_scanner(
                self,
                input,
                Language::scan_signed_fixed_type,
                TokenKind::SignedFixedType,
                "SignedFixedType",
            ),
            ProductionKind::SignedIntegerType => call_scanner(
                self,
                input,
                Language::scan_signed_integer_type,
                TokenKind::SignedIntegerType,
                "SignedIntegerType",
            ),
            ProductionKind::SingleLineComment => call_scanner(
                self,
                input,
                Language::scan_single_line_comment,
                TokenKind::SingleLineComment,
                "SingleLineComment",
            ),
            ProductionKind::SingleQuotedAsciiStringLiteral => call_scanner(
                self,
                input,
                Language::scan_single_quoted_ascii_string_literal,
                TokenKind::SingleQuotedAsciiStringLiteral,
                "SingleQuotedAsciiStringLiteral",
            ),
            ProductionKind::SingleQuotedUnicodeStringLiteral => call_scanner(
                self,
                input,
                Language::scan_single_quoted_unicode_string_literal,
                TokenKind::SingleQuotedUnicodeStringLiteral,
                "SingleQuotedUnicodeStringLiteral",
            ),
            ProductionKind::Slash => {
                call_scanner(self, input, Language::scan_slash, TokenKind::Slash, "Slash")
            }
            ProductionKind::SlashEqual => call_scanner(
                self,
                input,
                Language::scan_slash_equal,
                TokenKind::SlashEqual,
                "SlashEqual",
            ),
            ProductionKind::SolidityKeyword => call_scanner(
                self,
                input,
                Language::scan_solidity_keyword,
                TokenKind::SolidityKeyword,
                "SolidityKeyword",
            ),
            ProductionKind::Star => {
                call_scanner(self, input, Language::scan_star, TokenKind::Star, "Star")
            }
            ProductionKind::StarEqual => call_scanner(
                self,
                input,
                Language::scan_star_equal,
                TokenKind::StarEqual,
                "StarEqual",
            ),
            ProductionKind::StarStar => call_scanner(
                self,
                input,
                Language::scan_star_star,
                TokenKind::StarStar,
                "StarStar",
            ),
            ProductionKind::StorageKeyword => call_scanner(
                self,
                input,
                Language::scan_storage_keyword,
                TokenKind::StorageKeyword,
                "StorageKeyword",
            ),
            ProductionKind::StringKeyword => call_scanner(
                self,
                input,
                Language::scan_string_keyword,
                TokenKind::StringKeyword,
                "StringKeyword",
            ),
            ProductionKind::StructKeyword => call_scanner(
                self,
                input,
                Language::scan_struct_keyword,
                TokenKind::StructKeyword,
                "StructKeyword",
            ),
            ProductionKind::SwitchKeyword => call_scanner(
                self,
                input,
                Language::scan_switch_keyword,
                TokenKind::SwitchKeyword,
                "SwitchKeyword",
            ),
            ProductionKind::SzaboKeyword => call_scanner(
                self,
                input,
                Language::scan_szabo_keyword,
                TokenKind::SzaboKeyword,
                "SzaboKeyword",
            ),
            ProductionKind::Tilde => {
                call_scanner(self, input, Language::scan_tilde, TokenKind::Tilde, "Tilde")
            }
            ProductionKind::TrueKeyword => call_scanner(
                self,
                input,
                Language::scan_true_keyword,
                TokenKind::TrueKeyword,
                "TrueKeyword",
            ),
            ProductionKind::TryKeyword => call_scanner(
                self,
                input,
                Language::scan_try_keyword,
                TokenKind::TryKeyword,
                "TryKeyword",
            ),
            ProductionKind::TypeKeyword => call_scanner(
                self,
                input,
                Language::scan_type_keyword,
                TokenKind::TypeKeyword,
                "TypeKeyword",
            ),
            ProductionKind::UncheckedKeyword => call_scanner(
                self,
                input,
                Language::scan_unchecked_keyword,
                TokenKind::UncheckedKeyword,
                "UncheckedKeyword",
            ),
            ProductionKind::UnicodeEscape => call_scanner(
                self,
                input,
                Language::scan_unicode_escape,
                TokenKind::UnicodeEscape,
                "UnicodeEscape",
            ),
            ProductionKind::UnicodeStringLiteral => call_scanner(
                self,
                input,
                Language::scan_unicode_string_literal,
                TokenKind::UnicodeStringLiteral,
                "UnicodeStringLiteral",
            ),
            ProductionKind::UnsignedFixedType => call_scanner(
                self,
                input,
                Language::scan_unsigned_fixed_type,
                TokenKind::UnsignedFixedType,
                "UnsignedFixedType",
            ),
            ProductionKind::UnsignedIntegerType => call_scanner(
                self,
                input,
                Language::scan_unsigned_integer_type,
                TokenKind::UnsignedIntegerType,
                "UnsignedIntegerType",
            ),
            ProductionKind::UsingKeyword => call_scanner(
                self,
                input,
                Language::scan_using_keyword,
                TokenKind::UsingKeyword,
                "UsingKeyword",
            ),
            ProductionKind::VersionPragmaValue => call_scanner(
                self,
                input,
                Language::scan_version_pragma_value,
                TokenKind::VersionPragmaValue,
                "VersionPragmaValue",
            ),
            ProductionKind::ViewKeyword => call_scanner(
                self,
                input,
                Language::scan_view_keyword,
                TokenKind::ViewKeyword,
                "ViewKeyword",
            ),
            ProductionKind::VirtualKeyword => call_scanner(
                self,
                input,
                Language::scan_virtual_keyword,
                TokenKind::VirtualKeyword,
                "VirtualKeyword",
            ),
            ProductionKind::WeeksKeyword => call_scanner(
                self,
                input,
                Language::scan_weeks_keyword,
                TokenKind::WeeksKeyword,
                "WeeksKeyword",
            ),
            ProductionKind::WeiKeyword => call_scanner(
                self,
                input,
                Language::scan_wei_keyword,
                TokenKind::WeiKeyword,
                "WeiKeyword",
            ),
            ProductionKind::WhileKeyword => call_scanner(
                self,
                input,
                Language::scan_while_keyword,
                TokenKind::WhileKeyword,
                "WhileKeyword",
            ),
            ProductionKind::Whitespace => call_scanner(
                self,
                input,
                Language::scan_whitespace,
                TokenKind::Whitespace,
                "Whitespace",
            ),
            ProductionKind::YearsKeyword => call_scanner(
                self,
                input,
                Language::scan_years_keyword,
                TokenKind::YearsKeyword,
                "YearsKeyword",
            ),
            ProductionKind::YulDecimalLiteral => call_scanner(
                self,
                input,
                Language::scan_yul_decimal_literal,
                TokenKind::YulDecimalLiteral,
                "YulDecimalLiteral",
            ),
            ProductionKind::YulHexLiteral => call_scanner(
                self,
                input,
                Language::scan_yul_hex_literal,
                TokenKind::YulHexLiteral,
                "YulHexLiteral",
            ),
            ProductionKind::YulIdentifier => call_scanner(
                self,
                input,
                Language::scan_yul_identifier,
                TokenKind::YulIdentifier,
                "YulIdentifier",
            ),
            ProductionKind::YulKeyword => call_scanner(
                self,
                input,
                Language::scan_yul_keyword,
                TokenKind::YulKeyword,
                "YulKeyword",
            ),
            ProductionKind::YulReservedKeyword => call_scanner(
                self,
                input,
                Language::scan_yul_reserved_keyword,
                TokenKind::YulReservedKeyword,
                "YulReservedKeyword",
            ),
            ProductionKind::ABICoderPragma => {
                call_parser(self, input, Language::parse_abi_coder_pragma)
            }
            ProductionKind::AddressType => call_parser(self, input, Language::parse_address_type),
            ProductionKind::ArgumentList => call_parser(self, input, Language::parse_argument_list),
            ProductionKind::ArrayLiteral => call_parser(self, input, Language::parse_array_literal),
            ProductionKind::AssemblyFlags => {
                call_parser(self, input, Language::parse_assembly_flags)
            }
            ProductionKind::AssemblyStatement => {
                call_parser(self, input, Language::parse_assembly_statement)
            }
            ProductionKind::Block => call_parser(self, input, Language::parse_block),
            ProductionKind::BooleanLiteral => {
                call_parser(self, input, Language::parse_boolean_literal)
            }
            ProductionKind::BreakStatement => {
                call_parser(self, input, Language::parse_break_statement)
            }
            ProductionKind::CatchClause => call_parser(self, input, Language::parse_catch_clause),
            ProductionKind::ConstantDefinition => {
                call_parser(self, input, Language::parse_constant_definition)
            }
            ProductionKind::ConstructorAttribute => {
                call_parser(self, input, Language::parse_constructor_attribute)
            }
            ProductionKind::ConstructorDefinition => {
                try_call_parser(self, input, Language::maybe_parse_constructor_definition)
            }
            ProductionKind::ContinueStatement => {
                call_parser(self, input, Language::parse_continue_statement)
            }
            ProductionKind::ContractBodyElement => {
                call_parser(self, input, Language::parse_contract_body_element)
            }
            ProductionKind::ContractDefinition => {
                call_parser(self, input, Language::parse_contract_definition)
            }
            ProductionKind::DataLocation => call_parser(self, input, Language::parse_data_location),
            ProductionKind::Definition => call_parser(self, input, Language::parse_definition),
            ProductionKind::DeleteStatement => {
                call_parser(self, input, Language::parse_delete_statement)
            }
            ProductionKind::Directive => call_parser(self, input, Language::parse_directive),
            ProductionKind::DoWhileStatement => {
                call_parser(self, input, Language::parse_do_while_statement)
            }
            ProductionKind::ElementaryType => {
                call_parser(self, input, Language::parse_elementary_type)
            }
            ProductionKind::EmitStatement => {
                call_parser(self, input, Language::parse_emit_statement)
            }
            ProductionKind::EndOfFileTrivia => {
                call_parser(self, input, Language::parse_end_of_file_trivia)
            }
            ProductionKind::EnumDefinition => {
                call_parser(self, input, Language::parse_enum_definition)
            }
            ProductionKind::ErrorDefinition => {
                call_parser(self, input, Language::parse_error_definition)
            }
            ProductionKind::ErrorParameter => {
                call_parser(self, input, Language::parse_error_parameter)
            }
            ProductionKind::EventDefinition => {
                call_parser(self, input, Language::parse_event_definition)
            }
            ProductionKind::EventParameter => {
                call_parser(self, input, Language::parse_event_parameter)
            }
            ProductionKind::ExperimentalPragma => {
                call_parser(self, input, Language::parse_experimental_pragma)
            }
            ProductionKind::Expression => call_parser(self, input, Language::parse_expression),
            ProductionKind::ExpressionStatement => {
                call_parser(self, input, Language::parse_expression_statement)
            }
            ProductionKind::FallbackFunctionAttribute => {
                call_parser(self, input, Language::parse_fallback_function_attribute)
            }
            ProductionKind::FallbackFunctionDefinition => try_call_parser(
                self,
                input,
                Language::maybe_parse_fallback_function_definition,
            ),
            ProductionKind::ForStatement => call_parser(self, input, Language::parse_for_statement),
            ProductionKind::FunctionAttribute => {
                call_parser(self, input, Language::parse_function_attribute)
            }
            ProductionKind::FunctionDefinition => {
                call_parser(self, input, Language::parse_function_definition)
            }
            ProductionKind::FunctionType => call_parser(self, input, Language::parse_function_type),
            ProductionKind::IdentifierPath => {
                call_parser(self, input, Language::parse_identifier_path)
            }
            ProductionKind::IfStatement => call_parser(self, input, Language::parse_if_statement),
            ProductionKind::ImportDirective => {
                call_parser(self, input, Language::parse_import_directive)
            }
            ProductionKind::ImportPath => call_parser(self, input, Language::parse_import_path),
            ProductionKind::InheritanceSpecifier => {
                call_parser(self, input, Language::parse_inheritance_specifier)
            }
            ProductionKind::InheritanceSpecifierList => {
                call_parser(self, input, Language::parse_inheritance_specifier_list)
            }
            ProductionKind::InterfaceDefinition => {
                call_parser(self, input, Language::parse_interface_definition)
            }
            ProductionKind::LeadingTrivia => {
                call_parser(self, input, Language::parse_leading_trivia)
            }
            ProductionKind::LibraryDefinition => {
                call_parser(self, input, Language::parse_library_definition)
            }
            ProductionKind::MappingKeyType => {
                call_parser(self, input, Language::parse_mapping_key_type)
            }
            ProductionKind::MappingType => call_parser(self, input, Language::parse_mapping_type),
            ProductionKind::MappingValueType => {
                call_parser(self, input, Language::parse_mapping_value_type)
            }
            ProductionKind::ModifierAttribute => {
                call_parser(self, input, Language::parse_modifier_attribute)
            }
            ProductionKind::ModifierDefinition => {
                call_parser(self, input, Language::parse_modifier_definition)
            }
            ProductionKind::ModifierInvocation => {
                call_parser(self, input, Language::parse_modifier_invocation)
            }
            ProductionKind::NamedArgument => {
                call_parser(self, input, Language::parse_named_argument)
            }
            ProductionKind::NamedArgumentList => {
                call_parser(self, input, Language::parse_named_argument_list)
            }
            ProductionKind::NewExpression => {
                call_parser(self, input, Language::parse_new_expression)
            }
            ProductionKind::NumberUnit => call_parser(self, input, Language::parse_number_unit),
            ProductionKind::NumericLiteral => {
                call_parser(self, input, Language::parse_numeric_literal)
            }
            ProductionKind::OverrideSpecifier => {
                call_parser(self, input, Language::parse_override_specifier)
            }
            ProductionKind::ParameterDeclaration => {
                call_parser(self, input, Language::parse_parameter_declaration)
            }
            ProductionKind::ParameterList => {
                call_parser(self, input, Language::parse_parameter_list)
            }
            ProductionKind::PayableType => call_parser(self, input, Language::parse_payable_type),
            ProductionKind::PositionalArgumentList => {
                call_parser(self, input, Language::parse_positional_argument_list)
            }
            ProductionKind::PragmaDirective => {
                call_parser(self, input, Language::parse_pragma_directive)
            }
            ProductionKind::PrimaryExpression => {
                call_parser(self, input, Language::parse_primary_expression)
            }
            ProductionKind::ReceiveFunctionAttribute => {
                call_parser(self, input, Language::parse_receive_function_attribute)
            }
            ProductionKind::ReceiveFunctionDefinition => try_call_parser(
                self,
                input,
                Language::maybe_parse_receive_function_definition,
            ),
            ProductionKind::ReturnStatement => {
                call_parser(self, input, Language::parse_return_statement)
            }
            ProductionKind::RevertStatement => {
                call_parser(self, input, Language::parse_revert_statement)
            }
            ProductionKind::SelectedImport => {
                call_parser(self, input, Language::parse_selected_import)
            }
            ProductionKind::SelectingImportDirective => {
                call_parser(self, input, Language::parse_selecting_import_directive)
            }
            ProductionKind::SimpleImportDirective => {
                call_parser(self, input, Language::parse_simple_import_directive)
            }
            ProductionKind::SimpleStatement => {
                call_parser(self, input, Language::parse_simple_statement)
            }
            ProductionKind::SourceUnit => call_parser(self, input, Language::parse_source_unit),
            ProductionKind::StarImportDirective => {
                call_parser(self, input, Language::parse_star_import_directive)
            }
            ProductionKind::StateVariableAttribute => {
                call_parser(self, input, Language::parse_state_variable_attribute)
            }
            ProductionKind::StateVariableDeclaration => {
                call_parser(self, input, Language::parse_state_variable_declaration)
            }
            ProductionKind::Statement => call_parser(self, input, Language::parse_statement),
            ProductionKind::StringExpression => {
                call_parser(self, input, Language::parse_string_expression)
            }
            ProductionKind::StructDefinition => {
                call_parser(self, input, Language::parse_struct_definition)
            }
            ProductionKind::StructMember => call_parser(self, input, Language::parse_struct_member),
            ProductionKind::TrailingTrivia => {
                call_parser(self, input, Language::parse_trailing_trivia)
            }
            ProductionKind::TryStatement => call_parser(self, input, Language::parse_try_statement),
            ProductionKind::TupleDeconstructionStatement => {
                call_parser(self, input, Language::parse_tuple_deconstruction_statement)
            }
            ProductionKind::TupleExpression => {
                call_parser(self, input, Language::parse_tuple_expression)
            }
            ProductionKind::TypeExpression => {
                call_parser(self, input, Language::parse_type_expression)
            }
            ProductionKind::TypeName => call_parser(self, input, Language::parse_type_name),
            ProductionKind::UncheckedBlock => {
                call_parser(self, input, Language::parse_unchecked_block)
            }
            ProductionKind::UnnamedFunctionDefinition => try_call_parser(
                self,
                input,
                Language::maybe_parse_unnamed_function_definition,
            ),
            ProductionKind::UserDefinedValueTypeDefinition => call_parser(
                self,
                input,
                Language::parse_user_defined_value_type_definition,
            ),
            ProductionKind::UsingDirective => {
                call_parser(self, input, Language::parse_using_directive)
            }
            ProductionKind::VariableDeclarationStatement => {
                call_parser(self, input, Language::parse_variable_declaration_statement)
            }
            ProductionKind::VersionPragma => {
                call_parser(self, input, Language::parse_version_pragma)
            }
            ProductionKind::VersionPragmaOperator => {
                call_parser(self, input, Language::parse_version_pragma_operator)
            }
            ProductionKind::VersionPragmaSpecifier => {
                call_parser(self, input, Language::parse_version_pragma_specifier)
            }
            ProductionKind::WhileStatement => {
                call_parser(self, input, Language::parse_while_statement)
            }
            ProductionKind::YulAssignmentStatement => {
                call_parser(self, input, Language::parse_yul_assignment_statement)
            }
            ProductionKind::YulBlock => call_parser(self, input, Language::parse_yul_block),
            ProductionKind::YulBreakStatement => {
                call_parser(self, input, Language::parse_yul_break_statement)
            }
            ProductionKind::YulContinueStatement => {
                call_parser(self, input, Language::parse_yul_continue_statement)
            }
            ProductionKind::YulExpression => {
                call_parser(self, input, Language::parse_yul_expression)
            }
            ProductionKind::YulForStatement => {
                call_parser(self, input, Language::parse_yul_for_statement)
            }
            ProductionKind::YulFunctionCall => {
                call_parser(self, input, Language::parse_yul_function_call)
            }
            ProductionKind::YulFunctionDefinition => {
                call_parser(self, input, Language::parse_yul_function_definition)
            }
            ProductionKind::YulIdentifierPath => {
                call_parser(self, input, Language::parse_yul_identifier_path)
            }
            ProductionKind::YulIfStatement => {
                call_parser(self, input, Language::parse_yul_if_statement)
            }
            ProductionKind::YulLeaveStatement => {
                call_parser(self, input, Language::parse_yul_leave_statement)
            }
            ProductionKind::YulLiteral => call_parser(self, input, Language::parse_yul_literal),
            ProductionKind::YulStatement => call_parser(self, input, Language::parse_yul_statement),
            ProductionKind::YulSwitchStatement => {
                call_parser(self, input, Language::parse_yul_switch_statement)
            }
            ProductionKind::YulVariableDeclaration => {
                call_parser(self, input, Language::parse_yul_variable_declaration)
            }
        }
        .expect(&format!(
            "Production {production_kind:?} is not valid in this version of Solidity"
        ))
    }
}

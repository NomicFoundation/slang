// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use std::fmt::Display;
pub use std::{collections::BTreeSet, ops::Range, rc::Rc};

#[allow(deprecated, unused_imports)]
use semver::Version;
use serde::Serialize;

pub use super::{
    cst,
    kinds::*,
    parser_output::{ParseError, ParseOutput},
};

const DEBUG_ERROR_MERGING: bool = false;

impl ParseError {
    pub(crate) fn new<T: Into<String>>(position: TextPosition, expected: T) -> Self {
        Self {
            position,
            expected: BTreeSet::from([expected.into()]),
        }
    }

    pub(crate) fn merge_with(&mut self, other: Self) {
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

    #[allow(dead_code)]
    pub(crate) fn maybe_merge_with(mut self, other: Option<Self>) -> Self {
        if let Some(other) = other {
            self.merge_with(other)
        }
        self
    }
}

#[allow(dead_code)]
pub(crate) enum ParserResult {
    Pass {
        builder: cst::NodeBuilder,
        error: Option<ParseError>,
    },
    Fail {
        error: ParseError,
    },
}

#[allow(dead_code)]
impl ParserResult {
    pub(crate) fn with_kind(self, kind: RuleKind) -> Self {
        match self {
            Self::Pass { builder, error } => Self::Pass {
                builder: builder.with_kind(kind),
                error,
            },
            fail => fail,
        }
    }
}

#[derive(Default, Copy, Clone, PartialEq, Eq, Debug, Serialize)]
pub struct TextPosition {
    pub byte: usize,
    pub char: usize,
}

pub type TextRange = Range<TextPosition>;

impl PartialOrd for TextPosition {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.char.partial_cmp(&other.char)
    }
}

impl Ord for TextPosition {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.byte.cmp(&other.byte)
    }
}

impl Display for TextPosition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.char.fmt(f)
    }
}

pub struct Stream<'s> {
    source: &'s str,
    position: TextPosition,
    undo_position: TextPosition,
    has_undo: bool,
}

impl<'s> Stream<'s> {
    pub fn new(source: &'s str) -> Self {
        Self {
            source,
            position: Default::default(),
            undo_position: Default::default(),
            has_undo: false,
        }
    }

    pub fn position(&self) -> TextPosition {
        self.position
    }

    pub fn set_position(&mut self, position: TextPosition) {
        self.position = position;
    }

    pub fn peek(&self) -> Option<char> {
        self.source[self.position.byte..].chars().next()
    }

    pub fn next(&mut self) -> Option<char> {
        self.has_undo = true;
        self.undo_position = self.position;
        let mut chars = self.source[self.position.byte..].chars();
        if let Some(c) = chars.next() {
            self.position.byte += c.len_utf8();
            self.position.char += 1;
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
            format!("{position}: {message}", position = source_start.char)
        } else {
            message
        }
    };

    if source.is_empty() {
        return format!("{kind}: {message}\n   ─[{source_id}:0:0]");
    }

    let mut builder = Report::build(kind, source_id, source_start.byte)
        .with_config(Config::default().with_color(with_color))
        .with_message(message);

    builder.add_label(
        Label::new((source_id, source_start.char..source_end.char))
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
fn call_scanner<L, F>(language: &L, input: &str, scanner: F, kind: TokenKind) -> Option<ParseOutput>
where
    F: Fn(&L, &mut Stream) -> bool,
{
    let mut stream = Stream::new(input);
    Some(
        if scanner(language, &mut stream) && stream.peek().is_none() {
            let token = cst::Node::token(
                kind,
                Range {
                    start: Default::default(),
                    end: stream.position(),
                },
                None,
                None,
            );

            ParseOutput {
                parse_tree: Some(token),
                errors: vec![],
            }
        } else {
            ParseOutput {
                parse_tree: None,
                errors: vec![ParseError::new(stream.position(), kind.as_ref())],
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
) -> Option<ParseOutput>
where
    F: Fn(&L, &mut Stream) -> Option<bool>,
{
    let mut stream = Stream::new(input);
    scanner(language, &mut stream).map(|result| {
        if result && stream.peek().is_none() {
            let token = cst::Node::token(
                kind,
                Range {
                    start: Default::default(),
                    end: stream.position(),
                },
                None,
                None,
            );

            ParseOutput {
                parse_tree: Some(token),
                errors: vec![],
            }
        } else {
            ParseOutput {
                parse_tree: None,
                errors: vec![ParseError::new(stream.position(), kind.as_ref())],
            }
        }
    })
}

#[allow(dead_code)]
fn call_parser<L, F>(language: &L, input: &str, parser: F) -> Option<ParseOutput>
where
    F: Fn(&L, &mut Stream) -> ParserResult,
{
    let mut stream = Stream::new(input);
    Some(match parser(language, &mut stream) {
        ParserResult::Pass { builder, .. } if stream.peek().is_none() => ParseOutput {
            parse_tree: Some(builder.build()),
            errors: vec![],
        },
        ParserResult::Pass { .. } => ParseOutput {
            parse_tree: None,
            errors: vec![ParseError::new(stream.position(), "end of input")],
        },
        ParserResult::Fail { error } => ParseOutput {
            parse_tree: None,
            errors: vec![error],
        },
    })
}

#[allow(dead_code)]
fn try_call_parser<L, F>(language: &L, input: &str, parser: F) -> Option<ParseOutput>
where
    F: Fn(&L, &mut Stream) -> Option<ParserResult>,
{
    let mut stream = Stream::new(input);
    parser(language, &mut stream).map(|result| match result {
        ParserResult::Pass { builder, .. } if stream.peek().is_none() => ParseOutput {
            parse_tree: Some(builder.build()),
            errors: vec![],
        },
        ParserResult::Pass { .. } => ParseOutput {
            parse_tree: None,
            errors: vec![ParseError::new(stream.position(), "end of input")],
        },
        ParserResult::Fail { error } => ParseOutput {
            parse_tree: None,
            errors: vec![error],
        },
    })
}

#[napi]
pub struct Language {
    pub(crate) version: Version,
    pub(crate) version_is_equal_to_or_greater_than_0_4_21: bool,
    pub(crate) version_is_equal_to_or_greater_than_0_4_22: bool,
    pub(crate) version_is_equal_to_or_greater_than_0_5_0: bool,
    pub(crate) version_is_equal_to_or_greater_than_0_5_3: bool,
    pub(crate) version_is_equal_to_or_greater_than_0_6_0: bool,
    pub(crate) version_is_equal_to_or_greater_than_0_6_11: bool,
    pub(crate) version_is_equal_to_or_greater_than_0_6_2: bool,
    pub(crate) version_is_equal_to_or_greater_than_0_6_5: bool,
    pub(crate) version_is_equal_to_or_greater_than_0_7_0: bool,
    pub(crate) version_is_equal_to_or_greater_than_0_8_0: bool,
    pub(crate) version_is_equal_to_or_greater_than_0_8_18: bool,
    pub(crate) version_is_equal_to_or_greater_than_0_8_19: bool,
    pub(crate) version_is_equal_to_or_greater_than_0_8_8: bool,
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
    // Shared with Rust
    #[error("Unsupported Solidity language version '{0}'.")]
    UnsupportedLanguageVersion(Version),
    #[error("Production '{0:?}' is not valid in this version of Solidity.")]
    InvalidProductionVersion(ProductionKind),

    // TypeScript-specific
    #[error("Invalid semantic version '{0}'.")]
    InvalidSemanticVersion(String),
}

impl From<Error> for napi::Error {
    fn from(value: Error) -> Self {
        napi::Error::from_reason(value.to_string())
    }
}

const VERSIONS: &'static [&'static str] = &[
    "0.4.11", "0.4.12", "0.4.13", "0.4.14", "0.4.15", "0.4.16", "0.4.17", "0.4.18", "0.4.19",
    "0.4.20", "0.4.21", "0.4.22", "0.4.23", "0.4.24", "0.4.25", "0.4.26", "0.5.0", "0.5.1",
    "0.5.2", "0.5.3", "0.5.4", "0.5.5", "0.5.6", "0.5.7", "0.5.8", "0.5.9", "0.5.10", "0.5.11",
    "0.5.12", "0.5.13", "0.5.14", "0.5.15", "0.5.16", "0.5.17", "0.6.0", "0.6.1", "0.6.2", "0.6.3",
    "0.6.4", "0.6.5", "0.6.6", "0.6.7", "0.6.8", "0.6.9", "0.6.10", "0.6.11", "0.6.12", "0.7.0",
    "0.7.1", "0.7.2", "0.7.3", "0.7.4", "0.7.5", "0.7.6", "0.8.0", "0.8.1", "0.8.2", "0.8.3",
    "0.8.4", "0.8.5", "0.8.6", "0.8.7", "0.8.8", "0.8.9", "0.8.10", "0.8.11", "0.8.12", "0.8.13",
    "0.8.14", "0.8.15", "0.8.16", "0.8.17", "0.8.18", "0.8.19",
];

#[napi]
impl Language {
    #[napi(constructor)]
    pub fn new(version: String) -> Result<Self, napi::Error> {
        let version =
            Version::parse(&version).map_err(|_| Error::InvalidSemanticVersion(version))?;
        if VERSIONS.contains(&version.to_string().as_str()) {
            Ok(Self {
                version_is_equal_to_or_greater_than_0_4_21: Version::parse("0.4.21").unwrap()
                    <= version,
                version_is_equal_to_or_greater_than_0_4_22: Version::parse("0.4.22").unwrap()
                    <= version,
                version_is_equal_to_or_greater_than_0_5_0: Version::parse("0.5.0").unwrap()
                    <= version,
                version_is_equal_to_or_greater_than_0_5_3: Version::parse("0.5.3").unwrap()
                    <= version,
                version_is_equal_to_or_greater_than_0_6_0: Version::parse("0.6.0").unwrap()
                    <= version,
                version_is_equal_to_or_greater_than_0_6_11: Version::parse("0.6.11").unwrap()
                    <= version,
                version_is_equal_to_or_greater_than_0_6_2: Version::parse("0.6.2").unwrap()
                    <= version,
                version_is_equal_to_or_greater_than_0_6_5: Version::parse("0.6.5").unwrap()
                    <= version,
                version_is_equal_to_or_greater_than_0_7_0: Version::parse("0.7.0").unwrap()
                    <= version,
                version_is_equal_to_or_greater_than_0_8_0: Version::parse("0.8.0").unwrap()
                    <= version,
                version_is_equal_to_or_greater_than_0_8_18: Version::parse("0.8.18").unwrap()
                    <= version,
                version_is_equal_to_or_greater_than_0_8_19: Version::parse("0.8.19").unwrap()
                    <= version,
                version_is_equal_to_or_greater_than_0_8_8: Version::parse("0.8.8").unwrap()
                    <= version,
                version,
            })
        } else {
            Err(Error::UnsupportedLanguageVersion(version).into())
        }
    }

    #[napi(getter)]
    pub fn version(&self) -> String {
        self.version.to_string()
    }

    #[napi]
    pub fn supported_versions() -> Vec<String> {
        return VERSIONS.iter().map(|v| v.to_string()).collect();
    }

    #[napi]
    pub fn parse(
        &self,
        production_kind: ProductionKind,
        input: String,
    ) -> Result<ParseOutput, napi::Error> {
        let input = input.as_str();
        match production_kind {
            ProductionKind::AbicoderKeyword => call_scanner(
                self,
                input,
                Language::scan_abicoder_keyword,
                TokenKind::AbicoderKeyword,
            ),
            ProductionKind::AbstractKeyword => try_call_scanner(
                self,
                input,
                Language::maybe_scan_abstract_keyword,
                TokenKind::AbstractKeyword,
            ),
            ProductionKind::AddressKeyword => call_scanner(
                self,
                input,
                Language::scan_address_keyword,
                TokenKind::AddressKeyword,
            ),
            ProductionKind::Ampersand => {
                call_scanner(self, input, Language::scan_ampersand, TokenKind::Ampersand)
            }
            ProductionKind::AmpersandAmpersand => call_scanner(
                self,
                input,
                Language::scan_ampersand_ampersand,
                TokenKind::AmpersandAmpersand,
            ),
            ProductionKind::AmpersandEqual => call_scanner(
                self,
                input,
                Language::scan_ampersand_equal,
                TokenKind::AmpersandEqual,
            ),
            ProductionKind::AnonymousKeyword => call_scanner(
                self,
                input,
                Language::scan_anonymous_keyword,
                TokenKind::AnonymousKeyword,
            ),
            ProductionKind::AsKeyword => {
                call_scanner(self, input, Language::scan_as_keyword, TokenKind::AsKeyword)
            }
            ProductionKind::AsciiStringLiteral => call_scanner(
                self,
                input,
                Language::scan_ascii_string_literal,
                TokenKind::AsciiStringLiteral,
            ),
            ProductionKind::AssemblyKeyword => call_scanner(
                self,
                input,
                Language::scan_assembly_keyword,
                TokenKind::AssemblyKeyword,
            ),
            ProductionKind::Asterisk => {
                call_scanner(self, input, Language::scan_asterisk, TokenKind::Asterisk)
            }
            ProductionKind::AsteriskAsterisk => call_scanner(
                self,
                input,
                Language::scan_asterisk_asterisk,
                TokenKind::AsteriskAsterisk,
            ),
            ProductionKind::AsteriskEqual => call_scanner(
                self,
                input,
                Language::scan_asterisk_equal,
                TokenKind::AsteriskEqual,
            ),
            ProductionKind::Bang => call_scanner(self, input, Language::scan_bang, TokenKind::Bang),
            ProductionKind::BangEqual => {
                call_scanner(self, input, Language::scan_bang_equal, TokenKind::BangEqual)
            }
            ProductionKind::Bar => call_scanner(self, input, Language::scan_bar, TokenKind::Bar),
            ProductionKind::BarBar => {
                call_scanner(self, input, Language::scan_bar_bar, TokenKind::BarBar)
            }
            ProductionKind::BarEqual => {
                call_scanner(self, input, Language::scan_bar_equal, TokenKind::BarEqual)
            }
            ProductionKind::BoolKeyword => call_scanner(
                self,
                input,
                Language::scan_bool_keyword,
                TokenKind::BoolKeyword,
            ),
            ProductionKind::BreakKeyword => call_scanner(
                self,
                input,
                Language::scan_break_keyword,
                TokenKind::BreakKeyword,
            ),
            ProductionKind::ByteKeyword => try_call_scanner(
                self,
                input,
                Language::maybe_scan_byte_keyword,
                TokenKind::ByteKeyword,
            ),
            ProductionKind::CalldataKeyword => try_call_scanner(
                self,
                input,
                Language::maybe_scan_calldata_keyword,
                TokenKind::CalldataKeyword,
            ),
            ProductionKind::Caret => {
                call_scanner(self, input, Language::scan_caret, TokenKind::Caret)
            }
            ProductionKind::CaretEqual => call_scanner(
                self,
                input,
                Language::scan_caret_equal,
                TokenKind::CaretEqual,
            ),
            ProductionKind::CaseKeyword => call_scanner(
                self,
                input,
                Language::scan_case_keyword,
                TokenKind::CaseKeyword,
            ),
            ProductionKind::CatchKeyword => try_call_scanner(
                self,
                input,
                Language::maybe_scan_catch_keyword,
                TokenKind::CatchKeyword,
            ),
            ProductionKind::CloseBrace => call_scanner(
                self,
                input,
                Language::scan_close_brace,
                TokenKind::CloseBrace,
            ),
            ProductionKind::CloseBracket => call_scanner(
                self,
                input,
                Language::scan_close_bracket,
                TokenKind::CloseBracket,
            ),
            ProductionKind::CloseParen => call_scanner(
                self,
                input,
                Language::scan_close_paren,
                TokenKind::CloseParen,
            ),
            ProductionKind::Colon => {
                call_scanner(self, input, Language::scan_colon, TokenKind::Colon)
            }
            ProductionKind::ColonEqual => call_scanner(
                self,
                input,
                Language::scan_colon_equal,
                TokenKind::ColonEqual,
            ),
            ProductionKind::Comma => {
                call_scanner(self, input, Language::scan_comma, TokenKind::Comma)
            }
            ProductionKind::ConstantKeyword => call_scanner(
                self,
                input,
                Language::scan_constant_keyword,
                TokenKind::ConstantKeyword,
            ),
            ProductionKind::ConstructorKeyword => try_call_scanner(
                self,
                input,
                Language::maybe_scan_constructor_keyword,
                TokenKind::ConstructorKeyword,
            ),
            ProductionKind::ContinueKeyword => call_scanner(
                self,
                input,
                Language::scan_continue_keyword,
                TokenKind::ContinueKeyword,
            ),
            ProductionKind::ContractKeyword => call_scanner(
                self,
                input,
                Language::scan_contract_keyword,
                TokenKind::ContractKeyword,
            ),
            ProductionKind::DaysKeyword => call_scanner(
                self,
                input,
                Language::scan_days_keyword,
                TokenKind::DaysKeyword,
            ),
            ProductionKind::DecimalLiteral => call_scanner(
                self,
                input,
                Language::scan_decimal_literal,
                TokenKind::DecimalLiteral,
            ),
            ProductionKind::DefaultKeyword => call_scanner(
                self,
                input,
                Language::scan_default_keyword,
                TokenKind::DefaultKeyword,
            ),
            ProductionKind::DeleteKeyword => call_scanner(
                self,
                input,
                Language::scan_delete_keyword,
                TokenKind::DeleteKeyword,
            ),
            ProductionKind::DoKeyword => {
                call_scanner(self, input, Language::scan_do_keyword, TokenKind::DoKeyword)
            }
            ProductionKind::ElseKeyword => call_scanner(
                self,
                input,
                Language::scan_else_keyword,
                TokenKind::ElseKeyword,
            ),
            ProductionKind::EmitKeyword => try_call_scanner(
                self,
                input,
                Language::maybe_scan_emit_keyword,
                TokenKind::EmitKeyword,
            ),
            ProductionKind::EndOfLine => call_scanner(
                self,
                input,
                Language::scan_end_of_line,
                TokenKind::EndOfLine,
            ),
            ProductionKind::EnumKeyword => call_scanner(
                self,
                input,
                Language::scan_enum_keyword,
                TokenKind::EnumKeyword,
            ),
            ProductionKind::Equal => {
                call_scanner(self, input, Language::scan_equal, TokenKind::Equal)
            }
            ProductionKind::EqualEqual => call_scanner(
                self,
                input,
                Language::scan_equal_equal,
                TokenKind::EqualEqual,
            ),
            ProductionKind::EqualGreaterThan => call_scanner(
                self,
                input,
                Language::scan_equal_greater_than,
                TokenKind::EqualGreaterThan,
            ),
            ProductionKind::ErrorKeyword => call_scanner(
                self,
                input,
                Language::scan_error_keyword,
                TokenKind::ErrorKeyword,
            ),
            ProductionKind::EtherKeyword => call_scanner(
                self,
                input,
                Language::scan_ether_keyword,
                TokenKind::EtherKeyword,
            ),
            ProductionKind::EventKeyword => call_scanner(
                self,
                input,
                Language::scan_event_keyword,
                TokenKind::EventKeyword,
            ),
            ProductionKind::Evmasm => {
                call_scanner(self, input, Language::scan_evmasm, TokenKind::Evmasm)
            }
            ProductionKind::ExperimentalKeyword => call_scanner(
                self,
                input,
                Language::scan_experimental_keyword,
                TokenKind::ExperimentalKeyword,
            ),
            ProductionKind::ExternalKeyword => call_scanner(
                self,
                input,
                Language::scan_external_keyword,
                TokenKind::ExternalKeyword,
            ),
            ProductionKind::FallbackKeyword => call_scanner(
                self,
                input,
                Language::scan_fallback_keyword,
                TokenKind::FallbackKeyword,
            ),
            ProductionKind::FalseKeyword => call_scanner(
                self,
                input,
                Language::scan_false_keyword,
                TokenKind::FalseKeyword,
            ),
            ProductionKind::FinneyKeyword => try_call_scanner(
                self,
                input,
                Language::maybe_scan_finney_keyword,
                TokenKind::FinneyKeyword,
            ),
            ProductionKind::FixedBytesType => call_scanner(
                self,
                input,
                Language::scan_fixed_bytes_type,
                TokenKind::FixedBytesType,
            ),
            ProductionKind::ForKeyword => call_scanner(
                self,
                input,
                Language::scan_for_keyword,
                TokenKind::ForKeyword,
            ),
            ProductionKind::FromKeyword => call_scanner(
                self,
                input,
                Language::scan_from_keyword,
                TokenKind::FromKeyword,
            ),
            ProductionKind::FunctionKeyword => call_scanner(
                self,
                input,
                Language::scan_function_keyword,
                TokenKind::FunctionKeyword,
            ),
            ProductionKind::GlobalKeyword => call_scanner(
                self,
                input,
                Language::scan_global_keyword,
                TokenKind::GlobalKeyword,
            ),
            ProductionKind::GreaterThan => call_scanner(
                self,
                input,
                Language::scan_greater_than,
                TokenKind::GreaterThan,
            ),
            ProductionKind::GreaterThanEqual => call_scanner(
                self,
                input,
                Language::scan_greater_than_equal,
                TokenKind::GreaterThanEqual,
            ),
            ProductionKind::GreaterThanGreaterThan => call_scanner(
                self,
                input,
                Language::scan_greater_than_greater_than,
                TokenKind::GreaterThanGreaterThan,
            ),
            ProductionKind::GreaterThanGreaterThanEqual => call_scanner(
                self,
                input,
                Language::scan_greater_than_greater_than_equal,
                TokenKind::GreaterThanGreaterThanEqual,
            ),
            ProductionKind::GreaterThanGreaterThanGreaterThan => call_scanner(
                self,
                input,
                Language::scan_greater_than_greater_than_greater_than,
                TokenKind::GreaterThanGreaterThanGreaterThan,
            ),
            ProductionKind::GreaterThanGreaterThanGreaterThanEqual => call_scanner(
                self,
                input,
                Language::scan_greater_than_greater_than_greater_than_equal,
                TokenKind::GreaterThanGreaterThanGreaterThanEqual,
            ),
            ProductionKind::GweiKeyword => try_call_scanner(
                self,
                input,
                Language::maybe_scan_gwei_keyword,
                TokenKind::GweiKeyword,
            ),
            ProductionKind::HexLiteral => call_scanner(
                self,
                input,
                Language::scan_hex_literal,
                TokenKind::HexLiteral,
            ),
            ProductionKind::HexStringLiteral => call_scanner(
                self,
                input,
                Language::scan_hex_string_literal,
                TokenKind::HexStringLiteral,
            ),
            ProductionKind::HoursKeyword => call_scanner(
                self,
                input,
                Language::scan_hours_keyword,
                TokenKind::HoursKeyword,
            ),
            ProductionKind::Identifier => call_scanner(
                self,
                input,
                Language::scan_identifier,
                TokenKind::Identifier,
            ),
            ProductionKind::IfKeyword => {
                call_scanner(self, input, Language::scan_if_keyword, TokenKind::IfKeyword)
            }
            ProductionKind::ImmutableKeyword => try_call_scanner(
                self,
                input,
                Language::maybe_scan_immutable_keyword,
                TokenKind::ImmutableKeyword,
            ),
            ProductionKind::ImportKeyword => call_scanner(
                self,
                input,
                Language::scan_import_keyword,
                TokenKind::ImportKeyword,
            ),
            ProductionKind::IndexedKeyword => call_scanner(
                self,
                input,
                Language::scan_indexed_keyword,
                TokenKind::IndexedKeyword,
            ),
            ProductionKind::InterfaceKeyword => call_scanner(
                self,
                input,
                Language::scan_interface_keyword,
                TokenKind::InterfaceKeyword,
            ),
            ProductionKind::InternalKeyword => call_scanner(
                self,
                input,
                Language::scan_internal_keyword,
                TokenKind::InternalKeyword,
            ),
            ProductionKind::IsKeyword => {
                call_scanner(self, input, Language::scan_is_keyword, TokenKind::IsKeyword)
            }
            ProductionKind::LeaveKeyword => try_call_scanner(
                self,
                input,
                Language::maybe_scan_leave_keyword,
                TokenKind::LeaveKeyword,
            ),
            ProductionKind::LessThan => {
                call_scanner(self, input, Language::scan_less_than, TokenKind::LessThan)
            }
            ProductionKind::LessThanEqual => call_scanner(
                self,
                input,
                Language::scan_less_than_equal,
                TokenKind::LessThanEqual,
            ),
            ProductionKind::LessThanLessThan => call_scanner(
                self,
                input,
                Language::scan_less_than_less_than,
                TokenKind::LessThanLessThan,
            ),
            ProductionKind::LessThanLessThanEqual => call_scanner(
                self,
                input,
                Language::scan_less_than_less_than_equal,
                TokenKind::LessThanLessThanEqual,
            ),
            ProductionKind::LetKeyword => call_scanner(
                self,
                input,
                Language::scan_let_keyword,
                TokenKind::LetKeyword,
            ),
            ProductionKind::LibraryKeyword => call_scanner(
                self,
                input,
                Language::scan_library_keyword,
                TokenKind::LibraryKeyword,
            ),
            ProductionKind::MappingKeyword => call_scanner(
                self,
                input,
                Language::scan_mapping_keyword,
                TokenKind::MappingKeyword,
            ),
            ProductionKind::MemoryKeyword => call_scanner(
                self,
                input,
                Language::scan_memory_keyword,
                TokenKind::MemoryKeyword,
            ),
            ProductionKind::Minus => {
                call_scanner(self, input, Language::scan_minus, TokenKind::Minus)
            }
            ProductionKind::MinusEqual => call_scanner(
                self,
                input,
                Language::scan_minus_equal,
                TokenKind::MinusEqual,
            ),
            ProductionKind::MinusGreaterThan => call_scanner(
                self,
                input,
                Language::scan_minus_greater_than,
                TokenKind::MinusGreaterThan,
            ),
            ProductionKind::MinusMinus => call_scanner(
                self,
                input,
                Language::scan_minus_minus,
                TokenKind::MinusMinus,
            ),
            ProductionKind::MinutesKeyword => call_scanner(
                self,
                input,
                Language::scan_minutes_keyword,
                TokenKind::MinutesKeyword,
            ),
            ProductionKind::ModifierKeyword => call_scanner(
                self,
                input,
                Language::scan_modifier_keyword,
                TokenKind::ModifierKeyword,
            ),
            ProductionKind::MultilineComment => call_scanner(
                self,
                input,
                Language::scan_multiline_comment,
                TokenKind::MultilineComment,
            ),
            ProductionKind::NewKeyword => call_scanner(
                self,
                input,
                Language::scan_new_keyword,
                TokenKind::NewKeyword,
            ),
            ProductionKind::OpenBrace => {
                call_scanner(self, input, Language::scan_open_brace, TokenKind::OpenBrace)
            }
            ProductionKind::OpenBracket => call_scanner(
                self,
                input,
                Language::scan_open_bracket,
                TokenKind::OpenBracket,
            ),
            ProductionKind::OpenParen => {
                call_scanner(self, input, Language::scan_open_paren, TokenKind::OpenParen)
            }
            ProductionKind::OverrideKeyword => call_scanner(
                self,
                input,
                Language::scan_override_keyword,
                TokenKind::OverrideKeyword,
            ),
            ProductionKind::PayableKeyword => call_scanner(
                self,
                input,
                Language::scan_payable_keyword,
                TokenKind::PayableKeyword,
            ),
            ProductionKind::Percent => {
                call_scanner(self, input, Language::scan_percent, TokenKind::Percent)
            }
            ProductionKind::PercentEqual => call_scanner(
                self,
                input,
                Language::scan_percent_equal,
                TokenKind::PercentEqual,
            ),
            ProductionKind::Period => {
                call_scanner(self, input, Language::scan_period, TokenKind::Period)
            }
            ProductionKind::Plus => call_scanner(self, input, Language::scan_plus, TokenKind::Plus),
            ProductionKind::PlusEqual => {
                call_scanner(self, input, Language::scan_plus_equal, TokenKind::PlusEqual)
            }
            ProductionKind::PlusPlus => {
                call_scanner(self, input, Language::scan_plus_plus, TokenKind::PlusPlus)
            }
            ProductionKind::PragmaKeyword => call_scanner(
                self,
                input,
                Language::scan_pragma_keyword,
                TokenKind::PragmaKeyword,
            ),
            ProductionKind::PrivateKeyword => call_scanner(
                self,
                input,
                Language::scan_private_keyword,
                TokenKind::PrivateKeyword,
            ),
            ProductionKind::PublicKeyword => call_scanner(
                self,
                input,
                Language::scan_public_keyword,
                TokenKind::PublicKeyword,
            ),
            ProductionKind::PureKeyword => call_scanner(
                self,
                input,
                Language::scan_pure_keyword,
                TokenKind::PureKeyword,
            ),
            ProductionKind::QuestionMark => call_scanner(
                self,
                input,
                Language::scan_question_mark,
                TokenKind::QuestionMark,
            ),
            ProductionKind::ReceiveKeyword => call_scanner(
                self,
                input,
                Language::scan_receive_keyword,
                TokenKind::ReceiveKeyword,
            ),
            ProductionKind::ReturnKeyword => call_scanner(
                self,
                input,
                Language::scan_return_keyword,
                TokenKind::ReturnKeyword,
            ),
            ProductionKind::ReturnsKeyword => call_scanner(
                self,
                input,
                Language::scan_returns_keyword,
                TokenKind::ReturnsKeyword,
            ),
            ProductionKind::RevertKeyword => call_scanner(
                self,
                input,
                Language::scan_revert_keyword,
                TokenKind::RevertKeyword,
            ),
            ProductionKind::SecondsKeyword => call_scanner(
                self,
                input,
                Language::scan_seconds_keyword,
                TokenKind::SecondsKeyword,
            ),
            ProductionKind::Semicolon => {
                call_scanner(self, input, Language::scan_semicolon, TokenKind::Semicolon)
            }
            ProductionKind::SignedFixedType => call_scanner(
                self,
                input,
                Language::scan_signed_fixed_type,
                TokenKind::SignedFixedType,
            ),
            ProductionKind::SignedIntegerType => call_scanner(
                self,
                input,
                Language::scan_signed_integer_type,
                TokenKind::SignedIntegerType,
            ),
            ProductionKind::SingleLineComment => call_scanner(
                self,
                input,
                Language::scan_single_line_comment,
                TokenKind::SingleLineComment,
            ),
            ProductionKind::Slash => {
                call_scanner(self, input, Language::scan_slash, TokenKind::Slash)
            }
            ProductionKind::SlashEqual => call_scanner(
                self,
                input,
                Language::scan_slash_equal,
                TokenKind::SlashEqual,
            ),
            ProductionKind::SolidityKeyword => call_scanner(
                self,
                input,
                Language::scan_solidity_keyword,
                TokenKind::SolidityKeyword,
            ),
            ProductionKind::StorageKeyword => call_scanner(
                self,
                input,
                Language::scan_storage_keyword,
                TokenKind::StorageKeyword,
            ),
            ProductionKind::StringKeyword => call_scanner(
                self,
                input,
                Language::scan_string_keyword,
                TokenKind::StringKeyword,
            ),
            ProductionKind::StructKeyword => call_scanner(
                self,
                input,
                Language::scan_struct_keyword,
                TokenKind::StructKeyword,
            ),
            ProductionKind::SwitchKeyword => call_scanner(
                self,
                input,
                Language::scan_switch_keyword,
                TokenKind::SwitchKeyword,
            ),
            ProductionKind::SzaboKeyword => try_call_scanner(
                self,
                input,
                Language::maybe_scan_szabo_keyword,
                TokenKind::SzaboKeyword,
            ),
            ProductionKind::ThrowKeyword => try_call_scanner(
                self,
                input,
                Language::maybe_scan_throw_keyword,
                TokenKind::ThrowKeyword,
            ),
            ProductionKind::Tilde => {
                call_scanner(self, input, Language::scan_tilde, TokenKind::Tilde)
            }
            ProductionKind::TrueKeyword => call_scanner(
                self,
                input,
                Language::scan_true_keyword,
                TokenKind::TrueKeyword,
            ),
            ProductionKind::TryKeyword => try_call_scanner(
                self,
                input,
                Language::maybe_scan_try_keyword,
                TokenKind::TryKeyword,
            ),
            ProductionKind::TypeKeyword => try_call_scanner(
                self,
                input,
                Language::maybe_scan_type_keyword,
                TokenKind::TypeKeyword,
            ),
            ProductionKind::UncheckedKeyword => try_call_scanner(
                self,
                input,
                Language::maybe_scan_unchecked_keyword,
                TokenKind::UncheckedKeyword,
            ),
            ProductionKind::UnicodeStringLiteral => try_call_scanner(
                self,
                input,
                Language::maybe_scan_unicode_string_literal,
                TokenKind::UnicodeStringLiteral,
            ),
            ProductionKind::UnsignedFixedType => call_scanner(
                self,
                input,
                Language::scan_unsigned_fixed_type,
                TokenKind::UnsignedFixedType,
            ),
            ProductionKind::UnsignedIntegerType => call_scanner(
                self,
                input,
                Language::scan_unsigned_integer_type,
                TokenKind::UnsignedIntegerType,
            ),
            ProductionKind::UsingKeyword => call_scanner(
                self,
                input,
                Language::scan_using_keyword,
                TokenKind::UsingKeyword,
            ),
            ProductionKind::VarKeyword => try_call_scanner(
                self,
                input,
                Language::maybe_scan_var_keyword,
                TokenKind::VarKeyword,
            ),
            ProductionKind::VersionPragmaValue => call_scanner(
                self,
                input,
                Language::scan_version_pragma_value,
                TokenKind::VersionPragmaValue,
            ),
            ProductionKind::ViewKeyword => call_scanner(
                self,
                input,
                Language::scan_view_keyword,
                TokenKind::ViewKeyword,
            ),
            ProductionKind::VirtualKeyword => try_call_scanner(
                self,
                input,
                Language::maybe_scan_virtual_keyword,
                TokenKind::VirtualKeyword,
            ),
            ProductionKind::WeeksKeyword => call_scanner(
                self,
                input,
                Language::scan_weeks_keyword,
                TokenKind::WeeksKeyword,
            ),
            ProductionKind::WeiKeyword => call_scanner(
                self,
                input,
                Language::scan_wei_keyword,
                TokenKind::WeiKeyword,
            ),
            ProductionKind::WhileKeyword => call_scanner(
                self,
                input,
                Language::scan_while_keyword,
                TokenKind::WhileKeyword,
            ),
            ProductionKind::Whitespace => call_scanner(
                self,
                input,
                Language::scan_whitespace,
                TokenKind::Whitespace,
            ),
            ProductionKind::YearsKeyword => try_call_scanner(
                self,
                input,
                Language::maybe_scan_years_keyword,
                TokenKind::YearsKeyword,
            ),
            ProductionKind::YulDecimalLiteral => call_scanner(
                self,
                input,
                Language::scan_yul_decimal_literal,
                TokenKind::YulDecimalLiteral,
            ),
            ProductionKind::YulHexLiteral => call_scanner(
                self,
                input,
                Language::scan_yul_hex_literal,
                TokenKind::YulHexLiteral,
            ),
            ProductionKind::YulIdentifier => call_scanner(
                self,
                input,
                Language::scan_yul_identifier,
                TokenKind::YulIdentifier,
            ),
            ProductionKind::ABICoderPragma => {
                call_parser(self, input, Language::parse_abi_coder_pragma)
            }
            ProductionKind::AddSubOperator => {
                call_parser(self, input, Language::parse_add_sub_operator)
            }
            ProductionKind::AddressType => call_parser(self, input, Language::parse_address_type),
            ProductionKind::AndOperator => call_parser(self, input, Language::parse_and_operator),
            ProductionKind::ArgumentList => call_parser(self, input, Language::parse_argument_list),
            ProductionKind::ArrayLiteral => call_parser(self, input, Language::parse_array_literal),
            ProductionKind::AssemblyFlags => {
                call_parser(self, input, Language::parse_assembly_flags)
            }
            ProductionKind::AssemblyStatement => {
                call_parser(self, input, Language::parse_assembly_statement)
            }
            ProductionKind::AssignmentOperator => {
                call_parser(self, input, Language::parse_assignment_operator)
            }
            ProductionKind::AsteriskImport => {
                call_parser(self, input, Language::parse_asterisk_import)
            }
            ProductionKind::BitAndOperator => {
                call_parser(self, input, Language::parse_bit_and_operator)
            }
            ProductionKind::BitOrOperator => {
                call_parser(self, input, Language::parse_bit_or_operator)
            }
            ProductionKind::BitXOrOperator => {
                call_parser(self, input, Language::parse_bit_x_or_operator)
            }
            ProductionKind::Block => call_parser(self, input, Language::parse_block),
            ProductionKind::BooleanLiteral => {
                call_parser(self, input, Language::parse_boolean_literal)
            }
            ProductionKind::BreakStatement => {
                call_parser(self, input, Language::parse_break_statement)
            }
            ProductionKind::CatchClause => {
                try_call_parser(self, input, Language::maybe_parse_catch_clause)
            }
            ProductionKind::ConditionalOperator => {
                call_parser(self, input, Language::parse_conditional_operator)
            }
            ProductionKind::ConstantDefinition => {
                call_parser(self, input, Language::parse_constant_definition)
            }
            ProductionKind::ConstructorAttribute => {
                try_call_parser(self, input, Language::maybe_parse_constructor_attribute)
            }
            ProductionKind::ConstructorDefinition => {
                try_call_parser(self, input, Language::maybe_parse_constructor_definition)
            }
            ProductionKind::ContinueStatement => {
                call_parser(self, input, Language::parse_continue_statement)
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
                try_call_parser(self, input, Language::maybe_parse_emit_statement)
            }
            ProductionKind::EndOfFileTrivia => {
                call_parser(self, input, Language::parse_end_of_file_trivia)
            }
            ProductionKind::EnumDefinition => {
                call_parser(self, input, Language::parse_enum_definition)
            }
            ProductionKind::EqualityComparisonOperator => {
                call_parser(self, input, Language::parse_equality_comparison_operator)
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
            ProductionKind::ExponentiationOperator => {
                call_parser(self, input, Language::parse_exponentiation_operator)
            }
            ProductionKind::Expression => call_parser(self, input, Language::parse_expression),
            ProductionKind::ExpressionStatement => {
                call_parser(self, input, Language::parse_expression_statement)
            }
            ProductionKind::FallbackFunctionAttribute => try_call_parser(
                self,
                input,
                Language::maybe_parse_fallback_function_attribute,
            ),
            ProductionKind::FallbackFunctionDefinition => try_call_parser(
                self,
                input,
                Language::maybe_parse_fallback_function_definition,
            ),
            ProductionKind::ForStatement => call_parser(self, input, Language::parse_for_statement),
            ProductionKind::FunctionAttribute => {
                call_parser(self, input, Language::parse_function_attribute)
            }
            ProductionKind::FunctionCallOperator => {
                call_parser(self, input, Language::parse_function_call_operator)
            }
            ProductionKind::FunctionCallOptions => {
                try_call_parser(self, input, Language::maybe_parse_function_call_options)
            }
            ProductionKind::FunctionDefinition => {
                call_parser(self, input, Language::parse_function_definition)
            }
            ProductionKind::FunctionType => call_parser(self, input, Language::parse_function_type),
            ProductionKind::IdentifierPath => {
                call_parser(self, input, Language::parse_identifier_path)
            }
            ProductionKind::IfStatement => call_parser(self, input, Language::parse_if_statement),
            ProductionKind::ImportAlias => call_parser(self, input, Language::parse_import_alias),
            ProductionKind::ImportDirective => {
                call_parser(self, input, Language::parse_import_directive)
            }
            ProductionKind::ImportPath => call_parser(self, input, Language::parse_import_path),
            ProductionKind::IndexAccessOperator => {
                call_parser(self, input, Language::parse_index_access_operator)
            }
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
            ProductionKind::MemberAccessOperator => {
                call_parser(self, input, Language::parse_member_access_operator)
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
            ProductionKind::MulDivModOperator => {
                call_parser(self, input, Language::parse_mul_div_mod_operator)
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
            ProductionKind::NumericExpression => {
                call_parser(self, input, Language::parse_numeric_expression)
            }
            ProductionKind::OrOperator => call_parser(self, input, Language::parse_or_operator),
            ProductionKind::OrderComparisonOperator => {
                call_parser(self, input, Language::parse_order_comparison_operator)
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
            ProductionKind::ReceiveFunctionAttribute => try_call_parser(
                self,
                input,
                Language::maybe_parse_receive_function_attribute,
            ),
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
            ProductionKind::SelectiveImport => {
                call_parser(self, input, Language::parse_selective_import)
            }
            ProductionKind::ShiftOperator => {
                call_parser(self, input, Language::parse_shift_operator)
            }
            ProductionKind::SimpleImport => call_parser(self, input, Language::parse_simple_import),
            ProductionKind::SimpleStatement => {
                call_parser(self, input, Language::parse_simple_statement)
            }
            ProductionKind::SourceUnit => call_parser(self, input, Language::parse_source_unit),
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
            ProductionKind::ThrowStatement => {
                try_call_parser(self, input, Language::maybe_parse_throw_statement)
            }
            ProductionKind::TrailingTrivia => {
                call_parser(self, input, Language::parse_trailing_trivia)
            }
            ProductionKind::TryStatement => {
                try_call_parser(self, input, Language::maybe_parse_try_statement)
            }
            ProductionKind::TupleDeconstructionStatement => {
                call_parser(self, input, Language::parse_tuple_deconstruction_statement)
            }
            ProductionKind::TupleExpression => {
                call_parser(self, input, Language::parse_tuple_expression)
            }
            ProductionKind::TypeExpression => {
                try_call_parser(self, input, Language::maybe_parse_type_expression)
            }
            ProductionKind::TypeName => call_parser(self, input, Language::parse_type_name),
            ProductionKind::UnaryPostfixOperator => {
                call_parser(self, input, Language::parse_unary_postfix_operator)
            }
            ProductionKind::UnaryPrefixOperator => {
                call_parser(self, input, Language::parse_unary_prefix_operator)
            }
            ProductionKind::UncheckedBlock => {
                try_call_parser(self, input, Language::maybe_parse_unchecked_block)
            }
            ProductionKind::UnnamedFunctionAttribute => try_call_parser(
                self,
                input,
                Language::maybe_parse_unnamed_function_attribute,
            ),
            ProductionKind::UnnamedFunctionDefinition => try_call_parser(
                self,
                input,
                Language::maybe_parse_unnamed_function_definition,
            ),
            ProductionKind::UserDefinedOperator => {
                try_call_parser(self, input, Language::maybe_parse_user_defined_operator)
            }
            ProductionKind::UserDefinedValueTypeDefinition => try_call_parser(
                self,
                input,
                Language::maybe_parse_user_defined_value_type_definition,
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
            ProductionKind::YulDeclarationStatement => {
                call_parser(self, input, Language::parse_yul_declaration_statement)
            }
            ProductionKind::YulExpression => {
                call_parser(self, input, Language::parse_yul_expression)
            }
            ProductionKind::YulForStatement => {
                call_parser(self, input, Language::parse_yul_for_statement)
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
                try_call_parser(self, input, Language::maybe_parse_yul_leave_statement)
            }
            ProductionKind::YulLiteral => call_parser(self, input, Language::parse_yul_literal),
            ProductionKind::YulStatement => call_parser(self, input, Language::parse_yul_statement),
            ProductionKind::YulSwitchStatement => {
                call_parser(self, input, Language::parse_yul_switch_statement)
            }
        }
        .ok_or_else(|| Error::InvalidProductionVersion(production_kind).into())
    }
}

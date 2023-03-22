pub use std::ops::Range;
pub use std::rc::Rc;

#[allow(deprecated, unused_imports)]
use semver::Version;

pub use super::cst;
pub use super::kinds::*;
pub use super::parser_output::ParserOutput;

const DEBUG_ERROR_MERGING: bool = false;

#[derive(PartialEq)]
pub struct ParseError {
    pub position: usize,
    pub expected: String,
}

impl ParseError {
    pub fn new<T: Into<String>>(position: usize, expected: T) -> Self {
        Self {
            position,
            expected: expected.into(),
        }
    }

    pub fn merge_with(&mut self, other: Self) {
        if DEBUG_ERROR_MERGING {
            if self.position < other.position {
                self.expected = format!(
                    "O={other_expected}\nNOT {position}@[{expected}]",
                    other_expected = other.expected,
                    position = self.position,
                    expected = self.expected
                );
                self.position = other.position;
            } else if self.position == other.position {
                self.expected = format!(
                    "{other_expected}, or {expected}",
                    other_expected = other.expected,
                    expected = self.expected
                );
            } else {
                self.expected = format!(
                    "S={expected}\nNOT {other_position}@[{other_expected}]",
                    expected = self.expected,
                    other_position = other.position,
                    other_expected = other.expected
                );
            }
        } else {
            if self.position < other.position {
                *self = other;
            } else if self.position == other.position {
                self.expected = format!(
                    "{other_expected}, or {expected}",
                    other_expected = other.expected,
                    expected = self.expected
                );
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

use ariadne::{Color, Config, Label, Report, ReportKind, Source};

pub(crate) fn render_error_report(
    error: &ParseError,
    source_id: &str,
    source: &str,
    with_color: bool,
) -> String {
    let kind = ReportKind::Error;
    let color = if with_color { Color::Red } else { Color::Unset };
    let message = if DEBUG_ERROR_MERGING {
        format!(
            "{position}: Expected {expected}",
            position = error.position,
            expected = error.expected
        )
    } else {
        format!("Expected {expected}", expected = error.expected)
    };
    let source_start = error.position;
    let source_end = error.position;
    if source.is_empty() {
        return format!("{kind}: {message}\n   ─[{source_id}:{source_start}:{source_end}]");
    }
    let label = "Error occurred here.".to_string();
    let mut builder = Report::build(kind, source_id, source_start)
        .with_config(Config::default().with_color(with_color))
        .with_message(message);
    builder.add_label(
        Label::new((source_id, source_start..source_end))
            .with_color(color)
            .with_message(label),
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

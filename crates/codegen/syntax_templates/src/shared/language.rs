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

pub enum ParserResult {
    Pass {
        node: Rc<cst::Node>,
        error: Option<ParseError>,
    },
    Fail {
        error: ParseError,
    },
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
fn call_scanner<L, F>(
    language: &L,
    input: &str,
    scanner: F,
    kind: TokenKind,
    error_message: &str,
) -> Option<ParseOutput>
where
    F: Fn(&L, &mut Stream) -> bool,
{
    let mut stream = Stream::new(input);
    Some(
        if scanner(language, &mut stream) && stream.peek().is_none() {
            ParseOutput {
                parse_tree: Some(cst::Node::token(
                    kind,
                    Range {
                        start: Default::default(),
                        end: stream.position(),
                    },
                    None,
                    None,
                )),
                errors: vec![],
            }
        } else {
            ParseOutput {
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
) -> Option<ParseOutput>
where
    F: Fn(&L, &mut Stream) -> Option<bool>,
{
    let mut stream = Stream::new(input);
    scanner(language, &mut stream).map(|result| {
        if result && stream.peek().is_none() {
            ParseOutput {
                parse_tree: Some(cst::Node::token(
                    kind,
                    Range {
                        start: Default::default(),
                        end: stream.position(),
                    },
                    None,
                    None,
                )),
                errors: vec![],
            }
        } else {
            ParseOutput {
                parse_tree: None,
                errors: vec![ParseError::new(stream.position(), error_message)],
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
        ParserResult::Pass { node, .. } if stream.peek().is_none() => ParseOutput {
            parse_tree: Some(node),
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
        ParserResult::Pass { node, .. } if stream.peek().is_none() => ParseOutput {
            parse_tree: Some(node),
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

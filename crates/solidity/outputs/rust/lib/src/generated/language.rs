// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use super::{cst, kinds::ProductionKind, parse::create_parsers, parse::ErrorType};
use ariadne::{Color, Config, Fmt, Label, Report, ReportKind, Source};
use chumsky::{error::SimpleReason, BoxedParser, Parser as ChumskyParser, Span};
use semver::Version;
use std::collections::BTreeMap;
use std::rc::Rc;
pub struct Language {
    parsers: BTreeMap<ProductionKind, Parser>,
    version: Version,
}
impl Language {
    pub fn new(version: Version) -> Self {
        Self {
            parsers: create_parsers(&version),
            version,
        }
    }
    pub fn version(&self) -> &Version {
        &self.version
    }
    pub fn get_parser(&self, kind: ProductionKind) -> &Parser {
        &self.parsers[&kind]
    }
}
pub struct Parser(BoxedParser<'static, char, Rc<cst::Node>, ErrorType>);
impl Parser {
    pub(super) fn new(inner: BoxedParser<'static, char, Rc<cst::Node>, ErrorType>) -> Self {
        Self(inner)
    }
    pub fn parse(&self, input: &str) -> ParserOutput {
        let (parse_tree, errors) = self.0.parse_recovery(input);
        ParserOutput { parse_tree, errors }
    }
}
#[derive(PartialEq)]
pub struct ParserOutput {
    parse_tree: Option<Rc<cst::Node>>,
    errors: Vec<ErrorType>,
}
impl ParserOutput {
    pub fn parse_tree(&self) -> Option<Rc<cst::Node>> {
        self.parse_tree.clone()
    }
    pub fn error_count(&self) -> usize {
        self.errors.len()
    }
    pub fn errors_as_strings(
        &self,
        source_id: &str,
        source: &str,
        with_colour: bool,
    ) -> Vec<String> {
        return self
            .errors
            .iter()
            .map(|error| render_error_report(error, source_id, source, with_colour))
            .collect();
    }
    pub fn is_valid(&self) -> bool {
        self.parse_tree.is_some() && self.errors.is_empty()
    }
}
fn render_error_report(
    error: &ErrorType,
    source_id: &str,
    source: &str,
    with_color: bool,
) -> String {
    let kind = ReportKind::Error;
    let color = if with_color { Color::Red } else { Color::Unset };
    let message = match error.reason() {
        SimpleReason::Custom(message) => message.to_string(),
        SimpleReason::Unclosed { delimiter, .. } => {
            format!("Expected delimiter '{}' to be closed", delimiter.fg(color))
        }
        SimpleReason::Unexpected => {
            let mut expected: Vec<&Option<char>> = error.expected().collect();
            expected.sort();
            let expected = if expected.len() == 0 {
                "something else".to_string()
            } else {
                expected
                    .iter()
                    .map(|expected| match expected {
                        Some(expected) => format!("'{}'", expected),
                        None => "end of input".to_string(),
                    })
                    .collect::<Vec<_>>()
                    .join(" or ")
            };
            format!("Expected {expected}.")
        }
    };
    let source_start = error.span().start() as usize;
    let source_end = error.span().end() as usize;
    if source.is_empty() {
        return format!("{kind}: {message}\n   ─[{source_id}:{source_start}:{source_end}]");
    }
    let label = match error.reason() {
        SimpleReason::Custom(_) => "Error occurred here.".to_string(),
        SimpleReason::Unclosed { delimiter, .. } => {
            format!("Unclosed delimiter '{}'.", delimiter.fg(color))
        }
        SimpleReason::Unexpected => {
            if let Some(found) = error.found() {
                format!("Found '{}'.", found.fg(color))
            } else {
                "Found end of input.".to_string()
            }
        }
    };
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

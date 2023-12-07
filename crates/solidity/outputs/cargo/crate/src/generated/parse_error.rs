// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use std::collections::BTreeSet;

use super::{
    kinds::TokenKind,
    text_index::{TextIndex, TextRange},
};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ParseError {
    pub(crate) text_range: TextRange,
    pub(crate) tokens_that_would_have_allowed_more_progress: Vec<TokenKind>,
}

impl ParseError {
    pub fn text_range(&self) -> &TextRange {
        &self.text_range
    }

    pub fn tokens_that_would_have_allowed_more_progress(&self) -> Vec<String> {
        let tokens_that_would_have_allowed_more_progress = self
            .tokens_that_would_have_allowed_more_progress
            .iter()
            .collect::<BTreeSet<_>>();

        tokens_that_would_have_allowed_more_progress
            .into_iter()
            .map(TokenKind::to_string)
            .collect()
    }

    pub fn to_error_report(&self, source_id: &str, source: &str, with_color: bool) -> String {
        render_error_report(self, source_id, source, with_color)
    }
}

impl ParseError {
    #[allow(dead_code)]
    pub(crate) fn new_at_position(
        position: TextIndex,
        tokens_that_would_have_allowed_more_progress: Vec<TokenKind>,
    ) -> Self {
        Self {
            text_range: position..position,
            tokens_that_would_have_allowed_more_progress,
        }
    }

    pub(crate) fn new_covering_range(
        text_range: TextRange,
        tokens_that_would_have_allowed_more_progress: Vec<TokenKind>,
    ) -> Self {
        Self {
            text_range,
            tokens_that_would_have_allowed_more_progress,
        }
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
    let source_start = error.text_range.start.utf8;
    let source_end = error.text_range.end.utf8;

    let tokens_that_would_have_allowed_more_progress =
        error.tokens_that_would_have_allowed_more_progress();
    let message = if tokens_that_would_have_allowed_more_progress.is_empty() {
        "Expected end of file.".to_string()
    } else {
        format!(
            "Expected {expectations}.",
            expectations = tokens_that_would_have_allowed_more_progress.join(" or ")
        )
    };

    if source.is_empty() {
        return format!("{kind}: {message}\n   ─[{source_id}:0:0]");
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

//! Utilities for defining error diagnostics.

use std::ops::Range;

pub mod implementations;

/// The severity of a diagnostic.
///
/// Explicitly compatible with the [LSP protocol](https://microsoft.github.io/language-server-protocol/specifications/lsp/3.17/specification/#diagnosticSeverity).
#[allow(dead_code)]
#[repr(u8)]
pub enum Severity {
    Error = 1,
    Warning = 2,
    Information = 3,
    Hint = 4,
}

/// A compiler diagnostic that can be rendered to a user.
///
/// TODO(v2): Replace `trait Diagnostic` with a compilation-level parent enum for all errors
/// exposed in the public API, when `CompilationUnit` is introduced.
pub trait Diagnostic {
    /// The character range of the source that this diagnostic applies to.
    fn text_range(&self) -> Range<usize>;
    /// The severity of this diagnostic.
    fn severity(&self) -> Severity;
    /// The primary message associated with this diagnostic.
    fn message(&self) -> String;
}

pub fn render<D: Diagnostic>(error: &D, source_id: &str, source: &str, with_color: bool) -> String {
    use ariadne::{Color, Config, IndexType, Label, Report, ReportKind, Source};

    let (kind, color) = match error.severity() {
        Severity::Error => (ReportKind::Error, Color::Red),
        Severity::Warning => (ReportKind::Warning, Color::Yellow),
        Severity::Information => (ReportKind::Advice, Color::Blue),
        Severity::Hint => (ReportKind::Advice, Color::Blue),
    };

    let message = error.message();

    if source.is_empty() {
        return format!("{kind}: {message}\n   ─[{source_id}:0:0]");
    }

    let color = if with_color { color } else { Color::Primary };

    let range = error.text_range();

    let report = Report::build(kind, (source_id, range.clone()))
        .with_config(
            Config::default()
                .with_color(with_color)
                .with_index_type(IndexType::Byte),
        )
        .with_message(message)
        .with_label(
            Label::new((source_id, range))
                .with_color(color)
                .with_message("Error occurred here."),
        )
        .finish();

    let mut result = vec![];
    report
        .write((source_id, Source::from(&source)), &mut result)
        .expect("Failed to write report");

    String::from_utf8(result)
        .expect("Failed to convert report to utf8")
        .trim()
        .to_string()
}

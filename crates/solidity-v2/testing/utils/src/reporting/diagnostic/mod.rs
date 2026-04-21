//! Utilities for defining error diagnostics.

use std::ops::Range;

use slang_solidity_v2_common::diagnostics::DiagnosticSeverity;

mod implementations;

/// A compiler diagnostic that can be rendered to a user.
///
/// TODO(v2): remove [`RenderDiagnostic`] once `NodeChecker` is deprecated. We should be using
/// the `Diagnostic` public API directly and its provided extensions.
pub trait RenderDiagnostic {
    /// The character range of the source that this diagnostic applies to.
    fn text_range(&self) -> Range<usize>;
    /// The severity of this diagnostic.
    fn severity(&self) -> DiagnosticSeverity;
    /// The primary message associated with this diagnostic.
    fn message(&self) -> String;
}

pub fn render<D: RenderDiagnostic>(
    error: &D,
    source_id: &str,
    source: &str,
    with_color: bool,
) -> String {
    use ariadne::{Color, Config, Label, Report, ReportKind, Source};

    let (kind, color) = match error.severity() {
        DiagnosticSeverity::Error => (ReportKind::Error, Color::Red),
    };

    let message = error.message();

    if source.is_empty() {
        return format!("{kind}: {message}\n   ─[{source_id}:0:0]");
    }

    // TODO(v2): Once https://github.com/zesterer/ariadne/pull/159 is released we should be able to
    // skip this step
    let color = if with_color { color } else { Color::Unset };

    // TODO(v2): Once https://github.com/zesterer/ariadne/pull/159 is released we should be able to
    // move to a newer version of ariadne and use IndexType::Byte, to avoid this conversion.
    let range = {
        let start = source[..error.text_range().start].chars().count();
        let end = source[..error.text_range().end].chars().count();
        start..end
    };

    let report = Report::build(kind, source_id, range.start)
        .with_config(Config::default().with_color(with_color))
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

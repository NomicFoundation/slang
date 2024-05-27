use crate::text_index::TextRange;

/// The severity of a diagnostic.
///
/// Explicitly compatible with the [LSP protocol](https://microsoft.github.io/language-server-protocol/specifications/lsp/3.17/specification/#diagnosticSeverity).
#[repr(u8)]
pub enum Severity {
    Error = 1,
    Warning = 2,
    Information = 3,
    Hint = 4,
}

/// A compiler diagnostic that can be rendered to a user.
pub trait Diagnostic {
    /// The character range of the source that this diagnostic applies to.
    ///
    /// Note that this is not tracking columns, so it is not compatible with LSP's
    /// [`Position`](https://microsoft.github.io/language-server-protocol/specifications/lsp/3.17/specification/#position)
    /// at the moment.
    fn text_range(&self) -> TextRange;
    /// The severity of this diagnostic.
    fn severity(&self) -> Severity;
    /// The primary message associated with this diagnostic.
    fn message(&self) -> String;
}

#[cfg(feature = "__private_ariadne")]
pub fn render<D: Diagnostic>(error: &D, source_id: &str, source: &str, with_color: bool) -> String {
    use ariadne::{Color, Config, Label, Report, ReportKind, Source};

    let kind = match error.severity() {
        Severity::Error => ReportKind::Error,
        Severity::Warning => ReportKind::Warning,
        Severity::Information => ReportKind::Advice,
        Severity::Hint => ReportKind::Advice,
    };

    let color = if with_color { Color::Red } else { Color::Unset };

    let message = error.message();

    if source.is_empty() {
        return format!("{kind}: {message}\n   ─[{source_id}:0:0]");
    }

    let range = {
        let start = source[..error.text_range().start.utf8].chars().count();
        let end = source[..error.text_range().end.utf8].chars().count();
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

    return String::from_utf8(result)
        .expect("Failed to convert report to utf8")
        .trim()
        .to_string();
}

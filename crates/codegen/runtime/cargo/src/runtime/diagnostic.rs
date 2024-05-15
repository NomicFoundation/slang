use std::borrow::Cow;

use crate::text_index::TextRange;

#[repr(u8)]
pub enum Severity {
    Error = 1,
    Warning = 2,
    Information = 3,
    Hint = 4,
}

pub trait Diagnostic {
    fn range(&self) -> TextRange;
    fn code(&self) -> Option<Cow<'_, str>> {
        None
    }
    fn severity(&self) -> Severity;
    fn message(&self) -> String;
}

#[cfg(feature = "__private_ariadne")]
pub fn render<D: Diagnostic>(error: &D, source_id: &str, source: &str, with_color: bool) -> String {
    use ariadne::{Color, Config, Label, Report, ReportKind, Source};

    use crate::text_index::TextRangeExtensions as _;

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

    let range = error.range().char();

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

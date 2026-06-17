use std::collections::BTreeMap;

use anyhow::Result;
use ariadne::{Config, Label, Report, ReportKind, Source};

use crate::paths::PathExtensions;
use crate::solc::{Error, Severity};

pub fn render_solc_error(error: &Error, sources: &BTreeMap<String, String>) -> Result<String> {
    let message = &error.message;
    let kind = severity_to_kind(&error.severity);

    let code = format!(
        "{error_type} {error_code}",
        error_type = error.r#type,
        error_code = error.error_code.as_deref().unwrap_or("<no-code>")
    );

    let Some(source_location) = error.source_location.as_ref() else {
        return Ok(format!("[{code}] {kind}: {message}"));
    };

    let source_id = source_location.file.unwrap_string();
    let source = sources.get(&source_id).unwrap();

    let range = {
        let start = source_location.start;
        let end = source_location.end;

        let start_chars = source[..start].chars().count();
        let end_chars = start_chars + source[start..end].chars().count();

        start_chars..end_chars
    };

    let mut report = Vec::new();
    Report::build(kind, source_id.as_str(), range.start)
        .with_config(Config::default().with_color(false))
        .with_code(&code)
        .with_message(message)
        .with_label(Label::new((source_id.as_str(), range)).with_message("Error occurred here."))
        .finish()
        .write((source_id.as_str(), Source::from(&source)), &mut report)?;

    Ok(String::from_utf8(report)?.trim_end().to_string())
}

fn severity_to_kind(severity: &Severity) -> ReportKind<'static> {
    match severity {
        Severity::Error => ReportKind::Error,
        Severity::Warning => ReportKind::Warning,
        Severity::Info => ReportKind::Custom("Info", ariadne::Color::Unset),
    }
}

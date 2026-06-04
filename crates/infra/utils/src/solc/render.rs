use std::collections::BTreeMap;
use std::fmt::Write;

use anyhow::Result;
use ariadne::{Config, Label, Report, ReportKind, Source};

use crate::solc::{Error, Severity, SourceLocation};

pub fn render_solc_error(
    error: &Error,
    sources: &BTreeMap<String, String>,
    report: &mut String,
) -> Result<()> {
    let kind = severity_to_kind(&error.severity);
    let message = &error.message;

    let (source_id, source) = match &error.location {
        Some(loc) => {
            let file = loc.file.to_string_lossy().into_owned();
            let source = sources.get(&file).cloned();
            (file, source)
        }
        None => ("<no source location>".to_string(), None),
    };

    let Some(source) = source else {
        write!(report, "{kind}: {message}\n   ─[{source_id}:0:0]")?;
        return Ok(());
    };

    let range = match &error.location {
        Some(SourceLocation { start, end, .. }) if !start.is_negative() && !end.is_negative() => {
            #[allow(clippy::cast_sign_loss)]
            let start = *start as usize;
            #[allow(clippy::cast_sign_loss)]
            let end = *end as usize;

            if end > source.len() || start > end {
                write!(report, "{kind}: {message}\n   ─[{source_id}:0:0]")?;
                return Ok(());
            }

            let start_chars = source[..start].chars().count();
            let end_chars = start_chars + source[start..end].chars().count();
            start_chars..end_chars
        }
        _ => {
            write!(report, "{kind}: {message}\n   ─[{source_id}:0:0]")?;
            return Ok(());
        }
    };

    let mut buf = Vec::new();
    Report::build(kind, source_id.as_str(), range.start)
        .with_config(Config::default().with_color(false))
        .with_message(message)
        .with_label(Label::new((source_id.as_str(), range)).with_message("Error occurred here."))
        .finish()
        .write((source_id.as_str(), Source::from(&source)), &mut buf)?;
    report.push_str(std::str::from_utf8(&buf)?.trim_end());
    Ok(())
}

fn severity_to_kind(severity: &Severity) -> ReportKind<'static> {
    match severity {
        Severity::Error => ReportKind::Error,
        Severity::Warning => ReportKind::Warning,
        Severity::Info => ReportKind::Custom("Info", ariadne::Color::Unset),
    }
}

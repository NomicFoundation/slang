//! Renders compiler diagnostics for users.

use infra_utils::snapshot_markers::{
    CURRENT_SLANG_EVM_TARGET, CURRENT_SLANG_LANGUAGE_VERSION, replace_marker,
};
use slang_solidity_v2_common::diagnostics::{Diagnostic, DiagnosticExtensions, DiagnosticSeverity};
use slang_solidity_v2_common::evm_targets::EvmTarget;
use slang_solidity_v2_common::versions::LanguageVersion;

pub fn render(diagnostic: &Diagnostic, source_id: &str, source: &str, with_color: bool) -> String {
    render_message(
        diagnostic,
        source_id,
        source,
        with_color,
        diagnostic.message(),
    )
}

pub fn render_for_snapshot(
    diagnostic: &Diagnostic,
    source_id: &str,
    source: &str,
    version: LanguageVersion,
    target: EvmTarget,
) -> String {
    let version = version.to_string();
    let target = target.to_string();

    let message = replace_marker(
        &diagnostic.message(),
        &version,
        &format!(r"\b{}\b", regex::escape(&version)),
        CURRENT_SLANG_LANGUAGE_VERSION,
    );

    let message = replace_marker(
        &message,
        &target,
        &format!(r"\b{target}\b"),
        CURRENT_SLANG_EVM_TARGET,
    );

    render_message(diagnostic, source_id, source, false, message)
}

fn render_message(
    diagnostic: &Diagnostic,
    source_id: &str,
    source: &str,
    with_color: bool,
    message: String,
) -> String {
    use ariadne::{Color, Config, Label, Report, ReportKind, Source};

    let (kind, color) = match diagnostic.severity() {
        DiagnosticSeverity::Error => (ReportKind::Error, Color::Red),
        DiagnosticSeverity::Warning => (ReportKind::Warning, Color::Yellow),
    };

    let code = diagnostic.code();

    if source.is_empty() {
        return format!("[{code}] {kind}: {message}\n   ─[{source_id}:0:0]");
    }

    // TODO(v2): Once https://github.com/zesterer/ariadne/pull/159 is released we should be able to
    // skip this step
    let color = if with_color { color } else { Color::Unset };

    // TODO(v2): Once https://github.com/zesterer/ariadne/pull/159 is released we should be able to
    // move to a newer version of ariadne and use IndexType::Byte, to avoid this conversion.
    let range = {
        let text_range = diagnostic.text_range();
        let start = source[..text_range.start].chars().count();
        let end = source[..text_range.end].chars().count();
        start..end
    };

    let report = Report::build(kind, source_id, range.start)
        .with_config(Config::default().with_color(with_color))
        .with_code(code)
        .with_message(message)
        .with_label(
            Label::new((source_id, range))
                .with_color(color)
                .with_message(format!("{:?} occurred here.", diagnostic.severity())),
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

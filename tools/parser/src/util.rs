use std::{
    io::Write,
    process::{Command, Stdio},
};

use anyhow::Result;
use ariadne::{Color, Fmt, Label, Report, ReportKind, Source};
use chumsky::prelude::*;

pub fn print_errors(errs: Vec<Simple<char>>, src: &str) {
    errs.into_iter().for_each(|e| {
        let report = generate_report(e);
        let source = Source::from(src);
        report.finish().print(source).unwrap();
    });
}

fn generate_report(e: Simple<char>) -> ariadne::ReportBuilder<std::ops::Range<usize>> {
    let msg = if let chumsky::error::SimpleReason::Custom(msg) = e.reason() {
        msg.clone()
    } else {
        format!(
            "{}{}, expected {}",
            if e.found().is_some() {
                "Unexpected token"
            } else {
                "Unexpected end of input"
            },
            if let Some(label) = e.label() {
                format!(" while parsing {}", label)
            } else {
                String::new()
            },
            if e.expected().len() == 0 {
                "something else".to_string()
            } else {
                e.expected()
                    .map(|expected| match expected {
                        Some(expected) => expected.to_string(),
                        None => "end of input".to_string(),
                    })
                    .collect::<Vec<_>>()
                    .join(", ")
            },
        )
    };
    let report = Report::build(ReportKind::Error, (), e.span().start)
        .with_code(3)
        .with_message(msg)
        .with_label(
            Label::new(e.span())
                .with_message(match e.reason() {
                    chumsky::error::SimpleReason::Custom(msg) => msg.clone(),
                    _ => format!(
                        "Unexpected {}",
                        e.found()
                            .map(|c| format!("token {}", c.fg(Color::Red)))
                            .unwrap_or_else(|| "end of input".to_string())
                    ),
                })
                .with_color(Color::Red),
        );
    let report = match e.reason() {
        chumsky::error::SimpleReason::Unclosed { span, delimiter } => report.with_label(
            Label::new(span.clone())
                .with_message(format!(
                    "Unclosed delimiter {}",
                    delimiter.fg(Color::Yellow)
                ))
                .with_color(Color::Yellow),
        ),
        chumsky::error::SimpleReason::Unexpected => report,
        chumsky::error::SimpleReason::Custom(_) => report,
    };
    report
}

pub fn rustfmt(source: String) -> Result<String> {
    let mut child = Command::new("rustfmt")
        .arg("--edition")
        .arg("2021")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to spawn child process");

    let mut stdin = child.stdin.take().expect("Failed to open stdin");
    std::thread::spawn(move || {
        stdin
            .write_all(source.as_bytes())
            .expect("Failed to write to stdin");
    });

    let output = child.wait_with_output().expect("Failed to read stdout");

    Ok(String::from_utf8(output.stdout)?)
}

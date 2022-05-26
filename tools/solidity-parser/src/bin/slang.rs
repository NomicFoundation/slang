use std::fs;

use ariadne::{Color, Fmt, Label, Report, ReportKind, Source};
use chumsky::{prelude::*, Parser};
use clap::Parser as ClapParser;

use solidity_parser::parser::create_source_unit_parser;

#[derive(ClapParser, Debug)]
struct ProgramArgs {
    #[clap(long)]
    solidity_input: String,
}

fn main() {
    let args = ProgramArgs::parse();

    println!(" => Parsing Solidity");
    let solidity_src = fs::read_to_string(args.solidity_input).expect("Failed to read file");
    let (_parse_tree, errs) = create_source_unit_parser().parse_recovery(solidity_src.as_str());
    print_errors(errs, &solidity_src);
}

// TODO: encapsulate the following in a support library

fn print_errors(errs: Vec<Simple<char>>, src: &str) {
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

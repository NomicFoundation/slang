use std::fs;

use ariadne::{Color, Fmt, Label, Report, ReportKind, Source};
use chumsky::prelude::*;
use clap::Parser as ClapParser;
use semver::Version;
use solidity_parser::generated::parse::ErrorType;
use strum::EnumString;

use solidity_parser::generated::parse::Context;
use solidity_parser::generated::parse::Parsers;
use solidity_parser::generated::parse::SpanType;

#[derive(Copy, Clone, Debug, PartialEq, Eq, EnumString)]
enum ParserType {
    CST,
    AST,
}

#[derive(ClapParser, Debug)]
struct ProgramArgs {
    solidity_input: String,

    #[clap(long)]
    version: Version,

    #[clap(long)]
    json_output: Option<String>,

    #[clap(long)]
    yaml_output: Option<String>,
}

struct Input<'a, Iter: Iterator<Item = char>> {
    source: Iter,
    position: usize,
    context: &'a Context<'a>,
}

impl<'a> Input<'a, std::str::Chars<'a>> {
    fn new(source: &'a str, context: &'a Context<'a>) -> Self {
        Self {
            source: source.chars(),
            position: 0,
            context,
        }
    }
    fn eos(&self) -> (&'a Context<'a>, std::ops::Range<usize>) {
        (self.context, 0..0)
    }
}

impl<'a, Iter: Iterator<Item = char>> Iterator for Input<'a, Iter> {
    type Item = (char, (&'a Context<'a>, std::ops::Range<usize>));

    fn next(&mut self) -> Option<Self::Item> {
        let c = self.source.next();
        match c {
            Some(c) => {
                self.position += 1;
                Some((c, (self.context, self.position..(self.position + 1))))
            }
            None => None,
        }
    }
}

fn main() -> Result<(), usize> {
    let args = ProgramArgs::parse();

    let solidity_src = fs::read_to_string(args.solidity_input).expect("Failed to read file");

    let context = Context::new();
    let input = Input::new(&solidity_src, &context);
    let stream = chumsky::Stream::from_iter(input.eos(), input);
    let (parse_tree, errs) = Parsers::new(args.version)
        .source_unit
        .parse_recovery(stream);
    let number_of_errors = errs.len();
    print_errors(errs, &solidity_src);

    if let Some(source_unit) = parse_tree {
        if let Some(json_output) = args.json_output {
            let json = serde_json::to_string(&source_unit).expect("Failed to produce json");
            if json_output == "-" {
                println!("{}", json);
            } else {
                fs::write(json_output, json).expect("Failed to write json file");
            }
        }
        if let Some(yaml_output) = args.yaml_output {
            let yaml = serde_yaml::to_string(&source_unit).expect("Failed to produce yaml");
            if yaml_output == "-" {
                println!("{}", yaml);
            } else {
                fs::write(yaml_output, yaml).expect("Failed to write yaml file");
            }
        }
    }

    if number_of_errors == 0 {
        Ok(())
    } else {
        Err(number_of_errors)
    }
}

// TODO: encapsulate the following in a support library

fn print_errors<'a>(errs: Vec<ErrorType<'a>>, src: &'a str) {
    errs.into_iter().for_each(|e| {
        let context = e.span().context();
        let report = generate_report(e);
        let source = Source::from(src);
        report.finish().print((context, source)).unwrap();
    });
}

fn generate_report<'a>(e: ErrorType<'a>) -> ariadne::ReportBuilder<SpanType<'a>> {
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
                    .join(" or ")
            },
        )
    };
    let report = Report::build(ReportKind::Error, e.span().context(), e.span().start())
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

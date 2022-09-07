use std::fs;

use ariadne::{Color, Fmt, Label, Report, ReportKind, Source};
use chumsky::prelude::*;
use clap::Parser as ClapParser;
use strum::EnumString;

use solidity_parser::generated::{ast_parser, cst_parser};

#[derive(Copy, Clone, Debug, PartialEq, Eq, EnumString)]
enum ParserType {
    CST,
    AST,
}

#[derive(ClapParser, Debug)]
struct ProgramArgs {
    solidity_input: String,

    #[clap(long)]
    parser: ParserType,

    #[clap(long)]
    json_output: Option<String>,

    #[clap(long)]
    yaml_output: Option<String>,
}

fn main() -> Result<(), usize> {
    let args = ProgramArgs::parse();

    let solidity_src = fs::read_to_string(args.solidity_input).expect("Failed to read file");

    if args.parser == ParserType::AST {
        let (parse_tree, errs) = ast_parser::Parsers::new()
            .source_unit
            .parse_recovery(solidity_src.as_str());
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
    } else {
        let (parse_tree, errs) = cst_parser::Parsers::new()
            .source_unit
            .parse_recovery(solidity_src.as_str());
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
                    .join(" or ")
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

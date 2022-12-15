use ariadne::{Color, Config, Fmt, Label, Report, ReportBuilder, ReportKind, Source};
use chumsky::{error::SimpleReason, Parser, Span};

use crate::generated::parse::{ErrorType, ParserType, SpanType};

pub struct ParserOutput<TNode> {
    pub root_node: Option<TNode>,
    pub error_reports: Vec<String>,
}

pub fn parse<'a, TNode>(
    source: &'a str,
    parser: ParserType<'a, TNode>,
    with_color: bool,
) -> ParserOutput<TNode> {
    let (parse_tree, errors) = parser.parse_recovery(source);

    let mut error_reports = vec![];
    for error in &errors {
        let source = Source::from(source);
        let report = render_error_report(&error, with_color);

        let mut result = vec![];
        report
            .write(source, &mut result)
            .expect("Failed to write report");

        let result = String::from_utf8(result).expect("Failed to convert report to utf8");
        error_reports.push(result);
    }

    return ParserOutput {
        root_node: parse_tree,
        error_reports,
    };
}

fn render_error_report<'a>(error: &ErrorType, with_color: bool) -> Report<SpanType> {
    let mut builder: ReportBuilder<SpanType> = Report::build(
        ReportKind::Error,
        error.span().context(),
        error.span().start(),
    )
    .with_config(Config::default().with_color(with_color));

    let error_color = if with_color { Color::Red } else { Color::Unset };

    builder = match error.reason() {
        SimpleReason::Custom(message) => {
            // use custom message as-is
            builder.with_message(message)
        }
        SimpleReason::Unclosed { span, delimiter } => {
            builder.add_label(
                Label::<SpanType>::new(span.to_owned())
                    .with_color(error_color)
                    .with_message("Unclosed delimiter"),
            );

            builder.with_message(format!(
                "Expected delimiter '{}' to be closed",
                delimiter.fg(error_color)
            ))
        }
        SimpleReason::Unexpected => {
            let found = if let Some(found) = error.found() {
                format!("'{}'", found.fg(error_color))
            } else {
                "end of input".to_string()
            };

            builder.add_label(
                Label::<SpanType>::new(error.span())
                    .with_color(error_color)
                    .with_message(format!("Found {found}.")),
            );

            let mut expected: Vec<&Option<char>> = error.expected().collect();
            expected.sort();

            let expected = if expected.len() == 0 {
                "something else".to_string()
            } else {
                expected
                    .iter()
                    .map(|expected| match expected {
                        Some(expected) => format!("'{}'", expected),
                        None => "end of input".to_string(),
                    })
                    .collect::<Vec<_>>()
                    .join(" or ")
            };

            builder.with_message(format!("Expected {expected}."))
        }
    };

    return builder.finish();
}

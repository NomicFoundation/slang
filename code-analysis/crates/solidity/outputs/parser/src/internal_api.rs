use ariadne::{Color, Config, Fmt, Label, Report, ReportKind, Source};
use chumsky::{Parser, Span, Stream};

use crate::generated::parse::{Context, ErrorType, ParserType, SpanType};

pub struct ParserOutput<TNode> {
    pub root_node: Option<TNode>,
    pub error_reports: Vec<String>,
}

pub fn parse<'a, TNode>(
    context: &'a Context<'a>,
    source: &'a str,
    parser: ParserType<'a, TNode>,
    with_color: bool,
) -> ParserOutput<TNode> {
    let input = Input::new(source, context);
    let stream = Stream::from_iter(input.eos(), input);

    let (parse_tree, errors) = parser.parse_recovery(stream);

    let mut error_reports = vec![];
    for error in &errors {
        let source = Source::from(source);
        let context = error.span().context();
        let report = render_error_report(&error, with_color);

        let mut result = vec![];
        report
            .write((context, source), &mut result)
            .expect("Failed to write report");

        let result = String::from_utf8(result).expect("Failed to convert report to utf8");
        error_reports.push(result);
    }

    return ParserOutput {
        root_node: parse_tree,
        error_reports,
    };
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
        match self.source.next() {
            Some(c) => {
                self.position += 1;
                Some((c, (self.context, self.position..(self.position + 1))))
            }
            None => None,
        }
    }
}

fn render_error_report<'a>(error: &ErrorType<'a>, with_color: bool) -> Report<SpanType<'a>> {
    let builder = Report::build(
        ReportKind::Error,
        error.span().context(),
        error.span().start(),
    );

    let builder = builder.with_config(Config::default().with_color(with_color));

    // TODO(OmarTawfik): also disable colors below if with_color is false

    let msg = if let chumsky::error::SimpleReason::Custom(msg) = error.reason() {
        msg.clone()
    } else {
        format!(
            "{}{}, expected {}",
            if error.found().is_some() {
                "Unexpected token"
            } else {
                "Unexpected end of input"
            },
            if let Some(label) = error.label() {
                format!(" while parsing {}", label)
            } else {
                String::new()
            },
            if error.expected().len() == 0 {
                "something else".to_string()
            } else {
                error
                    .expected()
                    .map(|expected| match expected {
                        Some(expected) => expected.to_string(),
                        None => "end of input".to_string(),
                    })
                    .collect::<Vec<_>>()
                    .join(" or ")
            },
        )
    };

    let builder = builder.with_code(3).with_message(msg).with_label(
        Label::new(error.span())
            .with_message(match error.reason() {
                chumsky::error::SimpleReason::Custom(msg) => msg.clone(),
                _ => format!(
                    "Unexpected {}",
                    error
                        .found()
                        .map(|c| format!("token {}", c.fg(Color::Red)))
                        .unwrap_or_else(|| "end of input".to_string())
                ),
            })
            .with_color(Color::Red),
    );

    let builder = match error.reason() {
        chumsky::error::SimpleReason::Unclosed { span, delimiter } => builder.with_label(
            Label::new(span.clone())
                .with_message(format!(
                    "Unclosed delimiter {}",
                    delimiter.fg(Color::Yellow)
                ))
                .with_color(Color::Yellow),
        ),
        chumsky::error::SimpleReason::Unexpected => builder,
        chumsky::error::SimpleReason::Custom(_) => builder,
    };

    return builder.finish();
}

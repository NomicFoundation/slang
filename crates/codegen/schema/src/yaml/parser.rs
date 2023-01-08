use std::{
    cmp::{max, min},
    path::PathBuf,
    str::Chars,
};

use codegen_utils::errors::{CodegenErrors, CodegenResult, Position};
use indexmap::IndexMap;
use yaml_rust::{
    parser::Parser as YamlParser,
    scanner::{Marker, TScalarStyle},
    Event,
};

use crate::yaml::cst;

pub struct Parser<'a> {
    file_path: &'a PathBuf,
    parser: YamlParser<Chars<'a>>,
}

impl<'a> Parser<'a> {
    pub fn run_parser(file_path: &'a PathBuf, source: &'a str) -> CodegenResult<cst::NodeRef> {
        let mut instance = Self {
            file_path,
            parser: YamlParser::new(source.chars()),
        };

        assert_eq!(instance.consume()?.event, Event::StreamStart);
        assert_eq!(instance.consume()?.event, Event::DocumentStart);

        let root = instance.parse_value()?;

        assert_eq!(instance.consume()?.event, Event::DocumentEnd);
        assert_eq!(instance.consume()?.event, Event::StreamEnd);

        return Ok(root);
    }

    fn parse_value(&mut self) -> CodegenResult<cst::NodeRef> {
        let Token {
            event: current,
            position: start,
        } = self.consume()?;

        let value = match current {
            Event::Scalar(value, style, ..) => {
                let contents = match style {
                    TScalarStyle::SingleQuoted => format!("'{value}'"),
                    TScalarStyle::DoubleQuoted => format!("\"{value}\""),
                    _ => value,
                };

                let lines: Vec<&str> = contents.lines().collect();
                let lines_count = lines.len();

                let end = if lines_count < 2 {
                    let line_length = contents.chars().count();
                    Position::new(
                        start.offset + line_length,
                        start.line,
                        start.column + line_length,
                    )
                } else {
                    let last_line_length = lines.last().unwrap().chars().count();
                    Position::new(
                        start.offset + last_line_length,
                        start.line + lines_count - 1,
                        last_line_length,
                    )
                };

                cst::Node::Value { range: start..end }
            }
            Event::SequenceStart(_) => {
                let mut items = Vec::new();

                let mut start = start;
                let mut end = loop {
                    if self.peek()?.event == Event::SequenceEnd {
                        break self.consume()?.position;
                    }

                    items.push(self.parse_value()?);
                };

                if !items.is_empty() {
                    start = min(start, items.first().unwrap().range().start);
                    end = max(end, items.last().unwrap().range().end);
                }

                cst::Node::Array {
                    range: start..end,
                    items,
                }
            }
            Event::MappingStart(_) => {
                let mut fields = IndexMap::new();

                let mut start = start;
                let mut end = loop {
                    let property_name = match self.peek()?.event {
                        Event::MappingEnd => {
                            break self.consume()?.position;
                        }
                        Event::Scalar(value, ..) => value,
                        _ => unreachable!("Unexpected property name"),
                    };

                    let key = self.parse_value()?;
                    let value = self.parse_value()?;
                    fields.insert(
                        property_name,
                        cst::NodeFieldRef::new(cst::NodeField { key, value }),
                    );
                };

                if !fields.is_empty() {
                    start = min(start, fields.first().unwrap().1.key.range().start);
                    end = max(end, fields.last().unwrap().1.value.range().end);
                }

                cst::Node::Object {
                    range: start..end,
                    fields,
                }
            }
            Event::Nothing
            | Event::Alias(_)
            | Event::DocumentEnd
            | Event::DocumentStart
            | Event::MappingEnd
            | Event::SequenceEnd
            | Event::StreamEnd
            | Event::StreamStart => {
                unreachable!("Unexpected event '{current:?}' at {start:?}")
            }
        };

        return Ok(cst::NodeRef::new(value));
    }

    fn consume(&mut self) -> CodegenResult<Token> {
        let token = self.peek()?;
        self.parser.next().unwrap(); // advance the iterator
        return Ok(token);
    }

    fn peek(&mut self) -> CodegenResult<Token> {
        let (event, marker) = self.parser.peek().map_err(|error| {
            let position = marker_to_position(error.marker());
            let range = position..position;

            return CodegenErrors::single(self.file_path, &range, Errors::Parser(error));
        })?;

        return Ok(Token {
            event: event.to_owned(),
            position: marker_to_position(marker),
        });
    }
}

struct Token {
    event: Event,
    position: Position,
}

fn marker_to_position(marker: &Marker) -> Position {
    return Position {
        offset: marker.index(),
        line: marker.line(),
        column: marker.col() + 1, // parser uses 0-based columns
    };
}

#[derive(thiserror::Error, Debug)]
enum Errors {
    #[error("{0}")]
    Parser(yaml_rust::ScanError),
}

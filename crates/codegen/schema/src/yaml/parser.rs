use std::cmp::{max, min};
use std::path::PathBuf;
use std::str::Chars;
use std::unreachable;

use anyhow::Result;
use indexmap::IndexMap;
use infra_utils::errors::{InfraErrors, Position};
use yaml_rust::parser::Parser as YamlParser;
use yaml_rust::scanner::{Marker, TScalarStyle};
use yaml_rust::Event;

use crate::yaml::{Node, NodeField, NodeFieldRef, NodeRef};

pub struct Parser<'context> {
    file_path: &'context PathBuf,
    parser: YamlParser<Chars<'context>>,
}

impl<'context> Parser<'context> {
    pub fn run_parser(file_path: &'context PathBuf, source: &'context str) -> Result<NodeRef> {
        let mut instance = Self {
            file_path,
            parser: YamlParser::new(source.chars()),
        };

        assert_eq!(instance.consume()?.event, Event::StreamStart);
        assert_eq!(instance.consume()?.event, Event::DocumentStart);

        let root = instance.parse_value()?;

        assert_eq!(instance.consume()?.event, Event::DocumentEnd);
        assert_eq!(instance.consume()?.event, Event::StreamEnd);

        Ok(root)
    }

    fn parse_value(&mut self) -> Result<NodeRef> {
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

                Node::Value { range: start..end }
            }
            Event::SequenceStart(_) => {
                let mut nodes = Vec::new();

                let mut start = start;
                let mut end = loop {
                    if self.peek()?.event == Event::SequenceEnd {
                        break self.consume()?.position;
                    }

                    nodes.push(self.parse_value()?);
                };

                if !nodes.is_empty() {
                    start = min(start, nodes.first().unwrap().range().start);
                    end = max(end, nodes.last().unwrap().range().end);
                }

                Node::Array {
                    range: start..end,
                    nodes,
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
                    fields.insert(property_name, NodeFieldRef::new(NodeField { key, value }));
                };

                if !fields.is_empty() {
                    start = min(start, fields.first().unwrap().1.key.range().start);
                    end = max(end, fields.last().unwrap().1.value.range().end);
                }

                Node::Object {
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

        Ok(NodeRef::new(value))
    }

    fn consume(&mut self) -> Result<Token> {
        let token = self.peek()?;
        self.parser.next().unwrap(); // advance the iterator
        Ok(token)
    }

    fn peek(&mut self) -> Result<Token> {
        let (event, marker) = self.parser.peek().map_err(|error| {
            let position = marker_to_position(error.marker());
            let range = position..position;

            InfraErrors::single(self.file_path.to_owned(), range, error.to_string())
        })?;

        Ok(Token {
            event: event.to_owned(),
            position: marker_to_position(marker),
        })
    }
}

struct Token {
    event: Event,
    position: Position,
}

fn marker_to_position(marker: &Marker) -> Position {
    Position {
        offset: marker.index(),
        line: marker.line(),
        column: marker.col() + 1, // parser uses 0-based columns
    }
}

use std::{collections::VecDeque, mem::discriminant};

use codegen_schema::types::{LanguageDefinition, Production, ProductionRef};
use semver::Version;

use crate::nodes::EbnfNode;

pub trait GenerateEbnf {
    fn generate_ebnf(&self) -> EbnfNode;
}

pub struct EbnfSerializer<'language> {
    language: &'language LanguageDefinition,
    base_production: &'language str,

    buffer: String,
    queue: VecDeque<(String, Option<String>, EbnfNode)>,
}

impl<'language> EbnfSerializer<'language> {
    pub fn serialize_version(
        language: &'language LanguageDefinition,
        production: &'language ProductionRef,
        version: &Version,
    ) -> Option<String> {
        let body = match production.as_ref() {
            Production::Scanner { version_map, .. } => {
                version_map.get_for_version(version)?.generate_ebnf()
            }
            Production::TriviaParser { version_map, .. } => {
                version_map.get_for_version(version)?.generate_ebnf()
            }
            Production::Parser { version_map, .. } => {
                version_map.get_for_version(version)?.generate_ebnf()
            }
            Production::PrecedenceParser { version_map, .. } => {
                version_map.get_for_version(version)?.generate_ebnf()
            }
        };

        let mut instance = Self {
            language,
            buffer: String::new(),
            base_production: production.name(),
            queue: VecDeque::new(),
        };

        instance
            .queue
            .push_back((instance.base_production.to_owned(), None, body));

        while let Some((name, comment, body)) = instance.queue.pop_front() {
            instance.serialize_statement(&name, &comment, &body);
        }

        return Some(instance.buffer.trim_end().to_owned());
    }

    fn serialize_statement(&mut self, name: &String, comment: &Option<String>, body: &EbnfNode) {
        self.buffer.push_str(&self.display_name(&name));
        self.buffer.push_str(" = ");

        match body {
            // Break long choices (like operators and keywords) over multiple lines:
            EbnfNode::Choices { choices } if choices.len() >= 5 => {
                let mut choices = choices.iter();

                self.serialize_node(choices.next().unwrap());

                let padding = " ".repeat(self.display_name(name).chars().count());
                for choice in choices {
                    self.buffer.push_str("\n");
                    self.buffer.push_str(&padding);
                    self.buffer.push_str(" | ");
                    self.serialize_node(choice);
                }
            }
            // Otherwise, just render on a single line as-is:
            _ => {
                self.serialize_node(&body);
            }
        };

        self.buffer.push_str(";");

        if let Some(comment) = comment {
            self.buffer.push_str(" (* ");
            self.buffer.push_str(comment);
            self.buffer.push_str(" *)");
        }

        self.buffer.push_str("\n");
    }

    fn serialize_node(&mut self, node: &EbnfNode) {
        match node {
            EbnfNode::Choices { choices } => {
                for (i, choice) in choices.into_iter().enumerate() {
                    if i > 0 {
                        self.buffer.push_str(" | ");
                    }

                    self.serialize_child_node(node, choice);
                }
            }
            EbnfNode::BaseProduction => {
                self.buffer.push_str(&self.base_production);
            }
            EbnfNode::Difference {
                minuend,
                subtrahend,
            } => {
                self.serialize_child_node(node, minuend);
                self.buffer.push_str(" - ");
                self.serialize_child_node(node, subtrahend);
            }
            EbnfNode::Not { body } => {
                self.buffer.push_str("!");
                self.serialize_child_node(node, body);
            }
            EbnfNode::OneOrMore { body } => {
                self.serialize_child_node(node, body);
                self.buffer.push_str("+");
            }
            EbnfNode::Optional { body } => {
                self.serialize_child_node(node, body);
                self.buffer.push_str("?");
            }
            EbnfNode::Range { from, to } => {
                self.buffer
                    .push_str(&format_string_literal(&from.to_string()));
                self.buffer.push_str("…");
                self.buffer
                    .push_str(&format_string_literal(&to.to_string()));
            }
            EbnfNode::ProductionRef { name } => {
                self.buffer.push_str(&self.display_name(name));
            }
            EbnfNode::Repeat { min, max, body } => {
                self.serialize_child_node(node, body);
                self.buffer.push_str("{");
                self.buffer.push_str(&min.to_string());
                self.buffer.push_str(",");
                self.buffer.push_str(&max.to_string());
                self.buffer.push_str("}");
            }
            EbnfNode::Sequence { elements } => {
                for (i, element) in elements.into_iter().enumerate() {
                    if i > 0 {
                        self.buffer.push_str(" ");
                    }

                    self.serialize_child_node(node, element);
                }
            }
            EbnfNode::SubStatement {
                name,
                comment,
                body,
            } => {
                self.buffer.push_str(&name);
                self.queue
                    .push_back((name.to_owned(), comment.to_owned(), (**body).to_owned()));
            }
            EbnfNode::Terminal { value } => {
                self.buffer.push_str(&format_string_literal(value));
            }
            EbnfNode::ZeroOrMore { body } => {
                self.serialize_child_node(node, body);
                self.buffer.push_str("*");
            }
        };
    }

    fn serialize_child_node(&mut self, parent: &EbnfNode, child: &EbnfNode) {
        if discriminant(parent) != discriminant(child) && precedence(child) <= precedence(parent) {
            self.buffer.push_str("(");
            self.serialize_node(child);
            self.buffer.push_str(")");
        } else {
            self.serialize_node(child);
        }
    }

    fn display_name(&self, name: &String) -> String {
        if let Some(production) = self.language.productions.get(name) {
            if matches!(production.as_ref(), Production::Scanner { .. }) {
                return format!("«{name}»");
            }
        }

        return name.to_owned();
    }
}

fn precedence(node: &EbnfNode) -> u8 {
    // We are specifying precedence "groups" instead of a flat list.
    // This separates members of the same precedence, like both "a b (c | d)" and "a | b | (c d)".
    return match node {
        // Binary
        EbnfNode::Choices { .. }
        | EbnfNode::Difference { .. }
        | EbnfNode::Range { .. }
        | EbnfNode::Sequence { .. } => 0,

        // Prefix
        EbnfNode::Not { .. } => 1,

        // Postfix
        EbnfNode::OneOrMore { .. }
        | EbnfNode::Optional { .. }
        | EbnfNode::Repeat { .. }
        | EbnfNode::ZeroOrMore { .. } => 2,

        // Primary
        EbnfNode::BaseProduction
        | EbnfNode::ProductionRef { .. }
        | EbnfNode::SubStatement { .. }
        | EbnfNode::Terminal { .. } => 3,
    };
}

fn format_string_literal(value: &String) -> String {
    let delimiter = if value.contains('"') && !value.contains("'") {
        '\''
    } else {
        '"'
    };

    let formatted: String = value
        .chars()
        .map(|c| match c {
            c if c == '\\' || c == delimiter => format!("\\{c}"),
            c if c == ' ' || c.is_ascii_graphic() => c.to_string(),
            '\t' => "\\t".to_string(),
            '\r' => "\\r".to_string(),
            '\n' => "\\n".to_string(),
            _ => {
                panic!(
                    "Unexpected character in string literal: '{c}'",
                    c = c.escape_unicode().to_string()
                );
            }
        })
        .collect();

    return format!("{delimiter}{formatted}{delimiter}");
}

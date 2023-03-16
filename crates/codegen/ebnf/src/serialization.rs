use std::collections::VecDeque;

use codegen_schema::types::{
    grammar::Grammar,
    production::{Production, ProductionRef},
};
use semver::Version;

use crate::nodes::EbnfNode;

pub trait GenerateEbnf {
    fn generate_ebnf(&self) -> EbnfNode;
}

pub struct EbnfSerializer<'grammar> {
    grammar: &'grammar Grammar,
    base_production: &'grammar str,

    buffer: String,
    queue: VecDeque<(String, EbnfNode)>,
}

impl<'grammar> EbnfSerializer<'grammar> {
    pub fn serialize_version(
        grammar: &'grammar Grammar,
        production: &'grammar ProductionRef,
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
            grammar,
            buffer: String::new(),
            base_production: production.name(),
            queue: VecDeque::new(),
        };

        instance
            .queue
            .push_back((instance.base_production.to_owned(), body));

        while let Some((name, body)) = instance.queue.pop_front() {
            instance.buffer.push_str(&instance.display_name(name));
            instance.buffer.push_str(" = ");
            instance.serialize_node(body);
            instance.buffer.push_str(";\n");
        }

        return Some(instance.buffer.trim_end().to_owned());
    }

    fn serialize_node(&mut self, node: EbnfNode) {
        match node {
            EbnfNode::Alternatives { alternatives } => {
                for (i, alternative) in alternatives.into_iter().enumerate() {
                    if i > 0 {
                        self.buffer.push_str(" | ");
                    }

                    self.serialize_node(alternative);
                }
            }
            EbnfNode::BaseProduction => {
                self.buffer.push_str(&self.base_production);
            }
            EbnfNode::Difference {
                minuend,
                subtrahend,
            } => {
                self.serialize_node(*minuend);
                self.buffer.push_str(" - ");
                self.serialize_node(*subtrahend);
            }
            EbnfNode::Not { body } => {
                self.buffer.push_str("¬");
                self.serialize_node(*body);
            }
            EbnfNode::OneOrMore { body } => {
                self.buffer.push_str("1");
                self.buffer.push_str("…");
                self.buffer.push_str("{");
                self.serialize_node(*body);
                self.buffer.push_str("}");
            }
            EbnfNode::Optional { body } => {
                self.buffer.push_str("[");
                self.serialize_node(*body);
                self.buffer.push_str("]");
            }
            EbnfNode::Parenthesis { body } => {
                self.buffer.push_str("(");
                self.serialize_node(*body);
                self.buffer.push_str(")");
            }
            EbnfNode::Range { from, to } => {
                self.buffer
                    .push_str(&format_string_literal(from.to_string()));
                self.buffer.push_str("…");
                self.buffer.push_str(&format_string_literal(to.to_string()));
            }
            EbnfNode::Reference { name } => {
                self.buffer.push_str(&self.display_name(name));
            }
            EbnfNode::Repeat { min, max, body } => {
                self.buffer.push_str(&min.to_string());
                self.buffer.push_str("…");
                self.buffer.push_str(&max.to_string());
                self.buffer.push_str("{");
                self.serialize_node(*body);
                self.buffer.push_str("}");
            }
            EbnfNode::Sequence { elements } => {
                for (i, element) in elements.into_iter().enumerate() {
                    if i > 0 {
                        self.buffer.push_str(" ");
                    }

                    self.serialize_node(element);
                }
            }
            EbnfNode::Statement { name, body } => {
                self.buffer.push_str(&name);
                self.queue.push_back((name, *body));
            }
            EbnfNode::Terminal { value } => {
                self.buffer.push_str(&format_string_literal(value));
            }
            EbnfNode::ZeroOrMore { body } => {
                self.buffer.push_str("{");
                self.serialize_node(*body);
                self.buffer.push_str("}");
            }
        };
    }

    fn display_name(&self, name: String) -> String {
        if let Some(production) = self.grammar.productions.get(&name) {
            if matches!(production.as_ref(), Production::Scanner { .. }) {
                return format!("«{name}»");
            }
        }

        return name;
    }
}

fn format_string_literal(value: String) -> String {
    let delimiter = if value.len() == 1 {
        if value.contains("'") && !value.contains('"') {
            '"'
        } else {
            '\''
        }
    } else {
        if value.contains('"') && !value.contains("'") {
            '\''
        } else {
            '"'
        }
    };

    let formatted: String = value
        .chars()
        .map(|c| {
            if c == '\'' || c == '\\' {
                format!("\\{c}")
            } else if c.is_ascii_graphic() || c == '¬' || c == '…' || c == '«' || c == '»' {
                c.to_string()
            } else {
                c.escape_unicode().to_string()
            }
        })
        .collect();

    return format!("{delimiter}{formatted}{delimiter}");
}

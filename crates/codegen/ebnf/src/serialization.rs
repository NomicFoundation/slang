use std::mem::discriminant;

use codegen_schema::types::{LanguageDefinitionRef, ProductionDefinition, ProductionRef};
use indexmap::IndexSet;
use inflector::Inflector;
use semver::Version;

use crate::nodes::EbnfNode;

pub trait GenerateEbnf {
    fn generate_ebnf(&self) -> EbnfNode;
}

pub struct EbnfSerializer {
    language: LanguageDefinitionRef,
    base_production: ProductionRef,

    buffer: String,
    queue: IndexSet<QueuedStatement>,
}

#[derive(Eq, Hash, PartialEq)]
struct QueuedStatement {
    name: String,
    comment: Option<String>,
    root_node: EbnfNode,
}

impl EbnfSerializer {
    pub fn serialize_version(
        language: &LanguageDefinitionRef,
        production: &ProductionRef,
        version: &Version,
    ) -> Option<String> {
        let root_node = match &production.definition {
            ProductionDefinition::Scanner { version_map, .. } => {
                version_map.get_for_version(version)?.generate_ebnf()
            }
            ProductionDefinition::TriviaParser { version_map, .. } => {
                version_map.get_for_version(version)?.generate_ebnf()
            }
            ProductionDefinition::Parser { version_map, .. } => {
                version_map.get_for_version(version)?.generate_ebnf()
            }
            ProductionDefinition::PrecedenceParser { version_map, .. } => {
                version_map.get_for_version(version)?.generate_ebnf()
            }
        };

        let mut instance = Self {
            language: language.to_owned(),
            base_production: production.to_owned(),

            buffer: String::new(),
            queue: IndexSet::new(),
        };

        instance.queue.insert(QueuedStatement {
            name: instance.base_production.name.to_owned(),
            comment: None,
            root_node,
        });

        while let Some(statement) = instance.queue.shift_remove_index(0) {
            instance.serialize_statement(&statement);
        }

        return Some(instance.buffer.trim_end().to_owned());
    }

    fn serialize_statement(&mut self, statement: &QueuedStatement) {
        self.buffer.push_str(&self.display_name(&statement.name));
        self.buffer.push_str(" = ");

        // Naive version of formatting for long EBNF statements.
        // Long lines are usually choices of references (PrecedenceParser) or keywords (Scanner).
        // Let's just print one reference per line:
        if !self.serialize_top_node(statement) {
            // Otherwise, just print the entire thing on the same line:
            self.serialize_node(&statement.root_node);
        }

        self.buffer.push_str(";");

        if let Some(comment) = &statement.comment {
            self.buffer.push_str(" (* ");
            self.buffer.push_str(comment);
            self.buffer.push_str(" *)");
        }

        self.buffer.push_str("\n");
    }

    fn serialize_top_node(&mut self, statement: &QueuedStatement) -> bool {
        let choices = match &statement.root_node {
            EbnfNode::Choice { nodes } if nodes.len() >= 3 => nodes,
            _ => {
                // Skip if not choices, or less than three choices.
                return false;
            }
        };

        // Use `IndexSet` to deduplicate choices in case of multiple operators:
        let mut references = IndexSet::new();
        let mut sub_statements = IndexSet::new();

        for choice in choices {
            match choice {
                EbnfNode::BaseProduction => {
                    references.insert(self.display_name(&self.base_production.name));
                }
                EbnfNode::ProductionRef { name } => {
                    references.insert(self.display_name(name));
                }

                EbnfNode::SubStatement {
                    name,
                    comment,
                    root_node,
                } => {
                    references.insert(self.display_name(name));
                    sub_statements.insert(QueuedStatement {
                        name: name.to_owned(),
                        comment: comment.to_owned(),
                        root_node: (**root_node).to_owned(),
                    });
                }
                EbnfNode::Terminal { value } => {
                    references.insert(format_string_literal(value));
                }
                EbnfNode::Choice { .. }
                | EbnfNode::Difference { .. }
                | EbnfNode::Not { .. }
                | EbnfNode::OneOrMore { .. }
                | EbnfNode::Optional { .. }
                | EbnfNode::Range { .. }
                | EbnfNode::Sequence { .. }
                | EbnfNode::ZeroOrMore { .. } => {
                    // Skip if choice produces anything other than an identifier.
                    return false;
                }
            };
        }

        let padding = " ".repeat(self.display_name(&statement.name).chars().count());

        for (i, reference) in references.iter().enumerate() {
            if i > 0 {
                self.buffer.push_str("\n");
                self.buffer.push_str(&padding);
                self.buffer.push_str(" | ");
            }

            self.buffer.push_str(reference);
        }

        self.queue.extend(sub_statements.into_iter());

        return true;
    }

    fn serialize_node(&mut self, top_node: &EbnfNode) {
        match top_node {
            EbnfNode::Choice { nodes } => {
                for (i, node) in nodes.into_iter().enumerate() {
                    if i > 0 {
                        self.buffer.push_str(" | ");
                    }

                    self.serialize_child_node(top_node, node);
                }
            }
            EbnfNode::BaseProduction => {
                self.buffer
                    .push_str(&self.display_name(&self.base_production.name));
            }
            EbnfNode::Difference {
                minuend,
                subtrahend,
            } => {
                self.serialize_child_node(top_node, minuend);
                self.buffer.push_str(" - ");
                self.serialize_child_node(top_node, subtrahend);
            }
            EbnfNode::Not { node } => {
                self.buffer.push_str("!");
                self.serialize_child_node(top_node, node);
            }
            EbnfNode::OneOrMore { node } => {
                self.serialize_child_node(top_node, node);
                self.buffer.push_str("+");
            }
            EbnfNode::Optional { node } => {
                self.serialize_child_node(top_node, node);
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
            EbnfNode::Sequence { nodes } => {
                for (i, node) in nodes.into_iter().enumerate() {
                    if i > 0 {
                        self.buffer.push_str(" ");
                    }

                    self.serialize_child_node(top_node, node);
                }
            }
            EbnfNode::SubStatement {
                name,
                comment,
                root_node,
            } => {
                self.buffer.push_str(&self.display_name(name));

                self.queue.insert(QueuedStatement {
                    name: name.to_owned(),
                    comment: comment.to_owned(),
                    root_node: (**root_node).to_owned(),
                });
            }
            EbnfNode::Terminal { value } => {
                self.buffer.push_str(&format_string_literal(value));
            }
            EbnfNode::ZeroOrMore { node } => {
                self.serialize_child_node(top_node, node);
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
        let mut name = name.to_owned();

        let production = match self.language.productions.get(&name) {
            Some(production) => production,
            None => {
                // Not a top-level production, so it is an named parser.
                // Therefore, it is neither inlined nor a scanner. Return name as-is:
                return name;
            }
        };

        if matches!(production.definition, ProductionDefinition::Scanner { .. }) {
            name = name.to_screaming_snake_case();
        }

        if production.inlined {
            name = format!("«{name}»");
        }

        return name;
    }
}

fn precedence(node: &EbnfNode) -> u8 {
    // We are specifying precedence "groups" instead of a flat list.
    // This separates members of the same precedence, like both "a b (c | d)" and "a | b | (c d)".
    return match node {
        // Binary
        EbnfNode::Choice { .. } | EbnfNode::Difference { .. } | EbnfNode::Sequence { .. } => 0,

        // Prefix
        EbnfNode::Not { .. } => 1,

        // Postfix
        EbnfNode::OneOrMore { .. } | EbnfNode::Optional { .. } | EbnfNode::ZeroOrMore { .. } => 2,

        // Primary
        EbnfNode::BaseProduction
        | EbnfNode::ProductionRef { .. }
        | EbnfNode::Range { .. }
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

use std::mem::discriminant;

use codegen_schema::types::{LanguageDefinitionRef, ProductionDefinition, ProductionRef};
use indexmap::IndexMap;
use inflector::Inflector;
use semver::Version;

use crate::legacy::nodes::EbnfNode;

const MAX_LINE_WIDTH: usize = 80;

/// Serializes [`LanguageDefinition`](`codegen_schema::types::LanguageDefinition`) into EBNF snippets.
pub struct EbnfSerializer {
    language: LanguageDefinitionRef,
    outputs: IndexMap<String, String>,
}

impl EbnfSerializer {
    /// Returns `None` if version is not found.
    /// Otherwise, returns a map of LHS names to EBNF snippets.
    /// Some productions (like ([`PrecedenceParser`](`ProductionDefinition::PrecedenceParser`)) generate more than one statement.
    pub fn serialize_version(
        language: &LanguageDefinitionRef,
        production: &ProductionRef,
        version: &Version,
    ) -> Option<IndexMap<String, String>> {
        let mut instance = Self {
            language: language.to_owned(),
            outputs: IndexMap::new(),
        };

        // Reserve the first index for the root production, in case it added more:
        instance
            .outputs
            .insert(production.name.to_owned(), String::new());

        let root_node = match &production.definition {
            ProductionDefinition::Scanner { version_map, .. } => {
                EbnfNode::from_scanner(&version_map.get_for_version(version)?)
            }
            ProductionDefinition::TriviaParser { version_map, .. } => {
                EbnfNode::from_parser(&version_map.get_for_version(version)?)
            }
            ProductionDefinition::Parser { version_map, .. } => {
                EbnfNode::from_parser(&version_map.get_for_version(version)?)
            }
            ProductionDefinition::PrecedenceParser { version_map, .. } => {
                EbnfNode::from_precedence_parser(
                    &version_map.get_for_version(version)?,
                    &production.name,
                    &mut instance,
                )
            }
        };

        instance.serialize_statement(&production.name, &root_node);

        Some(instance.outputs)
    }

    /// Serializes a single EBNF statement.
    ///
    /// Example:
    /// ```ebnf
    /// UNSIGNED_INTEGER_TYPE = "uint" «INTEGER_TYPE_SIZE»?;
    /// ```
    pub fn serialize_statement(&mut self, name: &str, root_node: &EbnfNode) {
        let mut buffer = String::new();

        buffer.push_str(&self.display_name(name));
        buffer.push_str(" = ");
        buffer.push_str(&self.serialize_root_node(name, root_node));
        buffer.push(';');

        match self.outputs.get_mut(name) {
            Some(existing) => {
                if !existing.is_empty() {
                    existing.push('\n');
                }

                existing.push_str(&buffer);
            }
            _ => {
                self.outputs.insert(name.to_owned(), buffer);
            }
        };
    }

    /// Naive version of formatting for long EBNF statements.
    /// Tries to break long lines, which are usually choices of references
    /// ([`PrecedenceParser`](ProductionDefinition::PrecedenceParser)) or keywords ([`Scanner`](ProductionDefinition::Scanner)).
    /// Otherwise, prints everything on a single line.
    fn serialize_root_node(&mut self, name: &str, root_node: &EbnfNode) -> String {
        let EbnfNode::Choice { nodes: choices } = &root_node else {
            // Not a choice: Just flush everything on a single line:
            let mut buffer = String::new();
            self.serialize_node(root_node, &mut buffer);
            return buffer;
        };

        let choices = choices
            .iter()
            .map(|choice| {
                let mut buffer = String::new();
                self.serialize_node(choice, &mut buffer);
                buffer
            })
            .collect::<Vec<_>>();

        let name_width = self.display_name(name).chars().count();
        let choices_width = choices.iter().map(|choice| choice.len() + 3).sum::<usize>();

        if name_width + choices_width < MAX_LINE_WIDTH {
            // Everything fits on a single line. No need to split:
            let mut buffer = String::new();
            self.serialize_node(root_node, &mut buffer);
            return buffer;
        }

        // Otherwise, break into multiple lines:
        choices.join(&format!("\n{padding} | ", padding = " ".repeat(name_width)))
    }

    /// Serialize and append an EBNF node to the buffer.
    pub fn serialize_node(&mut self, top_node: &EbnfNode, buffer: &mut String) {
        match top_node {
            EbnfNode::Choice { nodes } => {
                for (i, node) in nodes.iter().enumerate() {
                    if i > 0 {
                        buffer.push_str(" | ");
                    }

                    self.serialize_child_node(top_node, node, buffer);
                }
            }
            EbnfNode::Difference {
                minuend,
                subtrahend,
            } => {
                self.serialize_child_node(top_node, minuend, buffer);
                buffer.push_str(" - ");
                self.serialize_child_node(top_node, subtrahend, buffer);
            }
            EbnfNode::Not { node } => {
                buffer.push('!');
                self.serialize_child_node(top_node, node, buffer);
            }
            EbnfNode::OneOrMore { node } => {
                self.serialize_child_node(top_node, node, buffer);
                buffer.push('+');
            }
            EbnfNode::Optional { node } => {
                self.serialize_child_node(top_node, node, buffer);
                buffer.push('?');
            }
            EbnfNode::Range { from, to } => {
                buffer.push_str(&format_string_literal(&from.to_string()));
                buffer.push('…');
                buffer.push_str(&format_string_literal(&to.to_string()));
            }
            EbnfNode::ProductionRef { name } => {
                buffer.push_str(&self.display_name(name));
            }
            EbnfNode::Sequence { nodes } => {
                for (i, node) in nodes.iter().enumerate() {
                    if i > 0 {
                        buffer.push(' ');
                    }

                    self.serialize_child_node(top_node, node, buffer);
                }
            }
            EbnfNode::Terminal { terminal: value } => {
                buffer.push_str(&format_string_literal(value));
            }
            EbnfNode::WithComment { node, comment } => {
                self.serialize_child_node(top_node, node, buffer);
                buffer.push_str(" (* ");
                buffer.push_str(comment);
                buffer.push_str(" *)");
            }
            EbnfNode::ZeroOrMore { node } => {
                self.serialize_child_node(top_node, node, buffer);
                buffer.push('*');
            }
        };
    }

    fn serialize_child_node(&mut self, parent: &EbnfNode, child: &EbnfNode, buffer: &mut String) {
        if discriminant(parent) != discriminant(child) && child.precedence() <= parent.precedence()
        {
            buffer.push('(');
            self.serialize_node(child, buffer);
            buffer.push(')');
        } else {
            self.serialize_node(child, buffer);
        }
    }

    fn display_name(&self, name: &str) -> String {
        let mut name = name.to_owned();

        let Some(production) = self.language.productions.get(&name) else {
            // Not a top-level production, so it is an named parser.
            // Therefore, it is neither inlined nor a scanner. Return name as-is:
            return name;
        };

        if matches!(production.definition, ProductionDefinition::Scanner { .. }) {
            name = name.to_screaming_snake_case();
        }

        if production.inlined {
            name = format!("«{name}»");
        }

        name
    }
}

fn format_string_literal(value: &str) -> String {
    let delimiter = if value.contains('"') && !value.contains('\'') {
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
                    c = c.escape_unicode()
                );
            }
        })
        .collect();

    format!("{delimiter}{formatted}{delimiter}")
}

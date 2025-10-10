use std::fmt::Display;
use std::mem::discriminant;

use language_definition::model::Identifier;

use crate::model::{Definition, DefinitionKind, Entry, Expression, Value};
use crate::{EbnfModel, EbnfWriter};

pub struct Serializer<'s, W: EbnfWriter> {
    model: &'s EbnfModel,
    writer: &'s mut W,
}

impl<'s, W: EbnfWriter> Serializer<'s, W> {
    pub fn serialize(
        model: &'s EbnfModel,
        name: &Identifier,
        writer: &'s mut W,
    ) -> std::fmt::Result {
        let entry = model
            .entries
            .get(name)
            .unwrap_or_else(|| panic!("Entry not defined: '{name}'."));

        Self { model, writer }.serialize_entry(entry)
    }

    fn serialize_entry(&mut self, entry: &Entry) -> std::fmt::Result {
        let Entry {
            name,
            ebnf_id: _,
            section_index: _,
            topic_index: _,
            definitions,
        } = entry;

        for (index, definition) in definitions.iter().enumerate() {
            if index > 0 {
                // Insert a blank line between definitions:
                self.writer.line_break()?;
                self.writer.line_break()?;
            }

            let Definition {
                leading_comments,
                values,
                kind,
            } = definition;

            for comment in leading_comments {
                self.serialize_comment(comment)?;
                self.writer.line_break()?;
            }

            self.serialize_definition(name, values, kind)?;
        }

        Ok(())
    }

    fn serialize_definition(
        &mut self,
        name: &Identifier,
        values: &[Value],
        kind: &DefinitionKind,
    ) -> std::fmt::Result {
        let separator = match kind {
            DefinitionKind::Sequence => ' ',
            DefinitionKind::Choice => '|',
        };

        for (index, value) in values.iter().enumerate() {
            let Value {
                expression,
                trailing_comment,
            } = value;

            if index == 0 {
                self.serialize_identifier(name)?;
                self.serialize_punctuation(" = ")?;
            } else {
                self.writer.line_break()?;

                self.serialize_punctuation(format!(
                    "{padding} {separator} ",
                    padding = " ".repeat(name.len()),
                ))?;
            }

            self.serialize_expr(expression)?;

            if index + 1 == values.len() {
                self.serialize_punctuation(";")?;
            }

            if let Some(comment) = trailing_comment {
                self.serialize_punctuation(" ")?;
                self.serialize_comment(comment)?;
            }
        }

        Ok(())
    }

    fn serialize_expr(&mut self, parent: &Expression) -> std::fmt::Result {
        match parent {
            Expression::Sequence { expressions } => {
                self.serialize_child_expr(parent, &expressions[0])?;

                for expression in expressions.iter().skip(1) {
                    self.serialize_punctuation(" ")?;
                    self.serialize_child_expr(parent, expression)?;
                }
            }
            Expression::Choice { expressions } => {
                self.serialize_child_expr(parent, &expressions[0])?;

                for expression in expressions.iter().skip(1) {
                    self.serialize_punctuation(" | ")?;
                    self.serialize_child_expr(parent, expression)?;
                }
            }
            Expression::Optional { expression } => {
                self.serialize_child_expr(parent, expression)?;
                self.serialize_punctuation("?")?;
            }
            Expression::ZeroOrMore { expression } => {
                self.serialize_child_expr(parent, expression)?;
                self.serialize_punctuation("*")?;
            }
            Expression::OneOrMore { expression } => {
                self.serialize_child_expr(parent, expression)?;
                self.serialize_punctuation("+")?;
            }
            Expression::Not { expression } => {
                self.serialize_punctuation("!")?;
                self.serialize_child_expr(parent, expression)?;
            }
            Expression::Range {
                inclusive_start,
                inclusive_end,
            } => {
                self.serialize_expr(inclusive_start)?;
                self.serialize_punctuation("…")?;
                self.serialize_expr(inclusive_end)?;
            }
            Expression::Atom { atom } => {
                self.serialize_string_literal(atom)?;
            }
            Expression::NegativeLookAhead { expression } => {
                self.serialize_punctuation("(?!")?;
                self.serialize_expr(expression)?;
                self.serialize_punctuation(")")?;
            }
            Expression::Reference {
                leading_comment,
                reference,
            } => {
                if let Some(comment) = leading_comment {
                    self.serialize_comment(comment)?;
                    self.serialize_punctuation(" ")?;
                }
                self.serialize_identifier(reference)?;
            }
        }

        Ok(())
    }

    fn serialize_child_expr(
        &mut self,
        parent: &Expression,
        child: &Expression,
    ) -> std::fmt::Result {
        if discriminant(parent) != discriminant(child) && child.precedence() <= parent.precedence()
        {
            self.serialize_punctuation("(")?;
            self.serialize_expr(child)?;
            self.serialize_punctuation(")")?;
        } else {
            self.serialize_expr(child)?;
        }

        Ok(())
    }

    fn serialize_comment(&mut self, value: impl Display) -> std::fmt::Result {
        self.writer.write_comment(format!("(* {value} *)"))
    }

    fn serialize_identifier(&mut self, name: &Identifier) -> std::fmt::Result {
        let entry = self
            .model
            .entries
            .get(name)
            .unwrap_or_else(|| panic!("Entry not defined: '{name}'."));

        self.writer.write_identifier(&entry.ebnf_id, &entry.name)
    }

    fn serialize_punctuation(&mut self, value: impl Display) -> std::fmt::Result {
        self.writer.write_punctuation(value)
    }

    fn serialize_string_literal(&mut self, value: &str) -> std::fmt::Result {
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

        self.writer
            .write_string_literal(format!("{delimiter}{formatted}{delimiter}"))
    }
}

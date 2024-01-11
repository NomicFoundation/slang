use std::mem::discriminant;

use crate::model::{Definition, DefinitionKind, Entry, Expression, Value};
use crate::{EbnfModel, EbnfWriter};

pub struct Serializer<'t, W: EbnfWriter> {
    model: &'t EbnfModel,
    writer: &'t mut W,
}

impl<'t, W: EbnfWriter> Serializer<'t, W> {
    pub fn serialize<'m: 't>(model: &'m EbnfModel, name: &str, writer: &'m mut W) {
        let entry = model
            .entries
            .get(name)
            .unwrap_or_else(|| panic!("Entry not defined: '{name}'."));

        Self { model, writer }.serialize_entry(entry);
    }

    fn serialize_entry(&mut self, entry: &Entry) {
        let Entry {
            name,
            ebnf_id: _,
            definitions,
        } = entry;

        for (index, definition) in definitions.iter().enumerate() {
            if index > 0 {
                // Insert a blank line between definitions:
                self.writer.start_line();
                self.writer.end_line();
            }

            let Definition {
                leading_comments,
                values,
                kind,
            } = definition;

            for comment in leading_comments {
                self.writer.start_line();
                self.serialize_comment(comment);
                self.writer.end_line();
            }

            self.serialize_definition(name, values, kind);
        }
    }

    fn serialize_definition(&mut self, name: &str, values: &Vec<Value>, kind: &DefinitionKind) {
        let separator = match kind {
            DefinitionKind::Sequence => ' ',
            DefinitionKind::Choice => '|',
        };

        for (index, value) in values.iter().enumerate() {
            let Value {
                expression,
                trailing_comment,
            } = value;

            self.writer.start_line();

            if index == 0 {
                self.serialize_identifier(name);
                self.serialize_punctuation(" = ");
            } else {
                self.serialize_punctuation(&format!(
                    "{padding} {separator} ",
                    padding = " ".repeat(name.len()),
                ));
            }

            self.serialize_expr(expression);

            if index + 1 == values.len() {
                self.serialize_punctuation(";");
            }

            if let Some(comment) = trailing_comment {
                self.serialize_punctuation(" ");
                self.serialize_comment(comment);
            }

            self.writer.end_line();
        }
    }

    fn serialize_expr(&mut self, parent: &Expression) {
        match parent {
            Expression::Sequence { expressions } => {
                self.serialize_child_expr(parent, &expressions[0]);

                for expression in expressions.iter().skip(1) {
                    self.serialize_punctuation(" ");
                    self.serialize_child_expr(parent, expression);
                }
            }
            Expression::Choice { expressions } => {
                self.serialize_child_expr(parent, &expressions[0]);

                for expression in expressions.iter().skip(1) {
                    self.serialize_punctuation(" | ");
                    self.serialize_child_expr(parent, expression);
                }
            }
            Expression::Optional { expression } => {
                self.serialize_child_expr(parent, expression);
                self.serialize_punctuation("?");
            }
            Expression::ZeroOrMore { expression } => {
                self.serialize_child_expr(parent, expression);
                self.serialize_punctuation("*");
            }
            Expression::OneOrMore { expression } => {
                self.serialize_child_expr(parent, expression);
                self.serialize_punctuation("+");
            }
            Expression::Not { expression } => {
                self.serialize_punctuation("!");
                self.serialize_child_expr(parent, expression);
            }
            Expression::Range {
                inclusive_start,
                inclusive_end,
            } => {
                self.serialize_child_expr(parent, inclusive_start);
                self.serialize_punctuation("…");
                self.serialize_child_expr(parent, inclusive_end);
            }
            Expression::Atom { atom } => {
                self.serialize_string_literal(atom);
            }
            Expression::Reference { reference } => {
                self.serialize_identifier(reference);
            }
        };
    }

    fn serialize_child_expr(&mut self, parent: &Expression, child: &Expression) {
        if discriminant(parent) != discriminant(child) && child.precedence() <= parent.precedence()
        {
            self.serialize_punctuation("(");
            self.serialize_expr(child);
            self.serialize_punctuation(")");
        } else {
            self.serialize_expr(child);
        }
    }

    fn serialize_comment(&mut self, value: &String) {
        self.writer.write_comment(format!("(* {value} *)"));
    }

    fn serialize_identifier(&mut self, name: &str) {
        let value = &self.model.entries[name].ebnf_id;
        self.writer.write_identifier(value, name);
    }

    fn serialize_punctuation(&mut self, value: &str) {
        self.writer.write_punctuation(value);
    }

    fn serialize_string_literal(&mut self, value: &str) {
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
            .write_string_literal(format!("{delimiter}{formatted}{delimiter}"));
    }
}

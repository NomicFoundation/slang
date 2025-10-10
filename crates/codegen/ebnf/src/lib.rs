//! Provides an [`EbnfModel`] that converts [`language_definition`] items into EBNF blocks of tokens
//! that can be serialized into strings (for doc comments) or HTML (for the human readable spec).

mod builder;
mod model;
mod serializer;

use std::collections::HashMap;
use std::fmt::{Display, Write};

use language_definition::model::{Identifier, Language};

use crate::builder::Builder;
use crate::model::Entry;
use crate::serializer::Serializer;

pub struct EbnfModel {
    entries: HashMap<Identifier, Entry>,
}

impl EbnfModel {
    pub fn build(language: &Language) -> Self {
        Self {
            entries: Builder::build(language),
        }
    }

    pub fn serialize(&self, name: &Identifier, writer: &mut impl EbnfWriter) -> std::fmt::Result {
        Serializer::serialize(self, name, writer)
    }

    pub fn locate(&self, name: &Identifier) -> (usize, usize) {
        let entry = self
            .entries
            .get(name)
            .unwrap_or_else(|| panic!("Entry not defined: '{name}'."));

        (entry.section_index, entry.topic_index)
    }
}

/// There can be many consumers that want to serialize EBNF, but they fall into two categories:
///
/// - The general use case: serializing to a plain string, for doc comments, or plain `.ebnf` files.
/// - More specialized use cases: like syntax highlighting in the human readable spec, that uses 'span' and 'a' HTML tags.
///
/// For the general use case, we introduce the reusable [`PlainWriter`] implementation below.
pub trait EbnfWriter {
    fn line_break(&mut self) -> std::fmt::Result;

    fn write_comment(&mut self, value: impl Display) -> std::fmt::Result;
    fn write_punctuation(&mut self, value: impl Display) -> std::fmt::Result;
    fn write_string_literal(&mut self, value: impl Display) -> std::fmt::Result;

    fn write_identifier(&mut self, value: impl Display, name: &Identifier) -> std::fmt::Result;
}

/// A reusable [`EbnfWriter`] implementation that serializes to a plain string.
/// Suitable for usage in doc comments, or plain `.ebnf` files.
#[derive(Default)]
pub struct PlainWriter {
    buffer: String,
}

impl PlainWriter {
    pub fn flush(&mut self) -> String {
        std::mem::take(&mut self.buffer)
    }
}

impl EbnfWriter for PlainWriter {
    fn line_break(&mut self) -> std::fmt::Result {
        writeln!(self.buffer)
    }

    fn write_comment(&mut self, value: impl Display) -> std::fmt::Result {
        write!(self.buffer, "{value}")
    }

    fn write_punctuation(&mut self, value: impl Display) -> std::fmt::Result {
        write!(self.buffer, "{value}")
    }

    fn write_string_literal(&mut self, value: impl Display) -> std::fmt::Result {
        write!(self.buffer, "{value}")
    }

    fn write_identifier(&mut self, value: impl Display, _name: &Identifier) -> std::fmt::Result {
        write!(self.buffer, "{value}")
    }
}

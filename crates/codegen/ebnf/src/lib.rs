//! Provides an [`EbnfModel`] that converts [`codegen_language_definition`] items into EBNF blocks of tokens
//! that can be serialized into strings (for doc comments) or HTML (for the human readable spec).

mod builder;
mod model;
mod serializer;

pub mod legacy;

use std::collections::HashMap;
use std::fmt::{Display, Write};

use codegen_language_definition::model::Language;

use crate::builder::Builder;
use crate::model::Entry;
use crate::serializer::Serializer;

pub struct EbnfModel {
    entries: HashMap<String, Entry>,
}

impl EbnfModel {
    pub fn build(language: &Language) -> Self {
        Self {
            entries: Builder::build(language),
        }
    }

    pub fn serialize(&self, name: &str, writer: &mut impl EbnfWriter) -> std::fmt::Result {
        Serializer::serialize(self, name, writer)
    }
}

/// There can be many consumers that want to serialize EBNF, but they fall into two categories:
///
/// - The general use case: serializing to a plain string, for doc comments, or plain `.ebnf` files.
/// - More specialized use cases: like syntax highlighting in the human readable spec, that uses '<span>' and '<a>' HTML tags.
///
/// For the general use case, we introduce the reusable [`PlainEbnfWriter`] implementation below.
pub trait EbnfWriter {
    fn start_line(&mut self) -> std::fmt::Result;
    fn end_line(&mut self) -> std::fmt::Result;

    fn write_comment(&mut self, value: impl Display) -> std::fmt::Result;
    fn write_identifier(&mut self, value: impl Display, entry_name: &str) -> std::fmt::Result;
    fn write_punctuation(&mut self, value: impl Display) -> std::fmt::Result;
    fn write_string_literal(&mut self, value: impl Display) -> std::fmt::Result;
}

/// A reusable [`EbnfWriter`] implementation that serializes to a plain string.
/// Suitable for usage in doc comments, or plain `.ebnf` files.
#[derive(Default)]
pub struct PlainEbnfWriter {
    buffer: String,
}

impl EbnfWriter for PlainEbnfWriter {
    fn start_line(&mut self) -> std::fmt::Result {
        Ok(()) // No-op
    }

    fn end_line(&mut self) -> std::fmt::Result {
        writeln!(self.buffer)
    }

    fn write_comment(&mut self, value: impl Display) -> std::fmt::Result {
        write!(self.buffer, "{value}")
    }

    fn write_identifier(&mut self, value: impl Display, _: &str) -> std::fmt::Result {
        write!(self.buffer, "{value}")
    }

    fn write_punctuation(&mut self, value: impl Display) -> std::fmt::Result {
        write!(self.buffer, "{value}")
    }

    fn write_string_literal(&mut self, value: impl Display) -> std::fmt::Result {
        write!(self.buffer, "{value}")
    }
}

impl Display for PlainEbnfWriter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{buffer}", buffer = self.buffer)
    }
}

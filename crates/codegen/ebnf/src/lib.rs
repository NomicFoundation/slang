//! Provides an [`legacy::EbnfSerializer`] type that converts [`codegen_language_definition`] items into EBNF blocks of tokens
//! that can be serialized into strings (for doc comments) or HTML (for the human readable  spec).

mod builder;
mod model;
mod serializer;

pub mod legacy;

use std::collections::HashMap;

use codegen_language_definition::model::Language;

use crate::builder::Builder;
use crate::model::Entry;
use crate::serializer::Serializer;

pub struct EbnfModel {
    entries: HashMap<String, Entry>,
}

pub trait EbnfWriter {
    fn start_line(&mut self);
    fn end_line(&mut self);

    fn write_comment(&mut self, value: String);
    fn write_identifier(&mut self, value: &str, name: &str);
    fn write_punctuation(&mut self, value: &str);
    fn write_string_literal(&mut self, value: String);
}

impl EbnfModel {
    pub fn build(language: &Language) -> Self {
        Self {
            entries: Builder::build(language),
        }
    }

    pub fn serialize(&self, name: &str, writer: &mut impl EbnfWriter) {
        Serializer::serialize(self, name, writer);
    }
}

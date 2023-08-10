//! Provides an [`EbnfSerializer`], which allows to serialize a [`LanguageDefinition`](`codegen_schema::types::LanguageDefinition`)
//! into EBNF snippets.
//!
//! Used for documentation and for comments in the generated parser code.
mod nodes;
mod parser;
mod precedence_parser;
mod scanner;
mod serialization;

pub use serialization::EbnfSerializer;

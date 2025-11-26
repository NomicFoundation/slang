mod item;
mod parser_builder;

use language_v2_definition::model::{Identifier, Language, Section, Topic};
use serde::Serialize;

use crate::lexer::LexerModel;
use crate::parser::item::LALRPOPItem;
use crate::parser::parser_builder::ParserBuilder;

/// A parser model used while generating the parser (and the lexer).
#[derive(Clone, Debug, Serialize)]
pub struct ParserModel {
    pub lexer: LexerModel,

    pub root_item: Identifier,
    sections: Vec<ParserSection>,
}

/// A section of the language within the parser.
///
/// Note: This division is not needed, for now we're using it so the generated parser 'looks' like the language definition.
#[derive(Clone, Debug, Serialize)]
struct ParserSection {
    pub title: String,
    pub topics: Vec<ParserTopic>,
}

/// A parser topic.
///
/// Note: Similar to `ParserSection`
#[derive(Clone, Debug, Serialize)]
struct ParserTopic {
    pub title: String,
    // Still missing producing type, and so on
    pub items: Vec<LALRPOPItem>,
}

impl ParserModel {
    /// Creates a parser model from a language.
    pub fn from_language(language: &Language) -> Self {
        Self {
            lexer: LexerModel::from_language(language),
            root_item: language.root_item.clone(),
            sections: ParserBuilder::new(language).build(),
        }
    }
}

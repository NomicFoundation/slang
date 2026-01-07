mod item;
mod parser_builder;

use std::collections::{BTreeMap, BTreeSet};

use language_v2_definition::model::Language;
use serde::Serialize;

use crate::lexer::LexerModel;
use crate::parser::item::LALRPOPItem;
use crate::parser::parser_builder::ParserBuilder;

/// A parser model used while generating the parser (and the lexer).
#[derive(Clone, Debug, Serialize)]
pub struct ParserModel {
    lexer: LexerModel,

    /// For each terminal we map the lexemes that can construct it
    ///
    /// We need this mapping since unreserved keywords can be parsed as other terminals
    terminals: BTreeMap<String, BTreeSet<String>>,
    sections: Vec<ParserSection>,
}

/// A section of the language within the parser.
///
/// Note: This division is not needed, for now we're using it so the generated parser 'looks' like the language definition.
#[derive(Clone, Debug, Serialize)]
struct ParserSection {
    title: String,
    topics: Vec<ParserTopic>,
}

/// A parser topic.
///
/// Note: Similar to `ParserSection`
#[derive(Clone, Debug, Serialize)]
struct ParserTopic {
    title: String,
    items: Vec<LALRPOPItem>,
}

impl ParserModel {
    /// Creates a parser model from a language.
    pub fn from_language(language: &Language) -> Self {
        let lexer = LexerModel::from_language(language);
        let terminals = ParserBuilder::collect_terminals(&lexer);
        Self {
            lexer,
            sections: ParserBuilder::new(language).build(),
            terminals,
        }
    }
}

mod builder;

use std::collections::BTreeSet;

use language_v2_definition::model::{Language, VersionSpecifier};
use serde::Serialize;

use crate::lexer::builder::LexerModelBuilder;

#[derive(Clone, Debug, Serialize)]
pub struct LexerModel {
    pub contexts: Vec<LexicalContext>,
    pub lexeme_kinds: BTreeSet<String>,
}

impl LexerModel {
    pub fn from_language(language: &Language) -> Self {
        LexerModelBuilder::build(language)
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct LexicalContext {
    pub name: String,
    pub lexemes: Vec<Lexeme>,
}

#[derive(Clone, Debug, Serialize)]
#[serde(tag = "type")]
pub enum Lexeme {
    Trivia {
        kind: String,
        regex: String,
    },
    Token {
        kind: String,
        regex: String,
    },
    Keyword {
        kind: String,
        regex: String,
        reserved: Option<VersionSpecifier>,
    },
}

mod builder;

use std::collections::BTreeSet;

use indexmap::IndexMap;
use language_v2_definition::model::{Identifier, Language, VersionSpecifier};
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
    pub subpatterns: IndexMap<Identifier, String>,
    pub callbacks: Vec<CallbackDef>,
}

#[derive(Clone, Debug, Serialize)]
pub struct CallbackDef {
    pub name: String,
    pub kind: String,
    // NOTE: Current not_followed_by patterns are simple character classes
    // (e.g., [a-zA-Z_$]). If performance becomes a concern, these could be
    // replaced with hand-written char checks instead of regex. We use regex
    // for generality in case future patterns are more complex.
    pub pattern: String,
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
        not_followed_by: Option<String>,
    },
    Keyword {
        kind: String,
        identifier: Option<String>,
        regex: String,
        reserved: VersionSpecifier,
    },
}

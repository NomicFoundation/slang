pub mod lexer;

use language_v2_definition::model::Language;
use serde::Serialize;

use crate::lexer::LexerModel;

#[derive(Clone, Debug, Serialize)]
pub struct ParserModel {
    pub lexer: LexerModel,
}

impl ParserModel {
    pub fn from_language(language: &Language) -> Self {
        Self {
            lexer: LexerModel::from_language(language),
        }
    }
}

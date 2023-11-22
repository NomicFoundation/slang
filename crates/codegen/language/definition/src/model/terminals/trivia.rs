use crate::model::{Identifier, Scanner, Spanned};
use codegen_language_internal_macros::{ParseInputTokens, WriteOutputTokens};
use serde::Serialize;

#[derive(Debug, Eq, ParseInputTokens, PartialEq, Serialize, WriteOutputTokens)]
pub enum TriviaParser {
    Sequence { parsers: Vec<TriviaParser> },
    Choice { parsers: Vec<TriviaParser> },

    ZeroOrMore { parser: Box<TriviaParser> },
    Optional { parser: Box<TriviaParser> },

    Trivia { trivia: Spanned<Identifier> },
    EndOfInput,
}

#[derive(Debug, Eq, ParseInputTokens, PartialEq, Serialize, WriteOutputTokens)]
pub struct TriviaItem {
    pub name: Spanned<Identifier>,

    pub scanner: Scanner,
}

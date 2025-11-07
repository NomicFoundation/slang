use language_v2_internal_macros::{derive_spanned_type, ParseInputTokens, WriteOutputTokens};
use serde::{Deserialize, Serialize};

use crate::model::{Identifier, Scanner};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[derive_spanned_type(Clone, Debug, ParseInputTokens, WriteOutputTokens)]
#[serde(tag = "type")]
pub enum TriviaParser {
    Sequence { parsers: Vec<TriviaParser> },
    Choice { parsers: Vec<TriviaParser> },

    Optional { parser: Box<TriviaParser> },
    OneOrMore { parser: Box<TriviaParser> },
    ZeroOrMore { parser: Box<TriviaParser> },

    Trivia { reference: Identifier },
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[derive_spanned_type(Clone, Debug, ParseInputTokens, WriteOutputTokens)]
pub struct TriviaItem {
    pub name: Identifier,

    pub scanner: Scanner,
}

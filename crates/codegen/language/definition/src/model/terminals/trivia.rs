use codegen_language_internal_macros::{derive_spanned_type, ParseInputTokens, WriteOutputTokens};
use serde::{Deserialize, Serialize};

use crate::model::{Identifier, Scanner};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[derive_spanned_type(ParseInputTokens, WriteOutputTokens)]
pub enum TriviaParser {
    Sequence { parsers: Vec<TriviaParser> },
    Choice { parsers: Vec<TriviaParser> },

    Optional { parser: Box<TriviaParser> },
    // TODO(#638): Remove this, once we adapt the DSL v1 codegen model to the new v2 definition.
    OneOrMore { parser: Box<TriviaParser> },
    ZeroOrMore { parser: Box<TriviaParser> },

    Trivia { trivia: Identifier },
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[derive_spanned_type(ParseInputTokens, WriteOutputTokens)]
pub struct TriviaItem {
    pub name: Identifier,

    pub scanner: Scanner,
}

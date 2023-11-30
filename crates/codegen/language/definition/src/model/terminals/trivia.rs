use crate::model::{Identifier, Scanner};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[codegen_language_internal_macros::derive_spanned_type(
    codegen_language_internal_macros::ParseInputTokens,
    codegen_language_internal_macros::WriteOutputTokens
)]
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
#[codegen_language_internal_macros::derive_spanned_type(
    codegen_language_internal_macros::ParseInputTokens,
    codegen_language_internal_macros::WriteOutputTokens
)]
pub struct TriviaItem {
    pub name: Identifier,

    pub scanner: Scanner,
}

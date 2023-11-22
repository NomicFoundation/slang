use crate::model::{Identifier, Section, Spanned, TriviaParser};
use codegen_language_internal_macros::{ParseInputTokens, WriteOutputTokens};
use indexmap::IndexSet;
use semver::Version;
use serde::Serialize;

#[derive(Debug, Eq, ParseInputTokens, PartialEq, Serialize, WriteOutputTokens)]
pub struct Language {
    pub name: Spanned<Identifier>,

    pub root_item: Spanned<Identifier>,

    pub leading_trivia: TriviaParser,
    pub trailing_trivia: TriviaParser,

    pub versions: IndexSet<Spanned<Version>>,

    pub sections: Vec<Section>,
}

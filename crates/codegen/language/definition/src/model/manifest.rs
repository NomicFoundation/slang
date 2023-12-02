use crate::model::{Identifier, Item, TriviaParser};
use codegen_language_internal_macros::{derive_spanned_type, ParseInputTokens, WriteOutputTokens};
use indexmap::IndexSet;
use semver::Version;
use serde::{Deserialize, Serialize};
use std::rc::Rc;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[derive_spanned_type(ParseInputTokens, WriteOutputTokens)]
pub struct Language {
    pub name: Identifier,

    pub root_item: Identifier,

    pub leading_trivia: TriviaParser,
    pub trailing_trivia: TriviaParser,

    pub versions: IndexSet<Version>,

    pub sections: Vec<Section>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[derive_spanned_type(ParseInputTokens, WriteOutputTokens)]
pub struct Section {
    pub title: String,
    pub topics: Vec<Topic>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[derive_spanned_type(ParseInputTokens, WriteOutputTokens)]
pub struct Topic {
    pub title: String,
    pub notes_file: Option<String>,
    pub lexical_context: Option<Identifier>,

    pub items: Vec<Rc<Item>>,
}

impl Language {
    /// Returns a flattened iterator over items along with section and topic they belong to.
    pub fn items(&self) -> impl Iterator<Item = (&Section, &Topic, &Rc<Item>)> {
        self.sections.iter().flat_map(|section| {
            section
                .topics
                .iter()
                .flat_map(move |topic| topic.items.iter().map(move |item| (section, topic, item)))
        })
    }
}

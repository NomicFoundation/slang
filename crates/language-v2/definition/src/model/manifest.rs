use indexmap::IndexSet;
use language_v2_internal_macros::{ParseInputTokens, WriteOutputTokens, derive_spanned_type};
use semver::Version;
use serde::{Deserialize, Serialize};

use crate::model::{BuiltInContext, Identifier, Item};

/// A representation of a Language definition
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[derive_spanned_type(Clone, Debug, ParseInputTokens, WriteOutputTokens)]
pub struct Language {
    /// The name of the language
    pub name: Identifier,

    /// Each language must have a single root item
    pub root_item: Identifier,

    /// The supported versions of the language
    pub versions: IndexSet<Version>,

    /// The supported targets of the EVM
    pub evm_targets: IndexSet<Identifier>,

    /// The lexical contexts of the language, splitting grammar elements based
    /// on which lexer can recognize their terminals.
    pub contexts: Vec<LexicalContext>,

    /// Built-in definitions for the language, organized by context and scope.
    pub built_ins: Vec<BuiltInContext>,
}

impl Language {
    /// Returns every item in the language definition (across all contexts).
    pub fn items(&self) -> impl Iterator<Item = &Item> {
        self.contexts.iter().flat_map(|context| context.items())
    }
}

/// A section is a named container for topics, used for organizing the large
/// grammar definition in user documentation.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[derive_spanned_type(Clone, Debug, ParseInputTokens, WriteOutputTokens)]
pub struct LexicalContext {
    pub name: Identifier,
    pub identifier_token: Option<Identifier>,
    pub sections: Vec<Section>,
}

impl LexicalContext {
    /// Returns every item in that context (across all sections).
    pub fn items(&self) -> impl Iterator<Item = &Item> {
        self.sections.iter().flat_map(|section| section.items())
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[derive_spanned_type(Clone, Debug, ParseInputTokens, WriteOutputTokens)]
pub struct Section {
    pub title: String,
    pub topics: Vec<Topic>,
}

impl Section {
    /// Returns every item in that section (across all topics).
    pub fn items(&self) -> impl Iterator<Item = &Item> {
        self.topics.iter().flat_map(|topic| &topic.items)
    }
}

/// A topic is a named container for items, used for organizing the large grammar definition in user documentation.  
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[derive_spanned_type(Clone, Debug, ParseInputTokens, WriteOutputTokens)]
pub struct Topic {
    pub title: String,
    pub items: Vec<Item>,
}

#[derive(Clone, Copy, Debug, strum::AsRefStr, strum::EnumIter, strum::VariantNames)]
#[strum(serialize_all = "snake_case")]
pub enum PredefinedLabel {
    Root,
    Unrecognized,
    Missing,
    Item,
    Variant,
    Separator,
    Operand,
    LeftOperand,
    RightOperand,
    LeadingTrivia,
    TrailingTrivia,
}

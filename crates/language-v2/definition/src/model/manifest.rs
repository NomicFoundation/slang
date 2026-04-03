use std::collections::BTreeSet;

use indexmap::IndexSet;
use language_v2_internal_macros::{derive_spanned_type, ParseInputTokens, WriteOutputTokens};
use semver::Version;
use serde::{Deserialize, Serialize};

use crate::model::{Field, Identifier, Item, VersionSpecifier};

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

    /// The lexical contexts of the language, splitting grammar elements based on which lexer can recognize their terminals.
    pub contexts: Vec<LexicalContext>,
}

impl Language {
    /// Returns every item in the language definition (across all contexts).
    pub fn items(&self) -> impl Iterator<Item = &Item> {
        self.contexts.iter().flat_map(|context| context.items())
    }

    /// Collects all versions that change the language in a breaking way.
    ///
    /// Includes the first supported version.
    pub fn collect_breaking_versions(&self) -> BTreeSet<Version> {
        let first = self.versions.first().unwrap().clone();
        let mut res = BTreeSet::from_iter([first]);

        let mut add_spec = |spec: &Option<VersionSpecifier>| {
            if let Some(spec) = spec {
                res.extend(spec.breaking_versions().cloned());
            }
        };

        for item in self.items() {
            match item {
                Item::Struct { item } => {
                    add_spec(&item.enabled);
                    for field in item.fields.values() {
                        match field {
                            Field::Required { .. } => (),
                            Field::Optional { enabled, .. } => add_spec(enabled),
                        }
                    }
                }
                Item::Enum { item } => {
                    add_spec(&item.enabled);
                    for variant in &item.variants {
                        add_spec(&variant.enabled);
                    }
                }
                Item::Repeated { item } => add_spec(&item.enabled),
                Item::Separated { item } => add_spec(&item.enabled),
                Item::Precedence { item } => {
                    add_spec(&item.enabled);
                    for prec in &item.precedence_expressions {
                        for op in &prec.operators {
                            add_spec(&op.enabled);
                            for field in op.fields.values() {
                                match field {
                                    Field::Required { .. } => (),
                                    Field::Optional { enabled, .. } => add_spec(enabled),
                                }
                            }
                        }
                    }
                    for prim in &item.primary_expressions {
                        add_spec(&prim.enabled);
                    }
                }
                Item::Keyword { item } => {
                    add_spec(&item.enabled);
                    add_spec(&item.reserved);
                }
                Item::Token { item } => {
                    add_spec(&item.enabled);
                }
                Item::Fragment { item } => add_spec(&item.enabled),
                Item::Trivia { .. } => {}
            }
        }

        res
    }
}

/// A section is a named container for topics, used for organizing the large grammar definition in user documentation.  
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

#[derive(
    Clone, Copy, Debug, strum_macros::AsRefStr, strum_macros::EnumIter, strum_macros::VariantNames,
)]
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

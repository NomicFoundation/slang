use std::collections::BTreeSet;
use std::path::PathBuf;

use indexmap::IndexSet;
use language_internal_macros::{derive_spanned_type, ParseInputTokens, WriteOutputTokens};
use semver::Version;
use serde::{Deserialize, Serialize};

use super::BuiltIn;
use crate::model::{BuiltInContext, Field, Identifier, Item, TriviaParser, VersionSpecifier};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[derive_spanned_type(Clone, Debug, ParseInputTokens, WriteOutputTokens)]
pub struct Language {
    pub name: Identifier,

    pub binding_rules_file: PathBuf,

    pub root_item: Identifier,

    pub leading_trivia: TriviaParser,
    pub trailing_trivia: TriviaParser,

    pub versions: IndexSet<Version>,

    pub sections: Vec<Section>,
    pub built_ins: Vec<BuiltInContext>,
}

impl Language {
    /// Returns every topic in the language definition (across all sections).
    pub fn topics(&self) -> impl Iterator<Item = &Topic> {
        self.sections.iter().flat_map(|section| &section.topics)
    }

    /// Returns every item in the language definition (across all sections and topics).
    pub fn items(&self) -> impl Iterator<Item = &Item> {
        self.topics().flat_map(|topic| &topic.items)
    }

    /// Collects all versions that change the language grammar in a breaking way.
    ///
    /// Includes the first supported version.
    pub fn collect_breaking_versions(&self) -> BTreeSet<Version> {
        let first = self.versions.first().unwrap().clone();
        let mut res = BTreeSet::from_iter([first]);

        let mut add_spec = |spec: &Option<VersionSpecifier>| {
            if let Some(spec) = spec {
                res.extend(spec.versions().cloned());
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
                    for definition in &item.definitions {
                        add_spec(&definition.enabled);
                        add_spec(&definition.reserved);
                    }
                }
                Item::Token { item } => {
                    for definition in &item.definitions {
                        add_spec(&definition.enabled);
                    }
                }
                Item::Fragment { item } => add_spec(&item.enabled),
                Item::Trivia { .. } => {}
            }
        }

        res
    }

    /// Collects all versions that change the language built-ins.
    ///
    /// Includes the first supported version. Returns an empty set if there are
    /// no built-ins defined.
    pub fn collect_built_ins_versions(&self) -> BTreeSet<Version> {
        if self.built_ins.is_empty() {
            return BTreeSet::new();
        }

        let first = self.versions.first().unwrap().clone();
        let mut res = BTreeSet::from_iter([first]);

        let mut add_spec = |spec: &Option<VersionSpecifier>| {
            if let Some(spec) = spec {
                res.extend(spec.versions().cloned());
            }
        };

        for context in &self.built_ins {
            for item in &context.definitions {
                match item {
                    BuiltIn::BuiltInFunction { item } => {
                        add_spec(&item.enabled);
                    }
                    BuiltIn::BuiltInType { item } => {
                        add_spec(&item.enabled);
                        for field in &item.fields {
                            add_spec(&field.enabled);
                        }
                        for function in &item.functions {
                            add_spec(&function.enabled);
                        }
                    }
                    BuiltIn::BuiltInVariable { item } => {
                        add_spec(&item.enabled);
                    }
                }
            }
        }

        res
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

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[derive_spanned_type(Clone, Debug, ParseInputTokens, WriteOutputTokens)]
pub struct Topic {
    pub title: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub lexical_context: Option<Identifier>,

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

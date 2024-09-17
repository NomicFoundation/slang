use std::collections::BTreeSet;
use std::path::PathBuf;
use std::rc::Rc;

use codegen_language_internal_macros::{derive_spanned_type, ParseInputTokens, WriteOutputTokens};
use indexmap::IndexSet;
use semver::Version;
use serde::{Deserialize, Serialize};
use strum_macros::EnumDiscriminants;

use crate::model::{Field, Identifier, Item, TriviaParser, VersionSpecifier};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[derive_spanned_type(Clone, Debug, ParseInputTokens, WriteOutputTokens)]
pub struct Language {
    pub name: Identifier,

    pub documentation_dir: PathBuf,
    pub binding_rules_file: PathBuf,

    pub root_item: Identifier,

    pub leading_trivia: TriviaParser,
    pub trailing_trivia: TriviaParser,

    pub versions: IndexSet<Version>,

    pub sections: Vec<Section>,
    pub built_ins: Vec<BuiltIn>,
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
    pub notes_file: Option<String>,
    pub lexical_context: Option<Identifier>,

    pub items: Vec<Item>,
}

#[derive(
    Clone, Copy, Debug, strum_macros::AsRefStr, strum_macros::EnumIter, strum_macros::VariantNames,
)]
#[strum(serialize_all = "snake_case")]
pub enum BuiltInLabel {
    Item,
    Variant,
    Separator,
    Operand,
    LeftOperand,
    RightOperand,
    LeadingTrivia,
    TrailingTrivia,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[derive_spanned_type(Clone, Debug, EnumDiscriminants, ParseInputTokens, WriteOutputTokens)]
pub enum BuiltIn {
    BuiltInFunction { item: Rc<BuiltInFunction> },
    BuiltInType { item: Rc<BuiltInType> },
    BuiltInVariable { item: Rc<TypedSlot> },
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[derive_spanned_type(Clone, Debug, ParseInputTokens, WriteOutputTokens)]
pub struct BuiltInFunction {
    pub name: String,
    pub return_type: String,
    pub parameters: Vec<TypedSlot>,
    pub enabled: Option<VersionSpecifier>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[derive_spanned_type(Clone, Debug, ParseInputTokens, WriteOutputTokens)]
pub struct BuiltInType {
    pub name: String,
    pub fields: Vec<TypedSlot>,
    pub enabled: Option<VersionSpecifier>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[derive_spanned_type(Clone, Debug, ParseInputTokens, WriteOutputTokens)]
pub struct TypedSlot {
    pub name: String,
    pub slot_type: String,
    pub enabled: Option<VersionSpecifier>,
}

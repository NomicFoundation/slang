use crate::model::{Field, Identifier, Item, TriviaParser, VersionSpecifier};
use codegen_language_internal_macros::{derive_spanned_type, ParseInputTokens, WriteOutputTokens};
use indexmap::IndexSet;
use semver::Version;
use serde::{Deserialize, Serialize};
use std::{collections::BTreeSet, rc::Rc};

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
    /// Returns every item in the language definition.
    pub fn items(&self) -> impl Iterator<Item = &Item> {
        self.sections
            .iter()
            .flat_map(|section| &*section.topics)
            .flat_map(|topic| topic.items.iter().map(AsRef::as_ref))
    }

    /// Returns a flattened iterator over items along with section and topic they belong to.
    pub fn items_with_section(&self) -> impl Iterator<Item = (&Section, &Topic, &Rc<Item>)> {
        self.sections.iter().flat_map(|section| {
            section
                .topics
                .iter()
                .flat_map(move |topic| topic.items.iter().map(move |item| (section, topic, item)))
        })
    }

    /// Collects all versions that change the language grammar in a breaking way.
    pub fn collect_breaking_versions(&self) -> BTreeSet<Version> {
        let first = self.versions.first().unwrap().clone();
        let mut res = BTreeSet::from_iter([first]);

        let mut add_spec = |spec: &Option<VersionSpecifier>| {
            let Some(spec) = spec else {
                return;
            };

            match spec.clone() {
                VersionSpecifier::Never => (),
                VersionSpecifier::From { from } => {
                    res.insert(from);
                }
                VersionSpecifier::Till { till } => {
                    res.insert(till);
                }
                VersionSpecifier::Range { from, till } => {
                    res.insert(from);
                    res.insert(till);
                }
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

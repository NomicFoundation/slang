use std::collections::BTreeSet;

use codegen_language_definition::model::{self, BuiltInLabel, Identifier, Item};
use serde::Serialize;
use strum::VariantNames;

#[derive(Serialize)]
pub struct KindsModel {
    /// Defines the `NonterminalKind` enum variants.
    nonterminal_kinds: BTreeSet<Identifier>,
    /// Defines the `TerminalKind` enum variants.
    terminal_kinds: BTreeSet<Identifier>,
    /// Defines `TerminalKind::is_trivia` method.
    trivia_scanner_names: BTreeSet<Identifier>,
    /// Defines `EdgeLabel` enum variants.
    labels: BTreeSet<Identifier>,
    /// Built-in labels for edges
    built_in_labels: &'static [&'static str],
    // Defines the `LexicalContext(Type)` enum and type-level variants.
    lexical_contexts: BTreeSet<Identifier>,
}

impl Default for KindsModel {
    fn default() -> Self {
        Self {
            nonterminal_kinds: Default::default(),
            terminal_kinds: Default::default(),
            trivia_scanner_names: Default::default(),
            labels: Default::default(),
            built_in_labels: BuiltInLabel::VARIANTS,
            lexical_contexts: Default::default(),
        }
    }
}

impl KindsModel {
    pub fn create(language: &model::Language) -> Self {
        let terminal_kinds = language
            .items()
            .filter(|item| item.is_terminal() && !matches!(item, Item::Fragment { .. }))
            .map(|item| item.name().clone())
            .collect();

        let mut nonterminal_kinds = BTreeSet::default();
        for item in language.items() {
            match item {
                Item::Struct { item } => {
                    nonterminal_kinds.insert(item.name.clone());
                }
                Item::Enum { item } => {
                    nonterminal_kinds.insert(item.name.clone());
                }
                Item::Repeated { item } => {
                    nonterminal_kinds.insert(item.name.clone());
                }
                Item::Separated { item } => {
                    nonterminal_kinds.insert(item.name.clone());
                }
                Item::Precedence { item } => {
                    nonterminal_kinds.insert(item.name.clone());
                    for op in &item.precedence_expressions {
                        nonterminal_kinds.insert(op.name.clone());
                    }
                }
                // Terminals
                _ => {}
            }
        }

        let trivia_scanner_names = language
            .items()
            .filter_map(|item| match item {
                Item::Trivia { item } => Some(item.name.clone()),
                _ => None,
            })
            .collect();

        let mut labels = BTreeSet::default();
        for item in language.items() {
            match item {
                Item::Struct { item } => {
                    for field_name in item.fields.keys() {
                        labels.insert(field_name.clone());
                    }
                }
                Item::Precedence { item } => {
                    for item in &item.precedence_expressions {
                        for item in &item.operators {
                            for field_name in item.fields.keys() {
                                labels.insert(field_name.clone());
                            }
                        }
                    }
                }
                _ => {}
            }
        }

        let lexical_contexts: BTreeSet<_> = language
            .topics()
            .filter_map(|t| t.lexical_context.as_ref())
            .cloned()
            .chain(std::iter::once(Identifier::from("Default")))
            .collect();

        KindsModel {
            nonterminal_kinds,
            terminal_kinds,
            trivia_scanner_names,
            labels,
            lexical_contexts,
            ..Self::default()
        }
    }
}

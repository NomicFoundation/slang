use std::collections::BTreeSet;

use codegen_ebnf::{EbnfModel, PlainWriter};
use codegen_language_definition::model::{self, Identifier, Item, PredefinedLabel};
use serde::Serialize;
use strum::VariantNames;

#[derive(Serialize, Ord, Eq, PartialEq, PartialOrd)]
pub struct Kind {
    id: Identifier,
    documentation: String,
}

struct EbnfBuilder {
    model: EbnfModel,
    writer: PlainWriter,
}

impl EbnfBuilder {
    fn new(language: &model::Language) -> EbnfBuilder {
        EbnfBuilder {
            model: EbnfModel::build(language),
            writer: PlainWriter::default(),
        }
    }

    fn build(&mut self, name: &Identifier) -> Kind {
        if self.model.serialize(name, &mut self.writer).is_ok() {
            Kind {
                id: name.clone(),
                documentation: self.writer.flush(),
            }
        } else {
            Kind {
                id: name.clone(),
                documentation: String::new(),
            }
        }
    }
}

#[derive(Serialize)]
pub struct KindsModel {
    /// Defines the `NonterminalKind` enum variants.
    nonterminal_kinds: BTreeSet<Kind>,
    /// Defines the `TerminalKind` enum variants.
    terminal_kinds: BTreeSet<Kind>,
    /// Defines `TerminalKind::is_identifier` method.
    identifier_terminals: BTreeSet<Identifier>,
    /// Defines `TerminalKind::is_trivia` method.
    trivia_terminals: BTreeSet<Identifier>,
    /// Defines `EdgeLabel` enum variants.
    labels: BTreeSet<Identifier>,
    /// Predefined labels for edges.
    predefined_labels: &'static [&'static str],
    // Defines the `LexicalContext(Type)` enum and type-level variants.
    lexical_contexts: BTreeSet<Identifier>,
    /// Defines the root `NonterminalKind` for a source file of the language.
    root_kind: Identifier,
}

impl Default for KindsModel {
    fn default() -> Self {
        Self {
            nonterminal_kinds: BTreeSet::default(),
            terminal_kinds: BTreeSet::default(),
            identifier_terminals: BTreeSet::default(),
            trivia_terminals: BTreeSet::default(),
            labels: BTreeSet::default(),
            predefined_labels: PredefinedLabel::VARIANTS,
            lexical_contexts: BTreeSet::default(),
            root_kind: Identifier::from("Stub1"),
        }
    }
}

impl KindsModel {
    pub fn from_language(language: &model::Language) -> Self {
        let mut kind_builder = EbnfBuilder::new(language);

        let terminal_kinds = language
            .items()
            .filter(|item| item.is_terminal() && !matches!(item, Item::Fragment { .. }))
            .map(|item| kind_builder.build(item.name()))
            .collect();

        let mut nonterminal_kinds = BTreeSet::default();
        for item in language.items() {
            match item {
                Item::Struct { item } => {
                    nonterminal_kinds.insert(kind_builder.build(&item.name));
                }
                Item::Enum { item } => {
                    nonterminal_kinds.insert(kind_builder.build(&item.name));
                }
                Item::Repeated { item } => {
                    nonterminal_kinds.insert(kind_builder.build(&item.name));
                }
                Item::Separated { item } => {
                    nonterminal_kinds.insert(kind_builder.build(&item.name));
                }
                Item::Precedence { item } => {
                    nonterminal_kinds.insert(kind_builder.build(&item.name));
                    for op in &item.precedence_expressions {
                        nonterminal_kinds.insert(kind_builder.build(&op.name));
                    }
                }
                // Terminals
                _ => {}
            }
        }

        let identifier_terminals = language
            .items()
            .filter_map(|item| match item {
                Item::Keyword { item } => Some(item.identifier.clone()),
                _ => None,
            })
            .collect();

        let trivia_terminals = language
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

        let root_kind = language.root_item.clone();

        KindsModel {
            nonterminal_kinds,
            terminal_kinds,
            identifier_terminals,
            trivia_terminals,
            labels,
            lexical_contexts,
            root_kind,
            ..Self::default()
        }
    }
}

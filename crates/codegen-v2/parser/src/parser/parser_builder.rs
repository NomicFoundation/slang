use std::collections::{BTreeMap, BTreeSet};

use language_v2_definition::model::{Item as LanguageItem, Language, Section, Topic};

use crate::lexer::{Lexeme, LexerModel};
use crate::parser::item::{
    contains_enabled_versions, enum_item_to_lalrpop_items, overlaps_with_version_range,
    precedence_item_to_lalrpop_items, repeated_item_to_lalrpop_items,
    separated_item_to_lalrpop_items, struct_item_to_lalrpop_items, LALRPOPItem,
};
use crate::parser::{ParserSection, ParserTopic};

pub(crate) struct ParserBuilder<'a> {
    language: &'a Language,
}

impl<'a> ParserBuilder<'a> {
    pub(crate) fn new(language: &'a Language) -> Self {
        Self { language }
    }

    pub(crate) fn build(self) -> Vec<ParserSection> {
        self.collect_sections()
    }

    fn collect_sections(&self) -> Vec<ParserSection> {
        self.language
            .sections
            .iter()
            .map(|section| self.collect_section(section))
            .collect()
    }

    fn collect_section(&self, section: &Section) -> ParserSection {
        ParserSection {
            title: section.title.clone(),
            topics: section
                .topics
                .iter()
                .map(|topic| self.collect_topic(topic))
                .collect(),
        }
    }

    fn collect_topic(&self, topic: &Topic) -> ParserTopic {
        ParserTopic {
            title: topic.title.clone(),
            items: topic
                .items
                .iter()
                .map(|item| self.language_item_to_lalrpop_item(item))
                .collect(),
        }
    }

    fn language_item_to_lalrpop_item(&self, item: &LanguageItem) -> LALRPOPItem {
        // Apply manual parser options
        let parser_options = match item {
            LanguageItem::Struct { item } => &item.parser_options,
            LanguageItem::Enum { item } => &item.parser_options,
            LanguageItem::Repeated { item } => &item.parser_options,
            LanguageItem::Separated { item } => &item.parser_options,
            LanguageItem::Precedence { item } => &item.parser_options,
            _ => &None,
        };

        if let Some(parser_options) = parser_options {
            // If there's a verbatim rule, then skip generating the rules
            if let Some(verbatim) = &parser_options.verbatim {
                return LALRPOPItem::Verbatim(verbatim.clone());
            }
        }

        let mut items = match item {
            LanguageItem::Struct { item } => struct_item_to_lalrpop_items(item),
            LanguageItem::Enum { item } => enum_item_to_lalrpop_items(item),
            LanguageItem::Repeated { item } => repeated_item_to_lalrpop_items(item),
            LanguageItem::Separated { item } => separated_item_to_lalrpop_items(item),
            LanguageItem::Precedence { item } => precedence_item_to_lalrpop_items(item),
            LanguageItem::Keyword { .. }
            | LanguageItem::Trivia { .. }
            | LanguageItem::Token { .. }
            // I don't think we care about fragments at all
            // ... but we'll see once versioning comes in place
            | LanguageItem::Fragment { .. } => vec![],
        };

        if let Some(parser_options) = parser_options {
            for item in &mut items {
                item.inline = parser_options.inline;
                item.pubb = parser_options.pubb;
            }
        }

        LALRPOPItem::Items(items)
    }

    pub(crate) fn collect_terminals(lexer: &LexerModel) -> BTreeMap<String, BTreeSet<String>> {
        let mut terminals: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();

        for context in &lexer.contexts {
            for lexeme in &context.lexemes {
                match lexeme {
                    Lexeme::Trivia { kind, .. } => {
                        terminals
                            .entry(kind.clone())
                            .or_default()
                            .insert(kind.clone());
                    }
                    Lexeme::Token { kind, .. } => {
                        terminals
                            .entry(kind.clone())
                            .or_default()
                            .insert(kind.clone());
                    }
                    Lexeme::Keyword {
                        kind,
                        identifier,
                        reserved,
                        ..
                    } => {
                        if match reserved {
                            None => true,
                            Some(spec) => overlaps_with_version_range(spec),
                        } {
                            terminals
                                .entry(kind.clone())
                                .or_default()
                                .insert(format!("{kind}_Reserved"));
                        }

                        if reserved
                            .as_ref()
                            .is_some_and(|rng| !contains_enabled_versions(rng))
                        {
                            terminals
                                .entry(identifier.clone())
                                .or_default()
                                .insert(format!("{kind}_Unreserved"));
                            terminals
                                .entry(kind.clone())
                                .or_default()
                                .insert(format!("{kind}_Unreserved"));
                        }
                    }
                }
            }
        }
        terminals
    }
}

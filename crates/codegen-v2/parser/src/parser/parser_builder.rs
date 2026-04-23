use std::collections::{BTreeMap, BTreeSet};

use language_v2_definition::model::{
    Item as LanguageItem, Language, LexicalContext, ParserOptions, Section, Topic, VersionSpecifier,
};

use crate::lexer::{Lexeme, LexerModel};
use crate::parser::item_builders::{
    enum_item_to_lalrpop_items, precedence_item_to_lalrpop_items, repeated_item_to_lalrpop_items,
    separated_item_to_lalrpop_items, struct_item_to_lalrpop_items,
};
use crate::parser::{LALRPOPItem, ParserSection, ParserTopic};

pub(crate) struct ParserBuilder<'a> {
    language: &'a Language,
}

impl<'a> ParserBuilder<'a> {
    pub(crate) fn new(language: &'a Language) -> Self {
        Self { language }
    }

    pub(crate) fn build(&self) -> Vec<ParserSection> {
        self.language
            .contexts
            .iter()
            .flat_map(|ctx| self.collect_context(ctx))
            .collect()
    }

    fn collect_context(&self, context: &LexicalContext) -> Vec<ParserSection> {
        context
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

        if let Some(ParserOptions {
            verbatim: Some(verbatim),
            ..
        }) = parser_options
        {
            // If there's a verbatim rule, then skip generating the rules
            return LALRPOPItem::Verbatim(verbatim.value.clone());
        }

        // Translate the LanguageItem into LALRPOP items
        let mut items = match item {
            LanguageItem::Struct { item } => struct_item_to_lalrpop_items(item),
            LanguageItem::Enum { item } => enum_item_to_lalrpop_items(item),
            LanguageItem::Repeated { item } => repeated_item_to_lalrpop_items(item),
            LanguageItem::Separated { item } => separated_item_to_lalrpop_items(item),
            LanguageItem::Precedence { item } => precedence_item_to_lalrpop_items(item),
            // I don't think we care about terminals at all
            // ... but we'll see once versioning comes in place
            LanguageItem::Keyword { .. }
            | LanguageItem::Trivia { .. }
            | LanguageItem::Token { .. }
            | LanguageItem::Fragment { .. } => vec![],
        };

        // Apply parser options to all generated items
        let is_root = item.name() == &self.language.root_item;
        if let Some(ParserOptions { inline, .. }) = parser_options {
            for item in &mut items {
                item.inline = *inline;
            }
        }

        debug_assert!(
            !is_root || items.len() == 1,
            "Root item should generate exactly one LALRPOP item"
        );
        for item in &mut items {
            item.public = is_root;
        }

        LALRPOPItem::Items(items)
    }

    /// Collects terminals from the lexer model.
    ///
    /// The resulting map contains, for each terminal, the set of lexemes that can produce it.
    pub(crate) fn collect_terminals(lexer: &LexerModel) -> BTreeMap<String, BTreeSet<String>> {
        let mut terminals: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();

        for context in &lexer.contexts {
            for lexeme in &context.lexemes {
                match lexeme {
                    Lexeme::Trivia { .. } => {
                        // Trivia items are skipped by the parser.
                    }
                    Lexeme::Token { kind, .. } => {
                        // For Token, the lexeme kind directly maps to the terminal
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
                        // For Keywords, we need to consider reserved and unreserved forms.
                        if *reserved != VersionSpecifier::Never {
                            terminals
                                .entry(kind.clone())
                                .or_default()
                                .insert(format!("{kind}_Reserved"));
                        }

                        if *reserved != VersionSpecifier::Always {
                            // In particular, for unreserved keywords we add it also to the
                            // corresponding identifier
                            if let Some(identifier) = identifier {
                                terminals
                                    .entry(identifier.clone())
                                    .or_default()
                                    .insert(format!("{kind}_Unreserved"));
                            }
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

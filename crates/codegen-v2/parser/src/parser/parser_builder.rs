use std::collections::{BTreeMap, BTreeSet};

use language_v2_definition::model::{
    Item as LanguageItem, Language, LexicalContext, ParserOptions, Section, Topic,
};

use crate::lexer::{Lexeme, LexerModel};
use crate::parser::item_builders::{
    enum_item_to_lalrpop_items, precedence_item_to_lalrpop_items, repeated_item_to_lalrpop_items,
    separated_item_to_lalrpop_items, struct_item_to_lalrpop_items, VERSION,
};
use crate::parser::{LALRPOPItem, ParserSection, ParserTopic};

pub(crate) struct ParserBuilder<'a> {
    language: &'a Language,
}

impl<'a> ParserBuilder<'a> {
    pub(crate) fn new(language: &'a Language) -> Self {
        Self { language }
    }

    pub(crate) fn build(self) -> Vec<ParserSection> {
        self.language
            .contexts
            .iter()
            .flat_map(Self::collect_context)
            .collect()
    }

    fn collect_context(context: &LexicalContext) -> Vec<ParserSection> {
        context.sections.iter().map(Self::collect_section).collect()
    }

    fn collect_section(section: &Section) -> ParserSection {
        ParserSection {
            title: section.title.clone(),
            topics: section.topics.iter().map(Self::collect_topic).collect(),
        }
    }

    fn collect_topic(topic: &Topic) -> ParserTopic {
        ParserTopic {
            title: topic.title.clone(),
            items: topic
                .items
                .iter()
                .map(Self::language_item_to_lalrpop_item)
                .collect(),
        }
    }

    fn language_item_to_lalrpop_item(item: &LanguageItem) -> LALRPOPItem {
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
            return LALRPOPItem::Verbatim(verbatim.clone());
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
        if let Some(ParserOptions { inline, pubb, .. }) = parser_options {
            for item in &mut items {
                item.inline = *inline;
                item.pubb = *pubb;
            }
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
                    Lexeme::Trivia { kind, .. } | Lexeme::Token { kind, .. } => {
                        // For Trivia and Token, the lexeme kind directly maps to the terminal
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
                        if reserved.contains(&VERSION) {
                            terminals
                                .entry(kind.clone())
                                .or_default()
                                .insert(format!("{kind}_Reserved"));
                        }

                        if !reserved.contains(&VERSION) {
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

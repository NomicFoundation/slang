use std::collections::HashMap;

use language_v2_definition::model::{
    Field, Identifier, Item as LanguageItem, Language, Section, Topic,
};

use crate::parser::item::{precedence_item_to_lalrpop_items, LALRPOPItem};
use crate::parser::{ParserSection, ParserTopic};

pub(crate) struct ParserBuilder<'a> {
    language: &'a Language,
    // For every item, how many times is it referenced
    // If it's only one, we make it inline
    references: HashMap<Identifier, usize>,
}

impl<'a> ParserBuilder<'a> {
    pub fn new(language: &'a Language) -> Self {
        Self {
            language,
            references: HashMap::new(),
        }
    }

    pub fn build(mut self) -> Vec<ParserSection> {
        self.count_references();
        self.collect_sections()
    }

    // We're sort of already doing this in the analysis phase, but we'll do it again here
    // for now, since that is not being exported
    fn count_references(&mut self) {
        // The root item always has 2 references, since I don't want to inline it
        *self
            .references
            .entry(self.language.root_item.clone())
            .or_insert(0) += 2;

        for section in &self.language.sections {
            for topic in &section.topics {
                for item in &topic.items {
                    self.count_references_for_item(item);
                }
            }
        }
    }

    fn count_references_for_item(&mut self, item: &LanguageItem) {
        match item {
            LanguageItem::Struct { item } => {
                item.fields.values().for_each(|field| {
                    self.count_references_for_field(field);
                });
            }
            LanguageItem::Enum { item } => {
                item.variants.iter().for_each(|variant| {
                    *self
                        .references
                        .entry(variant.reference.clone())
                        .or_insert(0) += 1;
                });
            }
            LanguageItem::Repeated { item } => {
                *self.references.entry(item.reference.clone()).or_insert(0) += 1;
            }
            LanguageItem::Separated { item } => {
                *self.references.entry(item.reference.clone()).or_insert(0) += 1;
                *self.references.entry(item.separator.clone()).or_insert(0) += 1;
            }
            LanguageItem::Precedence { item } => {
                item.precedence_expressions.iter().for_each(|expr| {
                    expr.operators.iter().for_each(|op| {
                        op.fields.values().for_each(|field| {
                            self.count_references_for_field(field);
                        });
                    });
                });
                item.primary_expressions.iter().for_each(|expr| {
                    *self.references.entry(expr.reference.clone()).or_insert(0) += 1;
                });
            }
            LanguageItem::Fragment { .. }
            | LanguageItem::Token { .. }
            | LanguageItem::Keyword { .. }
            | LanguageItem::Trivia { .. } => (),
        }
    }

    fn count_references_for_field(&mut self, field: &Field) {
        match field {
            Field::Required { reference } | Field::Optional { reference, .. } => {
                *self.references.entry(reference.clone()).or_insert(0) += 1;
            }
        }
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
                .flat_map(|item| self.language_item_to_lalrpop_item(item))
                .collect(),
        }
    }

    fn language_item_to_lalrpop_item(&self, item: &LanguageItem) -> Vec<LALRPOPItem> {
        // TODO: we're ignoring versions for now

        // TODO: use an actual type rather than string

        let mut items = match item {
            LanguageItem::Struct { item } => {
                item.as_ref().try_into().ok().iter().cloned().collect()
            }
            LanguageItem::Enum { item } => item.as_ref().try_into().ok().iter().cloned().collect(),
            LanguageItem::Repeated { item } => {
                item.as_ref().try_into().ok().iter().cloned().collect()
            }
            LanguageItem::Separated { item } => {
                item.as_ref().try_into().ok().iter().cloned().collect()
            }
            // TODO: No idea how to do this yet
            LanguageItem::Precedence { item } => precedence_item_to_lalrpop_items(item),
            // I don't think we care about fragments at all
            // ... but we'll see once versioning comes in place

            // Actually, trivia, keyword, and token should translate to references
            LanguageItem::Keyword { item } => vec![item.as_ref().into()],
            LanguageItem::Trivia { .. }
            | LanguageItem::Token { .. }
            | LanguageItem::Fragment { .. } => vec![],
        };

        // Remove all rules that we generate by hand, a bit hacky
        const EXCLUDED_ITEMS: [&str; 46] = [
            "StateVariableDefinition",
            "StateVariableAttributes",
            "StateVariableAttribute",
            "TypeName0",
            "ArgumentsDeclaration",
            "TypeName1",
            "TypeName",
            "FunctionType",
            "FunctionTypeAttributes",
            "FunctionTypeAttribute",
            "Statement",
            "TupleDeconstructionStatement",
            "TupleDeconstructionElements",
            "TupleDeconstructionElement",
            "TupleMember",
            "TypedTupleMember",
            "UntypedTupleMember",
            "IfStatement",
            "ElseBranch",
            "ForStatement",
            "WhileStatement",
            "Expression0",
            "Expression1",
            "Expression2",
            "Expression3",
            "Expression4",
            "Expression5",
            "Expression6",
            "Expression7",
            "Expression8",
            "Expression9",
            "Expression10",
            "Expression11",
            "Expression12",
            "Expression13",
            "Expression14",
            "Expression15",
            "Expression16",
            "Expression17",
            "Expression18",
            "Expression19",
            "Expression",
            "TupleExpression",
            "TupleValues",
            "TupleValue",
            "HexNumberExpression",
        ];

        items.retain(|item| !EXCLUDED_ITEMS.contains(&(*item.name).as_str()));

        items

        // This ended up making compilation slower, it may be worth checking the parser performance in the future with
        // this enabled
        // if let Some(val) = self.references.get(&item.name) {
        //     if val == &1 {
        //         item.inline = true;
        //     }
        // }
    }
}

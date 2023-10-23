mod definitions;
mod reachability;
mod references;

use crate::{
    compiler::{
        analysis::{
            definitions::analyze_definitions, reachability::analyze_reachability,
            references::analyze_references,
        },
        versions::VersionSet,
    },
    internals::{ErrorsCollection, ParseOutput, Spanned},
    spanned::{Item, ItemKind, Language},
    Identifier,
};
use indexmap::IndexMap;
use proc_macro2::Span;
use std::rc::Rc;

pub struct Analysis {
    pub errors: ErrorsCollection,
    pub language: Rc<Language>,
    pub metadata: IndexMap<Identifier, ItemMetadata>,
}

pub struct ItemMetadata {
    pub name: Spanned<Identifier>,
    pub kind: ItemKind,

    pub defined_in: VersionSet,
    pub used_in: VersionSet,

    pub referenced_from: Vec<Span>,
    pub referenced_items: Vec<Identifier>,
}

impl Analysis {
    pub fn analyze(parse_output: ParseOutput) -> Self {
        let ParseOutput { language, errors } = parse_output;

        let mut analysis = Self {
            errors,
            language: language.into(),
            metadata: IndexMap::new(),
        };

        // Early return if there are already errors, to prevent producing noise from later analysis:
        if analysis.errors.has_errors() {
            return analysis;
        }

        analyze_definitions(&mut analysis);

        // Early return if there are already errors, to prevent producing noise from later analysis:
        if analysis.errors.has_errors() {
            return analysis;
        }

        analyze_references(&mut analysis);

        // Early return if there are already errors, to prevent producing noise from later analysis:
        if analysis.errors.has_errors() {
            return analysis;
        }

        analyze_reachability(&mut analysis);

        return analysis;
    }
}

impl Language {
    fn items(&self) -> impl Iterator<Item = &Item> {
        return self
            .sections
            .iter()
            .flat_map(|section| &section.topics)
            .flat_map(|topic| &topic.items)
            .map(|item| &***item);
    }
}

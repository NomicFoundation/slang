mod definitions;
mod reachability;
mod references;
mod utils;

use std::rc::Rc;

use indexmap::IndexMap;
use proc_macro2::Span;

use crate::{
    compiler::analysis::{
        definitions::analyze_definitions, reachability::analyze_reachability,
        references::analyze_references,
    },
    compiler::version_set::VersionSet,
    internals::{ErrorsCollection, ParseOutput, Spanned},
    model::{Identifier, SpannedItem, SpannedLanguage},
};

pub(crate) struct Analysis {
    pub errors: ErrorsCollection,
    pub language: Rc<SpannedLanguage>,
    pub metadata: IndexMap<Identifier, ItemMetadata>,
}

pub(crate) struct ItemMetadata {
    pub name: Spanned<Identifier>,
    pub item: Rc<SpannedItem>,

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

        analysis
    }
}

impl SpannedLanguage {
    fn items(&self) -> impl Iterator<Item = &Rc<SpannedItem>> {
        return self
            .sections
            .iter()
            .flat_map(|section| &section.topics)
            .flat_map(|topic| &topic.items);
    }
}

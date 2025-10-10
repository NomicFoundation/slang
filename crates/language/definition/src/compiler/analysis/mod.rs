mod definitions;
mod reachability;
mod references;
mod utils;

use std::rc::Rc;

use indexmap::IndexMap;
use proc_macro2::Span;

use crate::compiler::analysis::definitions::analyze_definitions;
use crate::compiler::analysis::reachability::analyze_reachability;
use crate::compiler::analysis::references::analyze_references;
use crate::compiler::version_set::VersionSet;
use crate::internals::{ErrorsCollection, ParseOutput, Spanned};
use crate::model::{Identifier, SpannedItem, SpannedLanguage};

pub(crate) struct Analysis {
    pub errors: ErrorsCollection,
    pub language: Rc<SpannedLanguage>,
    pub metadata: IndexMap<Identifier, ItemMetadata>,
}

pub(crate) struct ItemMetadata {
    pub name: Spanned<Identifier>,
    pub item: SpannedItem,

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
    fn items(&self) -> impl Iterator<Item = &SpannedItem> {
        self.sections
            .iter()
            .flat_map(|section| &section.topics)
            .flat_map(|topic| &topic.items)
    }
}

use codegen_language_definition::model;
use indexmap::IndexMap;
use serde::Serialize;

use super::ir_model::{Choice, Collection, IrModel, Sequence};

#[derive(Default, Serialize)]
pub struct TransformedIrModel {
    pub source_name: String,

    pub sequences: IndexMap<model::Identifier, TransformedSequence>,
    pub choices: IndexMap<model::Identifier, TransformedChoice>,
    pub repeated: IndexMap<model::Identifier, Collection>,
    pub separated: IndexMap<model::Identifier, Collection>,
}

#[derive(Clone, Serialize)]
pub struct TransformedSequence {
    pub fields: Vec<TransformedField>,
    pub has_added_fields: bool,
}

#[derive(Clone, Serialize)]
pub struct TransformedField {
    pub label: model::Identifier,
    pub r#type: model::Identifier,

    pub is_terminal: bool,
    pub is_optional: bool,
    pub is_removed: bool,
}

#[derive(Clone, Serialize)]
pub struct TransformedChoice {
    pub nonterminal_types: Vec<model::Identifier>,
    pub terminal_types: Vec<model::Identifier>,
    pub unique_terminal_types: Vec<model::Identifier>,
    pub has_removed_variants: bool,
}

impl TransformedIrModel {
    pub fn diff(source: &IrModel, target: &IrModel) -> Self {
        let source_name = source.name.clone();

        let sequences = source
            .sequences
            .iter()
            .filter_map(|(identifier, source_sequence)| {
                let target_sequence = target.sequences.get(identifier)?;
                let transformed_sequence = Self::diff_sequence(source_sequence, target_sequence);
                Some((identifier.clone(), transformed_sequence))
            })
            .collect();

        let choices = source
            .choices
            .iter()
            .filter_map(|(identifier, source_choice)| {
                let target_choice = target.choices.get(identifier)?;
                let transformed_choice = Self::diff_choice(source_choice, target_choice);

                Some((identifier.clone(), transformed_choice))
            })
            .collect();

        // Remove repeated types not present in target. Assume others are identical.
        let repeated = source
            .repeated
            .iter()
            .filter_map(|(identifier, repeated)| {
                if target.repeated.contains_key(identifier) {
                    Some((identifier.clone(), repeated.clone()))
                } else {
                    None
                }
            })
            .collect();

        // Remove separated types not present in target. Assume others are identical.
        let separated = source
            .separated
            .iter()
            .filter_map(|(identifier, separated)| {
                if target.separated.contains_key(identifier) {
                    Some((identifier.clone(), separated.clone()))
                } else {
                    None
                }
            })
            .collect();

        TransformedIrModel {
            source_name,
            sequences,
            choices,
            repeated,
            separated,
        }
    }

    fn diff_sequence(
        source_sequence: &Sequence,
        target_sequence: &Sequence,
    ) -> TransformedSequence {
        // For sequences we compute determine which fields were removed, and if
        // *any* was added (because in that case we cannot generate
        // transformation code). We assume the order of the existing fields is
        // not changed.
        let mut fields = Vec::new();
        let mut target_index = 0;
        for source_field in &source_sequence.fields {
            let target_field = target_sequence.fields.get(target_index);

            let is_removed = if target_field
                .is_some_and(|target_field| target_field.label == source_field.label)
            {
                target_index += 1;
                false
            } else {
                true
            };

            fields.push(TransformedField {
                label: source_field.label.clone(),
                r#type: source_field.r#type.clone(),
                is_terminal: source_field.is_terminal,
                is_optional: source_field.is_optional,
                is_removed,
            });
        }
        let has_added_fields = target_index < target_sequence.fields.len();
        TransformedSequence {
            fields,
            has_added_fields,
        }
    }

    fn diff_choice(source_choice: &Choice, target_choice: &Choice) -> TransformedChoice {
        // For choices we want to find out if some variant was removed and mark the type if so.
        let (removed_nonterminals, nonterminal_types) = Self::filter_removed_variants(
            &source_choice.nonterminal_types,
            &target_choice.nonterminal_types,
        );
        let (removed_terminals, terminal_types) = Self::filter_removed_variants(
            &source_choice.terminal_types,
            &target_choice.terminal_types,
        );
        let (removed_unique_terminals, unique_terminal_types) = Self::filter_removed_variants(
            &source_choice.unique_terminal_types,
            &target_choice.unique_terminal_types,
        );
        let has_removed_variants =
            removed_nonterminals || removed_terminals || removed_unique_terminals;

        TransformedChoice {
            nonterminal_types,
            terminal_types,
            unique_terminal_types,
            has_removed_variants,
        }
    }

    fn filter_removed_variants(
        source_variants: &[model::Identifier],
        target_variants: &[model::Identifier],
    ) -> (bool, Vec<model::Identifier>) {
        let mut variants = Vec::new();
        let mut target_index = 0;
        let mut has_removed_variants = false;
        for source_variant in source_variants {
            let target_variant = target_variants.get(target_index);
            if target_variant.is_some_and(|target_variant| target_variant == source_variant) {
                variants.push(source_variant.clone());
                target_index += 1;
            } else {
                has_removed_variants = true;
            }
        }
        (has_removed_variants, variants)
    }
}

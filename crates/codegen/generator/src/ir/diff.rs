use indexmap::IndexMap;
use language_definition::model;
use serde::Serialize;

use super::model::{Choice, Collection, IrModel, Sequence};

#[derive(Default, Serialize)]
pub struct IrModelDiff {
    pub sequences: IndexMap<model::Identifier, SequenceDiff>,
    pub choices: IndexMap<model::Identifier, ChoiceDiff>,
    pub collections: IndexMap<model::Identifier, Collection>,
}

#[derive(Clone, Serialize)]
pub struct SequenceDiff {
    pub fields: Vec<FieldDiff>,
    pub has_added_fields: bool,
}

#[derive(Clone, Serialize)]
pub struct FieldDiff {
    pub label: model::Identifier,
    pub r#type: model::Identifier,

    pub is_terminal: bool,
    pub is_optional: bool,
    pub is_removed: bool,
}

#[derive(Clone, Serialize)]
pub struct ChoiceDiff {
    pub nonterminal_types: Vec<model::Identifier>,
    pub non_unique_terminal_types: Vec<model::Identifier>,
    pub unique_terminal_types: Vec<model::Identifier>,
    pub has_removed_variants: bool,
}

impl IrModelDiff {
    pub fn diff(source: &IrModel, target: &IrModel) -> Self {
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

        // Remove collections types not present in target. Assume others are identical.
        let collections = source
            .collections
            .iter()
            .filter_map(|(identifier, repeated)| {
                if target.collections.contains_key(identifier) {
                    Some((identifier.clone(), repeated.clone()))
                } else {
                    None
                }
            })
            .collect();

        IrModelDiff {
            sequences,
            choices,
            collections,
        }
    }

    fn diff_sequence(source_sequence: &Sequence, target_sequence: &Sequence) -> SequenceDiff {
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

            fields.push(FieldDiff {
                label: source_field.label.clone(),
                r#type: source_field.r#type.clone(),
                is_terminal: source_field.is_terminal,
                is_optional: source_field.is_optional,
                is_removed,
            });
        }
        let has_added_fields = target_index < target_sequence.fields.len();
        SequenceDiff {
            fields,
            has_added_fields,
        }
    }

    fn diff_choice(source_choice: &Choice, target_choice: &Choice) -> ChoiceDiff {
        // For choices we want to find out if some variant was removed and mark the type if so.
        let (removed_nonterminals, nonterminal_types) = Self::filter_removed_variants(
            &source_choice.nonterminal_types,
            &target_choice.nonterminal_types,
        );
        let (removed_terminals, terminal_types) = Self::filter_removed_variants(
            &source_choice.non_unique_terminal_types,
            &target_choice.non_unique_terminal_types,
        );
        let (removed_unique_terminals, unique_terminal_types) = Self::filter_removed_variants(
            &source_choice.unique_terminal_types,
            &target_choice.unique_terminal_types,
        );
        let has_removed_variants =
            removed_nonterminals || removed_terminals || removed_unique_terminals;

        ChoiceDiff {
            nonterminal_types,
            non_unique_terminal_types: terminal_types,
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

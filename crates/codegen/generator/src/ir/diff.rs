use indexmap::IndexMap;
use language_definition::model;
use serde::Serialize;

use super::model::{Collection, IrModel};

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
    pub fn new_from(source: &IrModel) -> Self {
        let sequences = source
            .sequences
            .iter()
            .map(|(identifier, sequence)| {
                let fields = sequence
                    .fields
                    .iter()
                    .map(|field| FieldDiff {
                        label: field.label.clone(),
                        r#type: field.r#type.clone(),
                        is_terminal: field.is_terminal,
                        is_optional: field.is_optional,
                        is_removed: false,
                    })
                    .collect();
                (
                    identifier.clone(),
                    SequenceDiff {
                        fields,
                        has_added_fields: false,
                    },
                )
            })
            .collect();

        let choices = source
            .choices
            .iter()
            .map(|(identifier, choice)| {
                (
                    identifier.clone(),
                    ChoiceDiff {
                        nonterminal_types: choice.nonterminal_types.clone(),
                        non_unique_terminal_types: choice.non_unique_terminal_types.clone(),
                        unique_terminal_types: choice.unique_terminal_types.clone(),
                        has_removed_variants: false,
                    },
                )
            })
            .collect();

        let collections = source.collections.clone();

        IrModelDiff {
            sequences,
            choices,
            collections,
        }
    }
}

use indexmap::{IndexMap, IndexSet};
use language_definition::model;
use serde::Serialize;

use super::model::{Choice, Collection, Field, IrModel, Sequence};

#[derive(Default, Serialize)]
pub struct IrModelMutator {
    pub sequences: IndexMap<model::Identifier, MutatedSequence>,
    pub choices: IndexMap<model::Identifier, MutatedChoice>,
    pub collections: IndexMap<model::Identifier, Collection>,

    // Single field sequences that should be collapsed to their content.
    pub collapsed_sequences: IndexMap<model::Identifier, CollapsedSequence>,

    // Set of non-unique terminals, ie. the value depends on the node contents,
    // eg. Identifier.
    #[serde(skip)]
    pub non_unique_terminals: IndexSet<model::Identifier>,

    // Set of unique terminals, ie. content is fixed for the kind, eg. Asterisk.
    #[serde(skip)]
    pub unique_terminals: IndexSet<model::Identifier>,
}

#[derive(Clone, Serialize)]
pub struct MutatedSequence {
    pub fields: Vec<MutatedField>,
    // Indicates that new fields where added to the sequence, making it
    // impossible to auto-generate a transformer function.
    pub has_added_fields: bool,
}

impl From<&Sequence> for MutatedSequence {
    fn from(value: &Sequence) -> Self {
        Self {
            fields: value.fields.iter().map(|field| field.into()).collect(),
            has_added_fields: false,
        }
    }
}

impl From<&MutatedSequence> for Sequence {
    fn from(value: &MutatedSequence) -> Self {
        let fields = value
            .fields
            .iter()
            .filter_map(|field| {
                if field.is_removed {
                    None
                } else {
                    Some(field.into())
                }
            })
            .collect();
        Self {
            fields,
            // NOTE: not strictly correct, but this info is only used for the
            // transformation from the CST model into L1
            multiple_operators: false,
        }
    }
}

#[derive(Clone, Serialize)]
pub struct CollapsedSequence {
    // Label of the field of the collapsed sequence.
    pub label: model::Identifier,
    // Original type of the field of the collapsed sequence, used to generate
    // the transformer function.
    pub r#type: model::Identifier,
    // Target type of the collapsed sequence. This should usually be `r#type`,
    // unless that it collapsed as well.
    pub target_type: model::Identifier,
    // Whether the target type is a terminal or not
    pub target_is_terminal: bool,
}

#[derive(Clone, Serialize)]
#[allow(clippy::struct_excessive_bools)]
pub struct MutatedField {
    pub label: model::Identifier,
    pub r#type: model::Identifier,
    pub is_terminal: bool,
    pub is_optional: bool,

    pub target_type: model::Identifier,
    pub target_is_terminal: bool,

    pub is_removed: bool,
}

impl From<&Field> for MutatedField {
    fn from(value: &Field) -> Self {
        Self {
            label: value.label.clone(),
            r#type: value.r#type.clone(),
            is_terminal: value.is_terminal,
            is_optional: value.is_optional,
            target_type: value.r#type.clone(),
            target_is_terminal: value.is_terminal,
            is_removed: false,
        }
    }
}

impl From<&MutatedField> for Field {
    fn from(value: &MutatedField) -> Self {
        Self {
            label: value.label.clone(),
            r#type: value.target_type.clone(),
            is_terminal: value.target_is_terminal,
            is_optional: value.is_optional,
        }
    }
}

#[derive(Clone, Serialize)]
pub struct MutatedChoice {
    pub nonterminal_types: Vec<model::Identifier>,
    pub non_unique_terminal_types: Vec<model::Identifier>,
    pub unique_terminal_types: Vec<model::Identifier>,

    pub added_nonterminal_types: Vec<model::Identifier>,
    pub added_non_unique_terminal_types: Vec<model::Identifier>,
    pub added_unique_terminal_types: Vec<model::Identifier>,

    // Indicates some variants have been removed so the transformer
    // generator can handle them.
    pub has_removed_variants: bool,

    // Indicates this is a new choice type, created in this language.
    pub is_new: bool,
}

impl From<&Choice> for MutatedChoice {
    fn from(value: &Choice) -> Self {
        Self {
            nonterminal_types: value.nonterminal_types.clone(),
            non_unique_terminal_types: value.non_unique_terminal_types.clone(),
            unique_terminal_types: value.unique_terminal_types.clone(),
            added_nonterminal_types: Vec::new(),
            added_non_unique_terminal_types: Vec::new(),
            added_unique_terminal_types: Vec::new(),
            has_removed_variants: false,
            is_new: false,
        }
    }
}

impl From<&MutatedChoice> for Choice {
    fn from(value: &MutatedChoice) -> Self {
        let mut nonterminal_types = value.nonterminal_types.clone();
        nonterminal_types.extend_from_slice(&value.added_nonterminal_types);
        let mut non_unique_terminal_types = value.non_unique_terminal_types.clone();
        non_unique_terminal_types.extend_from_slice(&value.added_non_unique_terminal_types);
        let mut unique_terminal_types = value.unique_terminal_types.clone();
        unique_terminal_types.extend_from_slice(&value.added_unique_terminal_types);
        Self {
            nonterminal_types,
            non_unique_terminal_types,
            unique_terminal_types,
        }
    }
}

impl IrModelMutator {
    pub fn create_from(source: &IrModel) -> Self {
        let sequences = source
            .sequences
            .iter()
            .map(|(identifier, sequence)| (identifier.clone(), sequence.into()))
            .collect();

        let choices = source
            .choices
            .iter()
            .map(|(identifier, choice)| (identifier.clone(), choice.into()))
            .collect();

        let collections = source.collections.clone();

        let unique_terminals = source.unique_terminals.clone();
        let non_unique_terminals = source.non_unique_terminals.clone();

        IrModelMutator {
            sequences,
            choices,
            collections,
            collapsed_sequences: IndexMap::new(),
            unique_terminals,
            non_unique_terminals,
        }
    }

    pub fn build_target(&self) -> IrModel {
        let non_unique_terminals = self.non_unique_terminals.clone();
        let unique_terminals = self.unique_terminals.clone();

        let sequences = self
            .sequences
            .iter()
            .map(|(identifier, sequence)| (identifier.clone(), sequence.into()))
            .collect();

        let choices = self
            .choices
            .iter()
            .map(|(identifier, choice)| (identifier.clone(), choice.into()))
            .collect();

        let collections = self.collections.clone();

        IrModel {
            non_unique_terminals,
            unique_terminals,
            sequences,
            choices,
            collections,
        }
    }

    pub fn add_choice_type(&mut self, name: &str) {
        self.choices.insert(
            name.into(),
            MutatedChoice {
                nonterminal_types: Vec::new(),
                non_unique_terminal_types: Vec::new(),
                unique_terminal_types: Vec::new(),
                added_nonterminal_types: Vec::new(),
                added_non_unique_terminal_types: Vec::new(),
                added_unique_terminal_types: Vec::new(),
                is_new: true,
                has_removed_variants: false,
            },
        );
    }

    pub fn add_choice_unique_terminal(&mut self, choice_id: &str, unique_terminal: &str) {
        let identifier: model::Identifier = choice_id.into();
        let Some(choice) = self.choices.get_mut(&identifier) else {
            panic!("Choice {choice_id} not found in IR model");
        };
        choice
            .added_unique_terminal_types
            .push(unique_terminal.into());
        // TODO: we should verify that the type added is indeed a terminal type
        // and is unique; but if we are adding a custom enum type we don't want
        // to do that
    }

    pub fn add_choice_nonterminal(&mut self, choice_id: &str, nonterminal: &str) {
        let identifier: model::Identifier = choice_id.into();
        let Some(choice) = self.choices.get_mut(&identifier) else {
            panic!("Choice {choice_id} not found in IR model");
        };
        choice.added_nonterminal_types.push(nonterminal.into());
        // TODO: we should verify that the type exists as a non-terminal
    }

    pub fn remove_type(&mut self, name: &str) {
        let identifier: model::Identifier = name.into();
        let removed = self.sequences.shift_remove(&identifier).is_some()
            || self.choices.shift_remove(&identifier).is_some()
            || self.collections.shift_remove(&identifier).is_some()
            || self.unique_terminals.shift_remove(&identifier)
            || self.non_unique_terminals.shift_remove(&identifier);

        assert!(removed, "Could not find type {name} to remove");

        for (_, sequence) in &mut self.sequences {
            for field in &mut sequence.fields {
                if field.target_type == identifier {
                    field.is_removed = true;
                }
            }
        }

        for (_, choice) in &mut self.choices {
            if choice.non_unique_terminal_types.contains(&identifier)
                || choice.nonterminal_types.contains(&identifier)
                || choice.unique_terminal_types.contains(&identifier)
            {
                choice.has_removed_variants = true;
            }
            choice.nonterminal_types.retain(|item| *item != identifier);
            choice
                .non_unique_terminal_types
                .retain(|item| *item != identifier);
            choice
                .unique_terminal_types
                .retain(|item| *item != identifier);
        }

        self.collections
            .retain(|_, repeated| repeated.item_type != identifier);
    }

    pub fn remove_sequence_field(&mut self, sequence_id: &str, field_label: &str) {
        let identifier: model::Identifier = sequence_id.into();
        let Some(sequence) = self.sequences.get_mut(&identifier) else {
            panic!("Sequence {sequence_id} not found in IR model");
        };
        let field_label: model::Identifier = field_label.into();

        let mut removed = false;
        for field in &mut sequence.fields {
            if field.label == field_label {
                field.is_removed = true;
                removed = true;
            }
        }

        assert!(
            removed,
            "Could not find field {field_label} to remove in {sequence_id}"
        );
    }

    pub fn add_sequence_field(
        &mut self,
        sequence_id: &str,
        field_label: &str,
        field_type: &str,
        is_optional: bool,
    ) {
        let identifier: model::Identifier = sequence_id.into();
        let Some(sequence) = self.sequences.get_mut(&identifier) else {
            panic!("Sequence {sequence_id} not found in IR model");
        };
        let is_terminal = self
            .non_unique_terminals
            .contains::<model::Identifier>(&field_type.into())
            || self
                .unique_terminals
                .contains::<model::Identifier>(&field_type.into());
        sequence.fields.push(MutatedField {
            label: field_label.into(),
            r#type: field_type.into(),
            is_terminal,
            target_type: field_type.into(),
            target_is_terminal: is_terminal,
            is_optional,
            is_removed: false,
        });
        sequence.has_added_fields = true;
    }

    // Removes a sequence type with a single field from the target language,
    // replacing all instances with the contents of such field.
    pub fn collapse_sequence(&mut self, sequence_id: &str) {
        let identifier: model::Identifier = sequence_id.into();
        let Some(mut sequence) = self.sequences.shift_remove(&identifier) else {
            panic!("Sequence {sequence_id} not found in IR model");
        };
        assert!(
            sequence.fields.len() == 1,
            "Sequence to collapse {sequence_id} contains does not contain a single field"
        );
        let replace_field = sequence.fields.remove(0);
        assert!(
            !replace_field.is_optional,
            "Cannot collapse sequence {sequence_id} of an optional field"
        );

        // Iterate remaining sequences and replace any fields referencing the
        // removed type by the target type
        for (_, sequence) in &mut self.sequences {
            for field in &mut sequence.fields {
                if field.target_type == identifier {
                    field.target_type = replace_field.target_type.clone();
                    field.target_is_terminal = replace_field.target_is_terminal;
                }
            }
        }

        // Determine the target type; the type of the single field may be
        // already collapsed, so we need to use it in that case
        let (target_type, target_is_terminal) =
            if let Some(collapsed) = self.collapsed_sequences.get(&replace_field.r#type) {
                (collapsed.target_type.clone(), collapsed.target_is_terminal)
            } else {
                (replace_field.r#type.clone(), replace_field.is_terminal)
            };

        // Create the collapsed sequence
        self.collapsed_sequences.insert(
            identifier.clone(),
            CollapsedSequence {
                label: replace_field.label,
                r#type: replace_field.r#type.clone(),
                target_type,
                target_is_terminal,
            },
        );

        // Conversely, check if we need to update any other previously collapsed
        // sequences
        for (_, collapsed) in &mut self.collapsed_sequences {
            if collapsed.target_type == identifier {
                collapsed.target_type = replace_field.r#type.clone();
                collapsed.target_is_terminal = replace_field.is_terminal;
            }
        }
    }
}

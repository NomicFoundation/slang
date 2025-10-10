use language_definition::model;

use super::diff::{CollapsedSequence, FieldDiff, IrModelDiff};
use super::model::{Choice, Field};
use super::IrModel;

pub struct IrModelMutator {
    pub target: IrModel,
    pub diff: IrModelDiff,
}

impl IrModelMutator {
    pub fn create_from(source: &IrModel) -> Self {
        let target = IrModel::from_model(source);
        let diff = IrModelDiff::new_from(source);
        Self { target, diff }
    }

    pub fn add_choice_type(&mut self, name: &str) {
        self.target.choices.insert(
            name.into(),
            Choice {
                nonterminal_types: Vec::new(),
                non_unique_terminal_types: Vec::new(),
                unique_terminal_types: Vec::new(),
            },
        );
        // new choice types are not added to the diff
    }

    pub fn add_choice_unique_terminal(&mut self, choice_id: &str, unique_terminal: &str) {
        let identifier: model::Identifier = choice_id.into();
        let Some(choice) = self.target.choices.get_mut(&identifier) else {
            panic!("Choice {choice_id} not found in IR model");
        };
        choice.unique_terminal_types.push(unique_terminal.into());

        // new choice types are not added to the diff
    }

    pub fn remove_type(&mut self, name: &str) {
        let identifier: model::Identifier = name.into();
        let removed = self.target.sequences.shift_remove(&identifier).is_some()
            || self.target.choices.shift_remove(&identifier).is_some()
            || self.target.collections.shift_remove(&identifier).is_some()
            || self.target.unique_terminals.shift_remove(&identifier)
            || self.target.non_unique_terminals.shift_remove(&identifier);

        assert!(removed, "Could not find type {name} to remove");

        self.diff.sequences.shift_remove(&identifier);
        self.diff.choices.shift_remove(&identifier);
        self.diff.collections.shift_remove(&identifier);

        for (_, sequence) in &mut self.target.sequences {
            sequence.fields.retain(|field| field.r#type != identifier);
        }
        for (_, sequence) in &mut self.diff.sequences {
            for field in &mut sequence.fields {
                if field.r#type == identifier {
                    field.is_removed = true;
                }
            }
        }

        for (_, choice) in &mut self.target.choices {
            choice.nonterminal_types.retain(|item| *item != identifier);
            choice
                .non_unique_terminal_types
                .retain(|item| *item != identifier);
            choice
                .unique_terminal_types
                .retain(|item| *item != identifier);
        }
        for (_, choice) in &mut self.diff.choices {
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

        self.target
            .collections
            .retain(|_, repeated| repeated.item_type != identifier);
        self.diff
            .collections
            .retain(|_, repeated| repeated.item_type != identifier);
    }

    pub fn remove_sequence_field(&mut self, sequence_id: &str, field_label: &str) {
        let identifier: model::Identifier = sequence_id.into();
        let Some(sequence) = self.target.sequences.get_mut(&identifier) else {
            panic!("Sequence {sequence_id} not found in IR model");
        };
        let fields_count = sequence.fields.len();
        let field_label: model::Identifier = field_label.into();
        sequence.fields.retain(|field| field.label != field_label);
        assert!(
            fields_count > sequence.fields.len(),
            "Could not find field {field_label} to remove in {sequence_id}"
        );

        let Some(sequence) = self.diff.sequences.get_mut(&identifier) else {
            panic!("Sequence {sequence_id} not found in IR model");
        };
        for field in &mut sequence.fields {
            if field.label == field_label {
                field.is_removed = true;
            }
        }
    }

    pub fn add_sequence_field(
        &mut self,
        sequence_id: &str,
        field_label: &str,
        field_type: &str,
        is_optional: bool,
    ) {
        let identifier: model::Identifier = sequence_id.into();
        let Some(sequence) = self.target.sequences.get_mut(&identifier) else {
            panic!("Sequence {sequence_id} not found in IR model");
        };
        let is_terminal = self
            .target
            .non_unique_terminals
            .contains::<model::Identifier>(&field_type.into())
            || self
                .target
                .unique_terminals
                .contains::<model::Identifier>(&field_type.into());
        sequence.fields.push(Field {
            label: field_label.into(),
            r#type: field_type.into(),
            is_terminal,
            is_optional,
        });

        let Some(sequence) = self.diff.sequences.get_mut(&identifier) else {
            panic!("Sequence {sequence_id} not found in IR model");
        };
        sequence.fields.push(FieldDiff {
            label: field_label.into(),
            r#type: field_type.into(),
            is_terminal,
            is_optional,
            is_removed: false,
        });
        sequence.has_added_fields = true;
    }

    // Removes a sequence type with a single field from the target language,
    // replacing all instances with the contents of such field.
    pub fn collapse_sequence(&mut self, sequence_id: &str) {
        let identifier: model::Identifier = sequence_id.into();
        let Some(mut sequence) = self.target.sequences.shift_remove(&identifier) else {
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
        for (_, sequence) in &mut self.target.sequences {
            for field in &mut sequence.fields {
                if field.r#type == identifier {
                    field.r#type = replace_field.r#type.clone();
                    field.is_terminal = replace_field.is_terminal;
                }
            }
        }

        // Create the CollapsedSequence to render the transformer method
        let Some(mut sequence) = self.diff.sequences.shift_remove(&identifier) else {
            panic!("Sequence {sequence_id} not found in IR diff model");
        };
        let diff_field = sequence.fields.remove(0);

        // Determine the target type; the type of the single field may be
        // already collapsed, so we need to use it in that case
        let (target_type, target_is_terminal) =
            if let Some(collapsed) = self.diff.collapsed_sequences.get(&diff_field.r#type) {
                (collapsed.target_type.clone(), collapsed.target_is_terminal)
            } else {
                (diff_field.r#type.clone(), diff_field.is_terminal)
            };
        self.diff.collapsed_sequences.insert(
            identifier.clone(),
            CollapsedSequence {
                label: diff_field.label,
                r#type: diff_field.r#type.clone(),
                target_type,
                target_is_terminal,
            },
        );

        // Conversely, check if we need to update any other previously collapsed
        // sequences
        for (_, collapsed) in &mut self.diff.collapsed_sequences {
            if collapsed.target_type == identifier {
                collapsed.target_type = diff_field.r#type.clone();
                collapsed.target_is_terminal = diff_field.is_terminal;
            }
        }
    }
}

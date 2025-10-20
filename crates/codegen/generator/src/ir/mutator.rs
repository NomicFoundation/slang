use indexmap::IndexMap;
use language_definition::model;
use serde::Serialize;

use super::model::{Choice, Collection, Field, IrModel, NodeType, Sequence};

#[derive(Default, Serialize)]
pub struct IrModelMutator {
    pub sequences: IndexMap<model::Identifier, MutatedSequence>,
    pub choices: IndexMap<model::Identifier, MutatedChoice>,
    pub collections: IndexMap<model::Identifier, Collection>,

    // Single field sequences that should be collapsed to their content.
    pub collapsed_sequences: IndexMap<model::Identifier, CollapsedSequence>,

    // Terminal nodes and whether they are unique or their value depends on the
    // content. The collection is not needed for generating the code, so it's
    // not necessary to serialize it.
    #[serde(skip)]
    pub terminals: IndexMap<model::Identifier, bool>,
}

#[derive(Clone, Serialize)]
pub struct MutatedSequence {
    pub fields: Vec<MutatedField>,
    // Indicates that new fields where added to the sequence, making it
    // impossible to auto-generate a transformer function.
    pub has_added_fields: bool,
    // If true, this sequence models a precedence expression with multiple
    // operators and the terminals should not be elided. This is only relevant
    // for the initial builder from the CST.
    pub multiple_operators: bool,
}

impl From<&Sequence> for MutatedSequence {
    fn from(value: &Sequence) -> Self {
        Self {
            fields: value.fields.iter().map(|field| field.into()).collect(),
            has_added_fields: false,
            multiple_operators: value.multiple_operators,
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
            multiple_operators: value.multiple_operators,
        }
    }
}

#[derive(Clone, Serialize)]
pub struct CollapsedSequence {
    // Label of the field of the collapsed sequence.
    pub label: model::Identifier,
    // Original type of the field of the collapsed sequence, used to generate
    // the transformer function.
    pub r#type: NodeType,
    // Target type of the collapsed sequence. This should usually be `r#type`,
    // unless that it collapsed as well.
    pub target_type: NodeType,
}

#[derive(Clone, Serialize)]
pub struct MutatedField {
    pub label: model::Identifier,
    pub r#type: NodeType,
    pub is_optional: bool,
    pub target_type: NodeType,

    pub is_removed: bool,
}

impl From<&Field> for MutatedField {
    fn from(value: &Field) -> Self {
        Self {
            label: value.label.clone(),
            r#type: value.r#type.clone(),
            is_optional: value.is_optional,
            target_type: value.r#type.clone(),
            is_removed: false,
        }
    }
}

impl From<&MutatedField> for Field {
    fn from(value: &MutatedField) -> Self {
        Self {
            label: value.label.clone(),
            r#type: value.target_type.clone(),
            is_optional: value.is_optional,
        }
    }
}

#[derive(Clone, Serialize)]
pub struct MutatedChoice {
    pub variants: Vec<NodeType>,
    pub added_variants: Vec<NodeType>,

    // Indicates some variants have been removed so the transformer
    // generator can handle them.
    pub has_removed_variants: bool,

    // Indicates this is a new choice type, created in this language.
    pub is_new: bool,
}

impl From<&Choice> for MutatedChoice {
    fn from(value: &Choice) -> Self {
        Self {
            variants: value.variants.clone(),
            added_variants: Vec::new(),
            has_removed_variants: false,
            is_new: false,
        }
    }
}

impl From<&MutatedChoice> for Choice {
    fn from(value: &MutatedChoice) -> Self {
        let mut variants = value.variants.clone();
        variants.extend_from_slice(&value.added_variants);
        Self { variants }
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

        let terminals = source.terminals.clone();

        IrModelMutator {
            sequences,
            choices,
            collections,
            collapsed_sequences: IndexMap::new(),
            terminals,
        }
    }

    pub fn build_target(&self) -> IrModel {
        let terminals = self.terminals.clone();

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
            terminals,
            sequences,
            choices,
            collections,
        }
    }

    pub fn add_choice_type(&mut self, name: &str) {
        self.choices.insert(
            name.into(),
            MutatedChoice {
                variants: Vec::new(),
                added_variants: Vec::new(),
                is_new: true,
                has_removed_variants: false,
            },
        );
    }

    pub fn add_choice_variant(&mut self, choice_id: &str, variant: &str) {
        let variant_type = self.find_node_type(&variant.into());
        let identifier: model::Identifier = choice_id.into();
        let Some(choice) = self.choices.get_mut(&identifier) else {
            panic!("Choice {choice_id} not found in IR model");
        };
        choice.added_variants.push(variant_type);
    }

    // Adds a synthetic (ie. not referencing any CST nodes) choice type by adding
    // unique terminal types as the variants
    pub fn add_enum_type(&mut self, name: &str, variants: &[&str]) {
        let mut added_variants = Vec::new();
        for variant in variants {
            assert!(
                self.terminals.insert((*variant).into(), true).is_none(),
                "Attempt to insert an already existing unique terminal variant"
            );
            added_variants.push(NodeType::UniqueTerminal((*variant).into()));
        }
        self.choices.insert(
            name.into(),
            MutatedChoice {
                variants: Vec::new(),
                added_variants,
                is_new: true,
                has_removed_variants: false,
            },
        );
    }

    pub fn remove_type(&mut self, name: &str) {
        let removed_type = self.find_node_type(&name.into());
        let identifier: model::Identifier = name.into();
        let removed = self.sequences.shift_remove(&identifier).is_some()
            || self.choices.shift_remove(&identifier).is_some()
            || self.collections.shift_remove(&identifier).is_some()
            || self.terminals.shift_remove(&identifier).is_some();

        assert!(removed, "Could not find type {name} to remove");

        for (_, sequence) in &mut self.sequences {
            for field in &mut sequence.fields {
                if field.target_type == identifier {
                    field.is_removed = true;
                }
            }
        }

        for (_, choice) in &mut self.choices {
            if choice.variants.contains(&removed_type) {
                choice.has_removed_variants = true;
            }
            choice.variants.retain(|item| *item != identifier);
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

    fn find_node_type(&self, identifier: &model::Identifier) -> NodeType {
        match self.terminals.get(identifier) {
            None => NodeType::Nonterminal(identifier.clone()),
            Some(false) => NodeType::Terminal(identifier.clone()),
            Some(true) => NodeType::UniqueTerminal(identifier.clone()),
        }
    }

    pub fn add_sequence_field(
        &mut self,
        sequence_id: &str,
        field_label: &str,
        field_type: &str,
        is_optional: bool,
    ) {
        let target_type = self.find_node_type(&field_type.into());
        let identifier: model::Identifier = sequence_id.into();
        let Some(sequence) = self.sequences.get_mut(&identifier) else {
            panic!("Sequence {sequence_id} not found in IR model");
        };
        sequence.fields.push(MutatedField {
            label: field_label.into(),
            r#type: target_type.clone(),
            is_optional,
            target_type,
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
                }
            }
        }

        // Determine the target type; the type of the single field may be
        // already collapsed, so we need to use it in that case
        let target_type = if let Some(collapsed) = self
            .collapsed_sequences
            .get(replace_field.r#type.as_identifier())
        {
            collapsed.target_type.clone()
        } else {
            replace_field.r#type.clone()
        };

        // Create the collapsed sequence
        self.collapsed_sequences.insert(
            identifier.clone(),
            CollapsedSequence {
                label: replace_field.label,
                r#type: replace_field.r#type.clone(),
                target_type,
            },
        );

        // Conversely, check if we need to update any other previously collapsed
        // sequences
        for (_, collapsed) in &mut self.collapsed_sequences {
            if collapsed.target_type == identifier {
                collapsed.target_type = replace_field.r#type.clone();
            }
        }
    }
}

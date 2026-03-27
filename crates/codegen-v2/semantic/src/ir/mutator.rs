use std::collections::BTreeMap;

use language_v2_definition::model;
use serde::Serialize;

use super::model::{Choice, Collection, Field, IrModel, NodeType, Sequence, Terminal};

#[derive(Default, Serialize)]
pub struct IrModelMutator {
    pub sequences: BTreeMap<model::Identifier, MutatedSequence>,
    pub choices: BTreeMap<model::Identifier, MutatedChoice>,
    pub collections: BTreeMap<model::Identifier, MutatedCollection>,

    // Single field sequences that should be collapsed to their content.
    pub collapsed_sequences: BTreeMap<model::Identifier, CollapsedSequence>,

    // Choice types with all terminal variants collapsed to a single target type.
    pub collapsed_choices: BTreeMap<model::Identifier, CollapsedChoice>,

    // Terminal nodes and whether they are unique or their value depends on the
    // content.
    pub terminals: BTreeMap<model::Identifier, MutatedTerminal>,

    // Terminals that have been normalized (renamed) to a canonical equivalent.
    pub normalized_terminals: BTreeMap<model::Identifier, NormalizedTerminal>,
}

#[derive(Clone, Serialize)]
pub struct MutatedSequence {
    pub fields: Vec<MutatedField>,
    // Indicates that new fields were added to the sequence, making it
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
        let fields = value.fields.iter().map(|field| field.into()).collect();
        Self { fields }
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
pub struct CollapsedChoice {
    // The variants of the original choice type. All must be terminals.
    pub variants: Vec<MutatedVariant>,
    // The target type that replaces the collapsed choice everywhere.
    pub target: NodeType,
    // Whether the target type is an identifier (needed for builder generation).
    pub target_is_identifier: bool,
}

#[derive(Clone, Serialize)]
pub struct MutatedField {
    pub label: model::Identifier,
    pub r#type: NodeType,
    pub is_optional: bool,
    pub target_type: NodeType,
}

impl From<&Field> for MutatedField {
    fn from(value: &Field) -> Self {
        Self {
            label: value.label.clone(),
            r#type: value.r#type.clone(),
            is_optional: value.is_optional,
            target_type: value.r#type.clone(),
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
pub struct MutatedVariant {
    pub source: NodeType,
    pub target: NodeType,
    pub is_new: bool,
}

#[derive(Clone, Serialize)]
pub struct MutatedChoice {
    pub variants: Vec<MutatedVariant>,

    // Indicates some variants have been removed so the transformer
    // generator can handle them.
    pub has_removed_variants: bool,

    // Indicates this is a new choice type, created in this language.
    pub is_new: bool,
}

impl From<&Choice> for MutatedChoice {
    fn from(value: &Choice) -> Self {
        Self {
            variants: value
                .variants
                .iter()
                .map(|v| MutatedVariant {
                    source: v.clone(),
                    target: v.clone(),
                    is_new: false,
                })
                .collect(),
            has_removed_variants: false,
            is_new: false,
        }
    }
}

impl From<&MutatedChoice> for Choice {
    fn from(value: &MutatedChoice) -> Self {
        let variants = value.variants.iter().map(|v| v.target.clone()).collect();
        Self { variants }
    }
}

#[derive(Clone, Serialize)]
pub struct MutatedCollection {
    pub item_type: NodeType,
    pub target_item_type: NodeType,

    // Indicates this is a new collection type, created in this language.
    pub is_new: bool,
}

impl From<&Collection> for MutatedCollection {
    fn from(value: &Collection) -> Self {
        Self {
            item_type: value.item_type.clone(),
            target_item_type: value.item_type.clone(),
            is_new: false,
        }
    }
}

impl From<&MutatedCollection> for Collection {
    fn from(value: &MutatedCollection) -> Self {
        Self {
            item_type: value.target_item_type.clone(),
        }
    }
}

#[derive(Clone, Serialize)]
pub struct MutatedTerminal {
    pub is_unique: bool,
    pub is_identifier: bool,

    // Indicates this is a new terminal, created in this language.
    pub is_new: bool,
}

impl From<&Terminal> for MutatedTerminal {
    fn from(terminal: &Terminal) -> Self {
        Self {
            is_unique: terminal.is_unique,
            is_identifier: terminal.is_identifier,
            is_new: false,
        }
    }
}

impl From<&MutatedTerminal> for Terminal {
    fn from(value: &MutatedTerminal) -> Self {
        Self {
            is_unique: value.is_unique,
            is_identifier: value.is_identifier,
        }
    }
}

#[derive(Clone, Serialize)]
pub struct NormalizedTerminal {
    pub target: model::Identifier,
    pub target_is_identifier: bool,
    pub is_unique: bool,
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

        let collections = source
            .collections
            .iter()
            .map(|(identifier, collection)| (identifier.clone(), collection.into()))
            .collect();

        let terminals = source
            .terminals
            .iter()
            .map(|(identifier, unique)| (identifier.clone(), unique.into()))
            .collect();

        IrModelMutator {
            sequences,
            choices,
            collections,
            collapsed_sequences: BTreeMap::new(),
            collapsed_choices: BTreeMap::new(),
            terminals,
            normalized_terminals: BTreeMap::new(),
        }
    }

    pub fn build_target(&self) -> IrModel {
        let terminals = self
            .terminals
            .iter()
            .map(|(identifier, terminal)| (identifier.clone(), terminal.into()))
            .collect();

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

        let collections = self
            .collections
            .iter()
            .map(|(identifier, collection)| (identifier.clone(), collection.into()))
            .collect();

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
        choice.variants.push(MutatedVariant {
            source: variant_type.clone(),
            target: variant_type,
            is_new: true,
        });
    }

    // Adds a synthetic (ie. not referencing any CST nodes) choice type by adding
    // unique terminal types as the variants
    pub fn add_enum_type(&mut self, name: &str, variants: &[&str]) {
        let mut mutated_variants = Vec::new();
        for variant in variants {
            let identifier: model::Identifier = (*variant).into();
            if let Some(existing) = self.terminals.get(&identifier) {
                assert!(
                    existing.is_unique,
                    "Attempt to insert an already existing terminal '{variant}' that is non-unique"
                );
            } else {
                self.terminals.insert(
                    identifier,
                    MutatedTerminal {
                        is_unique: true,
                        is_identifier: false,
                        is_new: true,
                    },
                );
            }
            let node_type = NodeType::UniqueTerminal((*variant).into());
            mutated_variants.push(MutatedVariant {
                source: node_type.clone(),
                target: node_type,
                is_new: true,
            });
        }
        self.choices.insert(
            name.into(),
            MutatedChoice {
                variants: mutated_variants,
                is_new: true,
                has_removed_variants: false,
            },
        );
    }

    pub fn remove_type(&mut self, name: &str) {
        let identifier: model::Identifier = name.into();
        let removed_type = self.find_node_type(&identifier);
        let removed = self.sequences.remove(&identifier).is_some()
            || self.choices.remove(&identifier).is_some()
            || self.collections.remove(&identifier).is_some()
            || self.terminals.remove(&identifier).is_some();

        assert!(removed, "Could not find type {name} to remove");

        for sequence in self.sequences.values_mut() {
            sequence
                .fields
                .retain(|field| field.target_type != identifier);
        }

        for choice in self.choices.values_mut() {
            let before_len = choice.variants.len();
            choice.variants.retain(|v| v.target != removed_type);
            if choice.variants.len() < before_len {
                choice.has_removed_variants = true;
            }
        }

        self.collections
            .retain(|_, repeated| repeated.target_item_type != identifier);
    }

    pub fn remove_sequence_field(&mut self, sequence_id: &str, field_label: &str) {
        let identifier: model::Identifier = sequence_id.into();
        let Some(sequence) = self.sequences.get_mut(&identifier) else {
            panic!("Sequence {sequence_id} not found in IR model");
        };
        let field_label: model::Identifier = field_label.into();

        let before_len = sequence.fields.len();
        sequence.fields.retain(|field| field.label != field_label);

        assert!(
            before_len > sequence.fields.len(),
            "Could not find field {field_label} to remove in {sequence_id}"
        );
    }

    fn replace_type_references(&mut self, old: &NodeType, new: &NodeType) {
        for sequence in self.sequences.values_mut() {
            for field in &mut sequence.fields {
                if field.target_type == *old {
                    field.target_type = new.clone();
                }
            }
        }
        for choice in self.choices.values_mut() {
            for variant in &mut choice.variants {
                if variant.target == *old {
                    variant.target = new.clone();
                }
            }
        }
        for collection in self.collections.values_mut() {
            if collection.target_item_type == *old {
                collection.target_item_type = new.clone();
            }
        }
        for collapsed in self.collapsed_sequences.values_mut() {
            if collapsed.target_type == *old {
                collapsed.target_type = new.clone();
            }
        }
        for collapsed in self.collapsed_choices.values_mut() {
            if collapsed.target == *old {
                collapsed.target = new.clone();
            }
        }
    }

    fn find_node_type(&self, identifier: &model::Identifier) -> NodeType {
        match self.terminals.get(identifier) {
            None => NodeType::Nonterminal(identifier.clone()),
            Some(MutatedTerminal { is_unique, .. }) => {
                if *is_unique {
                    NodeType::UniqueTerminal(identifier.clone())
                } else {
                    NodeType::Terminal(identifier.clone())
                }
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
        });
        sequence.has_added_fields = true;
    }

    pub fn insert_sequence_field_before(
        &mut self,
        sequence_id: &str,
        field_label: &str,
        field_type: &str,
        is_optional: bool,
        before_label: &str,
    ) {
        let target_type = self.find_node_type(&field_type.into());
        let identifier: model::Identifier = sequence_id.into();
        let Some(sequence) = self.sequences.get_mut(&identifier) else {
            panic!("Sequence {sequence_id} not found in IR model");
        };
        let before_label = before_label.into();
        let Some(insertion_index) = sequence
            .fields
            .iter()
            .position(|field| field.label == before_label)
        else {
            panic!("Could not find {before_label} in sequence {sequence_id}");
        };
        sequence.fields.insert(
            insertion_index,
            MutatedField {
                label: field_label.into(),
                r#type: target_type.clone(),
                is_optional,
                target_type,
            },
        );
        sequence.has_added_fields = true;
    }

    // Removes a sequence type with a single field from the target language,
    // replacing all instances with the contents of such field.
    pub fn collapse_sequence(&mut self, sequence_id: &str) {
        let identifier: model::Identifier = sequence_id.into();
        let Some(sequence) = self.sequences.remove(&identifier) else {
            panic!("Sequence {sequence_id} not found in IR model");
        };
        let mut fields_iter = sequence.fields.into_iter();
        let replace_field = fields_iter
            .next()
            .expect("Sequence to collapse {sequence_id} contains at least one field");
        assert!(
            fields_iter.next().is_none(),
            "Sequence to collapse {sequence_id} contains more than one field"
        );
        assert!(
            !replace_field.is_optional,
            "Cannot collapse sequence {sequence_id} of an optional field"
        );
        let replaced_type = self.find_node_type(&sequence_id.into());

        self.replace_type_references(&replaced_type, &replace_field.target_type);

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
    }

    // Removes a choice type whose variants are all terminals, replacing all
    // references with a single target type. Supports same-name targets (where
    // the choice is transmuted into a non-unique terminal with the same name).
    pub fn collapse_choice(&mut self, choice_id: &str, target: &str) {
        let identifier: model::Identifier = choice_id.into();
        let Some(choice) = self.choices.remove(&identifier) else {
            panic!("Choice {choice_id} not found in IR model");
        };

        // Assert all variants are terminals
        for variant in &choice.variants {
            assert!(
                variant.target.is_terminal(),
                "Cannot collapse choice {choice_id}: variant {} is not a terminal",
                variant.target.as_identifier()
            );
        }

        // The old type is always Nonterminal (choices are nonterminals)
        let collapsed_type = NodeType::Nonterminal(identifier.clone());

        // Ensure the target exists as a non-unique terminal
        let target_id: model::Identifier = target.into();
        if let Some(terminal) = self.terminals.get(&target_id) {
            assert!(
                !terminal.is_unique,
                "Cannot collapse choice {choice_id} to unique terminal {target}",
            );
        } else {
            // Target doesn't exist yet as a terminal
            self.terminals.insert(
                target_id.clone(),
                MutatedTerminal {
                    is_unique: false,
                    is_identifier: false,
                    is_new: true,
                },
            );
        }

        let target_is_identifier = self.terminals[&target_id].is_identifier;
        let target_type = self.find_node_type(&target_id);

        self.replace_type_references(&collapsed_type, &target_type);

        // Record the collapsed choice
        self.collapsed_choices.insert(
            identifier,
            CollapsedChoice {
                variants: choice.variants,
                target: target_type,
                target_is_identifier,
            },
        );
    }

    pub fn add_collection_type(&mut self, name: &str, item_type: &str) {
        let item_type = self.find_node_type(&item_type.into());
        self.collections.insert(
            name.into(),
            MutatedCollection {
                item_type: item_type.clone(),
                target_item_type: item_type,
                is_new: true,
            },
        );
    }

    pub fn normalize_terminal(&mut self, source: &str, target: &str) {
        let source_id: model::Identifier = source.into();
        let target_id: model::Identifier = target.into();

        let source_terminal = self
            .terminals
            .get(&source_id)
            .unwrap_or_else(|| panic!("Source terminal {source} not found"));
        let target_terminal = self
            .terminals
            .get(&target_id)
            .unwrap_or_else(|| panic!("Target terminal {target} not found"));

        assert_eq!(
            source_terminal.is_unique, target_terminal.is_unique,
            "Source and target terminals must have the same uniqueness"
        );

        let is_unique = source_terminal.is_unique;
        let target_is_identifier = target_terminal.is_identifier;
        let source_type = self.find_node_type(&source_id);
        let target_type = self.find_node_type(&target_id);

        // Remove source from terminals
        self.terminals.remove(&source_id);

        self.replace_type_references(&source_type, &target_type);

        // Record the normalization
        self.normalized_terminals.insert(
            source_id,
            NormalizedTerminal {
                target: target_id,
                target_is_identifier,
                is_unique,
            },
        );
    }

    pub fn add_non_unique_terminal(&mut self, identifier: &str) {
        let identifier: model::Identifier = (*identifier).into();
        if let Some(existing) = self.terminals.get(&identifier) {
            assert!(
                !existing.is_unique,
                "Existing terminal {identifier} is unique",
            );
        } else {
            self.terminals.insert(
                identifier,
                MutatedTerminal {
                    is_unique: false,
                    is_identifier: false,
                    is_new: true,
                },
            );
        }
    }
}

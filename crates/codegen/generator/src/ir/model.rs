use indexmap::{IndexMap, IndexSet};
use language_definition::model;
use serde::Serialize;

#[derive(Default, Serialize)]
pub struct IrModel {
    // set of non-unique terminals, ie. the value depends on the node contents, eg. Identifier
    #[serde(skip)]
    pub non_unique_terminals: IndexSet<model::Identifier>,

    // set of unique terminals, ie. content is fixed for the kind, eg. Asterisk
    #[serde(skip)]
    pub unique_terminals: IndexSet<model::Identifier>,

    pub sequences: IndexMap<model::Identifier, Sequence>,
    pub choices: IndexMap<model::Identifier, Choice>,
    pub collections: IndexMap<model::Identifier, Collection>,
}

#[derive(Clone, Serialize)]
pub struct Sequence {
    pub fields: Vec<Field>,
    pub has_nonterminals: bool,
    // if true, this sequence models a precedence expression with multiple
    // operators and the terminals should not be elided
    pub multiple_operators: bool,
}

#[derive(Clone, Serialize)]
pub struct Field {
    pub label: model::Identifier,
    pub r#type: model::Identifier,

    pub is_terminal: bool,
    pub is_optional: bool,
}

#[allow(clippy::struct_field_names)]
#[derive(Clone, Serialize)]
pub struct Choice {
    pub nonterminal_types: Vec<model::Identifier>,
    pub non_unique_terminal_types: Vec<model::Identifier>,
    pub unique_terminal_types: Vec<model::Identifier>,
}

#[derive(Clone, Serialize)]
pub struct Collection {
    pub item_type: model::Identifier,
    pub is_terminal: bool,
}

// Construction
impl IrModel {
    pub fn from_language(language: &model::Language) -> Self {
        let builder = IrModelBuilder::create(language);

        Self {
            non_unique_terminals: builder.non_unique_terminals,
            unique_terminals: builder.unique_terminals,

            sequences: builder.sequences,
            choices: builder.choices,
            collections: builder.collections,
        }
    }

    pub fn from_model(model: &Self) -> Self {
        Self {
            non_unique_terminals: model.non_unique_terminals.clone(),
            unique_terminals: model.unique_terminals.clone(),

            sequences: model.sequences.clone(),
            choices: model.choices.clone(),
            collections: model.collections.clone(),
        }
    }
}

// Mutation methods
impl IrModel {
    pub fn remove_type(&mut self, name: &str) {
        let identifier: model::Identifier = name.into();
        let removed = self.sequences.shift_remove(&identifier).is_some()
            || self.choices.shift_remove(&identifier).is_some()
            || self.collections.shift_remove(&identifier).is_some()
            || self.unique_terminals.shift_remove(&identifier)
            || self.non_unique_terminals.shift_remove(&identifier);

        assert!(removed, "Could not find type {name} to remove");

        for (_, sequence) in &mut self.sequences {
            sequence.fields.retain(|field| field.r#type != identifier);
        }

        for (_, choice) in &mut self.choices {
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
        let fields_count = sequence.fields.len();
        let field_label: model::Identifier = field_label.into();
        sequence.fields.retain(|field| field.label != field_label);
        assert!(
            fields_count > sequence.fields.len(),
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
        sequence.fields.push(Field {
            label: field_label.into(),
            r#type: field_type.into(),
            is_terminal,
            is_optional,
        });
    }
}

struct IrModelBuilder {
    // set of non-unique terminals, ie. the value depends on the node contents, eg. Identifier
    pub non_unique_terminals: IndexSet<model::Identifier>,

    // set of unique terminals, ie. content is fixed for the kind, eg. Asterisk
    pub unique_terminals: IndexSet<model::Identifier>,

    pub sequences: IndexMap<model::Identifier, Sequence>,
    pub choices: IndexMap<model::Identifier, Choice>,
    pub collections: IndexMap<model::Identifier, Collection>,
}

impl IrModelBuilder {
    fn create(language: &model::Language) -> Self {
        let mut builder = Self {
            non_unique_terminals: IndexSet::new(),
            unique_terminals: IndexSet::new(),

            sequences: IndexMap::new(),
            choices: IndexMap::new(),
            collections: IndexMap::new(),
        };

        // First pass: collect all terminals:
        builder.collect_terminals(language);

        // Second pass: use them to build nonterminals:
        builder.collect_nonterminals(language);

        builder
    }

    fn collect_terminals(&mut self, language: &model::Language) {
        for item in language.items() {
            match item {
                model::Item::Struct { .. }
                | model::Item::Enum { .. }
                | model::Item::Repeated { .. }
                | model::Item::Separated { .. }
                | model::Item::Precedence { .. } => {
                    // These items are nonterminals.
                }
                model::Item::Trivia { item } => {
                    self.non_unique_terminals.insert(item.name.clone());
                }
                model::Item::Keyword { item } => {
                    if item.is_unique() {
                        self.unique_terminals.insert(item.name.clone());
                    } else {
                        self.non_unique_terminals.insert(item.name.clone());
                    }
                }
                model::Item::Token { item } => {
                    if item.is_unique() {
                        self.unique_terminals.insert(item.name.clone());
                    } else {
                        self.non_unique_terminals.insert(item.name.clone());
                    }
                }
                model::Item::Fragment { .. } => {
                    // These items are inlined.
                }
            }
        }
    }

    fn collect_nonterminals(&mut self, language: &model::Language) {
        for item in language.items() {
            match item {
                model::Item::Struct { item } => {
                    self.add_struct_item(item);
                }
                model::Item::Enum { item } => {
                    self.add_enum_item(item);
                }
                model::Item::Repeated { item } => {
                    self.add_repeated_item(item);
                }
                model::Item::Separated { item } => {
                    self.add_separated_item(item);
                }
                model::Item::Precedence { item } => {
                    self.add_precedence_item(item);

                    for expr in &item.precedence_expressions {
                        self.add_precedence_expression(&item.name, expr);
                    }
                }
                model::Item::Trivia { .. }
                | model::Item::Keyword { .. }
                | model::Item::Token { .. } => {
                    // These items are terminals.
                }
                model::Item::Fragment { .. } => {
                    // These items are inlined.
                }
            }
        }
    }

    fn add_struct_item(&mut self, item: &model::StructItem) {
        let parent_type = item.name.clone();
        let fields: Vec<_> = self.convert_fields(&item.fields).collect();
        let has_nonterminals = fields.iter().any(|field| !field.is_terminal);

        self.sequences.insert(
            parent_type,
            Sequence {
                fields,
                has_nonterminals,
                multiple_operators: false,
            },
        );
    }

    fn partition_types(
        &self,
        types: impl Iterator<Item = model::Identifier>,
    ) -> (
        Vec<model::Identifier>,
        Vec<model::Identifier>,
        Vec<model::Identifier>,
    ) {
        let mut nonterminal_types = Vec::new();
        let mut terminal_types = Vec::new();
        let mut unique_terminal_types = Vec::new();

        for identifier in types {
            if self.unique_terminals.contains(&identifier) {
                unique_terminal_types.push(identifier);
            } else if self.non_unique_terminals.contains(&identifier) {
                terminal_types.push(identifier);
            } else {
                nonterminal_types.push(identifier);
            }
        }

        (nonterminal_types, terminal_types, unique_terminal_types)
    }

    fn add_enum_item(&mut self, item: &model::EnumItem) {
        let parent_type = item.name.clone();

        let (nonterminal_types, terminal_types, unique_terminal_types) = self.partition_types(
            item.variants
                .iter()
                .map(|variant| variant.reference.clone()),
        );

        self.choices.insert(
            parent_type,
            Choice {
                nonterminal_types,
                non_unique_terminal_types: terminal_types,
                unique_terminal_types,
            },
        );
    }

    fn add_repeated_item(&mut self, item: &model::RepeatedItem) {
        let parent_type = item.name.clone();

        self.collections.insert(
            parent_type,
            Collection {
                item_type: item.reference.clone(),
                is_terminal: self.non_unique_terminals.contains(&item.reference)
                    || self.unique_terminals.contains(&item.reference),
            },
        );
    }

    fn add_separated_item(&mut self, item: &model::SeparatedItem) {
        let parent_type = item.name.clone();

        self.collections.insert(
            parent_type,
            Collection {
                item_type: item.reference.clone(),
                is_terminal: self.non_unique_terminals.contains(&item.reference)
                    || self.unique_terminals.contains(&item.reference),
            },
        );
    }

    fn add_precedence_item(&mut self, item: &model::PrecedenceItem) {
        let parent_type = item.name.clone();

        let precedence_expressions = item
            .precedence_expressions
            .iter()
            .map(|expression| expression.name.clone());

        let primary_expressions = item
            .primary_expressions
            .iter()
            .map(|expression| expression.reference.clone());

        let (nonterminal_types, terminal_types, unique_terminal_types) =
            self.partition_types(precedence_expressions.chain(primary_expressions));

        self.choices.insert(
            parent_type,
            Choice {
                nonterminal_types,
                non_unique_terminal_types: terminal_types,
                unique_terminal_types,
            },
        );
    }

    fn add_precedence_expression(
        &mut self,
        base_name: &model::Identifier,
        expression: &model::PrecedenceExpression,
    ) {
        let parent_type = expression.name.clone();

        // All operators should have the same structure (validated at compile-time),
        // So let's pick up the first one to generate the types:
        let operator = &expression.operators[0];
        let mut fields = vec![];

        let operand = |label: model::PredefinedLabel| Field {
            label: label.as_ref().into(),
            r#type: base_name.clone(),
            is_terminal: false,
            is_optional: false,
        };

        match operator.model {
            model::OperatorModel::Prefix => {
                fields.extend(self.convert_fields(&operator.fields));
                fields.push(operand(model::PredefinedLabel::Operand));
            }
            model::OperatorModel::Postfix => {
                fields.push(operand(model::PredefinedLabel::Operand));
                fields.extend(self.convert_fields(&operator.fields));
            }
            model::OperatorModel::BinaryLeftAssociative
            | model::OperatorModel::BinaryRightAssociative => {
                fields.push(operand(model::PredefinedLabel::LeftOperand));
                fields.extend(self.convert_fields(&operator.fields));
                fields.push(operand(model::PredefinedLabel::RightOperand));
            }
        }
        let has_nonterminals = fields.iter().any(|field| !field.is_terminal);
        let multiple_operators = expression.operators.len() > 1;

        self.sequences.insert(
            parent_type,
            Sequence {
                fields,
                has_nonterminals,
                multiple_operators,
            },
        );
    }

    fn convert_fields<'a>(
        &'a self,
        fields: &'a IndexMap<model::Identifier, model::Field>,
    ) -> impl Iterator<Item = Field> + 'a {
        fields.iter().map(|(label, field)| {
            let (reference, is_optional) = match field {
                model::Field::Required { reference } => (reference, false),
                model::Field::Optional { reference, .. } => (reference, true),
            };

            Field {
                label: label.clone(),
                r#type: reference.clone(),
                is_terminal: self.non_unique_terminals.contains(reference)
                    || self.unique_terminals.contains(reference),
                is_optional,
            }
        })
    }
}

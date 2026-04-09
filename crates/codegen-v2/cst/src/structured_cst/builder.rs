use std::collections::{BTreeMap, BTreeSet};

use indexmap::IndexMap;
use language_v2_definition::model;

use super::model::{Choice, ChoiceVariant, Collection, Field, NodeType, Sequence};

pub(super) struct StructuredCstModelBuilder {
    pub terminals: BTreeSet<model::Identifier>,
    pub sequences: BTreeMap<model::Identifier, Sequence>,
    pub choices: BTreeMap<model::Identifier, Choice>,
    pub collections: BTreeMap<model::Identifier, Collection>,
}

impl StructuredCstModelBuilder {
    pub fn create(language: &model::Language) -> Self {
        let mut builder = Self {
            terminals: BTreeSet::new(),
            sequences: BTreeMap::new(),
            choices: BTreeMap::new(),
            collections: BTreeMap::new(),
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
                model::Item::Trivia { .. } => {
                    // Trivia items are skipped by the parser.
                }
                model::Item::Keyword { item } => {
                    self.terminals.insert(item.name.clone());
                }
                model::Item::Token { item } => {
                    self.terminals.insert(item.name.clone());
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

    fn find_node_type(&self, identifier: &model::Identifier) -> NodeType {
        if self.terminals.contains(identifier) {
            NodeType::Terminal(identifier.clone())
        } else {
            NodeType::Nonterminal(identifier.clone())
        }
    }

    fn add_struct_item(&mut self, item: &model::StructItem) {
        let parent_type = item.name.clone();
        let fields: Vec<_> = self.convert_fields(&item.fields).collect();
        let enabled = item.enabled.clone().unwrap_or_default();

        self.sequences
            .insert(parent_type, Sequence { fields, enabled });
    }

    fn add_enum_item(&mut self, item: &model::EnumItem) {
        let parent_type = item.name.clone();
        let enabled = item.enabled.clone().unwrap_or_default();

        let variants = item
            .variants
            .iter()
            .map(|variant| ChoiceVariant {
                variant_type: self.find_node_type(&variant.reference),
                enabled: variant.enabled.clone().unwrap_or_default(),
            })
            .collect();

        self.choices
            .insert(parent_type, Choice { variants, enabled });
    }

    fn add_repeated_item(&mut self, item: &model::RepeatedItem) {
        let parent_type = item.name.clone();
        let item_type = self.find_node_type(&item.reference);
        let enabled = item.enabled.clone().unwrap_or_default();

        self.collections
            .insert(parent_type, Collection { item_type, enabled });
    }

    fn add_separated_item(&mut self, item: &model::SeparatedItem) {
        let parent_type = item.name.clone();
        let item_type = self.find_node_type(&item.reference);
        let enabled = item.enabled.clone().unwrap_or_default();

        self.collections
            .insert(parent_type, Collection { item_type, enabled });
    }

    fn add_precedence_item(&mut self, item: &model::PrecedenceItem) {
        let parent_type = item.name.clone();
        let enabled = item.enabled.clone().unwrap_or_default();

        let precedence_variants =
            item.precedence_expressions
                .iter()
                .map(|expression| ChoiceVariant {
                    variant_type: self.find_node_type(&expression.name),
                    enabled: model::VersionSpecifier::Always,
                });

        let primary_variants = item
            .primary_expressions
            .iter()
            .map(|expression| ChoiceVariant {
                variant_type: self.find_node_type(&expression.reference),
                enabled: expression.enabled.clone().unwrap_or_default(),
            });

        let variants = precedence_variants.chain(primary_variants).collect();

        self.choices
            .insert(parent_type, Choice { variants, enabled });
    }

    fn add_precedence_expression(
        &mut self,
        base_name: &model::Identifier,
        expression: &model::PrecedenceExpression,
    ) {
        let parent_type = expression.name.clone();

        // TODO: The precedence operators is too complex, and we're making a lot
        // of assumptions everywere.
        // We should make it simpler, in particular it should be correct by construction

        let operator_fields = if expression.operators.len() == 1 {
            // If there's a single operator, we just use its fields directly
            self.convert_fields(&expression.operators[0].fields)
                .collect::<Vec<_>>()
        } else {
            // If there are multiple operators, we create a choice between them
            // They must have a single required field labeled "operator"
            //
            // Note: We use a set to avoid duplicate variants, in particular exponentiation
            // uses the same operator symbol for right and left associative versions, although they live in different
            // versions
            let variants: BTreeSet<_> = expression
                .operators
                .iter()
                .map(|operator| {
                    assert!(
                        operator.fields.len() == 1,
                        "Multiple operators with multiple fields are not supported"
                    );

                    let (ident, field) = operator.fields.first().unwrap();
                    assert_eq!(
                        ident,
                        &model::Identifier::from("operator"),
                        "Operator field must be labeled 'operator'"
                    );
                    if let model::Field::Required { reference } = field {
                        self.find_node_type(reference)
                    } else {
                        panic!("Operator field must be required");
                    }
                })
                .collect();

            let ident =
                model::Identifier::from(format!("{}_{}_Operator", base_name, expression.name));

            // Insert the created choice
            self.choices.insert(
                ident.clone(),
                Choice {
                    variants: variants
                        .into_iter()
                        .map(|variant_type| ChoiceVariant {
                            variant_type,
                            enabled: model::VersionSpecifier::Always,
                        })
                        .collect(),
                    enabled: model::VersionSpecifier::Always,
                },
            );

            // The only field we care about then is a reference to that special operator
            vec![Field {
                label: ident.clone(),
                field_type: NodeType::Nonterminal(ident),
                is_optional: false,
                enabled: model::VersionSpecifier::Always,
            }]
        };

        let operand = |label: model::PredefinedLabel| Field {
            label: label.as_ref().into(),
            field_type: NodeType::Nonterminal(base_name.clone()),
            is_optional: false,
            enabled: model::VersionSpecifier::Always,
        };

        let mut fields = vec![];

        // All operators should have the same structure (validated at compile-time),
        // So let's pick up the first one to generate the types:
        match expression.operators[0].model {
            model::OperatorModel::Prefix => {
                fields.extend(operator_fields);
                fields.push(operand(model::PredefinedLabel::Operand));
            }
            model::OperatorModel::Postfix => {
                fields.push(operand(model::PredefinedLabel::Operand));
                fields.extend(operator_fields);
            }
            model::OperatorModel::BinaryLeftAssociative
            | model::OperatorModel::BinaryRightAssociative => {
                fields.push(operand(model::PredefinedLabel::LeftOperand));
                fields.extend(operator_fields);
                fields.push(operand(model::PredefinedLabel::RightOperand));
            }
        }

        self.sequences.insert(
            parent_type,
            Sequence {
                fields,
                enabled: model::VersionSpecifier::Always,
            },
        );
    }

    fn convert_fields<'a>(
        &'a self,
        fields: &'a IndexMap<model::Identifier, model::Field>,
    ) -> impl Iterator<Item = Field> + 'a {
        fields.iter().map(|(label, field)| {
            let (reference, is_optional, enabled) = match field {
                model::Field::Required { reference } => {
                    (reference, false, model::VersionSpecifier::Always)
                }
                model::Field::Optional { reference, enabled } => {
                    (reference, true, enabled.clone().unwrap_or_default())
                }
            };
            let field_type = self.find_node_type(reference);

            Field {
                label: label.clone(),
                field_type,
                is_optional,
                enabled,
            }
        })
    }
}

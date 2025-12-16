//! This model abstracts away some details from our language definition in order to
//! facilitate generation of AST structures
//!
//! Note: This is a copy of the model in v1 (crates/codegen/generator/src/ir/model.rs) with
//! some small changes

// TODO(v2):
// - Collect the sizes of terminals and nonterminals, this should allow us to optimize space usage
//   in particular, terminals with size 1 can be represented as `()` (or anything 0 sized) and
//   terminals with size N can be represented with an enum

use std::collections::{BTreeMap, BTreeSet};

use indexmap::IndexMap;
use language_v2_definition::model;
use serde::ser::SerializeMap;
use serde::Serialize;

#[derive(Default, Serialize)]
pub struct IrModel {
    /// Terminal nodes and whether they are unique or their value depends on the
    /// content.
    pub terminals: BTreeMap<model::Identifier, bool>,

    /// Nonterminal nodes that are a fixed size group of potentially different nodes
    /// ie a struct
    pub sequences: BTreeMap<model::Identifier, Sequence>,

    /// Nonterminal nodes that are a choice between other nodes
    /// ie an enum
    pub choices: BTreeMap<model::Identifier, Choice>,

    /// Nonterminal nodes that are an unbounded collections of nodes of the same type
    /// ie a vector
    pub collections: BTreeMap<model::Identifier, Collection>,
}

#[derive(Clone, Serialize)]
pub struct Sequence {
    pub fields: Vec<Field>,
}

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum NodeType {
    Nonterminal(model::Identifier),
    Terminal(model::Identifier),
    UniqueTerminal(model::Identifier),
}

#[derive(Clone, Serialize)]
pub struct Field {
    pub label: model::Identifier,
    pub r#type: NodeType,
    pub is_optional: bool,
}

#[allow(clippy::struct_field_names)]
#[derive(Clone, Serialize)]
pub struct Choice {
    pub variants: Vec<NodeType>,
}

#[derive(Clone, Serialize)]
pub struct Collection {
    pub item_type: NodeType,
}

impl NodeType {
    pub fn as_identifier(&self) -> &model::Identifier {
        match self {
            NodeType::Nonterminal(identifier)
            | NodeType::Terminal(identifier)
            | NodeType::UniqueTerminal(identifier) => identifier,
        }
    }

    pub fn is_terminal(&self) -> bool {
        matches!(self, Self::Terminal(_) | Self::UniqueTerminal(_))
    }
}

impl PartialEq<model::Identifier> for NodeType {
    fn eq(&self, other: &model::Identifier) -> bool {
        match self {
            NodeType::Nonterminal(identifier)
            | NodeType::Terminal(identifier)
            | NodeType::UniqueTerminal(identifier) => identifier == other,
        }
    }
}

impl Serialize for NodeType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut map = serializer.serialize_map(Some(4))?;
        let (identifier, kind, is_terminal, is_unique) = match self {
            NodeType::Nonterminal(identifier) => (identifier, "Nonterminal", false, false),
            NodeType::Terminal(identifier) => (identifier, "Terminal", true, false),
            NodeType::UniqueTerminal(identifier) => (identifier, "UniqueTerminal", true, true),
        };
        map.serialize_entry("name", identifier)?;
        map.serialize_entry("kind", kind)?;
        map.serialize_entry("is_terminal", &is_terminal)?;
        map.serialize_entry("is_unique", &is_unique)?;
        map.end()
    }
}

// Construction
impl IrModel {
    pub fn from_language(language: &model::Language) -> Self {
        let builder = IrModelBuilder::create(language);

        Self {
            terminals: builder.terminals,
            sequences: builder.sequences,
            choices: builder.choices,
            collections: builder.collections,
        }
    }

    pub fn from_model(model: &Self) -> Self {
        Self {
            terminals: model.terminals.clone(),
            sequences: model.sequences.clone(),
            choices: model.choices.clone(),
            collections: model.collections.clone(),
        }
    }
}

struct IrModelBuilder {
    pub terminals: BTreeMap<model::Identifier, bool>,
    pub sequences: BTreeMap<model::Identifier, Sequence>,
    pub choices: BTreeMap<model::Identifier, Choice>,
    pub collections: BTreeMap<model::Identifier, Collection>,
}

impl IrModelBuilder {
    fn create(language: &model::Language) -> Self {
        let mut builder = Self {
            terminals: BTreeMap::new(),
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
                model::Item::Trivia { item } => {
                    self.terminals.insert(item.name.clone(), false);
                }
                model::Item::Keyword { item } => {
                    self.terminals.insert(item.name.clone(), item.is_unique());
                }
                model::Item::Token { item } => {
                    self.terminals.insert(item.name.clone(), item.is_unique());
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
        match self.terminals.get(identifier) {
            None => NodeType::Nonterminal(identifier.clone()),
            Some(false) => NodeType::Terminal(identifier.clone()),
            Some(true) => NodeType::UniqueTerminal(identifier.clone()),
        }
    }

    fn add_struct_item(&mut self, item: &model::StructItem) {
        let parent_type = item.name.clone();
        let fields: Vec<_> = self.convert_fields(&item.fields).collect();

        self.sequences.insert(parent_type, Sequence { fields });
    }

    fn add_enum_item(&mut self, item: &model::EnumItem) {
        let parent_type = item.name.clone();

        let variants = item
            .variants
            .iter()
            .map(|variant| self.find_node_type(&variant.reference))
            .collect();

        self.choices.insert(parent_type, Choice { variants });
    }

    fn add_repeated_item(&mut self, item: &model::RepeatedItem) {
        let parent_type = item.name.clone();
        let item_type = self.find_node_type(&item.reference);

        self.collections
            .insert(parent_type, Collection { item_type });
    }

    fn add_separated_item(&mut self, item: &model::SeparatedItem) {
        let parent_type = item.name.clone();
        let item_type = self.find_node_type(&item.reference);

        self.collections
            .insert(parent_type, Collection { item_type });
    }

    fn add_precedence_item(&mut self, item: &model::PrecedenceItem) {
        let parent_type = item.name.clone();

        let precedence_expressions = item
            .precedence_expressions
            .iter()
            .map(|expression| &expression.name);

        let primary_expressions = item
            .primary_expressions
            .iter()
            .map(|expression| &expression.reference);

        let variants = precedence_expressions
            .chain(primary_expressions)
            .map(|item| self.find_node_type(item))
            .collect();

        self.choices.insert(parent_type, Choice { variants });
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
                    variants: variants.into_iter().collect(),
                },
            );

            // The only field we care about then is a reference to that special operator
            vec![Field {
                label: ident.clone(),
                r#type: NodeType::Nonterminal(ident),
                is_optional: false,
            }]
        };

        let operand = |label: model::PredefinedLabel| Field {
            label: label.as_ref().into(),
            r#type: NodeType::Nonterminal(base_name.clone()),
            is_optional: false,
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

        self.sequences.insert(parent_type, Sequence { fields });
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
            let r#type = self.find_node_type(reference);

            Field {
                label: label.clone(),
                r#type,
                is_optional,
            }
        })
    }
}

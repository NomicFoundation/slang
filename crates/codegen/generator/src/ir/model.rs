use indexmap::IndexMap;
use language_definition::model;
use serde::ser::SerializeMap;
use serde::Serialize;

#[derive(Default, Serialize)]
pub struct IrModel {
    // Terminal nodes and whether they are unique or their value depends on the
    // content. The collection is not needed for generating the code, so it's
    // not necessary to serialize it.
    #[serde(skip)]
    pub terminals: IndexMap<model::Identifier, bool>,

    pub sequences: IndexMap<model::Identifier, Sequence>,
    pub choices: IndexMap<model::Identifier, Choice>,
    pub collections: IndexMap<model::Identifier, Collection>,
}

#[derive(Clone, Serialize)]
pub struct Sequence {
    pub fields: Vec<Field>,
    // If true, this sequence models a precedence expression with multiple
    // operators and the terminals should not be elided. This is only relevant
    // for the initial builder from the CST.
    pub multiple_operators: bool,
}

#[derive(Clone, Eq, PartialEq)]
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
    pub terminals: IndexMap<model::Identifier, bool>,
    pub sequences: IndexMap<model::Identifier, Sequence>,
    pub choices: IndexMap<model::Identifier, Choice>,
    pub collections: IndexMap<model::Identifier, Collection>,
}

impl IrModelBuilder {
    fn create(language: &model::Language) -> Self {
        let mut builder = Self {
            terminals: IndexMap::new(),
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

        self.sequences.insert(
            parent_type,
            Sequence {
                fields,
                multiple_operators: false,
            },
        );
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

        // All operators should have the same structure (validated at compile-time),
        // So let's pick up the first one to generate the types:
        let operator = &expression.operators[0];
        let mut fields = vec![];

        let operand = |label: model::PredefinedLabel| Field {
            label: label.as_ref().into(),
            r#type: NodeType::Nonterminal(base_name.clone()),
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
        let multiple_operators = expression.operators.len() > 1;

        self.sequences.insert(
            parent_type,
            Sequence {
                fields,
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
            let r#type = self.find_node_type(reference);

            Field {
                label: label.clone(),
                r#type,
                is_optional,
            }
        })
    }
}

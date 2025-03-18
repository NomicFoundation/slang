use codegen_ebnf::{EbnfModel, PlainWriter};
use codegen_language_definition::model::{self, PredefinedLabel};
use indexmap::{IndexMap, IndexSet};
use serde::Serialize;

#[derive(Default, Serialize)]
pub struct AstModel {
    #[serde(skip)]
    terminals: IndexSet<model::Identifier>,
    #[serde(skip)]
    ebnf: Option<EbnfModel>,

    pub unique_terminals: IndexSet<model::Identifier>,

    pub sequences: IndexMap<model::Identifier, Sequence>,
    pub choices: IndexMap<model::Identifier, Choice>,
    pub repeated: IndexMap<model::Identifier, Repeated>,
    pub separated: IndexMap<model::Identifier, Separated>,
}

#[derive(Serialize)]
pub struct Sequence {
    pub ebnf: String,

    pub fields: Vec<Field>,
}

#[derive(Clone, Serialize)]
pub struct Field {
    pub label: model::Identifier,

    /// AST Type of the field
    pub r#type: model::Identifier,

    pub is_terminal: bool,
    pub is_optional: bool,
}

#[derive(Serialize)]
pub struct Choice {
    pub ebnf: String,

    pub nonterminal_types: Vec<model::Identifier>,
    pub terminal_types: Vec<model::Identifier>,
}

#[derive(Serialize)]
pub struct Repeated {
    pub ebnf: String,

    /// AST Type of the field
    pub item_type: model::Identifier,
    pub is_terminal: bool,
}

#[derive(Serialize)]
pub struct Separated {
    pub ebnf: String,

    /// AST Type of the field
    pub item_type: model::Identifier,
    pub is_terminal: bool,
}

impl AstModel {
    pub fn from_language(language: &model::Language) -> Self {
        let mut model = Self {
            terminals: IndexSet::new(),
            ebnf: Some(EbnfModel::build(language)),

            unique_terminals: IndexSet::new(),

            sequences: IndexMap::new(),
            choices: IndexMap::new(),
            repeated: IndexMap::new(),
            separated: IndexMap::new(),
        };

        // First pass: collect all terminals:
        model.collect_terminals(language);

        // Second pass: use them to build nonterminals:
        model.collect_nonterminals(language);

        model
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
                    self.terminals.insert(item.name.clone());
                }
                model::Item::Keyword { item } => {
                    self.terminals.insert(item.name.clone());
                    if item.is_unique() {
                        self.unique_terminals.insert(item.name.clone());
                    }
                }
                model::Item::Token { item } => {
                    self.terminals.insert(item.name.clone());
                    if item.is_unique() {
                        self.unique_terminals.insert(item.name.clone());
                    }
                }
                model::Item::Fragment { .. } => {
                    // These items are inlined.
                }
            };
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
            };
        }
    }

    fn add_struct_item(&mut self, item: &model::StructItem) {
        let parent_type = item.name.clone();
        let ebnf = self.get_ebnf(&parent_type);
        let fields = self.convert_fields(&item.fields).collect();

        self.sequences
            .insert(parent_type, Sequence { ebnf, fields });
    }

    fn add_enum_item(&mut self, item: &model::EnumItem) {
        let parent_type = item.name.clone();
        let ebnf = self.get_ebnf(&parent_type);

        let (terminal_types, nonterminal_types): (Vec<_>, Vec<_>) = item
            .variants
            .iter()
            .map(|variant| variant.reference.clone())
            .partition(|reference| self.terminals.contains(reference));

        self.choices.insert(
            parent_type,
            Choice {
                ebnf,
                nonterminal_types,
                terminal_types,
            },
        );
    }

    fn add_repeated_item(&mut self, item: &model::RepeatedItem) {
        let parent_type = item.name.clone();
        let ebnf = self.get_ebnf(&parent_type);

        self.repeated.insert(
            parent_type,
            Repeated {
                ebnf,
                item_type: item.reference.clone(),
                is_terminal: self.terminals.contains(&item.reference),
            },
        );
    }

    fn add_separated_item(&mut self, item: &model::SeparatedItem) {
        let parent_type = item.name.clone();
        let ebnf = self.get_ebnf(&parent_type);

        self.separated.insert(
            parent_type,
            Separated {
                ebnf,
                item_type: item.reference.clone(),
                is_terminal: self.terminals.contains(&item.reference),
            },
        );
    }

    fn add_precedence_item(&mut self, item: &model::PrecedenceItem) {
        let parent_type = item.name.clone();
        let ebnf = self.get_ebnf(&parent_type);

        let precedence_expressions = item
            .precedence_expressions
            .iter()
            .map(|expression| expression.name.clone());

        let primary_expressions = item
            .primary_expressions
            .iter()
            .map(|expression| expression.reference.clone());

        let (terminal_types, nonterminal_types): (Vec<_>, Vec<_>) = precedence_expressions
            .chain(primary_expressions)
            .partition(|reference| self.terminals.contains(reference));

        self.choices.insert(
            parent_type,
            Choice {
                ebnf,
                nonterminal_types,
                terminal_types,
            },
        );
    }

    fn add_precedence_expression(
        &mut self,
        base_name: &model::Identifier,
        expression: &model::PrecedenceExpression,
    ) {
        let parent_type = expression.name.clone();
        let ebnf = self.get_ebnf(&parent_type);

        // All operators should have the same structure (validated at compile-time),
        // So let's pick up the first one to generate the types:
        let operator = &expression.operators[0];
        let mut fields = vec![];

        let operand = |label: PredefinedLabel| Field {
            label: label.as_ref().into(),
            r#type: base_name.clone(),
            is_terminal: false,
            is_optional: false,
        };

        match operator.model {
            model::OperatorModel::Prefix => {
                fields.extend(self.convert_fields(&operator.fields));
                fields.push(operand(PredefinedLabel::Operand));
            }
            model::OperatorModel::Postfix => {
                fields.push(operand(PredefinedLabel::Operand));
                fields.extend(self.convert_fields(&operator.fields));
            }
            model::OperatorModel::BinaryLeftAssociative
            | model::OperatorModel::BinaryRightAssociative => {
                fields.push(operand(PredefinedLabel::LeftOperand));
                fields.extend(self.convert_fields(&operator.fields));
                fields.push(operand(PredefinedLabel::RightOperand));
            }
        };

        self.sequences
            .insert(parent_type, Sequence { ebnf, fields });
    }

    fn convert_fields<'a>(
        &'a self,
        fields: &'a IndexMap<model::Identifier, model::Field>,
    ) -> impl Iterator<Item = Field> + 'a {
        fields.iter().map(|(label, field)| {
            let (reference, is_optional) = match field {
                model::Field::Required { reference } => (reference, false),
                model::Field::Optional {
                    reference,
                    enabled: _,
                } => (reference, true),
            };

            Field {
                label: label.clone(),
                r#type: reference.clone(),
                is_terminal: self.terminals.contains(reference),
                is_optional,
            }
        })
    }

    fn get_ebnf(&self, base_name: &model::Identifier) -> String {
        let Some(ebnf) = &self.ebnf else {
            return String::new();
        };

        let mut writer = PlainWriter::default();
        ebnf.serialize(base_name, &mut writer).unwrap();
        writer.flush()
    }
}

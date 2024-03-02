use codegen_language_definition::model;
use indexmap::{IndexMap, IndexSet};
use itertools::Itertools;
use serde::Serialize;

#[derive(Default, Serialize)]
pub struct AstModel {
    #[serde(skip)]
    terminals: IndexSet<model::Identifier>,

    pub sequences: Vec<Sequence>,
    pub choices: Vec<Choice>,
    pub repeated: Vec<Repeated>,
    pub separated: Vec<Separated>,
}

#[derive(Serialize)]
pub struct Sequence {
    pub name: model::Identifier,

    pub fields: Vec<Field>,
}

#[derive(Clone, Serialize)]
pub struct Field {
    pub name: model::Identifier,

    pub reference: model::Identifier,
    pub is_terminal: bool,
    pub is_optional: bool,
}

#[derive(Serialize)]
pub struct Choice {
    pub name: model::Identifier,

    pub terminals: Vec<model::Identifier>,
    pub non_terminals: Vec<model::Identifier>,
}

#[derive(Serialize)]
pub struct Repeated {
    pub name: model::Identifier,

    pub reference: model::Identifier,
    pub is_terminal: bool,
}

#[derive(Serialize)]
pub struct Separated {
    pub name: model::Identifier,

    pub reference: model::Identifier,
    pub is_terminal: bool,

    pub separator: model::Identifier,
}

impl AstModel {
    pub fn create(language: &model::Language) -> Self {
        let mut model = Self::default();

        // First pass: collect all terminals:
        model.collect_terminals(language);

        // Second pass: use them to build non-terminals:
        model.collect_non_terminals(language);

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
                    // These items are non-terminals.
                }
                model::Item::Trivia { item } => {
                    self.terminals.insert(item.name.clone());
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
            };
        }
    }

    fn collect_non_terminals(&mut self, language: &model::Language) {
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
        let name = item.name.clone();
        let fields = self.convert_fields(&item.fields);

        self.sequences.push(Sequence { name, fields });
    }

    fn add_enum_item(&mut self, item: &model::EnumItem) {
        let name = item.name.clone();

        let (terminals, non_terminals) = item
            .variants
            .iter()
            .map(|variant| variant.reference.clone())
            .partition(|reference| self.terminals.contains(reference));

        self.choices.push(Choice {
            name,
            terminals,
            non_terminals,
        });
    }

    fn add_repeated_item(&mut self, item: &model::RepeatedItem) {
        let name = item.name.clone();
        let reference = item.reference.clone();
        let is_terminal = self.terminals.contains(&reference);

        self.repeated.push(Repeated {
            name,
            reference,
            is_terminal,
        });
    }

    fn add_separated_item(&mut self, item: &model::SeparatedItem) {
        let name = item.name.clone();
        let reference = item.reference.clone();
        let is_terminal = self.terminals.contains(&reference);
        let separator = item.separator.clone();

        self.separated.push(Separated {
            name,
            reference,
            is_terminal,
            separator,
        });
    }

    fn add_precedence_item(&mut self, item: &model::PrecedenceItem) {
        let name = item.name.clone();

        let precedence_expressions = item
            .precedence_expressions
            .iter()
            .map(|expression| expression.name.clone());

        let primary_expressions = item
            .primary_expressions
            .iter()
            .map(|expression| expression.reference.clone());

        let (terminals, non_terminals) = precedence_expressions
            .chain(primary_expressions)
            .partition(|reference| self.terminals.contains(reference));

        self.choices.push(Choice {
            name,
            terminals,
            non_terminals,
        });
    }

    fn add_precedence_expression(
        &mut self,
        base_name: &model::Identifier,
        expression: &model::PrecedenceExpression,
    ) {
        let name = expression.name.clone();
        let operator = self.pick_operator(expression);

        let mut fields = self.convert_fields(&operator.fields);

        let operand = |name: &str| Field {
            name: name.into(),
            reference: base_name.clone(),
            is_terminal: self.terminals.contains(base_name),
            is_optional: false,
        };

        match operator.model {
            model::OperatorModel::Prefix => {
                fields.push(operand("operand"));
            }
            model::OperatorModel::Postfix => {
                fields.insert(0, operand("operand"));
            }
            model::OperatorModel::BinaryLeftAssociative
            | model::OperatorModel::BinaryRightAssociative => {
                fields.insert(0, operand("left_operand"));
                fields.push(operand("right_operand"));
            }
        };

        self.sequences.push(Sequence { name, fields });
    }

    /// Temporary hack to assert that types are consistent.
    /// TODO(#872): Replace with DSL validation/intellisense afterwards.
    fn pick_operator<'a>(
        &mut self,
        expression: &'a model::PrecedenceExpression,
    ) -> &'a model::PrecedenceOperator {
        for (left, right) in expression.operators.iter().tuple_windows() {
            let left_fields = self.convert_fields(&left.fields);
            let right_fields = self.convert_fields(&right.fields);

            assert_eq!(left_fields.len(), right_fields.len());

            for (left, right) in left_fields.iter().zip(right_fields.iter()) {
                assert_eq!(left.is_optional, right.is_optional);

                if self.terminals.contains(&left.reference) {
                    assert!(self.terminals.contains(&right.reference));
                } else {
                    assert_eq!(left.reference, right.reference);
                }
            }

            assert!(matches!(
                (left.model, right.model),
                (model::OperatorModel::Prefix, model::OperatorModel::Prefix)
                    | (model::OperatorModel::Postfix, model::OperatorModel::Postfix)
                    | (
                        model::OperatorModel::BinaryLeftAssociative
                            | model::OperatorModel::BinaryRightAssociative,
                        model::OperatorModel::BinaryLeftAssociative
                            | model::OperatorModel::BinaryRightAssociative,
                    )
            ));
        }

        &expression.operators[0]
    }

    fn convert_fields(&self, fields: &IndexMap<model::Identifier, model::Field>) -> Vec<Field> {
        fields
            .iter()
            .map(|(name, field)| match field {
                model::Field::Required { reference } => Field {
                    name: name.clone(),
                    reference: reference.clone(),
                    is_terminal: self.terminals.contains(reference),
                    is_optional: false,
                },
                model::Field::Optional {
                    reference,
                    enabled: _,
                } => Field {
                    name: name.clone(),
                    reference: reference.clone(),
                    is_terminal: self.terminals.contains(reference),
                    is_optional: true,
                },
            })
            .collect()
    }
}

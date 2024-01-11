use std::collections::HashMap;

use codegen_language_definition::model::{
    EnumItem, EnumVariant, Field, FragmentItem, Identifier, Item, KeywordDefinition, KeywordItem,
    KeywordValue, Language, OperatorModel, PrecedenceExpression, PrecedenceItem,
    PrecedenceOperator, PrimaryExpression, RepeatedItem, Scanner, SeparatedItem, StructItem,
    TokenDefinition, TokenItem, TriviaItem, VersionSpecifier,
};
use indexmap::IndexMap;
use inflector::Inflector;

use crate::model::{Definition, DefinitionKind, Entry, Expression, Value};

pub struct Builder {
    entries: HashMap<String, Entry>,
}

impl Builder {
    pub fn build(language: &Language) -> HashMap<String, Entry> {
        let mut builder = Self {
            entries: HashMap::new(),
        };

        for item in language.items() {
            match item {
                Item::Struct { item } => builder.add_struct_item(item),
                Item::Enum { item } => builder.add_enum_item(item),
                Item::Repeated { item } => builder.add_repeated_item(item),
                Item::Separated { item } => builder.add_separated_item(item),
                Item::Precedence { item } => builder.add_precedence_item(item),
                Item::Trivia { item } => builder.add_trivia_item(item),
                Item::Keyword { item } => builder.add_keyword_item(item),
                Item::Token { item } => builder.add_token_item(item),
                Item::Fragment { item } => builder.add_fragment_item(item),
            };
        }

        builder.entries
    }

    fn add_entry(&mut self, name: &Identifier, is_terminal: Terminal, is_inlined: Inlined) {
        let mut ebnf_id = name.to_string();

        if matches!(is_terminal, Terminal::Yes) {
            ebnf_id = ebnf_id.to_screaming_snake_case();
        }

        if matches!(is_inlined, Inlined::Yes) {
            ebnf_id = format!("«{ebnf_id}»");
        }

        let existing_entry = self.entries.insert(
            name.to_string(),
            Entry::new(name.to_string(), ebnf_id, vec![]),
        );

        assert!(
            existing_entry.is_none(),
            "Entry '{name}' is already defined."
        );
    }

    fn add_definition(
        &mut self,
        name: &Identifier,
        leading_comments: impl IntoIterator<Item = String>,
        values: impl IntoIterator<Item = Value>,
        kind: DefinitionKind,
    ) {
        let definition = Definition::new(
            leading_comments.into_iter().collect(),
            values.into_iter().collect(),
            kind,
        );

        self.entries
            .get_mut(name.as_str())
            .unwrap_or_else(|| panic!("Entry '{name}' is not defined."))
            .definitions
            .push(definition);
    }

    fn add_struct_item(&mut self, struct_item: &StructItem) {
        let StructItem {
            name,
            enabled,
            error_recovery: _,
            fields,
        } = struct_item;

        self.add_entry(name, Terminal::No, Inlined::No);

        self.add_definition(
            name,
            Self::build_enabled_comment(enabled),
            Self::build_fields(fields),
            DefinitionKind::Sequence,
        );
    }

    fn add_enum_item(&mut self, enum_item: &EnumItem) {
        let EnumItem {
            name,
            enabled,
            variants,
        } = enum_item;

        self.add_entry(name, Terminal::No, Inlined::No);

        let variants = variants.iter().map(|EnumVariant { reference, enabled }| {
            Value::new(
                Self::build_ref(reference),
                Self::build_enabled_comment(enabled),
            )
        });

        self.add_definition(
            name,
            Self::build_enabled_comment(enabled),
            variants,
            DefinitionKind::Choice,
        );
    }

    fn add_repeated_item(&mut self, repeated_item: &RepeatedItem) {
        let RepeatedItem {
            name,
            reference,
            enabled,
        } = repeated_item;

        self.add_entry(name, Terminal::No, Inlined::No);

        let expression = Expression::new_one_or_more(Self::build_ref(reference).into());

        self.add_definition(
            name,
            Self::build_enabled_comment(enabled),
            Some(Value::new(expression, None)),
            DefinitionKind::Sequence,
        );
    }

    fn add_separated_item(&mut self, separated_item: &SeparatedItem) {
        let SeparatedItem {
            name,
            reference,
            separator,
            enabled,
        } = separated_item;

        self.add_entry(name, Terminal::No, Inlined::No);

        let expression = Expression::new_sequence(vec![
            Self::build_ref(reference),
            Expression::new_zero_or_more(
                Expression::new_sequence(vec![
                    Self::build_ref(separator),
                    Self::build_ref(reference),
                ])
                .into(),
            ),
        ]);

        self.add_definition(
            name,
            Self::build_enabled_comment(enabled),
            Some(Value::new(expression, None)),
            DefinitionKind::Sequence,
        );
    }

    fn add_precedence_item(&mut self, precedence_item: &PrecedenceItem) {
        let PrecedenceItem {
            name: base_name,
            enabled,
            precedence_expressions,
            primary_expressions,
        } = precedence_item;

        self.add_entry(base_name, Terminal::No, Inlined::No);

        let mut values = vec![];

        for precedence_expression in precedence_expressions {
            let PrecedenceExpression { name, operators } = precedence_expression.as_ref();

            values.push(Value::new(Self::build_ref(name), None));

            self.add_entry(name, Terminal::No, Inlined::No);

            for operator in operators {
                self.add_precedence_operator(base_name, name, operator);
            }
        }

        for primary_expression in primary_expressions {
            let PrimaryExpression { reference, enabled } = primary_expression;

            values.push(Value::new(
                Self::build_ref(reference),
                Self::build_enabled_comment(enabled),
            ));
        }

        self.add_definition(
            base_name,
            Self::build_enabled_comment(enabled),
            values,
            DefinitionKind::Choice,
        );
    }

    fn add_precedence_operator(
        &mut self,
        base_name: &Identifier,
        name: &Identifier,
        precedence_operator: &PrecedenceOperator,
    ) {
        let PrecedenceOperator {
            model: operator_model,
            enabled,
            error_recovery: _,
            fields,
        } = precedence_operator;

        let mut leading_comments = vec![];
        let mut values = Self::build_fields(fields);

        match operator_model {
            OperatorModel::Prefix => {
                leading_comments.push("Prefix unary operator".to_string());

                values.push(Value::new(Self::build_ref(base_name), None));
            }
            OperatorModel::Postfix => {
                leading_comments.push("Postfix unary operator".to_string());

                values.insert(0, Value::new(Self::build_ref(base_name), None));
            }
            OperatorModel::BinaryLeftAssociative => {
                leading_comments.push("Left-associative binary operator".to_string());

                values.insert(0, Value::new(Self::build_ref(base_name), None));
                values.push(Value::new(Self::build_ref(base_name), None));
            }
            OperatorModel::BinaryRightAssociative => {
                leading_comments.push("Right-associative binary operator".to_string());

                values.insert(0, Value::new(Self::build_ref(base_name), None));
                values.push(Value::new(Self::build_ref(base_name), None));
            }
        };

        leading_comments.extend(Self::build_enabled_comment(enabled));

        self.add_definition(name, leading_comments, values, DefinitionKind::Sequence);
    }

    fn build_fields(fields: &IndexMap<Identifier, Field>) -> Vec<Value> {
        fields
            .values()
            .map(|field| match field {
                Field::Required { reference } => Value::new(Self::build_ref(reference), None),
                Field::Optional { reference, enabled } => Value::new(
                    Expression::new_optional(Self::build_ref(reference).into()),
                    Self::build_enabled_comment(enabled),
                ),
            })
            .collect()
    }

    fn add_trivia_item(&mut self, trivia_item: &TriviaItem) {
        let TriviaItem { name, scanner } = trivia_item;

        self.add_entry(name, Terminal::Yes, Inlined::No);

        self.add_definition(
            name,
            None,
            Some(Value::new(Self::build_scanner(scanner), None)),
            DefinitionKind::Sequence,
        );
    }

    fn add_keyword_item(&mut self, keyword_item: &KeywordItem) {
        let KeywordItem {
            name,
            identifier: _,
            definitions,
        } = keyword_item;

        self.add_entry(name, Terminal::Yes, Inlined::No);

        for KeywordDefinition {
            enabled,
            reserved,
            value,
        } in definitions
        {
            let mut leading_comments = vec![];

            leading_comments.extend(Self::build_enabled_comment(enabled));
            leading_comments.extend(Self::build_reserved_comment(reserved));

            self.add_definition(
                name,
                leading_comments,
                Some(Value::new(Self::build_keyword_value(value), None)),
                DefinitionKind::Sequence,
            );
        }
    }

    fn add_token_item(&mut self, token_item: &TokenItem) {
        let TokenItem { name, definitions } = token_item;

        self.add_entry(name, Terminal::Yes, Inlined::No);

        for TokenDefinition { enabled, scanner } in definitions {
            self.add_definition(
                name,
                Self::build_enabled_comment(enabled),
                Some(Value::new(Self::build_scanner(scanner), None)),
                DefinitionKind::Sequence,
            );
        }
    }

    fn add_fragment_item(&mut self, fragment_item: &FragmentItem) {
        let FragmentItem {
            name,
            enabled,
            scanner,
        } = fragment_item;

        self.add_entry(name, Terminal::Yes, Inlined::Yes);

        self.add_definition(
            name,
            Self::build_enabled_comment(enabled),
            Some(Value::new(Self::build_scanner(scanner), None)),
            DefinitionKind::Sequence,
        );
    }

    fn build_keyword_value(keyword_value: &KeywordValue) -> Expression {
        match keyword_value {
            KeywordValue::Sequence { values } => {
                Expression::new_sequence(values.iter().map(Self::build_keyword_value).collect())
            }
            KeywordValue::Optional { value } => {
                Expression::new_optional(Self::build_keyword_value(value).into())
            }
            KeywordValue::Choice { values } => {
                Expression::new_choice(values.iter().map(Self::build_keyword_value).collect())
            }
            KeywordValue::Atom { atom } => Expression::new_atom(atom.clone()),
        }
    }

    fn build_reserved_comment(reserved: &Option<VersionSpecifier>) -> Option<String> {
        match reserved {
            None => None,
            Some(VersionSpecifier::Never) => Some("Never reserved".to_string()),
            Some(VersionSpecifier::From { from }) => Some(format!("Reserved in {from}")),
            Some(VersionSpecifier::Till { till }) => Some(format!("Reserved until {till}")),
            Some(VersionSpecifier::Range { from, till }) => {
                Some(format!("Reserved from {from} until {till}"))
            }
        }
    }

    fn build_enabled_comment(enabled: &Option<VersionSpecifier>) -> Option<String> {
        match enabled {
            None => None,
            Some(VersionSpecifier::Never) => None,
            Some(VersionSpecifier::From { from }) => Some(format!("Introduced in {from}")),
            Some(VersionSpecifier::Till { till }) => Some(format!("Deprecated in {till}")),
            Some(VersionSpecifier::Range { from, till }) => {
                Some(format!("Introduced in {from} and deprecated in {till}."))
            }
        }
    }

    fn build_scanner(scanner: &Scanner) -> Expression {
        match scanner {
            Scanner::Sequence { scanners } => {
                Expression::new_sequence(scanners.iter().map(Self::build_scanner).collect())
            }
            Scanner::Choice { scanners } => {
                Expression::new_choice(scanners.iter().map(Self::build_scanner).collect())
            }
            Scanner::Optional { scanner } => {
                Expression::new_optional(Self::build_scanner(scanner).into())
            }
            Scanner::ZeroOrMore { scanner } => {
                Expression::new_zero_or_more(Self::build_scanner(scanner).into())
            }
            Scanner::OneOrMore { scanner } => {
                Expression::new_one_or_more(Self::build_scanner(scanner).into())
            }
            Scanner::Not { chars } => {
                let expression = if chars.len() == 1 {
                    Expression::new_atom(chars[0].to_string())
                } else {
                    Expression::new_sequence(
                        chars
                            .iter()
                            .map(|ch| Expression::new_atom(ch.to_string()))
                            .collect(),
                    )
                };
                Expression::new_not(expression.into())
            }
            Scanner::Range {
                inclusive_start,
                inclusive_end,
            } => Expression::new_range(
                Expression::new_atom(inclusive_start.to_string()).into(),
                Expression::new_atom(inclusive_end.to_string()).into(),
            ),
            Scanner::Atom { atom } => Expression::new_atom(atom.clone()),
            Scanner::TrailingContext {
                scanner,
                not_followed_by: _,
            } => Self::build_scanner(scanner),
            Scanner::Fragment { reference } => Self::build_ref(reference),
        }
    }

    fn build_ref(reference: &Identifier) -> Expression {
        Expression::new_reference(reference.to_string())
    }
}

#[derive(Clone, Copy)]
enum Terminal {
    Yes,
    No,
}

#[derive(Clone, Copy)]
enum Inlined {
    Yes,
    No,
}

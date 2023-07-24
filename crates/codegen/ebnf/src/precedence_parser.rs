use codegen_schema::types::{OperatorModel, PrecedenceParserRef};

use crate::{nodes::EbnfNode, EbnfSerializer};

impl EbnfNode {
    pub fn from_precedence_parser(
        precedence_parser: &PrecedenceParserRef,
        base_expression: &str,
        serializer: &mut EbnfSerializer,
    ) -> Self {
        let mut choices = vec![];

        for expression in &precedence_parser.operator_expressions {
            let (expression_body, model_description) = match expression.model {
                OperatorModel::BinaryLeftAssociative => (
                    Self::sequence(vec![
                        Self::production_ref(base_expression),
                        Self::from_parser(&expression.operator),
                        Self::production_ref(base_expression),
                    ]),
                    "Binary Operator, Left Associative",
                ),

                OperatorModel::BinaryRightAssociative => (
                    Self::sequence(vec![
                        Self::production_ref(base_expression),
                        Self::from_parser(&expression.operator),
                        Self::production_ref(base_expression),
                    ]),
                    "Binary Operator, Right Associative",
                ),

                OperatorModel::UnaryPrefix => (
                    Self::sequence(vec![
                        Self::from_parser(&expression.operator),
                        Self::production_ref(base_expression),
                    ]),
                    "Unary Operator, Prefix",
                ),

                OperatorModel::UnaryPostfix => (
                    Self::sequence(vec![
                        Self::production_ref(base_expression),
                        Self::from_parser(&expression.operator),
                    ]),
                    "Unary Operator, Postfix",
                ),
            };

            let serialized_expression_body = {
                let mut buffer = String::new();
                serializer.serialize_node(&expression_body, &mut buffer);
                buffer
            };

            choices.push(Self::with_comment(
                Self::with_comment(
                    Self::production_ref(&expression.name),
                    serialized_expression_body,
                ),
                model_description.to_owned(),
            ));

            serializer.serialize_statement(
                &expression.name,
                &Self::with_comment(expression_body, model_description.to_owned()),
            );
        }

        choices.push(Self::from_parser(&precedence_parser.primary_expression));

        return Self::choice(choices);
    }
}

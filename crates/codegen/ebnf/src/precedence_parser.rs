use codegen_schema::types::{OperatorModel, PrecedenceParserRef};

use crate::{nodes::EbnfNode, serialization::GenerateEbnf};

impl GenerateEbnf for PrecedenceParserRef {
    fn generate_ebnf(&self) -> EbnfNode {
        let mut nodes = vec![];

        for expression in &self.operator_expressions {
            let mut comment = None;

            let operator = match expression.model {
                OperatorModel::BinaryLeftAssociative => EbnfNode::sequence(vec![
                    EbnfNode::BaseProduction,
                    expression.operator.generate_ebnf(),
                    EbnfNode::BaseProduction,
                ]),

                OperatorModel::BinaryRightAssociative => {
                    comment = Some("Right Associative".to_owned());

                    EbnfNode::sequence(vec![
                        EbnfNode::BaseProduction,
                        expression.operator.generate_ebnf(),
                        EbnfNode::BaseProduction,
                    ])
                }
                OperatorModel::UnaryPrefix => EbnfNode::sequence(vec![
                    expression.operator.generate_ebnf(),
                    EbnfNode::BaseProduction,
                ]),

                OperatorModel::UnaryPostfix => EbnfNode::sequence(vec![
                    EbnfNode::BaseProduction,
                    expression.operator.generate_ebnf(),
                ]),
            };

            nodes.push(EbnfNode::sub_statement(
                expression.name.to_owned(),
                comment,
                operator,
            ));
        }

        nodes.push(self.primary_expression.generate_ebnf());

        return EbnfNode::choice(nodes);
    }
}

use codegen_schema::types::precedence_parser::{OperatorModel, PrecedenceParserRef};

use crate::{nodes::EbnfNode, serialization::GenerateEbnf};

impl GenerateEbnf for PrecedenceParserRef {
    fn generate_ebnf(&self) -> EbnfNode {
        let mut choices = vec![];

        for operator in &self.definition.operators {
            let mut comment = None;

            let operator_body = match operator.model {
                OperatorModel::BinaryLeftAssociative => EbnfNode::sequence(vec![
                    EbnfNode::BaseProduction,
                    operator.definition.generate_ebnf(),
                    EbnfNode::BaseProduction,
                ]),

                OperatorModel::BinaryRightAssociative => {
                    comment = Some("Right Associative".to_owned());

                    EbnfNode::sequence(vec![
                        EbnfNode::BaseProduction,
                        operator.definition.generate_ebnf(),
                        EbnfNode::BaseProduction,
                    ])
                }
                OperatorModel::UnaryPrefix => EbnfNode::sequence(vec![
                    operator.definition.generate_ebnf(),
                    EbnfNode::BaseProduction,
                ]),

                OperatorModel::UnaryPostfix => EbnfNode::sequence(vec![
                    EbnfNode::BaseProduction,
                    operator.definition.generate_ebnf(),
                ]),
            };

            choices.push(EbnfNode::sub_statement(
                operator.name.to_owned(),
                comment,
                operator_body,
            ));
        }

        choices.push(
            self.definition
                .primary_expression
                .definition
                .generate_ebnf(),
        );

        return EbnfNode::choices(choices);
    }
}

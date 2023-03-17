use codegen_schema::types::precedence_parser::{OperatorModel, PrecedenceParserRef};

use crate::{nodes::EbnfNode, serialization::GenerateEbnf};

impl GenerateEbnf for PrecedenceParserRef {
    fn generate_ebnf(&self) -> EbnfNode {
        let mut alternatives = vec![];

        for operator in &self.definition.operators {
            let operator_body = match operator.model {
                OperatorModel::BinaryRightAssociative | OperatorModel::BinaryLeftAssociative => {
                    EbnfNode::sequence(vec![
                        EbnfNode::BaseProduction,
                        EbnfNode::parenthesis(operator.definition.generate_ebnf()),
                        EbnfNode::BaseProduction,
                    ])
                }

                OperatorModel::UnaryPrefix => EbnfNode::sequence(vec![
                    EbnfNode::parenthesis(operator.definition.generate_ebnf()),
                    EbnfNode::BaseProduction,
                ]),

                OperatorModel::UnaryPostfix => EbnfNode::sequence(vec![
                    EbnfNode::BaseProduction,
                    EbnfNode::parenthesis(operator.definition.generate_ebnf()),
                ]),
            };

            alternatives.push(EbnfNode::statement(operator.name.to_owned(), operator_body));
        }

        alternatives.push(
            self.definition
                .primary_expression
                .definition
                .generate_ebnf(),
        );

        return EbnfNode::alternatives(alternatives);
    }
}

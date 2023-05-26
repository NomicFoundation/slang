use codegen_schema::types::{OperatorModel, PrecedenceParserRef};

use crate::{nodes::EbnfNode, serialization::GenerateEbnf};

impl GenerateEbnf for PrecedenceParserRef {
    fn generate_ebnf(&self) -> EbnfNode {
        let mut choices = vec![];

        for definition in &self.operators {
            let mut comment = None;

            let operator = match definition.model {
                OperatorModel::BinaryLeftAssociative => EbnfNode::sequence(vec![
                    EbnfNode::BaseProduction,
                    definition.operator.generate_ebnf(),
                    EbnfNode::BaseProduction,
                ]),

                OperatorModel::BinaryRightAssociative => {
                    comment = Some("Right Associative".to_owned());

                    EbnfNode::sequence(vec![
                        EbnfNode::BaseProduction,
                        definition.operator.generate_ebnf(),
                        EbnfNode::BaseProduction,
                    ])
                }
                OperatorModel::UnaryPrefix => EbnfNode::sequence(vec![
                    definition.operator.generate_ebnf(),
                    EbnfNode::BaseProduction,
                ]),

                OperatorModel::UnaryPostfix => EbnfNode::sequence(vec![
                    EbnfNode::BaseProduction,
                    definition.operator.generate_ebnf(),
                ]),
            };

            choices.push(EbnfNode::sub_statement(
                definition.name.to_owned(),
                comment,
                operator,
            ));
        }

        choices.push(self.primary_expression.definition.generate_ebnf());

        return EbnfNode::choices(choices);
    }
}

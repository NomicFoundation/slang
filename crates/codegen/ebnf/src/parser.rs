use codegen_schema::types::parser::{ParserDefinition, ParserRef};

use crate::{nodes::EbnfNode, serialization::GenerateEbnf};

impl GenerateEbnf for ParserRef {
    fn generate_ebnf(&self) -> EbnfNode {
        return self.definition.generate_ebnf();
    }
}

impl GenerateEbnf for ParserDefinition {
    fn generate_ebnf(&self) -> EbnfNode {
        match &self {
            ParserDefinition::Choice(sub_exprs) => {
                return EbnfNode::alternatives(
                    sub_exprs
                        .iter()
                        .map(|choice| generate_nested(self, &choice.definition))
                        .collect(),
                );
            }

            ParserDefinition::DelimitedBy {
                open,
                expression,
                close,
            } => {
                return EbnfNode::sequence(vec![
                    EbnfNode::reference(open.reference.to_owned()),
                    expression.definition.generate_ebnf(),
                    EbnfNode::reference(close.reference.to_owned()),
                ]);
            }

            ParserDefinition::OneOrMore(expr) => {
                return EbnfNode::one_or_more(expr.generate_ebnf());
            }

            ParserDefinition::Optional(expr) => {
                return EbnfNode::optional(expr.generate_ebnf());
            }

            ParserDefinition::Reference(name) => {
                return EbnfNode::reference(name.to_owned());
            }

            ParserDefinition::Repeat {
                min,
                max,
                expression,
            } => {
                return EbnfNode::repeat(*min, *max, expression.generate_ebnf());
            }

            ParserDefinition::SeparatedBy {
                expression,
                separator,
            } => {
                return EbnfNode::sequence(vec![
                    expression.definition.generate_ebnf(),
                    EbnfNode::zero_or_more(EbnfNode::sequence(vec![
                        EbnfNode::reference(separator.reference.to_owned()),
                        expression.definition.generate_ebnf(),
                    ])),
                ]);
            }

            ParserDefinition::Sequence(elements) => {
                return EbnfNode::sequence(
                    elements
                        .iter()
                        .map(|element| generate_nested(self, &element.definition))
                        .collect(),
                );
            }

            ParserDefinition::TerminatedBy {
                expression,
                terminator,
            } => {
                return EbnfNode::sequence(vec![
                    expression.definition.generate_ebnf(),
                    EbnfNode::reference(terminator.reference.to_owned()),
                ]);
            }

            ParserDefinition::ZeroOrMore(expr) => {
                return EbnfNode::zero_or_more(expr.generate_ebnf());
            }
        }
    }
}

fn generate_nested(
    parent_definition: &ParserDefinition,
    parser_definition: &ParserDefinition,
) -> EbnfNode {
    if precedence(parent_definition) < precedence(parser_definition) {
        return EbnfNode::parenthesis(parser_definition.generate_ebnf());
    } else {
        return parser_definition.generate_ebnf();
    }
}

fn precedence(parser_definition: &ParserDefinition) -> u8 {
    match parser_definition {
        ParserDefinition::OneOrMore(..)
        | ParserDefinition::Optional(..)
        | ParserDefinition::Reference(..)
        | ParserDefinition::Repeat { .. }
        | ParserDefinition::SeparatedBy { .. }
        | ParserDefinition::TerminatedBy { .. }
        | ParserDefinition::ZeroOrMore(..) => 0,
        ParserDefinition::DelimitedBy { .. } | ParserDefinition::Sequence(..) => 1,
        ParserDefinition::Choice(..) => 2,
    }
}

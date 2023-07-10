use codegen_schema::types::{ParserDefinition, ParserRef};

use crate::{nodes::EbnfNode, serialization::GenerateEbnf};

impl GenerateEbnf for ParserRef {
    fn generate_ebnf(&self) -> EbnfNode {
        return self.definition.generate_ebnf();
    }
}

impl GenerateEbnf for ParserDefinition {
    fn generate_ebnf(&self) -> EbnfNode {
        match &self {
            ParserDefinition::Choice(parsers) => {
                return EbnfNode::choice(
                    parsers
                        .iter()
                        .map(|parser| parser.generate_ebnf())
                        .collect(),
                );
            }

            ParserDefinition::DelimitedBy {
                open,
                parser,
                close,
            } => {
                return EbnfNode::sequence(vec![
                    EbnfNode::production_ref(open.reference.to_owned()),
                    parser.generate_ebnf(),
                    EbnfNode::production_ref(close.reference.to_owned()),
                ]);
            }

            ParserDefinition::OneOrMore(parser) => {
                return EbnfNode::one_or_more(parser.generate_ebnf());
            }

            ParserDefinition::Optional(parser) => {
                return EbnfNode::optional(parser.generate_ebnf());
            }

            ParserDefinition::Reference(name) => {
                return EbnfNode::production_ref(name.to_owned());
            }

            ParserDefinition::SeparatedBy { parser, separator } => {
                return EbnfNode::sequence(vec![
                    parser.generate_ebnf(),
                    EbnfNode::zero_or_more(EbnfNode::sequence(vec![
                        EbnfNode::production_ref(separator.reference.to_owned()),
                        parser.generate_ebnf(),
                    ])),
                ]);
            }

            ParserDefinition::Sequence(parsers) => {
                return EbnfNode::sequence(
                    parsers
                        .iter()
                        .map(|parser| parser.generate_ebnf())
                        .collect(),
                );
            }

            ParserDefinition::TerminatedBy { parser, terminator } => {
                return EbnfNode::sequence(vec![
                    parser.generate_ebnf(),
                    EbnfNode::production_ref(terminator.reference.to_owned()),
                ]);
            }

            ParserDefinition::ZeroOrMore(parser) => {
                return EbnfNode::zero_or_more(parser.generate_ebnf());
            }
        }
    }
}

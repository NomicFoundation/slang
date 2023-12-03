use codegen_schema::types::{ParserDefinition, ParserRef};

use crate::nodes::EbnfNode;

impl EbnfNode {
    pub fn from_parser(parser: &ParserRef) -> Self {
        match &parser.definition {
            ParserDefinition::Choice(parsers) => {
                return Self::choice(parsers.iter().map(Self::from_parser).collect());
            }

            ParserDefinition::DelimitedBy {
                open,
                parser,
                close,
            } => Self::sequence(vec![
                Self::production_ref(&open.reference),
                Self::from_parser(parser),
                Self::production_ref(&close.reference),
            ]),

            ParserDefinition::OneOrMore(parser) => Self::one_or_more(Self::from_parser(parser)),

            ParserDefinition::Optional(parser) => Self::optional(Self::from_parser(parser)),

            ParserDefinition::Reference(name) => Self::production_ref(name),

            ParserDefinition::SeparatedBy { parser, separator } => Self::sequence(vec![
                Self::from_parser(parser),
                Self::zero_or_more(Self::sequence(vec![
                    Self::production_ref(&separator.reference),
                    Self::from_parser(parser),
                ])),
            ]),

            ParserDefinition::Sequence(parsers) => {
                return Self::sequence(parsers.iter().map(Self::from_parser).collect());
            }

            ParserDefinition::TerminatedBy { parser, terminator } => Self::sequence(vec![
                Self::from_parser(parser),
                Self::production_ref(&terminator.reference),
            ]),

            ParserDefinition::ZeroOrMore(parser) => Self::zero_or_more(Self::from_parser(parser)),
        }
    }
}

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
            } => {
                return Self::sequence(vec![
                    Self::production_ref(&open.reference),
                    Self::from_parser(parser),
                    Self::production_ref(&close.reference),
                ]);
            }

            ParserDefinition::OneOrMore(parser) => {
                return Self::one_or_more(Self::from_parser(parser));
            }

            ParserDefinition::Optional(parser) => {
                return Self::optional(Self::from_parser(parser));
            }

            ParserDefinition::Reference(name) => {
                return Self::production_ref(name);
            }

            ParserDefinition::SeparatedBy { parser, separator } => {
                return Self::sequence(vec![
                    Self::from_parser(parser),
                    Self::zero_or_more(Self::sequence(vec![
                        Self::production_ref(&separator.reference),
                        Self::from_parser(parser),
                    ])),
                ]);
            }

            ParserDefinition::Sequence(parsers) => {
                return Self::sequence(parsers.iter().map(Self::from_parser).collect());
            }

            ParserDefinition::TerminatedBy { parser, terminator } => {
                return Self::sequence(vec![
                    Self::from_parser(parser),
                    Self::production_ref(&terminator.reference),
                ]);
            }

            ParserDefinition::ZeroOrMore(parser) => {
                return Self::zero_or_more(Self::from_parser(parser));
            }
        };
    }
}

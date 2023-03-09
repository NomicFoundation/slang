use std::rc::Rc;

use crate::{types, yaml::cst};

use super::{
    node::Node,
    production::{ConcreteAbstractPair, Reference},
};

pub type ParserRef = Rc<Parser>;

pub struct Parser {
    pub name: Option<Node<String>>,
    pub definition: Node<ParserDefinition>,
}

impl ConcreteAbstractPair for Parser {
    type AbstractType = types::parser::Parser;

    fn new(cst_node: &cst::NodeRef, value: Rc<Self::AbstractType>) -> Rc<Self> {
        return Rc::new(Self {
            name: value.name.clone().and_then(|name| {
                return Some(Node::new(cst_node.value_of_field("name"), name));
            }),
            definition: ParserDefinition::new(cst_node, value.definition.clone()),
        });
    }
}

pub enum ParserDefinition {
    Choice(Vec<ParserRef>),
    DelimitedBy {
        open: Node<Reference>,
        expression: ParserRef,
        close: Node<Reference>,
    },
    OneOrMore(ParserRef),
    Optional(ParserRef),
    Reference(Node<String>),
    Repeat {
        expression: ParserRef,
        min: Node<usize>,
        max: Node<usize>,
    },
    SeparatedBy {
        expression: ParserRef,
        separator: Node<Reference>,
    },
    Sequence(Vec<ParserRef>),
    TerminatedBy {
        expression: ParserRef,
        terminator: Node<Reference>,
    },
    ZeroOrMore(ParserRef),
}

impl ParserDefinition {
    pub fn new(cst_node: &cst::NodeRef, value: types::parser::ParserDefinition) -> Node<Self> {
        match value {
            types::parser::ParserDefinition::Choice(value) => {
                let cst_node = cst_node.field("choice");
                return Node::new(
                    &cst_node.key,
                    Self::Choice(cst_node.zip(value, Parser::new)),
                );
            }
            types::parser::ParserDefinition::DelimitedBy {
                open,
                expression,
                close,
            } => {
                let cst_node = cst_node.field("delimitedBy");
                return Node::new(
                    &cst_node.key,
                    Self::DelimitedBy {
                        open: Reference::new(&cst_node.value_of_field("open"), open),
                        expression: Parser::new(&cst_node.value_of_field("expression"), expression),
                        close: Reference::new(&cst_node.value_of_field("close"), close),
                    },
                );
            }
            types::parser::ParserDefinition::OneOrMore(value) => {
                let cst_node = cst_node.field("oneOrMore");
                return Node::new(
                    &cst_node.key,
                    Self::OneOrMore(Parser::new(&cst_node.value, value)),
                );
            }
            types::parser::ParserDefinition::Optional(value) => {
                let cst_node = cst_node.field("optional");
                return Node::new(
                    &cst_node.key,
                    Self::Optional(Parser::new(&cst_node.value, value)),
                );
            }
            types::parser::ParserDefinition::Reference(value) => {
                let cst_node = cst_node.field("reference");
                return Node::new(
                    &cst_node.key,
                    Self::Reference(Node::new(&cst_node.value, value)),
                );
            }
            types::parser::ParserDefinition::Repeat {
                min,
                max,
                expression,
            } => {
                let cst_node = cst_node.field("repeat");
                return Node::new(
                    &cst_node.key,
                    Self::Repeat {
                        expression: Parser::new(&cst_node.value_of_field("expression"), expression),
                        min: Node::new(&cst_node.value_of_field("min"), min),
                        max: Node::new(&cst_node.value_of_field("max"), max),
                    },
                );
            }
            types::parser::ParserDefinition::SeparatedBy {
                expression,
                separator,
            } => {
                let cst_node = cst_node.field("separatedBy");
                return Node::new(
                    &cst_node.key,
                    Self::SeparatedBy {
                        expression: Parser::new(&cst_node.value_of_field("expression"), expression),
                        separator: Reference::new(&cst_node.value_of_field("separator"), separator),
                    },
                );
            }
            types::parser::ParserDefinition::Sequence(value) => {
                let cst_node = cst_node.field("sequence");
                return Node::new(
                    &cst_node.key,
                    Self::Sequence(cst_node.zip(value, Parser::new)),
                );
            }
            types::parser::ParserDefinition::TerminatedBy {
                expression,
                terminator,
            } => {
                let cst_node = cst_node.field("terminatedBy");
                return Node::new(
                    &cst_node.key,
                    Self::TerminatedBy {
                        expression: Parser::new(&cst_node.value_of_field("expression"), expression),
                        terminator: Reference::new(
                            &cst_node.value_of_field("terminator"),
                            terminator,
                        ),
                    },
                );
            }
            types::parser::ParserDefinition::ZeroOrMore(value) => {
                let cst_node = cst_node.field("zeroOrMore");
                return Node::new(
                    &cst_node.key,
                    Self::ZeroOrMore(Parser::new(&cst_node.value, value)),
                );
            }
        };
    }
}

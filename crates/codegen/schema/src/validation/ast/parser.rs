use std::rc::Rc;

use crate::{types, yaml::cst};

use super::{node::Node, production::ConcreteAbstractPair};

pub type ParserRef = Rc<Parser>;

pub struct Parser {
    pub name: Option<Node<String>>,
    pub lookahead: Option<ParserRef>,
    pub definition: Node<ParserDefinition>,
}

impl ConcreteAbstractPair for Parser {
    type AbstractType = types::parser::Parser;

    fn new(cst_node: &cst::NodeRef, value: Rc<Self::AbstractType>) -> Rc<Self> {
        return Rc::new(Self {
            name: value.name.clone().and_then(|name| {
                return Some(Node::new(cst_node.get("name"), name));
            }),
            lookahead: value.lookahead.clone().and_then(|lookahead| {
                return Some(Parser::new(cst_node.get("lookahead"), lookahead));
            }),
            definition: ParserDefinition::new(cst_node, value.definition.clone()),
        });
    }
}

pub enum ParserDefinition {
    Choice(Vec<ParserRef>),
    DelimitedBy {
        open: Node<String>,
        expression: ParserRef,
        close: Node<String>,
    },
    Difference {
        minuend: ParserRef,
        subtrahend: ParserRef,
    },
    OneOrMore(ParserRef),
    Optional(ParserRef),
    Reference(Node<String>),
    Repeat {
        min: Node<usize>,
        max: Node<usize>,
        expression: ParserRef,
    },
    SeparatedBy {
        separator: Node<String>,
        expression: ParserRef,
    },
    Sequence(Vec<ParserRef>),
    Terminal(Node<String>),
    ZeroOrMore(ParserRef),
}

impl ParserDefinition {
    pub fn new(cst_node: &cst::NodeRef, value: types::parser::ParserDefinition) -> Node<Self> {
        match value {
            types::parser::ParserDefinition::Choice(value) => {
                let cst_node = cst_node.field("choice");
                return Node::new(
                    &cst_node.key,
                    Self::Choice(cst_node.value.zip(value, Parser::new)),
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
                        open: Node::new(&cst_node.value.get("open"), open),
                        expression: Parser::new(&cst_node.value.get("expression"), expression),
                        close: Node::new(&cst_node.value.get("close"), close),
                    },
                );
            }
            types::parser::ParserDefinition::Difference {
                minuend,
                subtrahend,
            } => {
                let cst_node = cst_node.field("difference");
                return Node::new(
                    &cst_node.key,
                    Self::Difference {
                        minuend: Parser::new(&cst_node.value.get("minuend"), minuend),
                        subtrahend: Parser::new(&cst_node.value.get("subtrahend"), subtrahend),
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
                        min: Node::new(&cst_node.value.get("min"), min),
                        max: Node::new(&cst_node.value.get("max"), max),
                        expression: Parser::new(&cst_node.value.get("expression"), expression),
                    },
                );
            }
            types::parser::ParserDefinition::SeparatedBy {
                separator,
                expression,
            } => {
                let cst_node = cst_node.field("separatedBy");
                return Node::new(
                    &cst_node.key,
                    Self::SeparatedBy {
                        separator: Node::new(&cst_node.value.get("separator"), separator),
                        expression: Parser::new(&cst_node.value.get("expression"), expression),
                    },
                );
            }
            types::parser::ParserDefinition::Sequence(value) => {
                let cst_node = cst_node.field("sequence");
                return Node::new(
                    &cst_node.key,
                    Self::Sequence(cst_node.value.zip(value, Parser::new)),
                );
            }
            types::parser::ParserDefinition::Terminal(value) => {
                let cst_node = cst_node.field("terminal");
                return Node::new(
                    &cst_node.key,
                    Self::Terminal(Node::new(&cst_node.value, value)),
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

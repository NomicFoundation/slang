use std::rc::Rc;

use crate::{types, yaml::cst};

use super::{node::Node, production::ConcreteAbstractPair};

pub type ScannerRef = Rc<Scanner>;

pub struct Scanner {
    pub lookahead: Option<ScannerRef>,
    pub definition: Node<ScannerDefinition>,
}

impl ConcreteAbstractPair for Scanner {
    type AbstractType = types::scanner::Scanner;

    fn new(cst_node: &cst::NodeRef, value: Rc<Self::AbstractType>) -> Rc<Self> {
        return Rc::new(Self {
            lookahead: value.lookahead.clone().and_then(|lookahead| {
                return Some(Scanner::new(cst_node.get("lookahead"), lookahead));
            }),
            definition: ScannerDefinition::new(cst_node, value.definition.clone()),
        });
    }
}

pub enum ScannerDefinition {
    Choice(Vec<ScannerRef>),
    DelimitedBy {
        open: Node<String>,
        expression: ScannerRef,
        close: Node<String>,
    },
    Difference {
        minuend: ScannerRef,
        subtrahend: ScannerRef,
    },
    Not(ScannerRef),
    OneOrMore(ScannerRef),
    Optional(ScannerRef),
    Range {
        from: Node<char>,
        to: Node<char>,
    },
    Reference(Node<String>),
    Repeat {
        min: Node<usize>,
        max: Node<usize>,
        expression: ScannerRef,
    },
    SeparatedBy {
        separator: Node<String>,
        expression: ScannerRef,
    },
    Sequence(Vec<ScannerRef>),
    Terminal(Node<String>),
    ZeroOrMore(ScannerRef),
}

impl ScannerDefinition {
    pub fn new(cst_node: &cst::NodeRef, value: types::scanner::ScannerDefinition) -> Node<Self> {
        match value {
            types::scanner::ScannerDefinition::Choice(value) => {
                let cst_node = cst_node.field("choice");
                return Node::new(
                    &cst_node.key,
                    Self::Choice(cst_node.value.zip(value, Scanner::new)),
                );
            }
            types::scanner::ScannerDefinition::DelimitedBy {
                open,
                expression,
                close,
            } => {
                let cst_node = cst_node.field("delimitedBy");
                return Node::new(
                    &cst_node.key,
                    Self::DelimitedBy {
                        open: Node::new(&cst_node.value.get("open"), open),
                        expression: Scanner::new(&cst_node.value.get("expression"), expression),
                        close: Node::new(&cst_node.value.get("close"), close),
                    },
                );
            }
            types::scanner::ScannerDefinition::Difference {
                minuend,
                subtrahend,
            } => {
                let cst_node = cst_node.field("difference");
                return Node::new(
                    &cst_node.key,
                    Self::Difference {
                        minuend: Scanner::new(&cst_node.value.get("minuend"), minuend),
                        subtrahend: Scanner::new(&cst_node.value.get("subtrahend"), subtrahend),
                    },
                );
            }
            types::scanner::ScannerDefinition::Not(value) => {
                let cst_node = cst_node.field("not");
                return Node::new(
                    &cst_node.key,
                    Self::Not(Scanner::new(&cst_node.value, value)),
                );
            }
            types::scanner::ScannerDefinition::OneOrMore(value) => {
                let cst_node = cst_node.field("oneOrMore");
                return Node::new(
                    &cst_node.key,
                    Self::OneOrMore(Scanner::new(&cst_node.value, value)),
                );
            }
            types::scanner::ScannerDefinition::Optional(value) => {
                let cst_node = cst_node.field("optional");
                return Node::new(
                    &cst_node.key,
                    Self::Optional(Scanner::new(&cst_node.value, value)),
                );
            }
            types::scanner::ScannerDefinition::Range { from, to } => {
                let cst_node = cst_node.field("range");
                return Node::new(
                    &cst_node.key,
                    Self::Range {
                        from: Node::new(&cst_node.value.get("from"), from),
                        to: Node::new(&cst_node.value.get("to"), to),
                    },
                );
            }
            types::scanner::ScannerDefinition::Reference(value) => {
                let cst_node = cst_node.field("reference");
                return Node::new(
                    &cst_node.key,
                    Self::Reference(Node::new(&cst_node.value, value)),
                );
            }
            types::scanner::ScannerDefinition::Repeat {
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
                        expression: Scanner::new(&cst_node.value.get("expression"), expression),
                    },
                );
            }
            types::scanner::ScannerDefinition::SeparatedBy {
                separator,
                expression,
            } => {
                let cst_node = cst_node.field("separatedBy");
                return Node::new(
                    &cst_node.key,
                    Self::SeparatedBy {
                        separator: Node::new(&cst_node.value.get("separator"), separator),
                        expression: Scanner::new(&cst_node.value.get("expression"), expression),
                    },
                );
            }
            types::scanner::ScannerDefinition::Sequence(value) => {
                let cst_node = cst_node.field("sequence");
                return Node::new(
                    &cst_node.key,
                    Self::Sequence(cst_node.value.zip(value, Scanner::new)),
                );
            }
            types::scanner::ScannerDefinition::Terminal(value) => {
                let cst_node = cst_node.field("terminal");
                return Node::new(
                    &cst_node.key,
                    Self::Terminal(Node::new(&cst_node.value, value)),
                );
            }
            types::scanner::ScannerDefinition::ZeroOrMore(value) => {
                let cst_node = cst_node.field("zeroOrMore");
                return Node::new(
                    &cst_node.key,
                    Self::ZeroOrMore(Scanner::new(&cst_node.value, value)),
                );
            }
        };
    }
}

use std::fmt::Debug;
use std::rc::Rc;

use crate::{GrammarVisitor, VersionQualityRange, Visitable};

pub trait ScannerDefinition: Debug {
    fn name(&self) -> &'static str;
    fn node(&self) -> &ScannerDefinitionNode;
}

pub type ScannerDefinitionRef = Rc<dyn ScannerDefinition>;

impl Visitable for ScannerDefinitionRef {
    fn accept_visitor<V: GrammarVisitor>(&self, visitor: &mut V) {
        visitor.scanner_definition_enter(self);
        self.node().accept_visitor(visitor);
    }
}

#[derive(Clone, Debug)]
pub enum ScannerDefinitionNode {
    Versioned(Box<Self>, Vec<VersionQualityRange>),
    Optional(Box<Self>),
    ZeroOrMore(Box<Self>),
    OneOrMore(Box<Self>),
    Sequence(Vec<Self>),
    Choice(Vec<Self>),
    NoneOf(String),
    NotFollowedBy(Box<Self>, Box<Self>),
    CharRange(char, char),
    Literal(String),
    ScannerDefinition(ScannerDefinitionRef),
}

impl From<ScannerDefinitionRef> for ScannerDefinitionNode {
    fn from(def_ref: ScannerDefinitionRef) -> Self {
        ScannerDefinitionNode::ScannerDefinition(def_ref)
    }
}

impl Visitable for ScannerDefinitionNode {
    fn accept_visitor<V: GrammarVisitor>(&self, visitor: &mut V) {
        visitor.scanner_definition_node_enter(self);
        match self {
            Self::Versioned(node, _)
            | Self::Optional(node)
            | Self::ZeroOrMore(node)
            | Self::OneOrMore(node) => node.accept_visitor(visitor),

            Self::Sequence(nodes) | Self::Choice(nodes) => {
                for node in nodes {
                    node.accept_visitor(visitor);
                }
            }

            Self::NotFollowedBy(node, lookahead) => {
                node.accept_visitor(visitor);
                lookahead.accept_visitor(visitor);
            }

            Self::NoneOf(_)
            | Self::CharRange(_, _)
            | Self::Literal(_)
            | Self::ScannerDefinition(_) => {}
        }
    }
}

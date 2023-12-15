use std::fmt::Debug;
use std::rc::Rc;

use crate::visitor::{GrammarVisitor, Visitable};
use crate::{PrecedenceParserDefinitionRef, ScannerDefinitionRef, VersionQualityRange};

/// A named wrapper, used to give a name to a [`ParserDefinitionNode`].
#[derive(Clone, Debug)]
pub struct Named<T> {
    pub name: String,
    pub node: T,
}

pub trait ParserDefinition: Debug {
    fn name(&self) -> &'static str;
    fn node(&self) -> &ParserDefinitionNode;
    fn context(&self) -> &'static str;
    fn is_inline(&self) -> bool;
}

pub type ParserDefinitionRef = Rc<dyn ParserDefinition>;

impl Visitable for ParserDefinitionRef {
    fn accept_visitor<V: GrammarVisitor>(&self, visitor: &mut V) {
        visitor.parser_definition_enter(self);
        self.node().accept_visitor(visitor);
    }
}

pub trait TriviaParserDefinition: Debug {
    fn name(&self) -> &'static str;
    fn node(&self) -> &ParserDefinitionNode;
    fn context(&self) -> &'static str;
}

pub type TriviaParserDefinitionRef = Rc<dyn TriviaParserDefinition>;

impl Visitable for TriviaParserDefinitionRef {
    fn accept_visitor<V: GrammarVisitor>(&self, visitor: &mut V) {
        visitor.trivia_parser_definition_enter(self);
        self.node().accept_visitor(visitor);
    }
}

#[derive(Clone, Debug)]
pub enum ParserDefinitionNode {
    Versioned(Box<Self>, Vec<VersionQualityRange>),
    Optional(Box<Self>),
    ZeroOrMore(Named<Box<Self>>),
    OneOrMore(Named<Box<Self>>),
    Sequence(Vec<Named<Self>>),
    Choice(Named<Vec<Self>>),
    ScannerDefinition(ScannerDefinitionRef),
    TriviaParserDefinition(TriviaParserDefinitionRef),
    ParserDefinition(ParserDefinitionRef),
    PrecedenceParserDefinition(PrecedenceParserDefinitionRef),
    DelimitedBy(Named<Box<Self>>, Box<Self>, Named<Box<Self>>),
    SeparatedBy(Named<Box<Self>>, Named<Box<Self>>),
    TerminatedBy(Box<Self>, Named<Box<Self>>),
}

impl From<ScannerDefinitionRef> for ParserDefinitionNode {
    fn from(def: ScannerDefinitionRef) -> Self {
        ParserDefinitionNode::ScannerDefinition(def)
    }
}

impl From<TriviaParserDefinitionRef> for ParserDefinitionNode {
    fn from(def: TriviaParserDefinitionRef) -> Self {
        ParserDefinitionNode::TriviaParserDefinition(def)
    }
}

impl From<ParserDefinitionRef> for ParserDefinitionNode {
    fn from(def: ParserDefinitionRef) -> Self {
        ParserDefinitionNode::ParserDefinition(def)
    }
}

impl From<PrecedenceParserDefinitionRef> for ParserDefinitionNode {
    fn from(def: PrecedenceParserDefinitionRef) -> Self {
        ParserDefinitionNode::PrecedenceParserDefinition(def)
    }
}

impl Visitable for ParserDefinitionNode {
    fn accept_visitor<V: GrammarVisitor>(&self, visitor: &mut V) {
        visitor.parser_definition_node_enter(self);
        match self {
            Self::Versioned(node, _)
            | Self::Optional(node)
            | Self::ZeroOrMore(Named { node, .. })
            | Self::OneOrMore(Named { node, .. }) => node.accept_visitor(visitor),

            Self::Sequence(nodes) => {
                for Named { node, .. } in nodes {
                    node.accept_visitor(visitor);
                }
            }
            Self::Choice(Named { node: nodes, .. }) => {
                for node in nodes {
                    node.accept_visitor(visitor);
                }
            }

            Self::DelimitedBy(open, body, close) => {
                open.node.accept_visitor(visitor);
                body.accept_visitor(visitor);
                close.node.accept_visitor(visitor);
            }

            Self::SeparatedBy(body, separator) => {
                body.node.accept_visitor(visitor);
                separator.node.accept_visitor(visitor);
            }

            Self::TerminatedBy(body, terminator) => {
                body.accept_visitor(visitor);
                terminator.node.accept_visitor(visitor);
            }

            Self::ScannerDefinition(_)
            | Self::TriviaParserDefinition(_)
            | Self::ParserDefinition(_)
            | Self::PrecedenceParserDefinition(_) => {}
        }
    }
}

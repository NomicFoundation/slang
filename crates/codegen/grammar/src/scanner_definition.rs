use std::rc::Rc;

use super::*;

pub trait ScannerDefinition
where
    Self: std::fmt::Debug,
{
    fn name(&self) -> &'static str;
    fn source_location(&self) -> SourceLocation;
    fn node(&self) -> &ScannerDefinitionNode;
}

pub type ScannerDefinitionRef = Rc<dyn ScannerDefinition>;

impl Visitable for ScannerDefinitionRef {
    fn accept_visitor<V: GrammarVisitor>(&self, visitor: &mut V) {
        visitor.scanner_definition_enter(self);
        self.node().accept_visitor(visitor);
        visitor.scanner_definition_leave(self);
    }
}

#[derive(Clone, Debug)]
pub enum ScannerDefinitionNode {
    Versioned(Box<Self>, Vec<VersionQualityRange>, SourceLocation),
    Optional(Box<Self>, SourceLocation),
    ZeroOrMore(Box<Self>, SourceLocation),
    OneOrMore(Box<Self>, SourceLocation),
    Sequence(Vec<Self>, SourceLocation),
    Choice(Vec<Self>, SourceLocation),
    NoneOf(String, SourceLocation),
    NotFollowedBy(Box<Self>, Box<Self>, SourceLocation),
    CharRange(char, char, SourceLocation),
    Literal(String, SourceLocation),
    ContextualKeyword(&'static str, Box<Self>, SourceLocation),
    ScannerDefinition(ScannerDefinitionRef, SourceLocation),
}

impl Into<ScannerDefinitionNode> for (ScannerDefinitionRef, SourceLocation) {
    fn into(self) -> ScannerDefinitionNode {
        ScannerDefinitionNode::ScannerDefinition(self.0, self.1)
    }
}

impl Visitable for ScannerDefinitionNode {
    fn accept_visitor<V: GrammarVisitor>(&self, visitor: &mut V) {
        visitor.scanner_definition_node_enter(self);
        match self {
            Self::Versioned(node, _, _)
            | Self::Optional(node, _)
            | Self::ZeroOrMore(node, _)
            | Self::OneOrMore(node, _) => node.accept_visitor(visitor),

            Self::Sequence(nodes, _) | Self::Choice(nodes, _) => {
                for node in nodes {
                    node.accept_visitor(visitor);
                }
            }

            Self::NotFollowedBy(node, lookahead, _) => {
                node.accept_visitor(visitor);
                lookahead.accept_visitor(visitor);
            }

            Self::NoneOf(_, _)
            | Self::CharRange(_, _, _)
            | Self::Literal(_, _)
            | Self::ContextualKeyword(_, _, _)
            | Self::ScannerDefinition(_, _) => {}
        }
        visitor.scanner_definition_node_leave(self);
    }
}

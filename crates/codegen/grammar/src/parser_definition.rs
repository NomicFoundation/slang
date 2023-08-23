use std::rc::Rc;

use super::*;

#[allow(dead_code)]
pub trait ParserDefinition
where
    Self: std::fmt::Debug,
{
    fn name(&self) -> &'static str;
    fn source_location(&self) -> SourceLocation;
    fn node(&self) -> &ParserDefinitionNode;
    fn context(&self) -> &'static str;
    fn is_inline(&self) -> bool;
}

pub type ParserDefinitionRef = Rc<dyn ParserDefinition>;

impl Visitable for ParserDefinitionRef {
    fn accept_visitor<V: GrammarVisitor>(&self, visitor: &mut V) {
        visitor.parser_definition_enter(self);
        self.node().accept_visitor(visitor);
        visitor.parser_definition_leave(self);
    }
}

#[allow(dead_code)]
pub trait TriviaParserDefinition
where
    Self: std::fmt::Debug,
{
    fn name(&self) -> &'static str;
    fn source_location(&self) -> SourceLocation;
    fn node(&self) -> &ParserDefinitionNode;
    fn context(&self) -> &'static str;
}

pub type TriviaParserDefinitionRef = Rc<dyn TriviaParserDefinition>;

impl Visitable for TriviaParserDefinitionRef {
    fn accept_visitor<V: GrammarVisitor>(&self, visitor: &mut V) {
        visitor.trivia_parser_definition_enter(self);
        self.node().accept_visitor(visitor);
        visitor.trivia_parser_definition_leave(self);
    }
}

#[derive(Clone, Debug)]
pub enum ParserDefinitionNode {
    Versioned(Box<Self>, Vec<VersionQualityRange>, SourceLocation),
    Optional(Box<Self>, SourceLocation),
    ZeroOrMore(Box<Self>, SourceLocation),
    OneOrMore(Box<Self>, SourceLocation),
    Sequence(Vec<Self>, SourceLocation),
    Choice(Vec<Self>, SourceLocation),
    ScannerDefinition(ScannerDefinitionRef, SourceLocation),
    TriviaParserDefinition(TriviaParserDefinitionRef, SourceLocation),
    ParserDefinition(ParserDefinitionRef, SourceLocation),
    PrecedenceParserDefinition(PrecedenceParserDefinitionRef, SourceLocation),
    DelimitedBy(Box<Self>, Box<Self>, Box<Self>, SourceLocation),
    SeparatedBy(Box<Self>, Box<Self>, SourceLocation),
    TerminatedBy(Box<Self>, Box<Self>, SourceLocation),
}

impl Into<ParserDefinitionNode> for (ScannerDefinitionRef, SourceLocation) {
    fn into(self) -> ParserDefinitionNode {
        ParserDefinitionNode::ScannerDefinition(self.0, self.1)
    }
}

impl Into<ParserDefinitionNode> for (TriviaParserDefinitionRef, SourceLocation) {
    fn into(self) -> ParserDefinitionNode {
        ParserDefinitionNode::TriviaParserDefinition(self.0, self.1)
    }
}

impl Into<ParserDefinitionNode> for (ParserDefinitionRef, SourceLocation) {
    fn into(self) -> ParserDefinitionNode {
        ParserDefinitionNode::ParserDefinition(self.0, self.1)
    }
}

impl Into<ParserDefinitionNode> for (PrecedenceParserDefinitionRef, SourceLocation) {
    fn into(self) -> ParserDefinitionNode {
        ParserDefinitionNode::PrecedenceParserDefinition(self.0, self.1)
    }
}

impl Visitable for ParserDefinitionNode {
    fn accept_visitor<V: GrammarVisitor>(&self, visitor: &mut V) {
        visitor.parser_definition_node_enter(self);
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

            Self::DelimitedBy(open, body, close, _) => {
                open.accept_visitor(visitor);
                body.accept_visitor(visitor);
                close.accept_visitor(visitor);
            }

            Self::SeparatedBy(body, separator, _) => {
                body.accept_visitor(visitor);
                separator.accept_visitor(visitor);
            }

            Self::TerminatedBy(body, terminator, _) => {
                body.accept_visitor(visitor);
                terminator.accept_visitor(visitor);
            }

            Self::ScannerDefinition(_, _)
            | Self::TriviaParserDefinition(_, _)
            | Self::ParserDefinition(_, _)
            | Self::PrecedenceParserDefinition(_, _) => {}
        }
        visitor.parser_definition_node_leave(self);
    }
}

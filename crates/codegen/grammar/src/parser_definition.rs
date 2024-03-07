use std::fmt::Debug;
use std::rc::Rc;

use codegen_language_definition::model;

use crate::visitor::{GrammarVisitor, Visitable};
use crate::{
    KeywordScannerDefinitionRef, PrecedenceParserDefinitionRef, ScannerDefinitionRef,
    VersionQualityRange,
};

/// A named wrapper, used to give a name to a [`ParserDefinitionNode`].
#[derive(Clone, Debug)]
pub struct Labeled<T> {
    pub label: String,
    pub value: T,
}

impl<T> std::ops::Deref for Labeled<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
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

/// How many tokens have to be matched to trigger the error recovery.
/// For ambiguous syntaxes this needs to be set to at least N, where N
/// is the token lookahead required to disambiguate the syntax.
///
// By default, we assume no lookahead (0) is required to recover from
// unrecognized body between delimiters, so it's always triggered.
#[derive(Clone, Debug, Default)]
pub struct DelimitedRecoveryTokenThreshold(pub u8);

impl From<model::FieldDelimiters> for DelimitedRecoveryTokenThreshold {
    fn from(delimiters: model::FieldDelimiters) -> Self {
        Self(
            delimiters
                .tokens_matched_acceptance_threshold
                .unwrap_or(DelimitedRecoveryTokenThreshold::default().0),
        )
    }
}

#[derive(Clone, Debug)]
pub enum ParserDefinitionNode {
    Versioned(Box<Self>, Vec<VersionQualityRange>),
    Optional(Box<Self>),
    ZeroOrMore(Labeled<Box<Self>>),
    OneOrMore(Labeled<Box<Self>>),
    Sequence(Vec<Labeled<Self>>),
    Choice(Labeled<Vec<Self>>),
    ScannerDefinition(ScannerDefinitionRef),
    KeywordScannerDefinition(KeywordScannerDefinitionRef),
    TriviaParserDefinition(TriviaParserDefinitionRef),
    ParserDefinition(ParserDefinitionRef),
    PrecedenceParserDefinition(PrecedenceParserDefinitionRef),
    DelimitedBy(
        Labeled<Box<Self>>,
        Box<Self>,
        Labeled<Box<Self>>,
        DelimitedRecoveryTokenThreshold,
    ),
    SeparatedBy(Labeled<Box<Self>>, Labeled<Box<Self>>),
    TerminatedBy(Box<Self>, Labeled<Box<Self>>),
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
            Self::Versioned(value, _)
            | Self::Optional(value)
            | Self::ZeroOrMore(Labeled { value, .. })
            | Self::OneOrMore(Labeled { value, .. }) => value.accept_visitor(visitor),

            Self::Sequence(nodes) => {
                for node in nodes {
                    node.accept_visitor(visitor);
                }
            }
            Self::Choice(Labeled { value: nodes, .. }) => {
                for node in nodes {
                    node.accept_visitor(visitor);
                }
            }

            Self::DelimitedBy(open, body, close, ..) => {
                open.accept_visitor(visitor);
                body.accept_visitor(visitor);
                close.accept_visitor(visitor);
            }

            Self::SeparatedBy(body, separator) => {
                body.accept_visitor(visitor);
                separator.accept_visitor(visitor);
            }

            Self::TerminatedBy(body, terminator) => {
                body.accept_visitor(visitor);
                terminator.accept_visitor(visitor);
            }

            Self::ScannerDefinition(def) => def.accept_visitor(visitor),
            Self::KeywordScannerDefinition(_)
            | Self::TriviaParserDefinition(_)
            | Self::ParserDefinition(_)
            | Self::PrecedenceParserDefinition(_) => {}
        }
    }
}

use std::fmt::Debug;
use std::rc::Rc;

use crate::parser::grammar::{GrammarVisitor, VersionQualityRange, Visitable};

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

pub trait KeywordScannerDefinition: Debug {
    fn name(&self) -> &'static str;
    fn identifier_scanner(&self) -> &'static str;
    fn definitions(&self) -> &[KeywordScannerDefinitionVersionedNode];
}

pub type KeywordScannerDefinitionRef = Rc<dyn KeywordScannerDefinition>;

impl Visitable for KeywordScannerDefinitionRef {
    fn accept_visitor<V: GrammarVisitor>(&self, visitor: &mut V) {
        visitor.keyword_scanner_definition_enter(self);
    }
}

#[derive(Debug)]
pub struct KeywordScannerDefinitionVersionedNode {
    // Underlying keyword scanner (i.e. identifier scanner)
    pub value: KeywordScannerDefinitionNode,
    /// When the keyword scanner is enabled
    pub enabled: Vec<VersionQualityRange>,
    /// When the keyword is reserved, i.e. can't be used in other position (e.g. as a name)
    pub reserved: Vec<VersionQualityRange>,
}

#[derive(Clone, Debug)]
pub enum KeywordScannerDefinitionNode {
    Optional(Box<Self>),
    Sequence(Vec<Self>),
    Choice(Vec<Self>),
    Atom(String),
    // No repeatable combinators, because keywords are assumed to be finite
}

impl From<KeywordScannerDefinitionNode> for ScannerDefinitionNode {
    fn from(val: KeywordScannerDefinitionNode) -> Self {
        match val {
            KeywordScannerDefinitionNode::Optional(node) => {
                ScannerDefinitionNode::Optional(Box::new((*node).into()))
            }
            KeywordScannerDefinitionNode::Sequence(nodes) => {
                ScannerDefinitionNode::Sequence(nodes.into_iter().map(Into::into).collect())
            }
            KeywordScannerDefinitionNode::Atom(string) => ScannerDefinitionNode::Literal(string),
            KeywordScannerDefinitionNode::Choice(nodes) => {
                ScannerDefinitionNode::Choice(nodes.into_iter().map(Into::into).collect())
            }
        }
    }
}

/// A [`KeywordScannerDefinitionRef`] that only has a single atom value.
///
/// The main usage for this type is to construct a keyword trie in parser generator, as trie will
/// only work with single atom values and keyword promotion needs to additionally account for
/// keyword reservation, rather than just literal presence.
#[derive(Clone)]
pub struct KeywordScannerAtomic(KeywordScannerDefinitionRef);

impl KeywordScannerAtomic {
    /// Wraps the keyword scanner definition if it is a single atom value.
    pub fn try_from_def(def: &KeywordScannerDefinitionRef) -> Option<Self> {
        match def.definitions() {
            [KeywordScannerDefinitionVersionedNode {
                value: KeywordScannerDefinitionNode::Atom(_),
                ..
            }] => Some(Self(Rc::clone(def))),
            _ => None,
        }
    }
}

impl std::ops::Deref for KeywordScannerAtomic {
    type Target = KeywordScannerDefinitionRef;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl KeywordScannerAtomic {
    pub fn definition(&self) -> &KeywordScannerDefinitionVersionedNode {
        let def = &self.0.definitions().first();
        def.expect("KeywordScannerAtomic should have exactly one definition")
    }
    pub fn value(&self) -> &str {
        match self.definition() {
            KeywordScannerDefinitionVersionedNode {
                value: KeywordScannerDefinitionNode::Atom(atom),
                ..
            } => atom,
            _ => unreachable!("KeywordScannerAtomic should have a single atom value"),
        }
    }
}

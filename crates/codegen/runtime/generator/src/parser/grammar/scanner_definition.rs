use std::fmt::Debug;
use std::rc::Rc;

use codegen_language_definition::model::{self, Identifier};

use crate::parser::grammar::{GrammarVisitor, Visitable};

pub trait ScannerDefinition: Debug {
    fn name(&self) -> &Identifier;
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
    Versioned(Box<Self>, model::VersionSpecifier),
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
    fn name(&self) -> &Identifier;
    fn identifier_scanner(&self) -> &Identifier;
    fn definitions(&self) -> &[model::KeywordDefinition];
}

pub type KeywordScannerDefinitionRef = Rc<dyn KeywordScannerDefinition>;

impl Visitable for KeywordScannerDefinitionRef {
    fn accept_visitor<V: GrammarVisitor>(&self, visitor: &mut V) {
        visitor.keyword_scanner_definition_enter(self);
    }
}

impl From<model::KeywordValue> for ScannerDefinitionNode {
    fn from(val: model::KeywordValue) -> Self {
        match val {
            model::KeywordValue::Optional { value } => {
                ScannerDefinitionNode::Optional(Box::new((*value).into()))
            }
            model::KeywordValue::Sequence { values } => {
                ScannerDefinitionNode::Sequence(values.into_iter().map(Into::into).collect())
            }
            model::KeywordValue::Atom { atom } => ScannerDefinitionNode::Literal(atom),
            model::KeywordValue::Choice { values } => {
                ScannerDefinitionNode::Choice(values.into_iter().map(Into::into).collect())
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
            [model::KeywordDefinition {
                value: model::KeywordValue::Atom { .. },
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
    pub fn definition(&self) -> &model::KeywordDefinition {
        self.0
            .definitions()
            .first()
            .expect("KeywordScannerAtomic should have exactly one definition")
    }

    pub fn value(&self) -> &str {
        match self.definition() {
            model::KeywordDefinition {
                value: model::KeywordValue::Atom { atom },
                ..
            } => atom,
            _ => unreachable!("KeywordScannerAtomic should have a single atom value"),
        }
    }
}

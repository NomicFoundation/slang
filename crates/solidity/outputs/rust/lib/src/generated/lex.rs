// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use super::kinds::TokenKind;
use serde::Serialize;
use std::ops::Range;
use std::rc::Rc;
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct Node {
    pub(crate) contents: NodeContents,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub(crate) enum NodeContents {
    Chars(Range<usize>),
    Sequence(Vec<Rc<Node>>),
    Named(TokenKind, Rc<Node>),
}
impl Node {
    pub(crate) fn new(contents: NodeContents) -> Self {
        Self { contents }
    }
    pub fn chars(range: Range<usize>) -> Option<Rc<Self>> {
        Some(Rc::new(Self::new(NodeContents::Chars(range))))
    }
    pub fn chars_unwrapped(range: Range<usize>) -> Rc<Self> {
        Rc::new(Self::new(NodeContents::Chars(range)))
    }
    pub fn sequence(elements: Vec<Option<Rc<Self>>>) -> Option<Rc<Self>> {
        let elements: Vec<_> = elements.into_iter().filter_map(|e| e).collect();
        if elements.is_empty() {
            None
        } else {
            Some(Rc::new(Self::new(NodeContents::Sequence(elements))))
        }
    }
    pub fn named(kind: TokenKind, element: Option<Rc<Self>>) -> Option<Rc<Self>> {
        element.map(|e| Rc::new(Self::new(NodeContents::Named(kind, e))))
    }
    pub fn range(&self) -> Range<usize> {
        match &self.contents {
            NodeContents::Chars(range) => range.clone(),
            NodeContents::Sequence(elements) => {
                elements[0].range().start..elements[elements.len() - 1].range().end
            }
            NodeContents::Named(_, element) => element.range(),
        }
    }
}

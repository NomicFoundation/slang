// This file is generated via `cargo build`. Please don't edit by hand.

use super::kinds;
use serde::Serialize;
use std::ops::Range;
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub enum Node {
    None,
    Chars(Range<usize>),
    Choice(usize, NodeRef),
    Sequence(Vec<NodeRef>),
    Named(kinds::Token, NodeRef),
}
pub type NodeRef = Box<Node>;
impl Node {
    #[inline]
    pub fn none() -> NodeRef {
        Box::new(Node::None)
    }
    #[inline]
    pub fn chars(range: Range<usize>) -> NodeRef {
        Box::new(Node::Chars(range))
    }
    #[inline]
    pub fn sequence(elements: Vec<NodeRef>) -> NodeRef {
        Box::new(if elements.is_empty() {
            Node::None
        } else {
            Node::Sequence(elements)
        })
    }
    #[inline]
    pub fn choice(number: usize, element: NodeRef) -> NodeRef {
        Box::new(Node::Choice(number, element))
    }
    #[inline]
    pub fn named(kind: kinds::Token, element: NodeRef) -> NodeRef {
        Box::new(Node::Named(kind, element))
    }
    pub fn range(&self) -> Range<usize> {
        match self {
            Node::None => 0..0,
            Node::Chars(range) => range.clone(),
            Node::Choice(_, element) => element.range(),
            Node::Sequence(elements) => {
                elements[0].range().start..elements[elements.len() - 1].range().end
            }
            Node::Named(_, element) => element.range(),
        }
    }
}

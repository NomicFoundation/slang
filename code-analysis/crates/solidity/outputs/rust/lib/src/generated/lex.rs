// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use super::kinds;
use serde::Serialize;
use std::ops::Range;
use std::rc::Rc;
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub enum Node {
    None,
    Chars(Range<usize>),
    Choice(usize, Rc<Node>),
    Sequence(Vec<Rc<Node>>),
    Named(kinds::Token, Rc<Node>),
}
impl Node {
    pub fn none() -> Rc<Self> {
        Rc::new(Self::None)
    }
    pub fn chars(range: Range<usize>) -> Rc<Self> {
        Rc::new(Self::Chars(range))
    }
    pub fn sequence(elements: Vec<Rc<Self>>) -> Rc<Self> {
        Rc::new(if elements.is_empty() {
            Self::None
        } else {
            Self::Sequence(elements)
        })
    }
    pub fn choice(number: usize, element: Rc<Self>) -> Rc<Self> {
        Rc::new(Self::Choice(number, element))
    }
    pub fn named(kind: kinds::Token, element: Rc<Self>) -> Rc<Self> {
        Rc::new(Self::Named(kind, element))
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

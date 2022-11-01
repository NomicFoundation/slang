// This file is generated via `cargo build`. Please don't edit by hand.

use super::kinds;
use super::parse::Context;
use serde::Serialize;
use std::ops::Range;
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub enum Node<'a> {
    None,
    Chars(Range<usize>),
    Choice(usize, &'a Node<'a>),
    Sequence(Vec<&'a Node<'a>>),
    Named(kinds::Token, &'a Node<'a>),
}
impl<'a> Node<'a> {
    #[inline]
    pub fn none(context: &'a Context<'a>) -> &'a Node<'a> {
        context.alloc_lex_node(Node::None)
    }
    #[inline]
    pub fn chars(context: &'a Context<'a>, range: Range<usize>) -> &'a Node<'a> {
        context.alloc_lex_node(Node::Chars(range))
    }
    #[inline]
    pub fn sequence(context: &'a Context<'a>, elements: Vec<&'a Node<'a>>) -> &'a Node<'a> {
        context.alloc_lex_node(if elements.is_empty() {
            Node::None
        } else {
            Node::Sequence(elements)
        })
    }
    #[inline]
    pub fn choice(context: &'a Context<'a>, number: usize, element: &'a Node<'a>) -> &'a Node<'a> {
        context.alloc_lex_node(Node::Choice(number, element))
    }
    #[inline]
    pub fn named(
        context: &'a Context<'a>,
        kind: kinds::Token,
        element: &'a Node<'a>,
    ) -> &'a Node<'a> {
        context.alloc_lex_node(Node::Named(kind, element))
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

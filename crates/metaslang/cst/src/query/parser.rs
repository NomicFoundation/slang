use std::rc::Rc;

use nom::branch::alt;
use nom::bytes::complete::{is_not, tag, take_while, take_while1, take_while_m_n};
use nom::character::complete::{alpha1, char, multispace0, multispace1, satisfy};
use nom::combinator::{
    all_consuming, cut, map_opt, map_res, opt, peek, recognize, success, value, verify,
};
use nom::error::{context, VerboseError};
use nom::multi::{fold_many0, many1, separated_list1};
use nom::sequence::{delimited, pair, preceded, terminated};
use nom::{Finish, IResult, Offset, Parser};
use thiserror::Error;

use super::model::{
    ASTNode, AlternativesASTNode, CaptureASTNode, NodeMatchASTNode, NodeSelector, OneOrMoreASTNode,
    OptionalASTNode, SequenceASTNode,
};
use crate::cst::NodeKind;
use crate::text_index::TextIndex;
use crate::{AbstractKind as _, KindTypes};

// ----------------------------------------------------------------------------
// Parse errors

#[derive(Clone, Debug, Error)]
pub struct QueryError {
    pub message: String,
    pub row: usize,
    pub column: usize,
}

impl std::fmt::Display for QueryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

// ----------------------------------------------------------------------------
// Parser

pub(super) fn parse_query<T: KindTypes>(input: &str) -> Result<ASTNode<T>, QueryError> {
    all_consuming(preceded(multispace0, parse_quantified_matcher::<T>))
        .parse(input)
        .finish()
        .map(|(_, query)| query)
        .map_err(|e| {
            let text_index = compute_row_and_column(e.errors[0].0, input);
            QueryError {
                message: e.to_string(),
                row: text_index.line,
                column: text_index.column,
            }
        })
}

fn compute_row_and_column(target: &str, from: &str) -> TextIndex {
    let target_offset = from.offset(target);
    let mut text_index = TextIndex::ZERO;
    let mut from_iter = from.chars();
    let Some(mut c) = from_iter.next() else {
        return text_index;
    };
    let mut next_c = from_iter.next();
    loop {
        if text_index.utf8 >= target_offset {
            break;
        }
        text_index.advance(c, next_c.as_ref());
        c = match next_c {
            Some(ch) => ch,
            None => break,
        };
        next_c = from_iter.next();
    }
    text_index
}

pub(super) fn parse_matcher_alternatives<T: KindTypes>(
    i: &str,
) -> IResult<&str, ASTNode<T>, VerboseError<&str>> {
    separated_list1(token('|'), parse_matcher_sequence::<T>)
        .map(|mut children| {
            if children.len() == 1 {
                children.pop().unwrap()
            } else {
                ASTNode::Alternatives(Rc::new(AlternativesASTNode { children }))
            }
        })
        .parse(i)
}

pub(super) fn parse_matcher_sequence<T: KindTypes>(
    i: &str,
) -> IResult<&str, ASTNode<T>, VerboseError<&str>> {
    many1(parse_quantified_matcher::<T>)
        .map(|mut children| {
            if children.len() == 1 {
                children.pop().unwrap()
            } else {
                ASTNode::Sequence(Rc::new(SequenceASTNode { children }))
            }
        })
        .parse(i)
}

pub(super) fn parse_quantified_matcher<T: KindTypes>(
    i: &str,
) -> IResult<&str, ASTNode<T>, VerboseError<&str>> {
    alt((
        ellipsis_token.map(|_| ASTNode::Ellipsis), // Cannot be quantified
        pair(
            parse_bound_matcher,
            parse_trailing_quantifier, // admits epsilon
        )
        .map(|(child, quantifier)| match quantifier {
            CaptureQuantifier::ZeroOrOne => ASTNode::Optional(Rc::new(OptionalASTNode { child })),
            CaptureQuantifier::ZeroOrMore => ASTNode::Optional(Rc::new(OptionalASTNode {
                child: ASTNode::OneOrMore(Rc::new(OneOrMoreASTNode { child })),
            })),
            CaptureQuantifier::OneOrMore => ASTNode::OneOrMore(Rc::new(OneOrMoreASTNode { child })),
            CaptureQuantifier::One => child,
        }),
    ))
    .parse(i)
}

pub(super) fn parse_bound_matcher<T: KindTypes>(
    i: &str,
) -> IResult<&str, ASTNode<T>, VerboseError<&str>> {
    pair(
        opt(capture_name_token),
        alt((
            delimited(token('('), parse_matcher_alternatives::<T>, token(')')),
            parse_edge,
        )),
    )
    .map(|(name, child)| match name {
        Some(name) => ASTNode::Capture(Rc::new(CaptureASTNode { name, child })),
        None => child,
    })
    .parse(i)
}

pub(super) fn parse_edge<T: KindTypes>(i: &str) -> IResult<&str, ASTNode<T>, VerboseError<&str>> {
    pair(opt(edge_label_token::<T>), parse_node)
        .map(|(label, (node_selector, child))| {
            let node_selector = match (label, node_selector) {
                (None, s) => s,
                (Some(edge_label), NodeSelector::Anonymous) => {
                    NodeSelector::EdgeLabel { edge_label }
                }
                (Some(edge_label), NodeSelector::NodeKind { node_kind }) => {
                    NodeSelector::EdgeLabelAndNodeKind {
                        edge_label,
                        node_kind,
                    }
                }
                (Some(edge_label), NodeSelector::NodeText { node_text }) => {
                    NodeSelector::EdgeLabelAndNodeText {
                        edge_label,
                        node_text,
                    }
                }
                _ => unreachable!(),
            };
            ASTNode::NodeMatch(Rc::new(NodeMatchASTNode {
                node_selector,
                child,
            }))
        })
        .parse(i)
}

#[allow(clippy::type_complexity)]
fn parse_node<T: KindTypes>(
    i: &str,
) -> IResult<&str, (NodeSelector<T>, Option<ASTNode<T>>), VerboseError<&str>> {
    delimited(
        token('['),
        pair(parse_node_selector, opt(parse_matcher_sequence::<T>)), // NOTE: not matching alternatives here
        token(']'),
    )
    .parse(i)
}

fn parse_node_selector<T: KindTypes>(
    input: &str,
) -> IResult<&str, NodeSelector<T>, VerboseError<&str>> {
    alt((
        token('_').map(|_| NodeSelector::Anonymous),
        kind_token.map(|node_kind| NodeSelector::NodeKind { node_kind }),
        text_token.map(|node_text| NodeSelector::NodeText { node_text }),
    ))
    .parse(input)
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CaptureQuantifier {
    One, // allows quantification to generalise to 'unquantified'
    ZeroOrOne,
    ZeroOrMore,
    OneOrMore,
}

fn parse_trailing_quantifier(i: &str) -> IResult<&str, CaptureQuantifier, VerboseError<&str>> {
    alt((
        value(CaptureQuantifier::ZeroOrOne, token('?')),
        value(CaptureQuantifier::ZeroOrMore, token('*')),
        value(CaptureQuantifier::OneOrMore, token('+')),
        success(CaptureQuantifier::One), // you can call this without wrapping in `opt`
    ))
    .parse(i)
}

fn raw_identifier(i: &str) -> IResult<&str, String, VerboseError<&str>> {
    let identifier_head = satisfy(|c| c.is_alphabetic());
    let is_identifier_tail = |c: char| c == '_' || c.is_alphanumeric();
    recognize(alt((
        // single underscore is the anonymous syntax item,
        // so we don't allow it as an identifier
        char('_').and(take_while1(is_identifier_tail)),
        identifier_head.and(take_while(is_identifier_tail)),
    )))
    .map(|s: &str| s.to_string())
    .parse(i)
}

fn capture_name_token(i: &str) -> IResult<&str, String, VerboseError<&str>> {
    terminated(preceded(char('@'), raw_identifier), multispace0).parse(i)
}

fn kind_token<T: KindTypes>(i: &str) -> IResult<&str, NodeKind<T>, VerboseError<&str>> {
    context(
        "parsing node kind",
        terminated(
            preceded(
                peek(alpha1),
                cut(map_res(raw_identifier, |id| {
                    T::TerminalKind::try_from_str(id.as_str())
                        .map(NodeKind::Terminal)
                        .or_else(|_| {
                            T::NonterminalKind::try_from_str(id.as_str()).map(NodeKind::Nonterminal)
                        })
                })),
            ),
            multispace0,
        ),
    )
    .parse(i)
}

fn edge_label_token<T: KindTypes>(i: &str) -> IResult<&str, T::EdgeLabel, VerboseError<&str>> {
    context(
        "parsing edge label",
        terminated(
            preceded(
                peek(alpha1),
                cut(map_res(cut(raw_identifier), |id| {
                    T::EdgeLabel::try_from_str(id.as_str())
                })),
            ),
            token(':'),
        ),
    )
    .parse(i)
}

fn text_token(i: &str) -> IResult<&str, String, VerboseError<&str>> {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    enum Fragment<'a> {
        EscapedChar(char),
        SwallowedWhitespace,
        UnescapedSequence(&'a str),
    }

    let escaped_char = preceded(
        char('\\'),
        alt((
            map_opt(
                map_res(
                    preceded(
                        char('u'),
                        delimited(
                            char('{'),
                            // 1 to 6 hex digits
                            take_while_m_n(1, 6, |c: char| c.is_ascii_hexdigit()),
                            char('}'),
                        ),
                    ),
                    // converted from hex
                    move |hex| u32::from_str_radix(hex, 16),
                ),
                // converted to a char
                std::char::from_u32,
            ),
            value('\n', char('n')),
            value('\r', char('r')),
            value('\t', char('t')),
            value('\u{08}', char('b')),
            value('\u{0C}', char('f')),
            value('\\', char('\\')),
            value('"', char('"')),
        )),
    )
    .map(Fragment::EscapedChar);

    // any amount of whitespace, collapsed to nothing
    let swallowed_whitespace = value(
        Fragment::SwallowedWhitespace,
        preceded(char('\\'), multispace1),
    );

    let unescaped_sequence =
        verify(is_not("\"\\"), |s: &str| !s.is_empty()).map(Fragment::UnescapedSequence);

    let fragment = alt((unescaped_sequence, escaped_char, swallowed_whitespace));

    delimited(
        char('"'),
        fold_many0(fragment, String::new, |mut string, fragment| {
            match fragment {
                Fragment::EscapedChar(c) => string.push(c),
                Fragment::SwallowedWhitespace => {}
                Fragment::UnescapedSequence(s) => string.push_str(s),
            }
            string
        }),
        char('"'),
    )
    .parse(i)
}

fn ellipsis_token(i: &str) -> IResult<&str, &str, VerboseError<&str>> {
    terminated(tag("..."), multispace0).parse(i)
}

fn token<'input>(c: char) -> impl Parser<&'input str, char, VerboseError<&'input str>> {
    terminated(char(c), multispace0)
}

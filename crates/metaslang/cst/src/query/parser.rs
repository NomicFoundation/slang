use std::fmt;
use std::rc::Rc;

use nom::branch::alt;
use nom::bytes::complete::{is_not, tag, take_while, take_while1, take_while_m_n};
use nom::character::complete::{char, multispace0, multispace1, satisfy};
use nom::combinator::{
    all_consuming, cut, map_opt, map_res, opt, peek, recognize, success, value, verify,
};
use nom::error::{ErrorKind, FromExternalError, ParseError};
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

struct QueryParserError<I> {
    errors: Vec<(I, QueryParserErrorKind)>,
}

enum QueryParserErrorKind {
    Char(char),
    Nom(ErrorKind),
    Syntax(QuerySyntaxError),
}

enum QuerySyntaxError {
    EdgeLabel(String),
    NodeKind(String),
    EscapedUnicode,
}

impl<I> ParseError<I> for QueryParserError<I> {
    fn from_error_kind(input: I, kind: ErrorKind) -> Self {
        QueryParserError {
            errors: vec![(input, QueryParserErrorKind::Nom(kind))],
        }
    }

    fn append(input: I, kind: ErrorKind, mut other: Self) -> Self {
        other.errors.push((input, QueryParserErrorKind::Nom(kind)));
        other
    }

    fn from_char(input: I, c: char) -> Self {
        QueryParserError {
            errors: vec![(input, QueryParserErrorKind::Char(c))],
        }
    }
}

impl<I> FromExternalError<I, QuerySyntaxError> for QueryParserError<I> {
    fn from_external_error(input: I, _kind: ErrorKind, e: QuerySyntaxError) -> Self {
        QueryParserError {
            errors: vec![(input, QueryParserErrorKind::Syntax(e))],
        }
    }
}

impl fmt::Display for QuerySyntaxError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            QuerySyntaxError::EdgeLabel(label) => write!(f, "'{label}' is not a valid edge label"),
            QuerySyntaxError::NodeKind(kind) => write!(f, "'{kind}' is not a valid node kind"),
            QuerySyntaxError::EscapedUnicode => {
                write!(f, "Invalid escaped Unicode character")
            }
        }
    }
}

impl<I: fmt::Display> fmt::Display for QueryParserError<I> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Parse error:")?;
        for (input, error) in &self.errors {
            match error {
                QueryParserErrorKind::Nom(e) => writeln!(f, "{e:?} at: {input}")?,
                QueryParserErrorKind::Char(c) => writeln!(f, "expected '{c}' at: {input}")?,
                QueryParserErrorKind::Syntax(e) => writeln!(f, "{e} at: {input}")?,
            }
        }
        Ok(())
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

fn compute_row_and_column(target: &str, input: &str) -> TextIndex {
    let target_offset = input.offset(target);
    let mut text_index = TextIndex::ZERO;
    let mut from_iter = input.chars();
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

fn parse_matcher_alternatives<T: KindTypes>(
    i: &str,
) -> IResult<&str, ASTNode<T>, QueryParserError<&str>> {
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

fn parse_matcher_sequence<T: KindTypes>(
    i: &str,
) -> IResult<&str, ASTNode<T>, QueryParserError<&str>> {
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

fn parse_quantified_matcher<T: KindTypes>(
    i: &str,
) -> IResult<&str, ASTNode<T>, QueryParserError<&str>> {
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

fn parse_bound_matcher<T: KindTypes>(i: &str) -> IResult<&str, ASTNode<T>, QueryParserError<&str>> {
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

fn parse_edge<T: KindTypes>(i: &str) -> IResult<&str, ASTNode<T>, QueryParserError<&str>> {
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
) -> IResult<&str, (NodeSelector<T>, Option<ASTNode<T>>), QueryParserError<&str>> {
    delimited(
        token('['),
        pair(parse_node_selector, opt(parse_matcher_sequence::<T>)), // NOTE: not matching alternatives here
        token(']'),
    )
    .parse(i)
}

fn parse_node_selector<T: KindTypes>(
    input: &str,
) -> IResult<&str, NodeSelector<T>, QueryParserError<&str>> {
    alt((
        anonymous_selector,
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

fn parse_trailing_quantifier(i: &str) -> IResult<&str, CaptureQuantifier, QueryParserError<&str>> {
    alt((
        value(CaptureQuantifier::ZeroOrOne, token('?')),
        value(CaptureQuantifier::ZeroOrMore, token('*')),
        value(CaptureQuantifier::OneOrMore, token('+')),
        success(CaptureQuantifier::One), // you can call this without wrapping in `opt`
    ))
    .parse(i)
}

fn raw_identifier(i: &str) -> IResult<&str, String, QueryParserError<&str>> {
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

fn capture_name_token(i: &str) -> IResult<&str, String, QueryParserError<&str>> {
    terminated(preceded(char('@'), raw_identifier), multispace0).parse(i)
}

fn anonymous_selector<T: KindTypes>(
    input: &str,
) -> IResult<&str, NodeSelector<T>, QueryParserError<&str>> {
    // match a single _ character followed by whitespace or any other
    // non-alphanumeric symbol; otherwise this would eat the initial underscore
    // in an identifier such as `_foo`
    terminated(
        terminated(
            char('_'),
            peek(satisfy(|c| c != '_' && !c.is_alphanumeric())),
        ),
        multispace0,
    )
    .map(|_| NodeSelector::Anonymous)
    .parse(input)
}

fn kind_token<T: KindTypes>(i: &str) -> IResult<&str, NodeKind<T>, QueryParserError<&str>> {
    terminated(
        preceded(
            peek(satisfy(|c| c.is_alphabetic() || c == '_')),
            cut(map_res(raw_identifier, |id| {
                T::TerminalKind::try_from_str(id.as_str())
                    .map(NodeKind::Terminal)
                    .or_else(|_| {
                        T::NonterminalKind::try_from_str(id.as_str()).map(NodeKind::Nonterminal)
                    })
                    .or(Err(QuerySyntaxError::NodeKind(id)))
            })),
        ),
        multispace0,
    )
    .parse(i)
}

fn edge_label_token<T: KindTypes>(i: &str) -> IResult<&str, T::EdgeLabel, QueryParserError<&str>> {
    terminated(
        preceded(
            peek(satisfy(|c| c.is_alphabetic() || c == '_')),
            cut(map_res(cut(raw_identifier), |id| {
                T::EdgeLabel::try_from_str(id.as_str()).or(Err(QuerySyntaxError::EdgeLabel(id)))
            })),
        ),
        token(':'),
    )
    .parse(i)
}

fn text_token(i: &str) -> IResult<&str, String, QueryParserError<&str>> {
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
                    move |hex| {
                        u32::from_str_radix(hex, 16).or(Err(QuerySyntaxError::EscapedUnicode))
                    },
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

fn ellipsis_token(i: &str) -> IResult<&str, &str, QueryParserError<&str>> {
    terminated(tag("..."), multispace0).parse(i)
}

fn token<'input>(c: char) -> impl Parser<&'input str, char, QueryParserError<&'input str>> {
    terminated(char(c), multispace0)
}

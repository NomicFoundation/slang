// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use std::rc::Rc;

use nom::branch::alt;
use nom::bytes::complete::{is_not, tag, take_while, take_while1, take_while_m_n};
use nom::character::complete::{char, multispace0, multispace1, satisfy};
use nom::combinator::{all_consuming, map_opt, map_res, opt, recognize, value, verify};
use nom::error::VerboseError;
use nom::multi::{fold_many0, many0, many1};
use nom::sequence::{delimited, pair, preceded, terminated};
use nom::{Finish, IResult, Parser};

use super::kinds::{FieldName, RuleKind, TokenKind};
use super::query_model::{
    AlternativesMatch, BindingMatch, Kind, NodeId, NodeMatch, OneOrMoreMatch, OptionalMatch, Query,
    SequenceMatch,
};

pub(super) fn parse_query(input: &str) -> Result<Query, String> {
    all_consuming(preceded(
        multispace0,
        opt(binding_name_token)
            .and(alt((
                parse_node,
                delimited(
                    token('('),
                    pair(parse_node, many1(preceded(token('|'), parse_node))),
                    token(')'),
                )
                .map(|(first, rest)| {
                    let mut children = vec![first];
                    children.extend(rest);
                    Query::Alternatives(Rc::new(AlternativesMatch { children }))
                }),
            )))
            .map(|(binding_name, child)| {
                if let Some(name) = binding_name {
                    Query::Binding(Rc::new(BindingMatch { name, child }))
                } else {
                    child
                }
            }),
    ))
    .parse(input)
    .finish()
    .map(|(_, query)| query)
    .map_err(|e| e.to_string())
}

fn parse_node(i: &str) -> IResult<&str, Query, VerboseError<&str>> {
    delimited(
        token('['),
        parse_node_id.and(many0(parse_match)),
        token(']'),
    )
    .map(|(id, mut children)| {
        let child = if children.is_empty() {
            None
        } else if children.len() == 1 {
            Some(children.pop().unwrap())
        } else {
            Some(Query::Sequence(Rc::new(SequenceMatch { children })))
        };
        Query::Node(Rc::new(NodeMatch { id, child }))
    })
    .parse(i)
}

fn parse_node_id(input: &str) -> IResult<&str, NodeId, VerboseError<&str>> {
    pub enum Tail {
        Anonymous,
        Kind(Kind),
        String(String),
    }

    opt(field_name_token)
        .and(alt((
            token('_').map(|_| Tail::Anonymous),
            kind_name_token.map(Tail::Kind),
            string_token.map(Tail::String),
        )))
        .map(|(field, tail)| match (field, tail) {
            (None, Tail::Anonymous) => NodeId::Anonymous,
            (None, Tail::Kind(kind)) => NodeId::Kind { kind },
            (None, Tail::String(string)) => NodeId::String { string },
            (Some(field), Tail::Anonymous) => NodeId::Field { field },
            (Some(field), Tail::Kind(kind)) => NodeId::FieldAndKind { field, kind },
            (Some(field), Tail::String(string)) => NodeId::FieldAndString { field, string },
        })
        .parse(input)
}

#[derive(Clone)]
enum Quantifier {
    ZeroOrOne,
    ZeroOrMore,
    OneOrMore,
}

fn parse_match(input: &str) -> IResult<&str, Query, VerboseError<&str>> {
    opt(binding_name_token)
        .and(alt((
            parse_node,
            pair(
                delimited(token('('), many1(parse_match), token(')')),
                parse_trailing_quantifier,
            )
            .map(|(mut children, quantifier)| {
                let child = if children.len() == 1 {
                    children.pop().unwrap()
                } else {
                    Query::Sequence(Rc::new(SequenceMatch { children }))
                };
                match quantifier {
                    Quantifier::ZeroOrOne => Query::Optional(Rc::new(OptionalMatch { child })),
                    Quantifier::ZeroOrMore => Query::Optional(Rc::new(OptionalMatch {
                        child: Query::OneOrMore(Rc::new(OneOrMoreMatch { child })),
                    })),
                    Quantifier::OneOrMore => Query::OneOrMore(Rc::new(OneOrMoreMatch { child })),
                }
            }),
            delimited(
                token('('),
                pair(parse_match, many1(preceded(token('|'), parse_match))),
                token(')'),
            )
            .map(|(first, rest)| {
                let mut children = vec![first];
                children.extend(rest);
                Query::Alternatives(Rc::new(AlternativesMatch { children }))
            }),
            ellipsis_token.map(|_| Query::Ellipsis),
        )))
        .map(|(binding, child)| {
            if let Some(name) = binding {
                Query::Binding(Rc::new(BindingMatch { name, child }))
            } else {
                child
            }
        })
        .parse(input)
}

fn parse_trailing_quantifier(i: &str) -> IResult<&str, Quantifier, VerboseError<&str>> {
    alt((
        value(Quantifier::ZeroOrOne, token('?')),
        value(Quantifier::ZeroOrMore, token('*')),
        value(Quantifier::OneOrMore, token('+')),
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

fn binding_name_token(i: &str) -> IResult<&str, String, VerboseError<&str>> {
    terminated(preceded(char('@'), raw_identifier), multispace0).parse(i)
}

fn kind_name_token(i: &str) -> IResult<&str, Kind, VerboseError<&str>> {
    terminated(raw_identifier, multispace0)
        .map(|id| {
            TokenKind::try_from(id.as_str())
                .map(Kind::Token)
                .or_else(|_| RuleKind::try_from(id.as_str()).map(Kind::Rule))
                .unwrap() // TODO
        })
        .parse(i)
}

fn field_name_token(i: &str) -> IResult<&str, FieldName, VerboseError<&str>> {
    terminated(raw_identifier, token(':'))
        .map(|id| FieldName::try_from(id.as_str()).unwrap())
        .parse(i)
}

fn string_token(i: &str) -> IResult<&str, String, VerboseError<&str>> {
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

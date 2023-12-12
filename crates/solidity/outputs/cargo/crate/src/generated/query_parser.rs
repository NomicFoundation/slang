// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use nom::branch::alt;
use nom::bytes::complete::{tag, take_while, take_while1};
use nom::character::complete::{char, multispace0, one_of, satisfy};
use nom::combinator::{opt, recognize, value};
use nom::error::VerboseError;
use nom::multi::{many0, many1};
use nom::sequence::{delimited, pair, preceded, terminated};
use nom::{IResult, Parser};

use super::query_model::*;

impl Query {
    pub fn parse(input: &str) -> IResult<&str, Self, VerboseError<&str>> {
        preceded(
            multispace0,
            opt(binding_name_token)
                .and(alt((
                    Node::parse.map(|node| vec![node]),
                    delimited(
                        token('('),
                        pair(Node::parse, many1(preceded(token('|'), Node::parse))),
                        token(')'),
                    )
                    .map(|(first, rest)| {
                        let mut children = vec![first];
                        children.extend(rest);
                        children
                    }),
                )))
                .map(|(binding, children)| Self { binding, children }),
        )
        .parse(input)
    }
}

impl Node {
    fn parse(i: &str) -> IResult<&str, Self, VerboseError<&str>> {
        delimited(
            token('['),
            NodeId::parse.and(many0(NodeChild::parse)),
            token(']'),
        )
        .map(|(id, children)| Node { id, children })
        .parse(i)
    }
}

impl NodeId {
    fn parse(input: &str) -> IResult<&str, Self, VerboseError<&str>> {
        pub enum Tail {
            Anonymous,
            Kind(String),
            String(String),
        }

        opt(field_name_token)
            .and(alt((
                token('_').map(|_| Tail::Anonymous),
                kind_name_token.map(|s| Tail::Kind(s)),
                string_token.map(|s| Tail::String(s)),
            )))
            .map(|(field, tail)| match (field, tail) {
                (None, Tail::Anonymous) => Self::Anonymous,
                (None, Tail::Kind(kind)) => Self::Kind { kind },
                (None, Tail::String(string)) => Self::String { string },
                (Some(field), Tail::Anonymous) => Self::Field { field },
                (Some(field), Tail::Kind(kind)) => Self::FieldAndKind { field, kind },
                (Some(field), Tail::String(string)) => Self::FieldAndString { field, string },
            })
            .parse(input)
    }
}

impl NodeChild {
    fn parse(input: &str) -> IResult<&str, Self, VerboseError<&str>> {
        enum Tail {
            Node(Node),
            Group(Vec<NodeChild>, Quantifier),
            Ellipsis,
        }

        opt(binding_name_token)
            .and(alt((
                Node::parse.map(Tail::Node),
                pair(
                    delimited(token('('), many1(Self::parse), token(')')),
                    Quantifier::parse_trailing,
                )
                .map(|(children, quantifier)| Tail::Group(children, quantifier)),
                delimited(
                    token('('),
                    pair(Self::parse, many1(preceded(token('|'), Self::parse))),
                    token(')'),
                )
                .map(|(first, rest)| {
                    let mut children = vec![first];
                    children.extend(rest);
                    Tail::Group(children, Quantifier::OneOf)
                }),
                ellipsis_token.map(|_| Tail::Ellipsis),
            )))
            .map(|(binding, tail)| match tail {
                Tail::Node(node) => Self::Node { binding, node },
                Tail::Group(children, quantifier) => Self::Group {
                    binding,
                    children,
                    quantifier,
                },
                Tail::Ellipsis => Self::Ellipsis { binding },
            })
            .parse(input)
    }
}

impl Quantifier {
    fn parse_trailing(i: &str) -> IResult<&str, Self, VerboseError<&str>> {
        alt((
            value(Self::ZeroOrOne, token('?')),
            value(Self::ZeroOrMore, token('*')),
            value(Self::OneOrMore, token('+')),
        ))
        .parse(i)
    }
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

fn kind_name_token(i: &str) -> IResult<&str, String, VerboseError<&str>> {
    terminated(raw_identifier, multispace0).parse(i)
}

fn field_name_token(i: &str) -> IResult<&str, String, VerboseError<&str>> {
    terminated(raw_identifier, token(':')).parse(i)
}

fn string_token(_i: &str) -> IResult<&str, String, VerboseError<&str>> {
    todo!()
}

fn ellipsis_token(i: &str) -> IResult<&str, &str, VerboseError<&str>> {
    terminated(tag("..."), multispace0).parse(i)
}

fn token<'input>(c: char) -> impl Parser<&'input str, char, VerboseError<&'input str>> {
    terminated(char(c), multispace0)
}

#[test]
fn test_parsing() {
    let input = r#"
        @root [Identifier]
    "#;
    Query::parse(input).unwrap();
}

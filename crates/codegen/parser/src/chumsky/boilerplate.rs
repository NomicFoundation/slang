use proc_macro2::TokenStream;
use quote::quote;

pub fn kinds_head() -> TokenStream {
    quote!(
        use serde::Serialize;
    )
}

pub fn lex_head() -> TokenStream {
    quote!(
        use std::ops::Range;
        use serde::Serialize;
        use std::rc::Rc;

        use super::kinds::TokenKind;

        #[derive(Clone, Debug, PartialEq, Eq, Serialize)]
        pub enum Node {
            Chars(Range<usize>),
            Choice(usize, Rc<Node>),
            Sequence(Vec<Rc<Node>>),
            Named(TokenKind, Rc<Node>),
        }

        impl Node {
            pub fn chars(range: Range<usize>) -> Option<Rc<Self>> {
                Some(Rc::new(Self::Chars(range)))
            }

            pub fn chars_unwrapped(range: Range<usize>) -> Rc<Self> {
                Rc::new(Self::Chars(range))
            }

            pub fn sequence(elements: Vec<Option<Rc<Self>>>) -> Option<Rc<Self>> {
                let elements: Vec<_> = elements.into_iter().filter_map(|e| e).collect();
                if elements.is_empty() { None } else { Some(Rc::new(Self::Sequence(elements))) }
            }

            pub fn choice(index: usize, element: Option<Rc<Self>>) -> Option<Rc<Self>> {
                // TODO: disallow empty choices
                element.map(|e| Rc::new(Self::Choice(index, e)))
            }

            pub fn named(kind: TokenKind, element: Option<Rc<Self>>) -> Option<Rc<Self>> {
                element.map(|e| Rc::new(Self::Named(kind, e)))
            }

            pub fn range(&self) -> Range<usize> {
                match self {
                    Node::Chars(range) => range.clone(),
                    Node::Choice(_, element) => element.range(),
                    Node::Sequence(elements) => {
                        elements[0].range().start..elements[elements.len() - 1].range().end
                    }
                    Node::Named(_, element) => element.range(),
                }
            }
        }
    )
}

pub fn cst_head() -> TokenStream {
    quote!(
        use serde::Serialize;
        use std::rc::Rc;

        use super::kinds::{TokenKind, RuleKind};
        use super::lex;

        #[derive(Clone, Debug, PartialEq, Eq, Serialize)]
        pub enum Node {
            Rule {
                kind: RuleKind,
                children: Vec<Rc<Node>>,
            },
            Token {
                kind: TokenKind,
                lex_node: Rc<lex::Node>,
                #[serde(skip_serializing_if = "Vec::is_empty")]
                trivia: Vec<Rc<Node>>,
            },
            /// For anonymous groups referenced from AST nodes i.e. `delimited_by`
            Group {
                children: Vec<Rc<Node>>,
            },
            // TODO: Error types
        }

        impl Node {
            pub fn rule(kind: RuleKind, children: Vec<Option<Rc<Self>>>) -> Option<Rc<Self>> {
                let children: Vec<_> = children.into_iter().filter_map(|e| e).collect();
                if children.is_empty() { None } else { Some(Rc::new(Self::Rule { kind, children })) }
            }

            pub fn trivia_token(kind: TokenKind, lex_node: Rc<lex::Node>) -> Option<Rc<Self>> {
                Some(Rc::new(Self::Token {
                    kind,
                    lex_node,
                    trivia: vec![],
                }))
            }

            pub fn token(
                kind: TokenKind,
                lex_node: Rc<lex::Node>,
                leading_trivia: Option<Rc<Self>>,
                trailing_trivia: Option<Rc<Self>>,
            ) -> Option<Rc<Self>> {
                let mut trivia = vec![];
                if let Some(leading_trivia) = leading_trivia {
                    trivia.push(leading_trivia)
                }
                if let Some(trailing_trivia) = trailing_trivia {
                    trivia.push(trailing_trivia)
                }
                Some(Rc::new(Self::Token {
                    kind,
                    lex_node,
                    trivia,
                }))
            }

            pub fn group(children: Vec<Option<Rc<Self>>>) -> Option<Rc<Self>> {
                let children: Vec<_> = children.into_iter().filter_map(|e| e).collect();
                if children.is_empty() { None } else { Some(Rc::new(Self::Group { children })) }
            }

            pub fn top_level_token(lex_node: Option<Rc<lex::Node>>) -> Rc<Self> {
                if let Some(lex_node) = lex_node {
                    if let lex::Node::Named(kind, lex_node) = lex_node.as_ref() {
                        Rc::new(Self::Token {
                            kind: *kind,
                            lex_node: lex_node.clone(),
                            trivia: vec![],
                        })
                    } else {
                        unreachable!("Top level token unexpected result: {:?}", lex_node)
                    }
                } else {
                    unreachable!("Top level token unexpected None")
                }
            }

            pub fn top_level_rule(kind: RuleKind, node: Option<Rc<Self>>) -> Rc<Self> {
                node.unwrap_or_else(|| Rc::new(Self::Rule { kind, children: vec![] }))
            }
        }
    )
}

pub fn parse_head() -> TokenStream {
    quote!(
        use chumsky::Parser;
        use chumsky::prelude::*;
        use semver::Version;
        use std::ops::Range;
        use std::rc::Rc;

        use super::kinds::{TokenKind, RuleKind};
        use super::lex;
        use super::cst;

        pub type SpanType = Range<usize>;
        pub type ErrorType = Simple<char, SpanType>;
        pub type BoxedParserType = BoxedParser<'static, char, Rc<cst::Node>, ErrorType>;

        #[allow(dead_code)]
        fn difference<M, S, T>(minuend: M, subtrahend: S) -> impl Parser<char, T, Error = ErrorType>
        where
            M: Parser<char, T, Error = ErrorType> + Clone,
            S: Parser<char, T, Error = ErrorType>,
        {
            // TODO This could be much more efficient if we were able
            // to conditionally rewind
            let minuend_end = minuend
                .clone()
                .map_with_span(|_, span: SpanType| span.end())
                .rewind();
            let subtrahend_end = subtrahend
                .map_with_span(|_, span: SpanType| span.end())
                .rewind()
                .or_else(|_| Ok(0));
            minuend_end
                .then(subtrahend_end)
                .validate(|(m, s), span, emit| {
                    if m == s {
                        emit(Simple::custom(span, "subtrahend matches minuend"))
                    }
                })
                .ignore_then(minuend)
        }
    )
}

pub fn parse_macros() -> TokenStream {
    quote!(
        #[allow(unused_macros)]
        macro_rules! lex_terminal {
            ($kind:ident, $literal:literal) => {
                just($literal).map_with_span(|_, span: SpanType| {
                    lex::Node::named(TokenKind::$kind, lex::Node::chars(span.start()..span.end()))
                })
            };
            ($kind:ident, $filter:expr) => {
                filter($filter).map_with_span(|_, span: SpanType| {
                    lex::Node::named(TokenKind::$kind, lex::Node::chars(span.start()..span.end()))
                })
            };
            ($literal:literal) => {
                just($literal)
                    .map_with_span(|_, span: SpanType| lex::Node::chars(span.start()..span.end()))
            };
            ($filter:expr) => {
                filter($filter)
                    .map_with_span(|_, span: SpanType| lex::Node::chars(span.start()..span.end()))
            };
        }

        #[allow(unused_macros)]
        macro_rules! lex_rule {
            ($rule:ident) => {
                $rule.clone()
            };
        }

        #[allow(unused_macros)]
        macro_rules! lex_choice {
            ($kind:ident, $($expr:expr),*) => {
                lex_choice!($($expr),*).map(|element| lex::Node::named(TokenKind::$kind, element))
            };
            ($($expr:expr),*) => {
                choice::<_, ErrorType>((
                    $($expr.map(|v| lex::Node::choice(0, v))),*
                ))
            };
        }

        #[allow(unused_macros)]
        macro_rules! lex_seq {
            // HELPERS -------------------------------------------------------------------------------

            /*
                (@exp a, b, c, d)
                => a.then(@exp b, c, d)
                => a.then(b.then(@exp c, d))
                => a.then(b.then(c.then(@exp d)))
                => a.then(b.then(c.then(d)))
            */

            (@exp $head:expr , $($tail:expr),+ ) => {
                $head.then(lex_seq!(@exp $($tail),+ ))
            };

            (@exp $head:expr ) => {
                $head
            };

            /*
                (@args [], v, a, b, c, d)
                => (@args [v.0,], v.1, b, c, d)
                => (@args [v.0, v.1.0,], v.1.1, c, d)
                => (@args [v.0, v.1.0, v.1.1.0,], v.1.1.1, d)
                => vec![v.0, v.1.0, v.1.1.0, v.1.1.0, v1.1.1, ]
            */

            (@args [ $($accum:expr,)* ] , $current:expr , $head:expr , $($tail:expr),+ ) => {
                lex_seq!(@args [ $($accum,)* $current.0, ] , $current.1 , $($tail),+ )
            };

            (@args [ $($accum:expr,)* ] , $current:expr , $head:expr ) => {
                vec![ $($accum,)* $current ]
            };

            //----------------------------------------------------------------------------------------

            ($kind:ident, $($expr:expr),+ ) => {
                lex_seq!(@exp $($expr),+ )
                    .map(|v| lex::Node::named(TokenKind::$kind, lex::Node::sequence(lex_seq!(@args [] , v , $($expr),+ ))))
            };

            ($($expr:expr),+ ) => {
                lex_seq!(@exp $($expr),+ )
                    .map(|v| lex::Node::sequence(lex_seq!(@args [] , v , $($expr),+ )))
            };
        }

        #[allow(unused_macros)]
        macro_rules! lex_zero_or_more {
            ($kind:ident, $expr:expr) => {
                lex_zero_or_more!($expr).map(|element| lex::Node::named(TokenKind::$kind, element))
            };
            ($expr:expr) => {
                $expr.repeated().map(|v| lex::Node::sequence(v))
            };
        }

        #[allow(unused_macros)]
        macro_rules! lex_one_or_more {
            ($kind:ident, $expr:expr) => {
                lex_one_or_more!($expr).map(|element| lex::Node::named(TokenKind::$kind, element))
            };
            ($expr:expr) => {
                $expr.repeated().at_least(1).map(|v| lex::Node::sequence(v))
            };
        }

        #[allow(unused_macros)]
        macro_rules! lex_repeated {
            ($kind:ident, $expr:expr, $min:literal, $max:literal) => {
                lex_repeated!($expr, $min, $max)
                    .map(|element| lex::Node::named(TokenKind::$kind, element))
            };
            ($expr:expr, $min:literal, $max:literal) => {
                $expr
                    .repeated()
                    .at_least($min)
                    .at_most($max)
                    .map(|v| lex::Node::sequence(v))
            };
        }

        #[allow(unused_macros)]
        macro_rules! lex_optional {
            ($expr:expr) => {
                $expr.or_not().map(|v| v.flatten())
            };
        }

        #[allow(unused_macros)]
        macro_rules! lex_separated_by {
            ($kind:ident, $expr:expr, $separator:expr) => {
                lex_separated_by!($expr, $separator)
                    .map(|element| lex::Node::named(TokenKind::$kind, element))
            };
            ($expr:expr, $separator:expr) => {
                $expr
                    .then($separator.then($expr).repeated())
                    .map(|(first, rest)| {
                        let mut v = vec![first];
                        for (separator, expr) in rest {
                            v.push(separator);
                            v.push(expr);
                        }
                        lex::Node::sequence(v)
                    })
            };
        }

        #[allow(unused_macros)]
        macro_rules! lex_trie {
            ($kind:ident, $($expr:expr),* ) => {
                choice::<_, ErrorType>(($($expr),*)).map_with_span(|leaf_kind, span: SpanType|
                    lex::Node::named(
                        TokenKind::$kind,
                        lex::Node::named(leaf_kind, lex::Node::chars(span.start()..span.end()))
                    )
                )
            };
            ($($expr:expr),* ) => {
                choice::<_, ErrorType>(($($expr),*)).map_with_span(|kind, span: SpanType|
                    lex::Node::named(kind, lex::Node::chars(span.start()..span.end())))
            };
        }

        #[allow(unused_macros)]
        macro_rules! trieleaf {
            ($kind:ident, $string:literal ) => {
                just($string).to(TokenKind::$kind)
            };
            ($kind:ident ) => {
                empty().to(TokenKind::$kind)
            };
        }

        #[allow(unused_macros)]
        macro_rules! trieprefix {
            ($string:literal , [ $($expr:expr),* ] ) => (
                just($string).ignore_then(choice::<_, ErrorType>(($($expr),*)))
            )
        }

        #[allow(unused_macros)]
        macro_rules! trivia_terminal {
            ($kind:ident, $literal:literal) => {
                just($literal).map_with_span(|_, span: SpanType| {
                    cst::Node::trivia_token(
                        TokenKind::$kind,
                        lex::Node::chars_unwrapped(span.start()..span.end()),
                    )
                })
            };
            ($kind:ident, $filter:expr) => {
                filter($filter).map_with_span(|_, span: SpanType| {
                    cst::Node::trivia_token(
                        TokenKind::$kind,
                        lex::Node::chars_unwrapped(span.start()..span.end()),
                    )
                })
            };
        }

        #[allow(unused_macros)]
        macro_rules! trivia_token {
            ($token_rule:ident) => {
                $token_rule.clone().map(|token: Option<Rc<lex::Node>>| {
                    let token = token.unwrap(); // token rule should always return a token
                    if let lex::Node::Named(kind, element) = token.as_ref() {
                        cst::Node::trivia_token(*kind, element.clone())
                    } else {
                        unreachable!("a token rule should always return a named token, but rule {} returned {:?}", stringify!($token_rule), token)
                    }
                })
            };
        }

        #[allow(unused_macros)]
        macro_rules! trivia_trie {
            ($($expr:expr),* ) => (
                choice::<_, ErrorType>(($($expr),*)).map_with_span(|kind, span: SpanType| cst::Node::trivia_token(kind, lex::Node::chars_unwrapped(span.start()..span.end())))
            )
        }

        #[allow(unused_macros)]
        macro_rules! terminal {
            ($kind:ident, $literal:literal) => {
                leading_trivia_parser
                    .clone()
                    .then(
                        just($literal).map_with_span(|_, span: SpanType| span.start()..span.end()),
                    )
                    .then(trailing_trivia_parser.clone())
                    .map(|((leading_trivia, range), trailing_trivia)| {
                        cst::Node::token(
                            TokenKind::$kind,
                            lex::Node::chars_unwrapped(range),
                            leading_trivia,
                            trailing_trivia,
                        )
                    })
            };
            ($kind:ident, $filter:expr) => {
                leading_trivia_parser
                    .clone()
                    .then(
                        filter($filter).map_with_span(|_, span: SpanType| span.start()..span.end()),
                    )
                    .then(trailing_trivia_parser.clone())
                    .map(|((leading_trivia, range), trailing_trivia)| {
                        cst::Node::token(
                            TokenKind::$kind,
                            lex::Node::chars_unwrapped(range),
                            leading_trivia,
                            trailing_trivia,
                        )
                    })
            };
        }

        #[allow(unused_macros)]
        macro_rules! token {
            ($token_rule:ident) => {
                leading_trivia_parser.clone()
                    .then($token_rule.clone())
                    .then(trailing_trivia_parser.clone())
                    .map(|((leading_trivia, token), trailing_trivia): ((_, Option<Rc<lex::Node>>), _)| {
                        let token = token.unwrap(); // token rule should always return a token
                        if let lex::Node::Named(kind, element) = token.as_ref() {
                            cst::Node::token(*kind, element.clone(), leading_trivia, trailing_trivia)
                        } else {
                            unreachable!("a token rule should always return a named token, but rule {} returned {:?}", stringify!($token_rule), token)
                        }
                    })
            };
        }

        #[allow(unused_macros)]
        macro_rules! rule {
            ($rule:ident) => {
                $rule.clone()
            };
        }

        #[allow(unused_macros)]
        macro_rules! choice {
            ($kind:ident, $($expr:expr),*) => {
                choice::<_, ErrorType>(($($expr),* ))
            };
            ($($expr:expr),* ) => {
                choice::<_, ErrorType>(($($expr),* ))
            };
        }

        #[allow(unused_macros)]
        macro_rules! seq {
            // HELPERS -------------------------------------------------------------------------------

            /*
                (@exp a, b, c, d)
                => a.then(@exp b, c, d)
                => a.then(b.then(@exp c, d))
                => a.then(b.then(c.then(@exp d)))
                => a.then(b.then(c.then(d)))
            */

            (@exp $head:expr , $($tail:expr),+ ) => {
                $head.then(seq!(@exp $($tail),+ ))
            };

            (@exp $head:expr ) => {
                $head
            };

            /*
                (@args [], v, a, b, c, d)
                => (@args [v.0,], v.1, b, c, d)
                => (@args [v.0, v.1.0,], v.1.1, c, d)
                => (@args [v.0, v.1.0, v.1.1.0,], v.1.1.1, d)
                => vec![v.0, v.1.0, v.1.1.0, v.1.1.0, v1.1.1, ]
            */

            (@args [ $($accum:expr,)* ] , $current:expr , $head:expr , $($tail:expr),+ ) => {
                seq!(@args [ $($accum,)* $current.0, ] , $current.1 , $($tail),+ )
            };

            (@args [ $($accum:expr,)* ] , $current:expr , $head:expr ) => {
                vec![ $($accum,)* $current ]
            };

            //----------------------------------------------------------------------------------------

            ($kind:ident, $($expr:expr),+ ) => {
                seq!(@exp $($expr),+ )
                    .map(|v| cst::Node::rule(RuleKind::$kind, seq!(@args [] , v , $($expr),+ )))
            };

            ($($expr:expr),+ ) => {
                seq!(@exp $($expr),+ )
                    .map(|v| cst::Node::group(seq!(@args [] , v , $($expr),+ )))
            };
        }

        #[allow(unused_macros)]
        macro_rules! zero_or_more {
            ($kind:ident, $expr:expr) => {
                $expr
                    .repeated()
                    .map(|children| cst::Node::rule(RuleKind::$kind, children))
            };
            ($expr:expr) => {
                $expr.repeated().map(|children| cst::Node::group(children))
            };
        }

        #[allow(unused_macros)]
        macro_rules! one_or_more {
            ($kind:ident, $expr:expr) => {
                $expr
                    .repeated()
                    .at_least(1)
                    .map(|children| cst::Node::rule(RuleKind::$kind, children))
            };
            ($expr:expr) => {
                $expr
                    .repeated()
                    .at_least(1)
                    .map(|children| cst::Node::group(children))
            };
        }

        #[allow(unused_macros)]
        macro_rules! repeated {
            ($kind:ident, $expr:expr, $min:literal, $max:literal) => {
                $expr
                    .repeated()
                    .at_least($min)
                    .at_most($max)
                    .map(|children| cst::Node::rule(RuleKind::$kind, children))
            };
            ($expr:expr, $min:literal, $max:literal) => {
                $expr
                    .repeated()
                    .at_least($min)
                    .at_most($max)
                    .map(|children| cst::Node::group(children))
            };
        }

        #[allow(unused_macros)]
        macro_rules! optional {
            ($expr:expr) => {
                $expr.or_not().map(|opt| opt.flatten())
            };
        }

        #[allow(unused_macros)]
        macro_rules! separated_by {
            ($kind:ident, $expr:expr, $separator:expr) => {
                $expr
                    .then($separator.then($expr).repeated())
                    .map(|(first, rest)| {
                        let mut v = vec![first];
                        for (separator, expr) in rest {
                            v.push(separator);
                            v.push(expr);
                        }
                        cst::Node::rule(RuleKind::$kind, v)
                    })
            };
            ($expr:expr, $separator:expr) => {
                $expr
                    .then($separator.then($expr).repeated())
                    .map(|(first, rest)| {
                        let mut v = vec![first];
                        for (separator, expr) in rest {
                            v.push(separator);
                            v.push(expr);
                        }
                        cst::Node::group(v)
                    })
            };
        }

        #[allow(unused_macros)]
        macro_rules! left_associative_binary_expression {
            ($kind:ident, $next_sibling:expr, $operator:expr) => {
                $next_sibling
                    .clone()
                    .then($operator.then($next_sibling.clone()).repeated())
                    .map(|(first, rest)| {
                        if rest.is_empty() {
                            first
                        } else {
                            // a [ (X b) (Y c) (Z d) ] => { { { a X b } Y c } Z d }
                            rest.into_iter().fold(
                                first,
                                |left_operand, (operator, right_operand)| {
                                    cst::Node::rule(
                                        RuleKind::$kind,
                                        vec![left_operand, operator, right_operand],
                                    )
                                },
                            )
                        }
                    })
            };
        }

        #[allow(unused_macros)]
        macro_rules! right_associative_binary_expression {
            ($kind:ident, $next_sibling:expr, $operator:expr) => {
                $next_sibling
                    .clone()
                    .then($operator.then($next_sibling.clone()).repeated())
                    .map(|(first, rest)| {
                        if rest.is_empty() {
                            first
                        } else {
                            // a [ (X b) (Y c) (Z d) ] => [ (a X) (b Y) (c Z) ] d
                            let mut last_operand = first;
                            let mut operand_operator_pairs = vec![];
                            for (operator, right_operand) in rest.into_iter() {
                                let left_operand =
                                    std::mem::replace(&mut last_operand, right_operand);
                                operand_operator_pairs.push((left_operand, operator))
                            }
                            // [ (a X) (b Y) (c Z) ] d => { a X { b Y { c Z d } } }
                            operand_operator_pairs.into_iter().rfold(
                                last_operand,
                                |right_operand, (left_operand, operator)| {
                                    cst::Node::rule(
                                        RuleKind::$kind,
                                        vec![left_operand, operator, right_operand],
                                    )
                                },
                            )
                        }
                    })
            };
        }

        #[allow(unused_macros)]
        macro_rules! unary_prefix_expression {
            ($kind:ident, $next_sibling:expr, $operator:expr) => {
                $operator
                    .repeated()
                    .then($next_sibling.clone())
                    .map(|(mut operators, operand)| {
                        if operators.is_empty() {
                            operand
                        } else {
                            operators.reverse();
                            operators
                                .into_iter()
                                .fold(operand, |right_operand, operator| {
                                    cst::Node::rule(RuleKind::$kind, vec![operator, right_operand])
                                })
                        }
                    })
            };
        }

        #[allow(unused_macros)]
        macro_rules! unary_suffix_expression {
            ($kind:ident, $next_sibling:expr, $operator:expr) => {
                $next_sibling
                    .clone()
                    .then($operator.repeated())
                    .map(|(operand, operators)| {
                        if operators.is_empty() {
                            operand
                        } else {
                            operators
                                .into_iter()
                                .fold(operand, |left_operand, operator| {
                                    cst::Node::rule(RuleKind::$kind, vec![left_operand, operator])
                                })
                        }
                    })
            };
        }

        #[allow(unused_macros)]
        macro_rules! delimited_by {
            ($kind:ident, $open:expr, $expr:expr, $close:expr) => {
                seq!($kind, $open, $expr, $close)
            };
            ($open:expr, $expr:expr, $close:expr) => {
                seq!($open, $expr, $close)
            };
        }

        #[allow(unused_macros)]
        macro_rules! trie {
            ($($expr:expr),* ) => (
                leading_trivia_parser.clone()
                    .then(choice::<_, ErrorType>(($($expr),*)).map_with_span(|kind, span: SpanType| (kind, span.start()..span.end())))
                    .then(trailing_trivia_parser.clone())
                    .map(|((leading_trivia, (kind, range)), trailing_trivia)| {
                        cst::Node::token(kind, lex::Node::chars(range), leading_trivia, trailing_trivia)
                    })
            )
        }
    )
}

pub fn error_renderer() -> TokenStream {
    quote!(
        fn render_error_report(
            error: &ErrorType,
            source_id: &str,
            source: &str,
            with_color: bool,
        ) -> String {
            let kind = ReportKind::Error;
            let color = if with_color { Color::Red } else { Color::Unset };

            let message = match error.reason() {
                SimpleReason::Custom(message) => {
                    // use custom message as-is
                    message.to_string()
                }
                SimpleReason::Unclosed { delimiter, .. } => {
                    format!("Expected delimiter '{}' to be closed", delimiter.fg(color))
                }
                SimpleReason::Unexpected => {
                    let mut expected: Vec<&Option<char>> = error.expected().collect();
                    expected.sort();

                    let expected = if expected.len() == 0 {
                        "something else".to_string()
                    } else {
                        expected
                            .iter()
                            .map(|expected| match expected {
                                Some(expected) => format!("'{}'", expected),
                                None => "end of input".to_string(),
                            })
                            .collect::<Vec<_>>()
                            .join(" or ")
                    };

                    format!("Expected {expected}.")
                }
            };

            let source_start = error.span().start() as usize;
            let source_end = error.span().end() as usize;

            if source.is_empty() {
                return format!("{kind}: {message}\n   ─[{source_id}:{source_start}:{source_end}]");
            }

            let label = match error.reason() {
                SimpleReason::Custom(_) => "Error occurred here.".to_string(),
                SimpleReason::Unclosed { delimiter, .. } => {
                    format!("Unclosed delimiter '{}'.", delimiter.fg(color))
                }
                SimpleReason::Unexpected => {
                    if let Some(found) = error.found() {
                        format!("Found '{}'.", found.fg(color))
                    } else {
                        "Found end of input.".to_string()
                    }
                }
            };

            let mut builder = Report::build(kind, source_id, source_start)
                .with_config(Config::default().with_color(with_color))
                .with_message(message);

            builder.add_label(
                Label::new((source_id, source_start..source_end))
                    .with_color(color)
                    .with_message(label),
            );

            let mut result = vec![];
            builder
                .finish()
                .write((source_id, Source::from(&source)), &mut result)
                .expect("Failed to write report");

            return String::from_utf8(result)
                .expect("Failed to convert report to utf8")
                .trim()
                .to_string();
        }
    )
}

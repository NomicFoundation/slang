use proc_macro2::TokenStream;
use quote::quote;

pub fn mod_head() -> TokenStream {
    quote!(
        pub mod kinds;
        pub mod lex;
        pub mod cst;
        pub mod ast;
        pub mod parse;
    )
}

pub fn kinds_head() -> TokenStream {
    quote!(
        use serde::Serialize;
    )
}

pub fn lex_head() -> TokenStream {
    quote!(
        use std::ops::Range;
        use serde::Serialize;

        use super::kinds;
        use super::parse::Context;

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
            pub fn choice(
                context: &'a Context<'a>,
                number: usize,
                element: &'a Node<'a>,
            ) -> &'a Node<'a> {
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
    )
}

pub fn cst_head() -> TokenStream {
    quote!(
        use std::ops::Range;
        use serde::Serialize;

        use super::parse::Context;
        use super::kinds;

        #[derive(Clone, Debug, PartialEq, Eq, Serialize)]
        pub enum Node<'a> {
            None,
            Rule {
                kind: kinds::Rule,
                children: Vec<&'a Node<'a>>,
            },
            Token {
                kind: kinds::Token,
                /// Range doesn't include the trivia
                range: Range<usize>,
                /// Only Trivia
                #[serde(skip_serializing_if = "Vec::is_empty")]
                trivia: Vec<&'a Node<'a>>,
            },
            /// For anonymous groups referenced from AST nodes i.e. `delimited_by`
            Group {
                #[serde(skip_serializing_if = "Vec::is_empty")]
                children: Vec<&'a Node<'a>>,
            }, // TODO: Error types
        }

        #[allow(unused_variables)]
        pub trait Visitor {
            fn enter_rule(
                &mut self,
                kind: kinds::Rule,
                children: &Vec<&Node<'_>>,
                node: &Node<'_>,
            ) -> VisitorEntryResponse {
                VisitorEntryResponse::StepIn
            }
            fn exit_rule(
                &mut self,
                kind: kinds::Rule,
                children: &Vec<&Node<'_>>,
                node: &Node<'_>,
            ) -> VisitorExitResponse {
                VisitorExitResponse::StepIn
            }
            fn enter_token(
                &mut self,
                kind: kinds::Token,
                range: &Range<usize>,
                trivia: &Vec<&Node<'_>>,
                node: &Node<'_>,
            ) -> VisitorEntryResponse {
                VisitorEntryResponse::StepIn
            }
            fn exit_token(
                &mut self,
                kind: kinds::Token,
                range: &Range<usize>,
                trivia: &Vec<&Node<'_>>,
                node: &Node<'_>,
            ) -> VisitorExitResponse {
                VisitorExitResponse::StepIn
            }
            fn enter_group(
                &mut self,
                children: &Vec<&Node<'_>>,
                node: &Node<'_>,
            ) -> VisitorEntryResponse {
                VisitorEntryResponse::StepIn
            }
            fn exit_group(
                &mut self,
                children: &Vec<&Node<'_>>,
                node: &Node<'_>,
            ) -> VisitorExitResponse {
                VisitorExitResponse::StepIn
            }
            fn visit_none(&mut self, node: &Node<'_>) -> VisitorExitResponse {
                VisitorExitResponse::StepIn
            }
        }

        pub enum VisitorEntryResponse {
            Quit,
            StepIn,
            StepOver,
        }

        pub enum VisitorExitResponse {
            Quit,
            StepIn,
        }

        pub trait Visitable {
            fn visit<T: Visitor>(&self, visitor: &mut T) -> VisitorExitResponse;
        }

        impl<'a> Node<'a> {
            #[inline]
            pub fn none(context: &'a Context<'a>) -> &'a Node<'a> {
                context.alloc_cst_node(Self::None)
            }
            #[inline]
            pub fn rule(
                context: &'a Context<'a>,
                kind: kinds::Rule,
                children: Vec<&'a Node<'a>>,
            ) -> &'a Node<'a> {
                context.alloc_cst_node(Self::Rule { kind, children })
            }
            #[inline]
            pub fn trivia_token(
                context: &'a Context<'a>,
                range: Range<usize>,
                kind: kinds::Token,
            ) -> &'a Node<'a> {
                context.alloc_cst_node(Self::Token {
                    range,
                    kind,
                    trivia: vec![],
                })
            }
            #[inline]
            pub fn token(
                context: &'a Context<'a>,
                range: Range<usize>,
                kind: kinds::Token,
                leading_trivia: &'a Node<'a>,
                trailing_trivia: &'a Node<'a>,
            ) -> &'a Node<'a> {
                let mut trivia = vec![];
                if *leading_trivia != Node::None {
                    trivia.push(leading_trivia)
                }
                if *trailing_trivia != Node::None {
                    trivia.push(trailing_trivia)
                }
                context.alloc_cst_node(Self::Token {
                    range,
                    kind,
                    trivia,
                })
            }
            #[inline]
            pub fn group(context: &'a Context<'a>, children: Vec<&'a Node<'a>>) -> &'a Node<'a> {
                if children.is_empty() {
                    Self::none(context)
                } else {
                    context.alloc_cst_node(Self::Group { children })
                }
            }
        }

        impl<'a> Visitable for &'a Node<'a> {
            fn visit<T: Visitor>(&self, visitor: &mut T) -> VisitorExitResponse {
                match self {
                    Node::None => visitor.visit_none(self),
                    Node::Rule { kind, children } => {
                        match visitor.enter_rule(*kind, children, self) {
                            VisitorEntryResponse::Quit => return VisitorExitResponse::Quit,
                            VisitorEntryResponse::StepOver => {}
                            VisitorEntryResponse::StepIn => {
                                for child in children {
                                    match child.visit(visitor) {
                                        VisitorExitResponse::Quit => {
                                            return VisitorExitResponse::Quit
                                        }
                                        VisitorExitResponse::StepIn => {}
                                    }
                                }
                            }
                        }
                        visitor.exit_rule(*kind, children, self)
                    }
                    Node::Token {
                        kind,
                        range,
                        trivia,
                    } => {
                        match visitor.enter_token(*kind, range, trivia, self) {
                            VisitorEntryResponse::Quit => return VisitorExitResponse::Quit,
                            VisitorEntryResponse::StepOver => {}
                            VisitorEntryResponse::StepIn => {
                                for child in trivia {
                                    match child.visit(visitor) {
                                        VisitorExitResponse::Quit => {
                                            return VisitorExitResponse::Quit
                                        }
                                        VisitorExitResponse::StepIn => {}
                                    }
                                }
                            }
                        }
                        visitor.exit_token(*kind, range, trivia, self)
                    }
                    Node::Group { children } => {
                        match visitor.enter_group(children, self) {
                            VisitorEntryResponse::Quit => return VisitorExitResponse::Quit,
                            VisitorEntryResponse::StepOver => {}
                            VisitorEntryResponse::StepIn => {
                                for child in children {
                                    match child.visit(visitor) {
                                        VisitorExitResponse::Quit => {
                                            return VisitorExitResponse::Quit
                                        }
                                        VisitorExitResponse::StepIn => {}
                                    }
                                }
                            }
                        }
                        visitor.exit_group(children, self)
                    }
                }
            }
        }
    )
}

pub fn ast_head() -> TokenStream {
    quote!(
        #[allow(unused_imports)]
        use super::kinds;
    )
}

pub fn parse_head() -> TokenStream {
    quote!(
        use chumsky::Parser;
        use chumsky::prelude::*;
        use semver::Version;
        use std::ops::Range;

        use super::kinds;
        use super::lex;
        use super::cst;
        #[allow(unused_imports)]
        use super::ast;

        pub struct Context<'a> {
            lex_node_arena: typed_arena::Arena<lex::Node<'a>>,
            cst_node_arena: typed_arena::Arena<cst::Node<'a>>,
        }

        impl std::hash::Hash for Context<'_> {
            fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
                std::ptr::hash(self, state)
            }
        }

        impl PartialEq for Context<'_> {
            fn eq(&self, other: &Self) -> bool {
                std::ptr::eq(self, other)
            }
        }

        impl Eq for Context<'_> {}

        impl std::fmt::Debug for Context<'_> {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.debug_struct("Context").finish()
            }
        }

        impl std::fmt::Display for Context<'_> {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                // TODO: implement
                write!(f, "Context")
            }
        }

        impl Context<'_> {
            pub fn new() -> Self {
                Self {
                    lex_node_arena: typed_arena::Arena::new(),
                    cst_node_arena: typed_arena::Arena::new(),
                }
            }
        }

        impl<'a> Context<'a> {
            pub fn alloc_lex_node(&'a self, node: lex::Node<'a>) -> &'a lex::Node<'a> {
                self.lex_node_arena.alloc(node)
            }
            pub fn alloc_cst_node(&'a self, node: cst::Node<'a>) -> &'a cst::Node<'a> {
                self.cst_node_arena.alloc(node)
            }
        }

        pub type SpanType<'a> = (&'a Context<'a>, Range<usize>);
        pub type ErrorType<'a> = Simple<char, SpanType<'a>>;
        pub type ParserType<'a, T> = BoxedParser<'a, char, T, ErrorType<'a>>;

        #[allow(dead_code)]
        fn difference<'a, M, S, T>(
            minuend: M,
            subtrahend: S,
        ) -> impl Parser<char, T, Error = ErrorType<'a>>
        where
            M: Clone + Parser<char, T, Error = ErrorType<'a>>,
            S: Parser<char, T, Error = ErrorType<'a>>,
        {
            // TODO This could be much more efficient if we were able
            // to conditionally rewind
            let minuend_end = minuend.clone().map_with_span(|_, span: SpanType<'a>| span.end()).rewind();
            let subtrahend_end = subtrahend
                .map_with_span(|_, span: SpanType<'a>| span.end())
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
                just($literal).map_with_span(|_, span: SpanType<'a>| {
                    lex::Node::named(
                        &span.context(),
                        kinds::Token::$kind,
                        lex::Node::chars(&span.context(), span.start()..span.end()),
                    )
                })
            };
            ($kind:ident, $filter:expr) => {
                filter($filter).map_with_span(|_, span: SpanType<'a>| {
                    lex::Node::named(
                        &span.context(),
                        kinds::Token::$kind,
                        lex::Node::chars(&span.context(), span.start()..span.end()),
                    )
                })
            };
            ($literal:literal) => {
                just($literal).map_with_span(|_, span: SpanType<'a>| {
                    lex::Node::chars(&span.context(), span.start()..span.end())
                })
            };
            ($filter:expr) => {
                filter($filter).map_with_span(|_, span: SpanType<'a>| {
                    lex::Node::chars(&span.context(), span.start()..span.end())
                })
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
                lex_choice!($($expr),*).map_with_span(|element, span: SpanType<'a>| lex::Node::named(&span.context(), kinds::Token::$kind, element))
            };
            ($($expr:expr),*) => {
                choice::<_, ErrorType>((
                    $($expr.map_with_span(|v, span: SpanType<'a>| lex::Node::choice(&span.context(), 0, v))),*
                ))
            };
        }

        #[allow(unused_macros)]
        macro_rules! lex_seq {
            ($kind:ident, $($expr:expr),*) => {
                lex_seq!($($expr),*).map_with_span(|element, span: SpanType<'a>| lex::Node::named(&span.context(), kinds::Token::$kind, element))
            };
            // THIS IS WRONG - it should accumulate the elements like seq! does below
            ($a:expr, $($b:expr),*) => {
                $a $(.then($b))*
                    .map_with_span(|_, span: SpanType<'a>| lex::Node::chars(&span.context(), span.start()..span.end()))
            };
        }

        #[allow(unused_macros)]
        macro_rules! lex_zero_or_more {
            ($kind:ident, $expr:expr) => {
                lex_zero_or_more!($expr).map_with_span(|element, span: SpanType<'a>| {
                    lex::Node::named(&span.context(), kinds::Token::$kind, element)
                })
            };
            ($expr:expr) => {
                $expr
                    .repeated()
                    .map_with_span(|v, span: SpanType<'a>| lex::Node::sequence(&span.context(), v))
            };
        }

        #[allow(unused_macros)]
        macro_rules! lex_one_or_more {
            ($kind:ident, $expr:expr) => {
                lex_one_or_more!($expr).map_with_span(|element, span: SpanType<'a>| {
                    lex::Node::named(&span.context(), kinds::Token::$kind, element)
                })
            };
            ($expr:expr) => {
                $expr
                    .repeated()
                    .at_least(1)
                    .map_with_span(|v, span: SpanType<'a>| lex::Node::sequence(&span.context(), v))
            };
        }

        #[allow(unused_macros)]
        macro_rules! lex_repeated {
            ($kind:ident, $expr:expr, $min:literal, $max:literal) => {
                lex_repeated!($expr, $min, $max).map_with_span(|element, span: SpanType<'a>| {
                    lex::Node::named(&span.context(), kinds::Token::$kind, element)
                })
            };
            ($expr:expr, $min:literal, $max:literal) => {
                $expr
                    .repeated()
                    .at_least($min)
                    .at_most($max)
                    .map_with_span(|v, span: SpanType<'a>| lex::Node::sequence(&span.context(), v))
            };
        }

        #[allow(unused_macros)]
        macro_rules! lex_optional {
            ($expr:expr) => {
                $expr.or_not().map_with_span(|v, span: SpanType<'a>| {
                    v.unwrap_or_else(|| lex::Node::none(&span.context()))
                })
            };
        }

        #[allow(unused_macros)]
        macro_rules! lex_separated_by {
            ($kind:ident, $expr:expr, $separator:expr) => {
                lex_separated_by!($expr, $separator).map_with_span(|element, span: SpanType<'a>| {
                    lex::Node::named(&span.context(), kinds::Token::$kind, element)
                })
            };
            ($expr:expr, $separator:expr) => {
                $expr.then($separator.then($expr).repeated()).map_with_span(
                    |(first, rest), span: SpanType<'a>| {
                        let mut v = vec![first];
                        for (separator, expr) in rest {
                            v.push(separator);
                            v.push(expr);
                        }
                        lex::Node::sequence(&span.context(), v)
                    },
                )
            };
        }

        #[allow(unused_macros)]
        macro_rules! lex_trie {
            ( $($expr:expr),* ) => (
                choice::<_, ErrorType>(($($expr),*)).map_with_span(|kind, span: SpanType<'a>| lex::Node::named(&span.context(), kind, lex::Node::chars(&span.context(), span.start()..span.end())))
            )
        }

        #[allow(unused_macros)]
        macro_rules! trieleaf {
            ( $kind:ident, $string:literal ) => {
                just($string).to(kinds::Token::$kind)
            };
            ( $kind:ident ) => {
                empty().to(kinds::Token::$kind)
            };
        }

        #[allow(unused_macros)]
        macro_rules! trieprefix {
            ( $string:literal , [ $($expr:expr),* ] ) => (
                just($string).ignore_then(choice::<_, ErrorType>(($($expr),*)))
            )
        }

        #[allow(unused_macros)]
        macro_rules! trivia_terminal {
            ($kind:ident, $literal:literal) => {
                just($literal).map_with_span(|_, span: SpanType<'a>| {
                    cst::Node::trivia_token(
                        &span.context(),
                        span.start()..span.end(),
                        kinds::Token::$kind,
                    )
                })
            };
            ($kind:ident, $filter:expr) => {
                filter($filter).map_with_span(|_, span: SpanType<'a>| {
                    cst::Node::trivia_token(
                        &span.context(),
                        span.start()..span.end(),
                        kinds::Token::$kind,
                    )
                })
            };
        }

        #[allow(unused_macros)]
        macro_rules! trivia_token {
            ($token_rule:ident) => {
                $token_rule.clone().map_with_span(|token: &'a lex::Node<'a>, span: SpanType<'a>| {
                    if let lex::Node::Named(kind, element) = token {
                        cst::Node::trivia_token(&span.context(), element.range(), *kind)
                    } else {
                        unreachable!("a token rule should always return a named token, but rule {} returned {:?}", stringify!($token_rule), token)
                    }
                })
            };
        }

        #[allow(unused_macros)]
        macro_rules! trivia_trie {
            ( $($expr:expr),* ) => (
                choice::<_, ErrorType>(($($expr),*)).map_with_span(|kind, span: SpanType<'a>| cst::Node::trivia_token(&span.context(), span.start()..span.end(), kind))
            )
        }

        #[allow(unused_macros)]
        macro_rules! terminal {
            ($kind:ident, $literal:literal) => {
                leading_trivia_parser
                    .clone()
                    .then(
                        just($literal)
                            .map_with_span(|_, span: SpanType<'a>| span.start()..span.end()),
                    )
                    .then(trailing_trivia_parser.clone())
                    .map_with_span(
                        |((leading_trivia, range), trailing_trivia), span: SpanType<'a>| {
                            cst::Node::token(
                                &span.context(),
                                range,
                                kinds::Token::$kind,
                                leading_trivia,
                                trailing_trivia,
                            )
                        },
                    )
            };
            ($kind:ident, $filter:expr) => {
                leading_trivia_parser
                    .clone()
                    .then(
                        filter($filter)
                            .map_with_span(|_, span: SpanType<'a>| span.start()..span.end()),
                    )
                    .then(trailing_trivia_parser.clone())
                    .map_with_span(
                        |((leading_trivia, range), trailing_trivia), span: SpanType<'a>| {
                            cst::Node::token(
                                &span.context(),
                                range,
                                kinds::Token::$kind,
                                leading_trivia,
                                trailing_trivia,
                            )
                        },
                    )
            };
        }

        #[allow(unused_macros)]
        macro_rules! token {
            ($token_rule:ident) => {
                leading_trivia_parser.clone()
                    .then($token_rule.clone())
                    .then(trailing_trivia_parser.clone())
                    .map_with_span(|((leading_trivia, token), trailing_trivia): ((_, &'a lex::Node<'a>), _), span: SpanType<'a>| {
                        if let lex::Node::Named(kind, element) = *token {
                            cst::Node::token(&span.context(), element.range(), kind, leading_trivia, trailing_trivia)
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
            ( $kind:ident, $($expr:expr),*) => {
                choice::<_, ErrorType>(( $($expr),* ))
            };
            ( $($expr:expr),* ) => {
                choice::<_, ErrorType>(( $($expr),* ))
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

            ( @exp $head:expr , $($tail:expr),+ ) => {
                $head.then(seq!( @exp $($tail),+ ))
            };

            ( @exp $head:expr ) => {
                $head
            };

            /*
                (@args [], v, a, b, c, d)
                => (@args [v.0,], v.1, b, c, d)
                => (@args [v.0, v.1.0,], v.1.1, c, d)
                => (@args [v.0, v.1.0, v.1.1.0,], v.1.1.1, d)
                => vec![v.0, v.1.0, v.1.1.0, v.1.1.0, v1.1.1, ]
            */

            ( @args [ $($accum:expr,)* ] , $current:expr , $head:expr , $($tail:expr),+ ) => {
                seq!( @args [ $($accum,)* $current.0, ] , $current.1 , $($tail),+ )
            };

            ( @args [ $($accum:expr,)* ] , $current:expr , $head:expr ) => {
                vec![ $($accum,)* $current ]
            };

            //----------------------------------------------------------------------------------------

            ( $kind:ident, $($expr:expr),+ ) => {
                seq!( @exp $($expr),+ )
                    .map_with_span(|v, span: SpanType<'a>| cst::Node::rule(&span.context(), kinds::Rule::$kind, seq!( @args [] , v , $($expr),+ )))
            };

            ( $($expr:expr),+ ) => {
                seq!( @exp $($expr),+ )
                    .map_with_span(|v, span: SpanType<'a>| cst::Node::group(&span.context(), seq!( @args [] , v , $($expr),+ )))
            };
        }

        #[allow(unused_macros)]
        macro_rules! zero_or_more {
            ($kind:ident, $expr:expr) => {
                $expr
                    .repeated()
                    .map_with_span(|children, span: SpanType<'a>| {
                        cst::Node::rule(&span.context(), kinds::Rule::$kind, children)
                    })
            };
            ($expr:expr) => {
                $expr
                    .repeated()
                    .map_with_span(|children, span: SpanType<'a>| {
                        cst::Node::group(&span.context(), children)
                    })
            };
        }

        #[allow(unused_macros)]
        macro_rules! one_or_more {
            ($kind:ident, $expr:expr) => {
                $expr
                    .repeated()
                    .at_least(1)
                    .map_with_span(|children, span: SpanType<'a>| {
                        cst::Node::rule(&span.context(), kinds::Rule::$kind, children)
                    })
            };
            ($expr:expr) => {
                $expr
                    .repeated()
                    .at_least(1)
                    .map_with_span(|children, span: SpanType<'a>| {
                        cst::Node::group(&span.context(), children)
                    })
            };
        }

        #[allow(unused_macros)]
        macro_rules! repeated {
            ($kind:ident, $expr:expr, $min:literal, $max:literal) => {
                $expr.repeated().at_least($min).at_most($max).map_with_span(
                    |children, span: SpanType<'a>| {
                        cst::Node::rule(&span.context(), kinds::Rule::$kind, children)
                    },
                )
            };
            ($expr:expr, $min:literal, $max:literal) => {
                $expr.repeated().at_least($min).at_most($max).map_with_span(
                    |children, span: SpanType<'a>| cst::Node::group(&span.context(), children),
                )
            };
        }

        #[allow(unused_macros)]
        macro_rules! optional {
            ($expr:expr) => {
                $expr.or_not().map_with_span(|v, span: SpanType<'a>| {
                    v.unwrap_or_else(|| cst::Node::none(&span.context()))
                })
            };
        }

        #[allow(unused_macros)]
        macro_rules! separated_by {
            ($kind:ident, $expr:expr, $separator:expr) => {
                $expr.then($separator.then($expr).repeated()).map_with_span(
                    |(first, rest), span: SpanType<'a>| {
                        let mut v = vec![first];
                        for (separator, expr) in rest {
                            v.push(separator);
                            v.push(expr);
                        }
                        cst::Node::rule(&span.context(), kinds::Rule::$kind, v)
                    },
                )
            };
            ($expr:expr, $separator:expr) => {
                $expr.then($separator.then($expr).repeated()).map_with_span(
                    |(first, rest), span: SpanType<'a>| {
                        let mut v = vec![first];
                        for (separator, expr) in rest {
                            v.push(separator);
                            v.push(expr);
                        }
                        cst::Node::group(&span.context(), v)
                    },
                )
            };
        }

        #[allow(unused_macros)]
        macro_rules! left_associative_binary_expression {
            ($kind:ident, $next_sibling:expr, $operator:expr) => {
                $next_sibling
                    .clone()
                    .then($operator.then($next_sibling.clone()).repeated())
                    .map_with_span(|(first, rest), span: SpanType<'a>| {
                        if rest.is_empty() {
                            first
                        } else {
                            // a [ (X b) (Y c) (Z d) ] => { { { a X b } Y c } Z d }
                            rest.into_iter().fold(
                                first,
                                |left_operand, (operator, right_operand)| {
                                    cst::Node::rule(
                                        &span.context(),
                                        kinds::Rule::$kind,
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
                    .map_with_span(|(first, rest), span: SpanType<'a>| {
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
                                        &span.context(),
                                        kinds::Rule::$kind,
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
                    .map_with_span(|(mut operators, operand), span: SpanType<'a>| {
                        if operators.is_empty() {
                            operand
                        } else {
                            operators.reverse();
                            operators
                                .into_iter()
                                .fold(operand, |right_operand, operator| {
                                    cst::Node::rule(
                                        &span.context(),
                                        kinds::Rule::$kind,
                                        vec![operator, right_operand],
                                    )
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
                    .map_with_span(|(operand, operators), span: SpanType<'a>| {
                        if operators.is_empty() {
                            operand
                        } else {
                            operators
                                .into_iter()
                                .fold(operand, |left_operand, operator| {
                                    cst::Node::rule(
                                        &span.context(),
                                        kinds::Rule::$kind,
                                        vec![left_operand, operator],
                                    )
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
            ( $($expr:expr),* ) => (
                leading_trivia_parser.clone()
                    .then(choice::<_, ErrorType>(($($expr),*)).map_with_span(|kind, span: SpanType<'a>| (kind, span.start()..span.end())))
                    .then(trailing_trivia_parser.clone())
                    .map_with_span(|((leading_trivia, (kind, range)), trailing_trivia), span: SpanType<'a>| {
                        cst::Node::token(&span.context(), range, kind, leading_trivia, trailing_trivia)
                    })
            )
        }
    )
}

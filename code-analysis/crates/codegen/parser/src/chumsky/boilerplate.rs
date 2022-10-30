use proc_macro2::TokenStream;
use quote::quote;

pub fn mod_head() -> TokenStream {
    quote!(
        pub mod kinds;
        pub mod lex;
        pub mod cst;
        pub mod parse;
        pub mod language;
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
        use std::rc::Rc;

        use super::kinds;

        #[derive(Clone, Debug, PartialEq, Eq, Serialize)]
        pub enum Node {
            None,
            Chars(Range<usize>),
            Choice(usize, Rc<Node>),
            Sequence(Vec<Rc<Node>>),
            Named(kinds::Token, Rc<Node>),
        }

        impl Node {
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
        use std::rc::Rc;

        use super::kinds;

        #[derive(Clone, Debug, PartialEq, Eq, Serialize)]
        pub enum Node {
            None,
            Rule {
                kind: kinds::Rule,
                #[serde(skip_serializing_if = "Vec::is_empty")]
                children: Vec<Rc<Node>>,
            },
            Token {
                kind: kinds::Token,
                /// Range doesn't include the trivia
                range: Range<usize>,
                /// Only Trivia
                #[serde(skip_serializing_if = "Vec::is_empty")]
                trivia: Vec<Rc<Node>>,
            },
            /// For anonymous groups referenced from AST nodes i.e. `delimited_by`
            Group {
                #[serde(skip_serializing_if = "Vec::is_empty")]
                children: Vec<Rc<Node>>,
            }, // TODO: Error types
        }

        #[allow(unused_variables)]
        pub trait Visitor {
            fn visit_none(&mut self, node: &Rc<Node>) -> VisitorExitResponse {
                VisitorExitResponse::StepIn
            }

            fn enter_rule(
                &mut self,
                kind: kinds::Rule,
                children: &Vec<Rc<Node>>,
                node: &Rc<Node>,
            ) -> VisitorEntryResponse {
                VisitorEntryResponse::StepIn
            }

            fn exit_rule(
                &mut self,
                kind: kinds::Rule,
                children: &Vec<Rc<Node>>,
                node: &Rc<Node>,
            ) -> VisitorExitResponse {
                VisitorExitResponse::StepIn
            }

            fn enter_token(
                &mut self,
                kind: kinds::Token,
                range: &Range<usize>,
                trivia: &Vec<Rc<Node>>,
                node: &Rc<Node>,
            ) -> VisitorEntryResponse {
                VisitorEntryResponse::StepIn
            }

            fn exit_token(
                &mut self,
                kind: kinds::Token,
                range: &Range<usize>,
                trivia: &Vec<Rc<Node>>,
                node: &Rc<Node>,
            ) -> VisitorExitResponse {
                VisitorExitResponse::StepIn
            }

            fn enter_group(
                &mut self,
                children: &Vec<Rc<Node>>,
                node: &Rc<Node>,
            ) -> VisitorEntryResponse {
                VisitorEntryResponse::StepIn
            }

            fn exit_group(
                &mut self,
                children: &Vec<Rc<Node>>,
                node: &Rc<Node>,
            ) -> VisitorExitResponse {
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

        impl Visitable for Rc<Node> {
            fn visit<T: Visitor>(&self, visitor: &mut T) -> VisitorExitResponse {
                match self.as_ref() {
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

pub fn language_head() -> TokenStream {
    quote!(
        use std::rc::Rc;

        use chumsky::Parser;
        use semver::Version;

        use super::{cst, lex, parse::Parsers};

        pub struct Language {
            parsers: Parsers<'static>,
            version: Version,
        }

        impl Language {
            pub fn new(version: Version) -> Self {
                Self {
                    parsers: Parsers::new(&version),
                    version,
                }
            }

            pub fn version(&self) -> &Version {
                &self.version
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

        use super::kinds;
        use super::lex;
        use super::cst;

        mod factory {
            use std::ops::Range;
            use std::rc::Rc;

            use super::cst;
            use super::kinds;
            use super::lex;

            pub fn lex_none() -> Rc<lex::Node> {
                Rc::new(lex::Node::None)
            }

            pub fn lex_chars(range: Range<usize>) -> Rc<lex::Node> {
                Rc::new(lex::Node::Chars(range))
            }

            pub fn lex_sequence(elements: Vec<Rc<lex::Node>>) -> Rc<lex::Node> {
                Rc::new(if elements.is_empty() {
                    lex::Node::None
                } else {
                    lex::Node::Sequence(elements)
                })
            }

            pub fn lex_choice(number: usize, element: Rc<lex::Node>) -> Rc<lex::Node> {
                Rc::new(lex::Node::Choice(number, element))
            }

            pub fn lex_named(kind: kinds::Token, element: Rc<lex::Node>) -> Rc<lex::Node> {
                Rc::new(lex::Node::Named(kind, element))
            }

            pub fn cst_none() -> Rc<cst::Node> {
                Rc::new(cst::Node::None)
            }

            pub fn cst_rule(kind: kinds::Rule, children: Vec<Rc<cst::Node>>) -> Rc<cst::Node> {
                Rc::new(cst::Node::Rule { kind, children })
            }

            pub fn cst_trivia_token(range: Range<usize>, kind: kinds::Token) -> Rc<cst::Node> {
                Rc::new(cst::Node::Token {
                    range,
                    kind,
                    trivia: vec![],
                })
            }

            pub fn cst_token(
                range: Range<usize>,
                kind: kinds::Token,
                leading_trivia: Rc<cst::Node>,
                trailing_trivia: Rc<cst::Node>,
            ) -> Rc<cst::Node> {
                let mut trivia = vec![];
                if *leading_trivia != cst::Node::None {
                    trivia.push(leading_trivia)
                }
                if *trailing_trivia != cst::Node::None {
                    trivia.push(trailing_trivia)
                }
                Rc::new(cst::Node::Token {
                    range,
                    kind,
                    trivia,
                })
            }

            pub fn cst_group(children: Vec<Rc<cst::Node>>) -> Rc<cst::Node> {
                if children.is_empty() {
                    cst_none()
                } else {
                    Rc::new(cst::Node::Group { children })
                }
            }
        }

        pub type SpanType = Range<usize>;
        pub type ErrorType = Simple<char, SpanType>;
        pub type ParserType<'p, T> = BoxedParser<'p, char, T, ErrorType>;

        #[allow(dead_code)]
        fn difference<M, S, T>(minuend: M, subtrahend: S) -> impl Parser<char, T, Error = ErrorType>
        where
            M: Clone + Parser<char, T, Error = ErrorType>,
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
                    factory::lex_named(
                        kinds::Token::$kind,
                        factory::lex_chars(span.start()..span.end()),
                    )
                })
            };
            ($kind:ident, $filter:expr) => {
                filter($filter).map_with_span(|_, span: SpanType| {
                    factory::lex_named(
                        kinds::Token::$kind,
                        factory::lex_chars(span.start()..span.end()),
                    )
                })
            };
            ($literal:literal) => {
                just($literal)
                    .map_with_span(|_, span: SpanType| factory::lex_chars(span.start()..span.end()))
            };
            ($filter:expr) => {
                filter($filter)
                    .map_with_span(|_, span: SpanType| factory::lex_chars(span.start()..span.end()))
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
                lex_choice!($($expr),*).map(|element| factory::lex_named(kinds::Token::$kind, element))
            };
            ($($expr:expr),*) => {
                choice::<_, ErrorType>((
                    $($expr.map(|v| factory::lex_choice(0, v))),*
                ))
            };
        }

        #[allow(unused_macros)]
        macro_rules! lex_seq {
            ($kind:ident, $($expr:expr),*) => {
                lex_seq!($($expr),*).map(|element| factory::lex_named(kinds::Token::$kind, element))
            };
            // THIS IS WRONG - it should accumulate the elements like seq! does below
            ($a:expr, $($b:expr),*) => {
                $a $(.then($b))*
                    .map_with_span(|_, span: SpanType| factory::lex_chars(span.start()..span.end()))
            };
        }

        #[allow(unused_macros)]
        macro_rules! lex_zero_or_more {
            ($kind:ident, $expr:expr) => {
                lex_zero_or_more!($expr)
                    .map(|element| factory::lex_named(kinds::Token::$kind, element))
            };
            ($expr:expr) => {
                $expr.repeated().map(|v| factory::lex_sequence(v))
            };
        }

        #[allow(unused_macros)]
        macro_rules! lex_one_or_more {
            ($kind:ident, $expr:expr) => {
                lex_one_or_more!($expr)
                    .map(|element| factory::lex_named(kinds::Token::$kind, element))
            };
            ($expr:expr) => {
                $expr
                    .repeated()
                    .at_least(1)
                    .map(|v| factory::lex_sequence(v))
            };
        }

        #[allow(unused_macros)]
        macro_rules! lex_repeated {
            ($kind:ident, $expr:expr, $min:literal, $max:literal) => {
                lex_repeated!($expr, $min, $max)
                    .map(|element| factory::lex_named(kinds::Token::$kind, element))
            };
            ($expr:expr, $min:literal, $max:literal) => {
                $expr
                    .repeated()
                    .at_least($min)
                    .at_most($max)
                    .map(|v| factory::lex_sequence(v))
            };
        }

        #[allow(unused_macros)]
        macro_rules! lex_optional {
            ($expr:expr) => {
                $expr
                    .or_not()
                    .map(|v| v.unwrap_or_else(|| factory::lex_none()))
            };
        }

        #[allow(unused_macros)]
        macro_rules! lex_separated_by {
            ($kind:ident, $expr:expr, $separator:expr) => {
                lex_separated_by!($expr, $separator)
                    .map(|element| factory::lex_named(kinds::Token::$kind, element))
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
                        factory::lex_sequence(v)
                    })
            };
        }

        #[allow(unused_macros)]
        macro_rules! lex_trie {
            ($($expr:expr),* ) => (
                choice::<_, ErrorType>(($($expr),*)).map_with_span(|kind, span: SpanType|
                    factory::lex_named(kind, factory::lex_chars(span.start()..span.end())))
            )
        }

        #[allow(unused_macros)]
        macro_rules! trieleaf {
            ($kind:ident, $string:literal ) => {
                just($string).to(kinds::Token::$kind)
            };
            ($kind:ident ) => {
                empty().to(kinds::Token::$kind)
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
                    factory::cst_trivia_token(span.start()..span.end(), kinds::Token::$kind)
                })
            };
            ($kind:ident, $filter:expr) => {
                filter($filter).map_with_span(|_, span: SpanType| {
                    factory::cst_trivia_token(span.start()..span.end(), kinds::Token::$kind)
                })
            };
        }

        #[allow(unused_macros)]
        macro_rules! trivia_token {
            ($token_rule:ident) => {
                $token_rule.clone().map(|token: Rc<lex::Node>| {
                    if let lex::Node::Named(kind, element) = token.as_ref() {
                        factory::cst_trivia_token(element.range(), *kind)
                    } else {
                        unreachable!("a token rule should always return a named token, but rule {} returned {:?}", stringify!($token_rule), token)
                    }
                })
            };
        }

        #[allow(unused_macros)]
        macro_rules! trivia_trie {
            ($($expr:expr),* ) => (
                choice::<_, ErrorType>(($($expr),*)).map_with_span(|kind, span: SpanType| factory::cst_trivia_token(span.start()..span.end(), kind))
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
                        factory::cst_token(
                            range,
                            kinds::Token::$kind,
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
                        factory::cst_token(
                            range,
                            kinds::Token::$kind,
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
                    .map(|((leading_trivia, token), trailing_trivia): ((_, Rc<lex::Node>), _)| {
                        if let lex::Node::Named(kind, element) = token.as_ref() {
                            factory::cst_token(element.range(), *kind, leading_trivia, trailing_trivia)
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
                    .map(|v| factory::cst_rule(kinds::Rule::$kind, seq!(@args [] , v , $($expr),+ )))
            };

            ($($expr:expr),+ ) => {
                seq!(@exp $($expr),+ )
                    .map(|v| factory::cst_group(seq!(@args [] , v , $($expr),+ )))
            };
        }

        #[allow(unused_macros)]
        macro_rules! zero_or_more {
            ($kind:ident, $expr:expr) => {
                $expr
                    .repeated()
                    .map(|children| factory::cst_rule(kinds::Rule::$kind, children))
            };
            ($expr:expr) => {
                $expr
                    .repeated()
                    .map(|children| factory::cst_group(children))
            };
        }

        #[allow(unused_macros)]
        macro_rules! one_or_more {
            ($kind:ident, $expr:expr) => {
                $expr
                    .repeated()
                    .at_least(1)
                    .map(|children| factory::cst_rule(kinds::Rule::$kind, children))
            };
            ($expr:expr) => {
                $expr
                    .repeated()
                    .at_least(1)
                    .map(|children| factory::cst_group(children))
            };
        }

        #[allow(unused_macros)]
        macro_rules! repeated {
            ($kind:ident, $expr:expr, $min:literal, $max:literal) => {
                $expr
                    .repeated()
                    .at_least($min)
                    .at_most($max)
                    .map(|children| factory::cst_rule(kinds::Rule::$kind, children))
            };
            ($expr:expr, $min:literal, $max:literal) => {
                $expr
                    .repeated()
                    .at_least($min)
                    .at_most($max)
                    .map(|children| factory::cst_group(children))
            };
        }

        #[allow(unused_macros)]
        macro_rules! optional {
            ($expr:expr) => {
                $expr
                    .or_not()
                    .map(|v| v.unwrap_or_else(|| factory::cst_none()))
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
                        factory::cst_rule(kinds::Rule::$kind, v)
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
                        factory::cst_group(v)
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
                                    factory::cst_rule(
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
                                    factory::cst_rule(
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
                    .map(|(mut operators, operand)| {
                        if operators.is_empty() {
                            operand
                        } else {
                            operators.reverse();
                            operators
                                .into_iter()
                                .fold(operand, |right_operand, operator| {
                                    factory::cst_rule(
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
                    .map(|(operand, operators)| {
                        if operators.is_empty() {
                            operand
                        } else {
                            operators
                                .into_iter()
                                .fold(operand, |left_operand, operator| {
                                    factory::cst_rule(
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
            ($($expr:expr),* ) => (
                leading_trivia_parser.clone()
                    .then(choice::<_, ErrorType>(($($expr),*)).map_with_span(|kind, span: SpanType| (kind, span.start()..span.end())))
                    .then(trailing_trivia_parser.clone())
                    .map(|((leading_trivia, (kind, range)), trailing_trivia)| {
                        factory::cst_token(range, kind, leading_trivia, trailing_trivia)
                    })
            )
        }
    )
}

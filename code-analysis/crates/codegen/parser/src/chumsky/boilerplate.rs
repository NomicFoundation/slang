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
            #[inline] pub fn none() -> NodeRef { Box::new(Node::None) }
            #[inline] pub fn chars(range: Range<usize>) -> NodeRef { Box::new(Node::Chars(range)) }
            #[inline] pub fn sequence(elements: Vec<NodeRef>) -> NodeRef { Box::new(Node::Sequence(elements)) }
            #[inline] pub fn choice(number: usize, element: NodeRef) -> NodeRef { Box::new(Node::Choice(number, element)) }
            #[inline] pub fn named(kind: kinds::Token, element: NodeRef) -> NodeRef { Box::new(Node::Named(kind, element)) }

            pub fn range(&self) -> Range<usize> {
                match self {
                    Node::None => 0..0,
                    Node::Chars(range) => range.clone(),
                    Node::Choice(_, element) => element.range(),
                    Node::Sequence(elements) => elements[0].range().start..elements[elements.len() - 1].range().end,
                    Node::Named(_, element) => element.range(),
                }
            }
        }
    )
}

pub fn cst_head() -> TokenStream {
    quote!(
        use std::rc::Rc;
        use std::ops::Range;
        use serde::Serialize;

        use super::kinds;

        #[derive(Clone, Debug, PartialEq, Eq, Serialize)]
        pub enum Node {
            None,
            Rule {
                kind: kinds::Rule,
                children: Vec<NodeRef>,
            },
            Token {
                kind: kinds::Token,
                /// Range doesn't include the trivia
                range: Range<usize>,
                /// Only Trivia
                #[serde(skip_serializing_if = "Vec::is_empty")]
                trivia: Vec<NodeRef>,
            },
            /// For anonymous groups referenced from AST nodes i.e. `delimited_by`
            Group {
                #[serde(skip_serializing_if = "Vec::is_empty")]
                children: Vec<NodeRef>,
            }
            // TODO: Error types
        }

        pub type NodeRef = Rc<Node>;

        #[allow(unused_variables)]
        pub trait Visitor {
            fn enter_rule(&mut self, kind: &kinds::Rule, children: &Vec<NodeRef>, node: &NodeRef) -> VisitorEntryResponse {
                VisitorEntryResponse::StepIn
            }
            fn exit_rule(&mut self, kind: &kinds::Rule, children: &Vec<NodeRef>, node: &NodeRef) -> VisitorExitResponse {
                VisitorExitResponse::StepIn
            }
            fn enter_token(&mut self, kind: &kinds::Token, range: &Range<usize>, trivia: &Vec<NodeRef>, node: &NodeRef) -> VisitorEntryResponse {
                VisitorEntryResponse::StepIn
            }
            fn exit_token(&mut self, kind: &kinds::Token, range: &Range<usize>, trivia: &Vec<NodeRef>, node: &NodeRef) -> VisitorExitResponse {
                VisitorExitResponse::StepIn
            }
            fn enter_group(&mut self, children: &Vec<NodeRef>, node: &NodeRef) -> VisitorEntryResponse {
                VisitorEntryResponse::StepIn
            }
            fn exit_group(&mut self, children: &Vec<NodeRef>, node: &NodeRef) -> VisitorExitResponse {
                VisitorExitResponse::StepIn
            }
            fn visit_none(&mut self, node: &NodeRef) -> VisitorExitResponse {
                VisitorExitResponse::StepIn
            }
        }

        pub enum VisitorEntryResponse {
            Quit,
            StepIn,
            StepOver
        }

        pub enum VisitorExitResponse {
            Quit,
            StepIn
        }

        pub trait Visitable {
            fn visit<T: Visitor>(&self, visitor: &mut T) -> VisitorExitResponse;
        }

        impl Node {
            #[inline] pub fn none() -> NodeRef {
                Rc::new(Self::None)
            }
            #[inline] pub fn rule(kind: kinds::Rule, children: Vec<NodeRef>) -> NodeRef {
                Rc::new(Self::Rule { kind, children })
            }
            #[inline] pub fn trivia_token(range: Range<usize>, kind: kinds::Token) -> NodeRef {
                Rc::new(Self::Token { range, kind, trivia: vec![] })
            }
            #[inline] pub fn token(range: Range<usize>, kind: kinds::Token, leading_trivia: NodeRef, trailing_trivia: NodeRef) -> NodeRef {
                let mut trivia = vec![];
                if *leading_trivia != Node::None { trivia.push(leading_trivia) }
                if *trailing_trivia != Node::None { trivia.push(trailing_trivia) }
                Rc::new(Self::Token { range, kind, trivia })
            }
            #[inline] pub fn group(children: Vec<NodeRef>) -> NodeRef {
                Rc::new(Self::Group { children })
            }
        }

        impl Visitable for NodeRef {
            fn visit<T: Visitor>(&self, visitor: &mut T) -> VisitorExitResponse {
                match self.as_ref() {
                    Node::None => {
                        visitor.visit_none(self)
                    }
                    Node::Rule { kind, children } => {
                        match visitor.enter_rule(kind, children, self) {
                            VisitorEntryResponse::Quit => return VisitorExitResponse::Quit,
                            VisitorEntryResponse::StepOver => {}
                            VisitorEntryResponse::StepIn => {
                                for child in children {
                                    match child.visit(visitor) {
                                        VisitorExitResponse::Quit => return VisitorExitResponse::Quit,
                                        VisitorExitResponse::StepIn => {}
                                    }
                                }
                            }
                        }
                        visitor.exit_rule(kind, children, self)
                    }
                    Node::Token { kind, range, trivia } => {
                        match visitor.enter_token(kind, range, trivia, self) {
                            VisitorEntryResponse::Quit => return VisitorExitResponse::Quit,
                            VisitorEntryResponse::StepOver => {},
                            VisitorEntryResponse::StepIn => {
                                for child in trivia {
                                    match child.visit(visitor) {
                                        VisitorExitResponse::Quit => return VisitorExitResponse::Quit,
                                        VisitorExitResponse::StepIn => {}
                                    }
                                }
                            }
                        }
                        visitor.exit_token(kind, range, trivia, self)
                    }
                    Node::Group { children } => {
                        match visitor.enter_group(children, self) {
                            VisitorEntryResponse::Quit => return VisitorExitResponse::Quit,
                            VisitorEntryResponse::StepOver => {}
                            VisitorEntryResponse::StepIn => {
                                for child in children {
                                    match child.visit(visitor) {
                                        VisitorExitResponse::Quit => return VisitorExitResponse::Quit,
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

        use super::kinds;
        use super::lex;
        use super::cst;
        #[allow(unused_imports)]
        use super::ast;

        pub type ParserType<T> = BoxedParser<'static, char, T, ErrorType>;
        pub type ErrorType = Simple<char>;

        #[allow(dead_code)]
        fn difference<M, S, T>(minuend: M, subtrahend: S) -> impl Parser<char, T, Error = ErrorType>
        where
            M: Clone + Parser<char, T, Error = ErrorType>,
            S: Parser<char, T, Error = ErrorType>,
        {
            // TODO This could be much more efficient if we were able
            // to conditionally rewind
            let minuend_end = minuend.clone().map_with_span(|_, span| span.end).rewind();
            let subtrahend_end = subtrahend
                .map_with_span(|_, span| span.end)
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
                just($literal).map_with_span(|_, span| {
                    lex::Node::named(kinds::Token::$kind, lex::Node::chars(span))
                })
            };
            ($kind:ident, $filter:expr) => {
                filter($filter).map_with_span(|_, span| {
                    lex::Node::named(kinds::Token::$kind, lex::Node::chars(span))
                })
            };
            ($literal:literal) => {
                just($literal).map_with_span(|_, span| lex::Node::chars(span))
            };
            ($filter:expr) => {
                filter($filter).map_with_span(|_, span| lex::Node::chars(span))
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
                lex_choice!($($expr),*).map(|element| lex::Node::named(kinds::Token::$kind, element))
            };
            ($($expr:expr),*) => {
                choice::<_, ErrorType>((
                    $($expr.map(|v| lex::Node::choice(0, v))),*
                ))
            };
        }

        #[allow(unused_macros)]
        macro_rules! lex_seq {
            ($kind:ident, $($expr:expr),*) => {
                lex_seq!($($expr),*).map(|element| lex::Node::named(kinds::Token::$kind, element))
            };
            // THIS IS WRONG - it should accumulate the elements like seq! does below
            ($a:expr, $($b:expr),*) => {
                $a $(.then($b))*
                    .map_with_span(|_, span| lex::Node::chars(span))
            };
        }

        #[allow(unused_macros)]
        macro_rules! lex_zero_or_more {
            ($kind:ident, $expr:expr) => {
                lex_zero_or_more!($expr)
                    .map(|element| lex::Node::named(kinds::Token::$kind, element))
            };
            ($expr:expr) => {
                $expr.repeated().map(|v| lex::Node::sequence(v))
            };
        }

        #[allow(unused_macros)]
        macro_rules! lex_one_or_more {
            ($kind:ident, $expr:expr) => {
                lex_one_or_more!($expr)
                    .map(|element| lex::Node::named(kinds::Token::$kind, element))
            };
            ($expr:expr) => {
                $expr.repeated().at_least(1).map(|v| lex::Node::sequence(v))
            };
        }

        #[allow(unused_macros)]
        macro_rules! lex_repeated {
            ($kind:ident, $expr:expr, $min:literal, $max:literal) => {
                lex_repeated!($expr, $min, $max)
                    .map(|element| lex::Node::named(kinds::Token::$kind, element))
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
                $expr
                    .or_not()
                    .map(|v| v.unwrap_or_else(|| lex::Node::none()))
            };
        }

        #[allow(unused_macros)]
        macro_rules! lex_separated_by {
            ($kind:ident, $expr:expr, $separator:expr) => {
                lex_separated_by!($expr, $separator)
                    .map(|element| lex::Node::named(kinds::Token::$kind, element))
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
            ( $($expr:expr),* ) => (
                choice::<_, ErrorType>(($($expr),*)).map_with_span(|kind, span| lex::Node::named(kind, lex::Node::chars(span)))
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
                just($literal)
                    .map_with_span(|_, range| cst::Node::trivia_token(range, kinds::Token::$kind))
            };
            ($kind:ident, $filter:expr) => {
                filter($filter)
                    .map_with_span(|_, range| cst::Node::trivia_token(span, kinds::Token::$kind))
            };
        }

        #[allow(unused_macros)]
        macro_rules! trivia_token {
            ($token_rule:ident) => {
                $token_rule.clone().map(|token: lex::NodeRef| {
                    if let lex::Node::Named(kind, element) = *token {
                        cst::Node::trivia_token(element.range(), kind)
                    } else {
                        unreachable!("a token rule should always return a named token, but rule {} returned {:?}", stringify!($token_rule), token)
                    }
                })
            };
        }

        #[allow(unused_macros)]
        macro_rules! trivia_trie {
            ( $($expr:expr),* ) => (
                choice::<_, ErrorType>(($($expr),*)).map_with_span(|kind, span| cst::Node::trivia_token(span, kind))
            )
        }

        #[allow(unused_macros)]
        macro_rules! terminal {
            ($kind:ident, $literal:literal) => {
                leading_trivia_parser
                    .clone()
                    .then(just($literal).map_with_span(|_, span| span))
                    .then(trailing_trivia_parser.clone())
                    .map(|((leading_trivia, range), trailing_trivia)| {
                        cst::Node::token(
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
                    .then(filter($filter).map_with_span(|_, span| span))
                    .then(trailing_trivia_parser.clone())
                    .map(|((leading_trivia, span), trailing_trivia)| {
                        cst::Node::token(span, kinds::Token::$kind, leading_trivia, trailing_trivia)
                    })
            };
        }

        #[allow(unused_macros)]
        macro_rules! token {
            ($token_rule:ident) => {
                leading_trivia_parser.clone()
                    .then($token_rule.clone())
                    .then(trailing_trivia_parser.clone())
                    .map(|((leading_trivia, token), trailing_trivia): ((_, lex::NodeRef), _)| {
                        if let lex::Node::Named(kind, element) = *token {
                            cst::Node::token(element.range(), kind, leading_trivia, trailing_trivia)
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
                    .map(|v| cst::Node::rule(kinds::Rule::$kind, seq!( @args [] , v , $($expr),+ )))
            };

            ( $($expr:expr),+ ) => {
                seq!( @exp $($expr),+ )
                    .map(|v| cst::Node::group(seq!( @args [] , v , $($expr),+ )))
            };
        }

        #[allow(unused_macros)]
        macro_rules! zero_or_more {
            ($kind:ident, $expr:expr) => {
                $expr
                    .repeated()
                    .map(|children| cst::Node::rule(kinds::Rule::$kind, children))
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
                    .map(|children| cst::Node::rule(kinds::Rule::$kind, children))
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
                    .map(|children| cst::Node::rule(kinds::Rule::$kind, children))
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
                $expr
                    .or_not()
                    .map(|v| v.unwrap_or_else(|| cst::Node::none()))
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
                        cst::Node::rule(kinds::Rule::$kind, v)
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
                                    cst::Node::rule(
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
                                    cst::Node::rule(
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
                                    cst::Node::rule(
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
                    .then(choice::<_, ErrorType>(($($expr),*)).map_with_span(|kind, span| (kind, span)))
                    .then(trailing_trivia_parser.clone())
                    .map(|((leading_trivia, (kind, span)), trailing_trivia)| {
                        cst::Node::token(span, kind, leading_trivia, trailing_trivia)
                    })
            )
        }
    )
}

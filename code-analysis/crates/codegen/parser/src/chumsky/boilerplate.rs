use proc_macro2::TokenStream;
use quote::quote;

pub fn r#mod() -> TokenStream {
    quote!(
        pub mod cst;
        pub mod cst_parser;
        pub mod cst_parser_impl;

        pub mod ast;
        pub mod ast_impl;
        pub mod ast_parser;
        pub mod ast_parser_impl;
    )
}

pub fn cst() -> TokenStream {
    quote!(
        use std::rc::Rc;
        use serde::{Serialize, Deserialize};

        #[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
        pub enum Node {
            None,
            AnonymousRule(Vec<NodeRef>),
            AnonymousToken(usize),
            Rule {
                kind: RuleKind,
                children: Vec<NodeRef>,
            },
            Token {
                kind: TokenKind,
                children: Vec<NodeRef>,
            },
            TokenPart {
                kind: TokenPartKind,
                length: usize,
            },
        }

        impl Node {
            pub fn new_none() -> NodeRef {
                Rc::new(Self::None)
            }

            pub fn new_rule(kind: RuleKind, children: Vec<NodeRef>) -> NodeRef {
                Rc::new(Self::Rule { kind, children })
            }

            pub fn new_token(kind: TokenKind, children: Vec<NodeRef>) -> NodeRef {
                Rc::new(Self::Token { kind, children })
            }

            pub fn new_token_part(kind: TokenPartKind, length: usize) -> NodeRef {
                Rc::new(Self::TokenPart { kind, length })
            }

            pub fn new_anonymous_rule(children: Vec<NodeRef>) -> NodeRef {
                if children.is_empty() {
                    Self::new_none()
                } else if children.len() == 1 {
                    children[0].clone()
                } else {
                    Rc::new(Self::AnonymousRule(children))
                }
            }

            pub fn new_anonymous_token(length: usize) -> NodeRef {
                if length == 0 {
                    Self::new_none()
                } else {
                    Rc::new(Self::AnonymousToken(length))
                }
            }

            pub fn new_with_trivia(((leading_trivia, token), trailing_trivia): ((NodeRef, NodeRef), NodeRef)) -> NodeRef {
                match (*leading_trivia == Self::None, *trailing_trivia == Self::None) {
                    (true, true) => token,
                    (true, false) => Rc::new(Self::AnonymousRule(vec![ token, trailing_trivia ])),
                    (false, true) => Rc::new(Self::AnonymousRule(vec![ leading_trivia, token ])),
                    (false, false) => Rc::new(Self::AnonymousRule(vec![ leading_trivia, token, trailing_trivia ])),
                }
            }
        }

        pub type NodeRef = Rc<Node>;

        pub enum VisitorResult {
            Continue,
            Skip,
            Quit,
        }

        pub trait Visitor {
            fn enter_rule(&mut self, _kind: RuleKind) -> VisitorResult {
                VisitorResult::Continue
            }

            fn leave_rule(&mut self, _kind: RuleKind) -> VisitorResult {
                VisitorResult::Continue
            }

            fn enter_token(&mut self, _kind: TokenKind, _length: usize) -> VisitorResult {
                VisitorResult::Continue
            }

            fn leave_token(&mut self, _kind: TokenKind, _length: usize) -> VisitorResult {
                VisitorResult::Continue
            }

            fn token_part(&mut self, _kind: TokenPartKind, _length: usize) -> VisitorResult {
                VisitorResult::Continue
            }
        }
    )
}

pub fn ast() -> TokenStream {
    quote!(
        #[allow(unused_imports)]
        use serde::{Serialize, Deserialize};

        pub trait DefaultTest {
            fn is_default(&self) -> bool {
                false
            }
        }

        #[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
        pub struct FixedSizeTerminal<const N: usize>();

        #[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
        pub struct FixedSizeTerminalWithTrivia<const N: usize> {
            #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
            pub leading_trivia: LeadingTrivia,
            #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
            pub terminal: FixedSizeTerminal<N>,
            #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
            pub trailing_trivia: TrailingTrivia,
        }

        #[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
        pub struct VariableSizeTerminal(pub usize);

        #[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
        pub struct VariableSizeTerminalWithTrivia {
            #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
            pub leading_trivia: LeadingTrivia,
            #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
            pub terminal: VariableSizeTerminal,
            #[serde(default, skip_serializing_if = "DefaultTest::is_default")]
            pub trailing_trivia: TrailingTrivia,
        }
    )
}

pub fn ast_impl() -> TokenStream {
    quote!(
        use super::ast::*;

        impl<T: DefaultTest> DefaultTest for Box<T> {
            fn is_default(&self) -> bool {
                self.as_ref().is_default()
            }
        }

        impl<T> DefaultTest for Vec<T> {
            fn is_default(&self) -> bool {
              self.is_empty()
            }
        }

        impl<T> DefaultTest for Option<T> {
            fn is_default(&self) -> bool {
              self.is_none()
            }
        }
        impl DefaultTest for () {
            fn is_default(&self) -> bool {
              true
            }
        }

        impl DefaultTest for VariableSizeTerminal {
            fn is_default(&self) -> bool {
                self.0 == 0
            }
        }

        impl DefaultTest for VariableSizeTerminalWithTrivia {
            fn is_default(&self) -> bool {
                self.terminal.is_default() && self.leading_trivia.is_default() && self.trailing_trivia.is_default()
            }
        }

        impl<const N: usize> DefaultTest for FixedSizeTerminal<N> {
            fn is_default(&self) -> bool {
                true
            }
        }

        impl<const N: usize> DefaultTest for FixedSizeTerminalWithTrivia<N> {
            fn is_default(&self) -> bool {
                self.leading_trivia.is_default() && self.trailing_trivia.is_default()
            }
        }
    )
}

pub fn cst_parser() -> TokenStream {
    quote!(
        use chumsky::prelude::{Simple, BoxedParser};

        pub type ErrorType = Simple<char>;
        pub type ParserType<T> = BoxedParser<'static, char, T, ErrorType>;

        use super::cst::*;
    )
}

pub fn cst_parser_impl() -> TokenStream {
    quote!(
        use chumsky::Parser;
        use chumsky::prelude::*;
        use chumsky::primitive::Just;

        use super::cst::*;
        use super::cst_parser::*;

        #[allow(dead_code)]
        fn repetition_mapper((e, es): (NodeRef, Vec<(NodeRef, NodeRef)>)) -> Vec<NodeRef> {
            let mut result = vec![e];
            for (s, e) in es.into_iter() {
                result.push(s);
                result.push(e);
            }
            result
        }

        #[allow(dead_code)]
        fn difference<M, S>(
            minuend: M,
            subtrahend: S,
        ) -> impl Parser<char, NodeRef, Error = ErrorType>
        where
            M: Clone + Parser<char, NodeRef, Error = ErrorType>,
            S: Parser<char, NodeRef, Error = ErrorType>,
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

        #[allow(dead_code)]
        #[inline]
        fn terminal(str: &str) -> Just<char, &str, ErrorType> {
            just(str)
        }
    )
}

pub fn ast_parser() -> TokenStream {
    quote!(
        use chumsky::prelude::{Simple, BoxedParser};

        pub type ErrorType = Simple<char>;
        pub type ParserType<T> = BoxedParser<'static, char, T, ErrorType>;

        use super::ast::*;
    )
}

pub fn ast_parser_impl() -> TokenStream {
    quote!(
        use chumsky::Parser;
        use chumsky::prelude::*;
        use chumsky::primitive::Just;

        use super::ast::*;
        use super::ast_parser::*;

        #[allow(dead_code)]
        fn repetition_mapper<E, S>((e, es): (E, Vec<(S, E)>)) -> (Vec<E>, Vec<S>) {
            let mut elements = vec![e];
            let mut separators = vec![];
            for (s, e) in es.into_iter() {
                separators.push(s);
                elements.push(e);
            }
            (elements, separators)
        }

        #[allow(dead_code)]
        fn difference<M, MO, S, SO>(
            minuend: M,
            subtrahend: S,
        ) -> impl Parser<char, MO, Error = ErrorType>
        where
            M: Clone + Parser<char, MO, Error = ErrorType>,
            S: Parser<char, SO, Error = ErrorType>,
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

        #[allow(dead_code)]
        #[inline]
        fn terminal(str: &str) -> Just<char, &str, ErrorType> {
            just(str)
        }
    )
}

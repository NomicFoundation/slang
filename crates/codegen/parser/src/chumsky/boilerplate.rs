use proc_macro2::TokenStream;
use quote::quote;

pub fn lex_head() -> TokenStream {
    quote!(
        use std::ops::Range;
        use serde::Serialize;
        use std::rc::Rc;

        use super::kinds::TokenKind;

        #[derive(Clone, Debug, PartialEq, Eq, Serialize)]
        pub enum Node {
            Chars(Range<usize>),
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

            pub fn named(kind: TokenKind, element: Option<Rc<Self>>) -> Option<Rc<Self>> {
                element.map(|e| Rc::new(Self::Named(kind, e)))
            }

            pub fn range(&self) -> Range<usize> {
                match self {
                    Node::Chars(range) => range.clone(),
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
        }

        impl Node {
            pub fn rule(kind: RuleKind, children: Vec<Option<Rc<Self>>>) -> Option<Rc<Self>> {
                let children: Vec<_> = children.into_iter().filter_map(|e| e).collect();
                if children.is_empty() { return None }
                if children.len() == 1 {
                    if let Self::Group { children } = children[0].as_ref() {
                        return Some(Rc::new(Self::Rule { kind, children: children.clone() }))
                    }
                }
                return Some(Rc::new(Self::Rule { kind, children }))
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
        use chumsky::Parser as ChumskyParser;
        #[allow(deprecated)]
        use chumsky::debug::{Debugger, Verbose, Silent};
        use chumsky::error::{Located};
        use chumsky::Stream;
        use chumsky::Error;
        use chumsky::prelude::*;
        use semver::Version;
        use std::ops::Range;
        use std::rc::Rc;
        use std::collections::BTreeMap;

        use super::kinds::*;
        use super::lex;
        use super::cst;
        use super::language::Parser;

        pub type SpanType = Range<usize>;
        pub type ErrorType = Simple<char, SpanType>;
        pub type LocatedType = Located<char, ErrorType>;

        type PResult<O> = (
            Vec<LocatedType>,
            Result<(O, Option<LocatedType>), LocatedType>,
        );

        type StreamOf<'a> = Stream<'a, char, SpanType>;

        #[derive(Copy, Clone)]
        struct Difference<M, S> {
            minuend: M,
            subtrahend: S,
        }

        impl<
                O,
                M: ChumskyParser<char, O, Error = ErrorType> + Clone,
                S: ChumskyParser<char, O, Error = ErrorType> + Clone,
            > ChumskyParser<char, O> for Difference<M, S>
        {
            type Error = ErrorType;

            #[allow(deprecated)]
            fn parse_inner<D: Debugger>(
                &self,
                debugger: &mut D,
                stream: &mut StreamOf,
            ) -> PResult<O> {
                let start_position = stream.save();
                #[allow(deprecated)]
                match debugger.invoke(&self.minuend, stream) {
                    result @ (_, Ok((_, _))) => {
                        let end_position = stream.save();
                        stream.revert(start_position);
                        #[allow(deprecated)]
                        match debugger.invoke(&self.subtrahend, stream) {
                            (_, Ok(_)) if end_position == stream.save() => {
                                stream.revert(start_position);
                                let (at, span, found) = stream.next();
                                stream.revert(start_position);
                                return (
                                    Vec::new(),
                                    Err(Located::at(
                                        at,
                                        ErrorType::expected_input_found(span, Vec::new(), found),
                                    )),
                                );
                            }
                            _ => {
                                stream.revert(end_position);
                                return result;
                            }
                        }
                    }
                    result => return result,
                }
            }

            fn parse_inner_verbose(&self, d: &mut Verbose, s: &mut StreamOf) -> PResult<O> {
                #[allow(deprecated)]
                self.parse_inner(d, s)
            }
            fn parse_inner_silent(&self, d: &mut Silent, s: &mut StreamOf) -> PResult<O> {
                #[allow(deprecated)]
                self.parse_inner(d, s)
            }
        }

        #[allow(dead_code)]
        fn difference<M, S, O>(minuend: M, subtrahend: S) -> Difference<M, S>
        where
            M: ChumskyParser<char, O, Error = ErrorType> + Clone,
            S: ChumskyParser<char, O, Error = ErrorType>,
        {
            Difference {
                minuend,
                subtrahend,
            }
        }

        #[allow(unused_macros)]
        macro_rules! declare_rule {
            ($kind:ident) => {
                #[allow(non_snake_case)]
                let mut $kind =
                    Recursive::<'static, char, Option<Rc<cst::Node>>, ErrorType>::declare();
            };
        }

        #[allow(unused_macros)]
        macro_rules! declare_token {
            ($kind:ident) => {
                #[allow(non_snake_case)]
                let mut $kind =
                    Recursive::<'static, char, Option<Rc<lex::Node>>, ErrorType>::declare();
            };
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

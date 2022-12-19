use proc_macro2::TokenStream;
use quote::quote;

use super::boilerplate;

pub fn mod_head() -> TokenStream {
    quote!(
        pub mod kinds;
        pub mod lex;
        pub mod cst;
        pub mod parse;
        pub mod language;
    )
}

pub fn cst_visitor_head() -> TokenStream {
    quote!(
        #[allow(unused_variables)]
        pub trait Visitor<E> {
            fn enter_rule(
                &mut self,
                kind: RuleKind,
                children: &Vec<Rc<Node>>,
                node: &Rc<Node>,
                path: &Vec<Rc<Node>>,
            ) -> Result<VisitorEntryResponse, E> {
                Ok(VisitorEntryResponse::StepIn)
            }

            fn exit_rule(
                &mut self,
                kind: RuleKind,
                children: &Vec<Rc<Node>>,
                node: &Rc<Node>,
                path: &Vec<Rc<Node>>,
            ) -> Result<VisitorExitResponse, E> {
                Ok(VisitorExitResponse::StepIn)
            }

            fn enter_token(
                &mut self,
                kind: TokenKind,
                lex_node: &Rc<lex::Node>,
                trivia: &Vec<Rc<Node>>,
                node: &Rc<Node>,
                path: &Vec<Rc<Node>>,
            ) -> Result<VisitorEntryResponse, E> {
                Ok(VisitorEntryResponse::StepIn)
            }

            fn exit_token(
                &mut self,
                kind: TokenKind,
                lex_node: &Rc<lex::Node>,
                trivia: &Vec<Rc<Node>>,
                node: &Rc<Node>,
                path: &Vec<Rc<Node>>,
            ) -> Result<VisitorExitResponse, E> {
                Ok(VisitorExitResponse::StepIn)
            }

            fn enter_group(
                &mut self,
                children: &Vec<Rc<Node>>,
                node: &Rc<Node>,
                path: &Vec<Rc<Node>>,
            ) -> Result<VisitorEntryResponse, E> {
                Ok(VisitorEntryResponse::StepIn)
            }

            fn exit_group(
                &mut self,
                children: &Vec<Rc<Node>>,
                node: &Rc<Node>,
                path: &Vec<Rc<Node>>,
            ) -> Result<VisitorExitResponse, E> {
                Ok(VisitorExitResponse::StepIn)
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

        pub trait Visitable<T: Visitor<E>, E> {
            fn visit(&self, visitor: &mut T) -> Result<VisitorExitResponse, E>;
            fn visit_with_path(
                &self,
                visitor: &mut T,
                path: &mut Vec<Rc<Node>>,
            ) -> Result<VisitorExitResponse, E>;
        }

        impl<T: Visitor<E>, E> Visitable<T, E> for Rc<Node> {
            fn visit(&self, visitor: &mut T) -> Result<VisitorExitResponse, E> {
                self.visit_with_path(visitor, &mut Vec::new())
            }

            fn visit_with_path(
                &self,
                visitor: &mut T,
                path: &mut Vec<Rc<Node>>,
            ) -> Result<VisitorExitResponse, E> {
                match self.as_ref() {
                    Node::Rule { kind, children } => {
                        match visitor.enter_rule(*kind, children, self, path)? {
                            VisitorEntryResponse::Quit => return Ok(VisitorExitResponse::Quit),
                            VisitorEntryResponse::StepOver => {}
                            VisitorEntryResponse::StepIn => {
                                path.push(self.clone());
                                for child in children {
                                    match child.visit_with_path(visitor, path)? {
                                        VisitorExitResponse::Quit => {
                                            path.pop();
                                            return Ok(VisitorExitResponse::Quit);
                                        }
                                        VisitorExitResponse::StepIn => {}
                                    }
                                }
                                path.pop();
                            }
                        }
                        visitor.exit_rule(*kind, children, self, path)
                    }
                    Node::Token {
                        kind,
                        lex_node,
                        trivia,
                    } => {
                        match visitor.enter_token(*kind, lex_node, trivia, self, path)? {
                            VisitorEntryResponse::Quit => return Ok(VisitorExitResponse::Quit),
                            VisitorEntryResponse::StepOver => {}
                            VisitorEntryResponse::StepIn => {
                                path.push(self.clone());
                                for child in trivia {
                                    match child.visit_with_path(visitor, path)? {
                                        VisitorExitResponse::Quit => {
                                            path.pop();
                                            return Ok(VisitorExitResponse::Quit);
                                        }
                                        VisitorExitResponse::StepIn => {}
                                    }
                                }
                                path.pop();
                            }
                        }
                        visitor.exit_token(*kind, lex_node, trivia, self, path)
                    }
                    Node::Group { children } => {
                        match visitor.enter_group(children, self, path)? {
                            VisitorEntryResponse::Quit => return Ok(VisitorExitResponse::Quit),
                            VisitorEntryResponse::StepOver => {}
                            VisitorEntryResponse::StepIn => {
                                path.push(self.clone());
                                for child in children {
                                    match child.visit_with_path(visitor, path)? {
                                        VisitorExitResponse::Quit => {
                                            path.pop();
                                            return Ok(VisitorExitResponse::Quit);
                                        }
                                        VisitorExitResponse::StepIn => {}
                                    }
                                }
                                path.pop();
                            }
                        }
                        visitor.exit_group(children, self, path)
                    }
                }
            }
        }
    )
}

pub fn language_head() -> TokenStream {
    let error_renderer = boilerplate::error_renderer();

    quote!(
        use std::rc::Rc;

        use chumsky::{error::SimpleReason, Parser, Span};
        use ariadne::{Color, Config, Fmt, Label, Report, ReportBuilder, ReportKind, Source};
        use semver::Version;

        use super::{
            cst,
            parse::{Parsers, BoxedParserType, ErrorType, SpanType},
        };

        pub struct Language {
            parsers: Parsers,
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

        pub struct ParserOutput {
            parse_tree: Option<Rc<cst::Node>>,
            errors: Vec<ErrorType>,
        }

        impl ParserOutput {
            fn new(source: &str, parser: &BoxedParserType) -> Self {
                let (parse_tree, errors) = parser.parse_recovery(source);
                Self { parse_tree, errors }
            }

            pub fn parse_tree(&self) -> Option<Rc<cst::Node>> {
                self.parse_tree.clone()
            }

            pub fn error_count(&self) -> usize {
                self.errors.len()
            }

            pub fn errors_as_strings(&self, source: &str, with_colour: bool) -> Vec<String> {
                let mut results = vec![];
                for error in &self.errors {
                    let report = render_error_report(&error, with_colour);

                    let mut result = vec![];
                    report
                        .write(Source::from(source), &mut result)
                        .expect("Failed to write report");

                    let result = String::from_utf8(result).expect("Failed to convert report to utf8");
                    results.push(result);
                }

                results
            }

            pub fn is_valid(&self) -> bool {
                self.parse_tree.is_some() && self.errors.is_empty()
            }
        }

        #error_renderer
    )
}

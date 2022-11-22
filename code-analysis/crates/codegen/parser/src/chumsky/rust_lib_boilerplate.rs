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
    )
}

pub fn cst_visitor_head() -> TokenStream {
    quote!(
        #[allow(unused_variables)]
        pub trait Visitor {
            fn visit_none(&mut self, node: &Rc<Node>, path: &Vec<Rc<Node>>) -> VisitorExitResponse {
                VisitorExitResponse::StepIn
            }

            fn enter_rule(
                &mut self,
                kind: kinds::Rule,
                children: &Vec<Rc<Node>>,
                node: &Rc<Node>,
                path: &Vec<Rc<Node>>,
            ) -> VisitorEntryResponse {
                VisitorEntryResponse::StepIn
            }

            fn exit_rule(
                &mut self,
                kind: kinds::Rule,
                children: &Vec<Rc<Node>>,
                node: &Rc<Node>,
                path: &Vec<Rc<Node>>,
            ) -> VisitorExitResponse {
                VisitorExitResponse::StepIn
            }

            fn enter_token(
                &mut self,
                kind: kinds::Token,
                range: &Range<usize>,
                trivia: &Vec<Rc<Node>>,
                node: &Rc<Node>,
                path: &Vec<Rc<Node>>,
            ) -> VisitorEntryResponse {
                VisitorEntryResponse::StepIn
            }

            fn exit_token(
                &mut self,
                kind: kinds::Token,
                range: &Range<usize>,
                trivia: &Vec<Rc<Node>>,
                node: &Rc<Node>,
                path: &Vec<Rc<Node>>,
            ) -> VisitorExitResponse {
                VisitorExitResponse::StepIn
            }

            fn enter_group(
                &mut self,
                children: &Vec<Rc<Node>>,
                node: &Rc<Node>,
                path: &Vec<Rc<Node>>,
            ) -> VisitorEntryResponse {
                VisitorEntryResponse::StepIn
            }

            fn exit_group(
                &mut self,
                children: &Vec<Rc<Node>>,
                node: &Rc<Node>,
                path: &Vec<Rc<Node>>,
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
            fn visit_with_path<T: Visitor>(&self, visitor: &mut T, path: &mut Vec<Rc<Node>>) -> VisitorExitResponse;
        }

        impl Visitable for Rc<Node> {
            fn visit<T: Visitor>(&self, visitor: &mut T) -> VisitorExitResponse {
                self.visit_with_path(visitor, &mut Vec::new())
            }

            fn visit_with_path<T: Visitor>(&self, visitor: &mut T, path: &mut Vec<Rc<Node>>) -> VisitorExitResponse {
                match self.as_ref() {
                    Node::None => visitor.visit_none(self, path),
                    Node::Rule { kind, children } => {
                        match visitor.enter_rule(*kind, children, self, path) {
                            VisitorEntryResponse::Quit => return VisitorExitResponse::Quit,
                            VisitorEntryResponse::StepOver => {}
                            VisitorEntryResponse::StepIn => {
                                path.push(self.clone());
                                for child in children {
                                    match child.visit_with_path(visitor, path) {
                                        VisitorExitResponse::Quit => {
                                            path.pop();
                                            return VisitorExitResponse::Quit
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
                        range,
                        trivia,
                    } => {
                        match visitor.enter_token(*kind, range, trivia, self, path) {
                            VisitorEntryResponse::Quit => return VisitorExitResponse::Quit,
                            VisitorEntryResponse::StepOver => {}
                            VisitorEntryResponse::StepIn => {
                                path.push(self.clone());
                                for child in trivia {
                                    match child.visit_with_path(visitor, path) {
                                        VisitorExitResponse::Quit => {
                                            path.pop();
                                            return VisitorExitResponse::Quit
                                        }
                                        VisitorExitResponse::StepIn => {}
                                    }
                                }
                                path.pop();
                            }
                        }
                        visitor.exit_token(*kind, range, trivia, self, path)
                    }
                    Node::Group { children } => {
                        match visitor.enter_group(children, self, path) {
                            VisitorEntryResponse::Quit => return VisitorExitResponse::Quit,
                            VisitorEntryResponse::StepOver => {}
                            VisitorEntryResponse::StepIn => {
                                path.push(self.clone());
                                for child in children {
                                    match child.visit_with_path(visitor, path) {
                                        VisitorExitResponse::Quit => {
                                            path.pop();
                                            return VisitorExitResponse::Quit
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

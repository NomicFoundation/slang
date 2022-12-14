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

pub fn cst_visitor_head() -> TokenStream {
    quote!(
        #[allow(unused_variables)]
        pub trait Visitor<E> {
            fn enter_rule(
                &mut self,
                kind: kinds::Rule,
                children: &Vec<Rc<Node>>,
                node: &Rc<Node>,
                path: &Vec<Rc<Node>>,
            ) -> Result<VisitorEntryResponse, E> {
                Ok(VisitorEntryResponse::StepIn)
            }

            fn exit_rule(
                &mut self,
                kind: kinds::Rule,
                children: &Vec<Rc<Node>>,
                node: &Rc<Node>,
                path: &Vec<Rc<Node>>,
            ) -> Result<VisitorExitResponse, E> {
                Ok(VisitorExitResponse::StepIn)
            }

            fn enter_token(
                &mut self,
                kind: kinds::Token,
                lex_node: &Rc<lex::Node>,
                trivia: &Vec<Rc<Node>>,
                node: &Rc<Node>,
                path: &Vec<Rc<Node>>,
            ) -> Result<VisitorEntryResponse, E> {
                Ok(VisitorEntryResponse::StepIn)
            }

            fn exit_token(
                &mut self,
                kind: kinds::Token,
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
    quote!(
        use std::rc::Rc;

        use chumsky::Parser;
        use semver::Version;

        use super::{cst, parse::Parsers};

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

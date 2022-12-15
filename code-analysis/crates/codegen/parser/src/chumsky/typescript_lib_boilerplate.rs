use proc_macro2::TokenStream;
use quote::quote;

use super::boilerplate;

pub fn mod_head() -> TokenStream {
    quote!(
        pub mod kinds;
        pub mod lex;
        pub mod cst;
        mod parse;
        pub mod language;
    )
}

pub fn kinds_head() -> TokenStream {
    let base = boilerplate::kinds_head();
    quote!(
        #base
        use napi::bindgen_prelude::*;
        use napi_derive::napi;
    )
}

pub fn lex_head() -> TokenStream {
    let base = boilerplate::lex_head();
    quote!(
        #base
        use napi::bindgen_prelude::*;
        use napi::JsObject;
        use napi::NapiValue;

        #[napi]
        pub enum LexNodeType {
            Chars,
            Choice,
            Sequence,
            Named,
        }

        #[napi(object)]
        pub struct TokenRange {
            pub start: u32,
            pub end: u32
        }

        #[napi]
        pub struct LexCharsNode(Rc<Node>);
        #[napi]
        pub struct LexChoiceNode(Rc<Node>);
        #[napi]
        pub struct LexSequenceNode(Rc<Node>);
        #[napi]
        pub struct LexNamedNode(Rc<Node>);

        #[napi]
        impl LexCharsNode {
            #[napi(getter, js_name = "type", ts_return_type = "LexNodeType.Chars")]
            pub fn tipe(&self) -> LexNodeType { LexNodeType::Chars }

            #[napi(getter)]
            pub fn range(&self) -> TokenRange {
                match self.0.as_ref() {
                    Node::Chars(range) => TokenRange { start: range.start as u32, end: range.end as u32 },
                    _  => unreachable!()
                }
            }
        }

        #[napi]
        impl LexChoiceNode {
            #[napi(getter, js_name = "type", ts_return_type = "LexNodeType.Choice")]
            pub fn tipe(&self) -> LexNodeType { LexNodeType::Choice }

            #[napi(getter)]
            pub fn range(&self) -> TokenRange {
                let range = self.0.range();
                TokenRange { start: range.start as u32, end: range.end as u32 }
            }

            #[napi(getter)]
            pub fn index(&self) -> usize {
                match self.0.as_ref() {
                    Node::Choice(index, _) => *index,
                    _  => unreachable!()
                }
            }

            #[napi(ts_return_type = "LexEmptyNode | LexCharsNode | LexChoiceNode | LexSequenceNode | LexNamedNode")]
            pub fn child(&self, env: Env) -> JsObject {
                match self.0.as_ref() {
                    Node::Choice(_, child) => child.to_js(&env),
                    _  => unreachable!()
                }
            }
        }

        #[napi]
        impl LexSequenceNode {
            #[napi(getter, js_name = "type", ts_return_type = "LexNodeType.Sequence")]
            pub fn tipe(&self) -> LexNodeType { LexNodeType::Sequence }

            #[napi(getter)]
            pub fn range(&self) -> TokenRange {
                let range = self.0.range();
                TokenRange { start: range.start as u32, end: range.end as u32 }
            }

            #[napi(ts_return_type = "(LexEmptyNode | LexCharsNode | LexChoiceNode | LexSequenceNode | LexNamedNode)[]")]
            pub fn children(&self, env: Env) -> Vec<JsObject> {
                match self.0.as_ref() {
                    Node::Sequence(children) => children.iter().map(|child| child.to_js(&env)).collect(),
                    _  => unreachable!()
                }
            }
        }

        #[napi]
        impl LexNamedNode {
            #[napi(getter, js_name = "type", ts_return_type = "LexNodeType.Named")]
            pub fn tipe(&self) -> LexNodeType { LexNodeType::Named }

            #[napi(getter)]
            pub fn range(&self) -> TokenRange {
                let range = self.0.range();
                TokenRange { start: range.start as u32, end: range.end as u32 }
            }

            #[napi(getter)]
            pub fn kind(&self) -> kinds::Token {
                match self.0.as_ref() {
                    Node::Named(kind, _) => *kind,
                    _  => unreachable!()
                }
            }

            #[napi(ts_return_type = "LexCharsNode | LexChoiceNode | LexSequenceNode | LexNamedNode")]
            pub fn child(&self, env: Env) -> JsObject {
                match self.0.as_ref() {
                    Node::Named(_, child) => child.to_js(&env),
                    _  => unreachable!()
                }
            }
        }

        pub trait RcNodeExtensions {
            fn to_js(&self, env: &Env) -> JsObject;
        }

        impl RcNodeExtensions for Rc<Node> {
            fn to_js(&self, env: &Env) -> JsObject {
                let obj = match self.as_ref() {
                    Node::Chars(_) => unsafe { <LexCharsNode as ToNapiValue>::to_napi_value(env.raw(), LexCharsNode(self.clone())) }
                    Node::Choice(_, _) => unsafe { <LexChoiceNode as ToNapiValue>::to_napi_value(env.raw(), LexChoiceNode(self.clone())) }
                    Node::Sequence(_) => unsafe { <LexSequenceNode as ToNapiValue>::to_napi_value(env.raw(), LexSequenceNode(self.clone())) }
                    Node::Named(_, _) => unsafe { <LexNamedNode as ToNapiValue>::to_napi_value(env.raw(), LexNamedNode(self.clone())) }
                };
                return unsafe { JsObject::from_raw_unchecked(env.raw(), obj.unwrap()) };
            }
        }
    )
}

pub fn cst_head() -> TokenStream {
    let base = boilerplate::cst_head();
    quote!(
        #base
        use napi::bindgen_prelude::*;
        use napi::JsObject;
        use napi::NapiValue;

        use super::lex::RcNodeExtensions as LexRcNodeExtensions;

        #[napi]
        pub enum CSTNodeType {
            Rule,
            Token,
            Group,
        }

        #[napi]
        pub struct CSTRuleNode(Rc<Node>);
        #[napi]
        pub struct CSTTokenNode(Rc<Node>);
        #[napi]
        pub struct CSTGroupNode(Rc<Node>);

        #[napi]
        impl CSTRuleNode {
            #[napi(getter, js_name = "type", ts_return_type = "CSTNodeType.Rule")]
            pub fn tipe(&self) -> CSTNodeType { CSTNodeType::Rule }

            #[napi(getter)]
            pub fn kind(&self) -> kinds::Rule {
                match self.0.as_ref() {
                    Node::Rule { kind, .. } => *kind,
                    _ => unreachable!()
                }
            }

            #[napi(ts_return_type = "(CSTRuleNode | CSTTokenNode | CSTGroupNode)[]")]
            pub fn children(&self, env: Env) -> Vec<JsObject> {
                match self.0.as_ref() {
                    Node::Rule { children, .. } => children.iter().map(|child| child.to_js(&env)).collect(),
                    _ => unreachable!()
                }
            }
        }

        #[napi]
        impl CSTTokenNode {
            #[napi(getter, js_name = "type", ts_return_type = "CSTNodeType.Token")]
            pub fn tipe(&self) -> CSTNodeType { CSTNodeType::Token }

            #[napi(getter)]
            pub fn kind(&self) -> kinds::Token {
                match self.0.as_ref() {
                    Node::Token { kind, .. } => *kind,
                    _ => unreachable!()
                }
            }

            #[napi(ts_return_type = "LexCharsNode | LexChoiceNode | LexSequenceNode | LexNamedNode")]
            pub fn lex_node(&self, env: Env) -> JsObject {
                match self.0.as_ref() {
                    Node::Token { lex_node, .. } => lex_node.to_js(&env),
                    _ => unreachable!()
                }
            }

            #[napi(ts_return_type = "(CSTRuleNode | CSTTokenNode | CSTGroupNode)[]")]
            pub fn trivia(&self, env: Env) -> Vec<JsObject> {
                match self.0.as_ref() {
                    Node::Token { trivia, .. } => trivia.iter().map(|trivium| trivium.to_js(&env)).collect(),
                    _ => unreachable!()
                }
            }
        }

        #[napi]
        impl CSTGroupNode {
            #[napi(getter, js_name = "type", ts_return_type = "CSTNodeType.Group")]
            pub fn tipe(&self) -> CSTNodeType { CSTNodeType::Group }

            #[napi(ts_return_type = "(CSTRuleNode | CSTTokenNode | CSTGroupNode)[]")]
            pub fn children(&self, env: Env) -> Vec<JsObject> {
                match self.0.as_ref() {
                    Node::Group { children, .. } => children.iter().map(|child| child.to_js(&env)).collect(),
                    _ => unreachable!()
                }
            }
        }

        pub trait RcNodeExtensions {
            fn to_js(&self, env: &Env) -> JsObject;
        }

        impl RcNodeExtensions for Rc<Node> {
            fn to_js(&self, env: &Env) -> JsObject {
                let obj = match self.as_ref() {
                    Node::Rule { .. } => unsafe { <CSTRuleNode as ToNapiValue>::to_napi_value(env.raw(), CSTRuleNode(self.clone())) }
                    Node::Token { .. } => unsafe { <CSTTokenNode as ToNapiValue>::to_napi_value(env.raw(), CSTTokenNode(self.clone())) }
                    Node::Group { .. } => unsafe { <CSTGroupNode as ToNapiValue>::to_napi_value(env.raw(), CSTGroupNode(self.clone())) }
                };
                return unsafe { JsObject::from_raw_unchecked(env.raw(), obj.unwrap()) };
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
            cst::RcNodeExtensions as CSTRcNodeExtensions,
            parse::{Parsers, BoxedParserType, ErrorType, SpanType},
        };
        use napi::bindgen_prelude::*;

        #[napi]
        pub struct Language {
            parsers: Parsers,
            version: Version,
        }

        #[napi]
        impl Language {
            #[napi(constructor)]
            pub fn new(version: String) -> Self {
                let version = Version::parse(&version).unwrap();
                Self {
                    parsers: Parsers::new(&version),
                    version,
                }
            }

            #[napi]
            pub fn version(&self) -> String {
                self.version.to_string()
            }
        }

        #[napi]
        pub struct ParserOutput {
            parse_tree: Option<Rc<cst::Node>>,
            errors: Vec<ErrorType>,
        }

        #[napi]
        impl ParserOutput {
            fn new(source: String, parser: &BoxedParserType) -> Self {
                let (parse_tree, errors) = parser.parse_recovery(source.as_str());
                Self { parse_tree, errors }
            }

            #[napi(ts_return_type = "CSTRuleNode | CSTTokenNode | null")]
            pub fn parse_tree(&self, env: Env) -> Option<napi::JsObject> {
                self.parse_tree.clone().map(|n|n.to_js(&env))
            }

            #[napi]
            pub fn errors_as_strings(&self, source: String, with_colour: bool) -> Vec<String> {
                let mut results = vec![];
                for error in &self.errors {
                    let report = render_error_report(&error, with_colour);

                    let mut result = vec![];
                    report
                        .write(Source::from(source.as_str()), &mut result)
                        .expect("Failed to write report");

                    let result = String::from_utf8(result).expect("Failed to convert report to utf8");
                    results.push(result);
                }

                results
            }

            #[napi]
            pub fn is_valid(&self) -> bool {
                self.parse_tree.is_some() && self.errors.is_empty()
            }
        }

        #error_renderer
    )
}

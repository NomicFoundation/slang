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
            None,
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
        pub struct LexNoneNode;
        #[napi]
        pub struct LexCharsNode(Rc<Node>);
        #[napi]
        pub struct LexChoiceNode(Rc<Node>);
        #[napi]
        pub struct LexSequenceNode(Rc<Node>);
        #[napi]
        pub struct LexNamedNode(Rc<Node>);

        #[napi]
        impl LexNoneNode {
            #[napi(getter, js_name = "type", ts_return_type = "LexNodeType.None")]
            pub fn tipe(&self) -> LexNodeType { LexNodeType::None }

            #[napi(getter)]
            pub fn range(&self) -> TokenRange {
                TokenRange { start: 0, end: 0 }
            }
        }

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

            #[napi(ts_return_type = "LexNoneNode | LexCharsNode | LexChoiceNode | LexSequenceNode | LexNamedNode")]
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

            #[napi(ts_return_type = "(LexNoneNode | LexCharsNode | LexChoiceNode | LexSequenceNode | LexNamedNode)[]")]
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

            #[napi(ts_return_type = "LexNoneNode | LexCharsNode | LexChoiceNode | LexSequenceNode | LexNamedNode")]
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
                    Node::None => unsafe { <LexNoneNode as ToNapiValue>::to_napi_value(env.raw(), LexNoneNode) }
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
            None,
            Rule,
            Token,
            Group,
        }

        #[napi]
        pub struct CSTNoneNode;
        #[napi]
        pub struct CSTRuleNode(Rc<Node>);
        #[napi]
        pub struct CSTTokenNode(Rc<Node>);
        #[napi]
        pub struct CSTGroupNode(Rc<Node>);

        #[napi]
        impl CSTNoneNode {
            #[napi(getter, js_name = "type", ts_return_type = "CSTNodeType.None")]
            pub fn tipe(&self) -> CSTNodeType { CSTNodeType::None }
        }

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

            #[napi(ts_return_type = "(CSTNoneNode | CSTRuleNode | CSTTokenNode | CSTGroupNode)[]")]
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

            #[napi(ts_return_type = "LexNoneNode | LexCharsNode | LexChoiceNode | LexSequenceNode | LexNamedNode")]
            pub fn lex_node(&self, env: Env) -> JsObject {
                match self.0.as_ref() {
                    Node::Token { lex_node, .. } => lex_node.to_js(&env),
                    _ => unreachable!()
                }
            }

            #[napi(ts_return_type = "(CSTNoneNode | CSTRuleNode | CSTTokenNode | CSTGroupNode)[]")]
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

            #[napi(ts_return_type = "(CSTNoneNode | CSTRuleNode | CSTTokenNode | CSTGroupNode)[]")]
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
                    Node::None => unsafe { <CSTNoneNode as ToNapiValue>::to_napi_value(env.raw(), CSTNoneNode) }
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
    quote!(
        use chumsky::Parser;
        use semver::Version;
        use napi::bindgen_prelude::*;

        use super::parse::Parsers;
        use super::cst::RcNodeExtensions as CSTRcNodeExtensions;

        #[napi]
        pub struct Language {
            parsers: Parsers<'static>,
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
    )
}

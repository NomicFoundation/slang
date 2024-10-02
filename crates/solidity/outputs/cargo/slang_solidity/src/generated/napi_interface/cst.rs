// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

// NAPI-exposed functions have to accept owned values
#![allow(clippy::needless_pass_by_value)]
// The functions are meant to be definitions for export, so they're not really used
#![allow(clippy::return_self_not_must_use)]

use std::collections::HashMap;
use std::rc::Rc;

use napi::bindgen_prelude::ClassInstance;
use napi::{Either, Env};
use napi_derive::napi;

use crate::napi_interface::{
    EdgeLabel, NonterminalKind, RustCursor, RustNode, RustNonterminalNode, RustQuery,
    RustQueryMatch, RustQueryMatchIterator, RustTerminalNode, RustTextIndex, RustTextRange,
    TerminalKind,
};

#[napi(namespace = "cst", string_enum)]
pub enum NodeType {
    Nonterminal,
    Terminal,
}

pub trait NAPINodeExtensions {
    fn into_js_either_node(self) -> Either<NonterminalNode, TerminalNode>;
}

impl NAPINodeExtensions for RustNode {
    /// Converts the node into `napi` wrapper for `NonterminalNode | TerminalNode` JS object.
    fn into_js_either_node(self) -> Either<NonterminalNode, TerminalNode> {
        match self {
            RustNode::Nonterminal(nonterminal) => Either::A(NonterminalNode(nonterminal)),
            RustNode::Terminal(terminal) => Either::B(TerminalNode(terminal)),
        }
    }
}

#[derive(Debug)]
#[napi(namespace = "cst")]
pub struct NonterminalNode(pub(crate) Rc<RustNonterminalNode>);

#[derive(Debug)]
#[napi(namespace = "cst")]
pub struct TerminalNode(pub(crate) Rc<RustTerminalNode>);

#[napi(namespace = "cst")]
impl NonterminalNode {
    #[napi(
        getter,
        js_name = "type",
        ts_return_type = "cst.NodeType.Nonterminal",
        catch_unwind
    )]
    pub fn tipe(&self) -> NodeType {
        NodeType::Nonterminal
    }

    #[napi(getter, ts_return_type = "cst.NonterminalKind", catch_unwind)]
    pub fn kind(&self) -> NonterminalKind {
        self.0.kind
    }

    #[napi(
        getter,
        js_name = "textLength",
        ts_return_type = "cst.TextIndex",
        catch_unwind
    )]
    pub fn text_len(&self) -> TextIndex {
        self.0.text_len.into()
    }

    #[napi(ts_return_type = "Array<cst.Node>", catch_unwind)]
    pub fn children(&self) -> Vec<Either<NonterminalNode, TerminalNode>> {
        self.0
            .children
            .iter()
            .map(|child| child.node.clone().into_js_either_node())
            .collect()
    }

    #[napi(ts_return_type = "cst.Cursor", catch_unwind)]
    pub fn create_cursor(
        &self,
        #[napi(ts_arg_type = "cst.TextIndex")] text_offset: TextIndex,
    ) -> Cursor {
        RustNode::Nonterminal(Rc::clone(&self.0))
            .cursor_with_offset(text_offset.into())
            .into()
    }

    /// Serialize the node to JSON.
    #[napi(catch_unwind, js_name = "toJSON")]
    pub fn to_json(&self) -> String {
        serde_json::to_string(&self.0).unwrap()
    }

    #[napi(catch_unwind)]
    pub fn unparse(&self) -> String {
        Rc::clone(&self.0).unparse()
    }

    // Expose the children as a hidden (non-enumerable, don't generate type definition)
    // property that's eagerly evaluated (getter) for an inspected parent object in the debugger context.
    #[napi(
        enumerable = false,
        configurable = false,
        writable = false,
        getter,
        js_name = "__children", // Needed; otherwise, the property name would shadow `children`.
        skip_typescript,
        catch_unwind
    )]
    pub fn __children(&self) -> Vec<Either<NonterminalNode, TerminalNode>> {
        Self::children(self)
    }

    // Similarly, expose the eagerly evaluated unparsed text in the debugger context.
    #[napi(
        enumerable = false,
        configurable = false,
        writable = false,
        getter,
        js_name = "__text",
        skip_typescript,
        catch_unwind
    )]
    pub fn __text(&self) -> String {
        self.unparse()
    }
}

#[napi(namespace = "cst")]
impl TerminalNode {
    #[napi(
        getter,
        js_name = "type",
        ts_return_type = "cst.NodeType.Terminal",
        catch_unwind
    )]
    pub fn tipe(&self) -> NodeType {
        NodeType::Terminal
    }

    #[napi(getter, ts_return_type = "cst.TerminalKind", catch_unwind)]
    pub fn kind(&self) -> TerminalKind {
        self.0.kind
    }

    #[napi(
        getter,
        js_name = "textLength",
        ts_return_type = "cst.TextIndex",
        catch_unwind
    )]
    pub fn text_len(&self) -> TextIndex {
        let text_len: RustTextIndex = (&self.0.text).into();
        text_len.into()
    }

    #[napi(getter, catch_unwind)]
    pub fn text(&self) -> String {
        self.0.text.clone()
    }

    /// Serialize the node to JSON.
    #[napi(catch_unwind, js_name = "toJSON")]
    pub fn to_json(&self) -> String {
        serde_json::to_string(&self.0).unwrap()
    }

    #[napi(ts_return_type = "cst.Cursor", catch_unwind)]
    pub fn create_cursor(
        &self,
        #[napi(ts_arg_type = "cst.TextIndex")] text_offset: TextIndex,
    ) -> Cursor {
        RustNode::Terminal(Rc::clone(&self.0))
            .cursor_with_offset(text_offset.into())
            .into()
    }
}

#[napi(namespace = "cst")]
pub struct Cursor(pub(super) RustCursor);

impl From<RustCursor> for Cursor {
    fn from(value: RustCursor) -> Self {
        Self(value)
    }
}

#[napi(namespace = "cst")]
impl Cursor {
    pub(crate) fn new(cursor: RustCursor) -> Self {
        Self(cursor)
    }

    #[napi(catch_unwind)]
    pub fn reset(&mut self) {
        self.0.reset();
    }

    #[napi(catch_unwind)]
    pub fn complete(&mut self) {
        self.0.complete();
    }

    #[must_use]
    #[napi(catch_unwind)]
    #[allow(clippy::should_implement_trait)] // These are meant to be explicitly exposed to NAPI
    pub fn clone(&self) -> Self {
        Self(self.0.clone())
    }

    #[must_use]
    #[napi(catch_unwind)]
    pub fn spawn(&self) -> Self {
        Self::new(self.0.spawn())
    }

    #[napi(getter, catch_unwind)]
    pub fn is_completed(&self) -> bool {
        self.0.is_completed()
    }

    #[napi(ts_return_type = "cst.Node", catch_unwind)]
    pub fn node(&self) -> Either<NonterminalNode, TerminalNode> {
        self.0.node().into_js_either_node()
    }

    #[napi(getter, ts_return_type = "cst.EdgeLabel", catch_unwind)]
    pub fn label(&self) -> Option<EdgeLabel> {
        self.0.label()
    }

    #[napi(getter, ts_return_type = "cst.TextIndex", catch_unwind)]
    pub fn text_offset(&self) -> TextIndex {
        self.0.text_offset().into()
    }

    #[napi(getter, ts_return_type = "cst.TextRange", catch_unwind)]
    pub fn text_range(&self) -> TextRange {
        self.0.text_range().into()
    }

    #[allow(clippy::cast_possible_truncation)] // Cursor depth can't reasonably be larger than u32
    #[napi(getter, catch_unwind)]
    pub fn depth(&self) -> u32 {
        self.0.depth() as u32
    }

    #[napi(ts_return_type = "Array<cst.NonterminalNode>", catch_unwind)]
    pub fn ancestors(&self) -> Vec<NonterminalNode> {
        self.0.ancestors().map(NonterminalNode).collect()
    }

    #[napi(catch_unwind)]
    pub fn go_to_next(&mut self) -> bool {
        self.0.go_to_next()
    }

    #[napi(catch_unwind)]
    pub fn go_to_next_non_descendent(&mut self) -> bool {
        self.0.go_to_next_non_descendent()
    }

    #[napi(catch_unwind)]
    pub fn go_to_previous(&mut self) -> bool {
        self.0.go_to_previous()
    }

    #[napi(catch_unwind)]
    pub fn go_to_parent(&mut self) -> bool {
        self.0.go_to_parent()
    }

    #[napi(catch_unwind)]
    pub fn go_to_first_child(&mut self) -> bool {
        self.0.go_to_first_child()
    }

    #[napi(catch_unwind)]
    pub fn go_to_last_child(&mut self) -> bool {
        self.0.go_to_last_child()
    }

    #[napi(catch_unwind)]
    pub fn go_to_nth_child(&mut self, child_number: u32) -> bool {
        self.0.go_to_nth_child(child_number as usize)
    }

    #[napi(catch_unwind)]
    pub fn go_to_next_sibling(&mut self) -> bool {
        self.0.go_to_next_sibling()
    }

    #[napi(catch_unwind)]
    pub fn go_to_previous_sibling(&mut self) -> bool {
        self.0.go_to_previous_sibling()
    }

    #[napi(catch_unwind)]
    pub fn go_to_next_terminal(&mut self) -> bool {
        self.0.go_to_next_terminal()
    }

    #[napi(catch_unwind)]
    pub fn go_to_next_terminal_with_kind(
        &mut self,
        #[napi(ts_arg_type = "cst.TerminalKind")] kind: TerminalKind,
    ) -> bool {
        self.0.go_to_next_terminal_with_kind(kind)
    }

    #[napi(catch_unwind)]
    pub fn go_to_next_terminal_with_kinds(
        &mut self,
        #[napi(ts_arg_type = "Array<cst.TerminalKind>")] kinds: Vec<TerminalKind>,
    ) -> bool {
        self.0.go_to_next_terminal_with_kinds(&kinds)
    }

    #[napi(catch_unwind)]
    pub fn go_to_next_nonterminal(&mut self) -> bool {
        self.0.go_to_next_nonterminal()
    }

    #[napi(catch_unwind)]
    pub fn go_to_next_nonterminal_with_kind(
        &mut self,
        #[napi(ts_arg_type = "cst.NonterminalKind")] kind: NonterminalKind,
    ) -> bool {
        self.0.go_to_next_nonterminal_with_kind(kind)
    }

    #[napi(catch_unwind)]
    pub fn go_to_next_nonterminal_with_kinds(
        &mut self,
        #[napi(ts_arg_type = "Array<cst.NonterminalKind>")] kinds: Vec<NonterminalKind>,
    ) -> bool {
        self.0.go_to_next_nonterminal_with_kinds(&kinds)
    }
}

#[napi(namespace = "cst")]
pub struct Query(RustQuery);

impl From<RustQuery> for Query {
    fn from(value: RustQuery) -> Self {
        Self(value)
    }
}

#[napi(namespace = "cst")]
impl Query {
    #[napi(factory, catch_unwind)]
    pub fn parse(text: String) -> napi::Result<Query> {
        RustQuery::parse(text.as_str()).map_or_else(
            |err| Err(napi::Error::from_reason(err.message)),
            |query| Ok(query.into()),
        )
    }
}

#[napi(namespace = "cst")]
pub struct QueryMatchIterator(RustQueryMatchIterator);

#[napi(object, namespace = "cst")]
pub struct QueryMatch {
    pub query_number: u32,
    #[napi(ts_type = "{ [key: string]: cst.Cursor[] }")]
    pub captures: HashMap<String, Vec<ClassInstance<Cursor>>>,
}

impl QueryMatch {
    fn new(env: Env, r#match: RustQueryMatch) -> napi::Result<Self> {
        #[allow(clippy::cast_possible_truncation)]
        let query_number = r#match.query_number as u32;
        // transfer all of the captures eagerly on the assumption
        // that they've all been explicitly requested.
        let captures = r#match
            .captures
            .into_iter()
            .map(|(key, values)| {
                let instances = values
                    .into_iter()
                    .map(|cursor| Cursor(cursor).into_instance(env))
                    .collect::<napi::Result<_>>()?;

                Ok((key, instances))
            })
            .collect::<napi::Result<_>>()?;

        Ok(Self {
            query_number,
            captures,
        })
    }
}

impl From<RustQueryMatchIterator> for QueryMatchIterator {
    fn from(value: RustQueryMatchIterator) -> Self {
        Self(value)
    }
}

#[napi(namespace = "cst")]
impl QueryMatchIterator {
    #[napi(catch_unwind)]
    pub fn next(&mut self, env: Env) -> napi::Result<Option<QueryMatch>> {
        match self.0.next() {
            Some(r#match) => Ok(Some(QueryMatch::new(env, r#match)?)),
            None => Ok(None),
        }
    }
}

#[napi(namespace = "cst")]
impl Cursor {
    #[napi(ts_return_type = "cst.QueryMatchIterator", catch_unwind)]
    pub fn query(
        &self,
        #[napi(ts_arg_type = "Array<cst.Query>")] queries: Vec<&Query>,
    ) -> QueryMatchIterator {
        self.0
            .clone()
            .query(queries.into_iter().map(|x| x.0.clone()).collect())
            .into()
    }
}

#[napi(object, namespace = "cst")]
#[derive(Copy, Clone)]
pub struct TextIndex {
    pub utf8: u32,
    pub utf16: u32,
    pub line: u32,
    pub column: u32,
}

impl From<RustTextIndex> for TextIndex {
    fn from(value: RustTextIndex) -> Self {
        // We only support 32-byte indices on TS side.
        #[allow(clippy::cast_possible_truncation)]
        Self {
            utf8: value.utf8 as u32,
            utf16: value.utf16 as u32,
            line: value.line as u32,
            column: value.column as u32,
        }
    }
}

impl From<TextIndex> for RustTextIndex {
    fn from(value: TextIndex) -> Self {
        Self {
            utf8: value.utf8 as usize,
            utf16: value.utf16 as usize,
            line: value.line as usize,
            column: value.column as usize,
        }
    }
}

#[napi(object, namespace = "cst")]
#[derive(Copy, Clone)]
pub struct TextRange {
    pub start: TextIndex,
    pub end: TextIndex,
}

impl From<RustTextRange> for TextRange {
    fn from(value: RustTextRange) -> Self {
        Self {
            start: value.start.into(),
            end: value.end.into(),
        }
    }
}

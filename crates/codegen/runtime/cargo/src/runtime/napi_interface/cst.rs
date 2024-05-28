use std::rc::Rc;

use napi::Either;
use napi_derive::napi;

use crate::napi_interface::cursor::Cursor;
use crate::napi_interface::text_index::TextIndex;
use crate::napi_interface::{
    NonterminalKind, RustNode, RustNonterminalNode, RustTerminalNode, RustTextIndex, TerminalKind,
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
        ts_return_type = "NodeType.Nonterminal",
        catch_unwind
    )]
    pub fn tipe(&self) -> NodeType {
        NodeType::Nonterminal
    }

    #[napi(getter, ts_return_type = "kinds.NonterminalKind", catch_unwind)]
    pub fn kind(&self) -> NonterminalKind {
        self.0.kind
    }

    #[napi(
        getter,
        js_name = "textLength",
        ts_return_type = "text_index.TextIndex",
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

    #[napi(ts_return_type = "cursor.Cursor", catch_unwind)]
    pub fn create_cursor(
        &self,
        #[napi(ts_arg_type = "text_index.TextIndex")] text_offset: TextIndex,
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
        ts_return_type = "NodeType.Terminal",
        catch_unwind
    )]
    pub fn tipe(&self) -> NodeType {
        NodeType::Terminal
    }

    #[napi(getter, ts_return_type = "kinds.TerminalKind", catch_unwind)]
    pub fn kind(&self) -> TerminalKind {
        self.0.kind
    }

    #[napi(
        getter,
        js_name = "textLength",
        ts_return_type = "text_index.TextIndex",
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

    #[napi(ts_return_type = "cursor.Cursor", catch_unwind)]
    pub fn create_cursor(
        &self,
        #[napi(ts_arg_type = "text_index.TextIndex")] text_offset: TextIndex,
    ) -> Cursor {
        RustNode::Terminal(Rc::clone(&self.0))
            .cursor_with_offset(text_offset.into())
            .into()
    }
}

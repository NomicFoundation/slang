use std::rc::Rc;

use napi::Either;
use napi_derive::napi;

use crate::napi_interface::cursor::Cursor;
use crate::napi_interface::text_index::TextIndex;
use crate::napi_interface::{
    RuleKind, RustNode, RustRuleNode, RustTextIndex, RustTokenNode, TokenKind,
};

#[napi(namespace = "cst", string_enum)]
pub enum NodeType {
    Rule,
    Token,
}

pub trait NAPINodeExtensions {
    fn into_js_either_node(self) -> Either<RuleNode, TokenNode>;
}

impl NAPINodeExtensions for RustNode {
    /// Converts the node into `napi` wrapper for `RuleNode | TokenNode` JS object.
    fn into_js_either_node(self) -> Either<RuleNode, TokenNode> {
        match self {
            RustNode::Rule(rule) => Either::A(RuleNode(rule)),
            RustNode::Token(token) => Either::B(TokenNode(token)),
        }
    }
}

#[derive(Debug)]
#[napi(namespace = "cst")]
pub struct RuleNode(pub(crate) Rc<RustRuleNode>);

#[derive(Debug)]
#[napi(namespace = "cst")]
pub struct TokenNode(pub(crate) Rc<RustTokenNode>);

#[napi(namespace = "cst")]
impl RuleNode {
    #[napi(
        getter,
        js_name = "type",
        ts_return_type = "NodeType.Rule",
        catch_unwind
    )]
    pub fn tipe(&self) -> NodeType {
        NodeType::Rule
    }

    #[napi(getter, ts_return_type = "kinds.RuleKind", catch_unwind)]
    pub fn kind(&self) -> RuleKind {
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
    pub fn children(&self) -> Vec<Either<RuleNode, TokenNode>> {
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
        RustNode::Rule(Rc::clone(&self.0))
            .cursor_with_offset(text_offset.into())
            .into()
    }

    #[napi(catch_unwind, js_name = "toJSON")]
    /// Serialize the token node to JSON.
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
    pub fn __children(&self) -> Vec<Either<RuleNode, TokenNode>> {
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
impl TokenNode {
    #[napi(
        getter,
        js_name = "type",
        ts_return_type = "NodeType.Token",
        catch_unwind
    )]
    pub fn tipe(&self) -> NodeType {
        NodeType::Token
    }

    #[napi(getter, ts_return_type = "kinds.TokenKind", catch_unwind)]
    pub fn kind(&self) -> TokenKind {
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

    #[napi(catch_unwind, js_name = "toJSON")]
    /// Serialize the token node to JSON.
    ///
    /// This method is intended for debugging purposes and may not be stable.
    pub fn to_json(&self) -> String {
        serde_json::to_string(&self.0).unwrap()
    }

    #[napi(ts_return_type = "cursor.Cursor", catch_unwind)]
    pub fn create_cursor(
        &self,
        #[napi(ts_arg_type = "text_index.TextIndex")] text_offset: TextIndex,
    ) -> Cursor {
        RustNode::Token(Rc::clone(&self.0))
            .cursor_with_offset(text_offset.into())
            .into()
    }
}

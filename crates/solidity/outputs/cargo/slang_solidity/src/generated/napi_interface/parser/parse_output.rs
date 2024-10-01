// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use napi::Either;
use napi_derive::napi;

use crate::napi_interface::cst::{Cursor, NAPINodeExtensions, NonterminalNode, TerminalNode};
use crate::napi_interface::parser::ParseError;
use crate::napi_interface::RustParseOutput;

#[napi(namespace = "parser")]
pub struct ParseOutput(RustParseOutput);

impl From<RustParseOutput> for ParseOutput {
    fn from(value: RustParseOutput) -> Self {
        Self(value)
    }
}

#[napi(namespace = "parser")]
impl ParseOutput {
    #[napi(ts_return_type = "cst.Node", catch_unwind)]
    pub fn tree(&self) -> Either<NonterminalNode, TerminalNode> {
        self.0.tree().into_js_either_node()
    }

    #[napi(ts_return_type = "Array<parser.ParseError>", catch_unwind)]
    pub fn errors(&self) -> Vec<ParseError> {
        self.0.errors().iter().map(|x| x.clone().into()).collect()
    }

    #[napi(getter, catch_unwind)]
    pub fn is_valid(&self) -> bool {
        self.0.is_valid()
    }

    /// Creates a cursor that starts at the root of the parse tree.
    #[napi(ts_return_type = "cst.Cursor", catch_unwind)]
    pub fn create_tree_cursor(&self) -> Cursor {
        self.0.create_tree_cursor().into()
    }
}

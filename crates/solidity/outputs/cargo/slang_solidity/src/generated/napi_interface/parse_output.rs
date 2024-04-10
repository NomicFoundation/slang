// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use napi::Either;
use napi_derive::napi;

use crate::napi_interface::cst::{RuleNode, TokenNode};
use crate::napi_interface::{cursor, parse_error, RustParseOutput};

#[napi(namespace = "parse_output")]
pub struct ParseOutput(RustParseOutput);

impl From<RustParseOutput> for ParseOutput {
    fn from(value: RustParseOutput) -> Self {
        Self(value)
    }
}

#[napi(namespace = "parse_output")]
impl ParseOutput {
    #[napi(ts_return_type = "cst.Node", catch_unwind)]
    pub fn tree(&self) -> Either<RuleNode, TokenNode> {
        self.0.tree().into()
    }

    #[napi(ts_return_type = "Array<parse_error.ParseError>", catch_unwind)]
    pub fn errors(&self) -> Vec<parse_error::ParseError> {
        self.0.errors().iter().map(|x| x.clone().into()).collect()
    }

    #[napi(getter, catch_unwind)]
    pub fn is_valid(&self) -> bool {
        self.0.is_valid()
    }

    /// Creates a cursor that starts at the root of the parse tree.
    #[napi(ts_return_type = "cursor.Cursor", catch_unwind)]
    pub fn create_tree_cursor(&self) -> cursor::Cursor {
        self.0.create_tree_cursor().into()
    }
}

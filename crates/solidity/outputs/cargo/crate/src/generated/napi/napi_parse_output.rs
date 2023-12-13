// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use napi::bindgen_prelude::Env;
use napi_cst::ToJS;
use napi_derive::napi;

use crate::napi::{napi_cst, napi_cursor, napi_parse_error, RustParseOutput};

#[napi(namespace = "parse_output")]
pub struct ParseOutput(RustParseOutput);

impl From<RustParseOutput> for ParseOutput {
    fn from(value: RustParseOutput) -> Self {
        Self(value)
    }
}

#[napi(namespace = "parse_output")]
impl ParseOutput {
    #[napi(ts_return_type = "cst.Node")]
    pub fn tree(&self, env: Env) -> napi::JsObject {
        self.0.tree().to_js(&env)
    }

    #[napi(ts_return_type = "Array<parse_error.ParseError>")]
    pub fn errors(&self) -> Vec<napi_parse_error::ParseError> {
        self.0.errors().iter().map(|x| x.clone().into()).collect()
    }

    #[napi(getter)]
    pub fn is_valid(&self) -> bool {
        self.0.is_valid()
    }

    /// Creates a cursor that starts at the root of the parse tree.
    #[napi(ts_return_type = "cursor.Cursor")]
    pub fn create_tree_cursor(&self) -> napi_cursor::Cursor {
        self.0.create_tree_cursor().into()
    }
}

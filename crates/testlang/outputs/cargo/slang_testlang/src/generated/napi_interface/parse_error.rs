// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

// NAPI-exposed functions have to accept owned values.
#![allow(clippy::needless_pass_by_value)]

use napi_derive::napi;
use text_index::TextRange;

use crate::napi_interface::diagnostic::Diagnostic;
use crate::napi_interface::{text_index, RustParseError};

#[napi(namespace = "parse_error")]
#[derive(PartialEq, Clone)]
pub struct ParseError(RustParseError);

impl From<RustParseError> for ParseError {
    fn from(value: RustParseError) -> Self {
        Self(value)
    }
}

#[napi(namespace = "parse_error")]
impl ParseError {
    #[napi(getter, ts_return_type = "text_index.TextRange", catch_unwind)]
    pub fn text_range(&self) -> TextRange {
        self.0.text_range().clone().into()
    }

    #[napi(ts_return_type = "diagnostic.Diagnostic", catch_unwind)]
    pub fn to_diagnostic(&self) -> Diagnostic {
        // TODO: Figure out if we can auto-gen Diagnostics methods
        // in TS for this implementor rather than having to clone here
        Diagnostic(Box::new(self.0.clone()))
    }
}

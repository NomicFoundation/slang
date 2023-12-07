// NAPI-exposed functions have to accept owned values.
#![allow(clippy::needless_pass_by_value)]

use napi_derive::napi;

use super::{napi_text_index, RustParseError};
use napi_text_index::TextRange;

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
    #[napi(getter, ts_return_type = "text_index.TextRange")]
    pub fn text_range(&self) -> TextRange {
        self.0.text_range().into()
    }

    pub fn tokens_that_would_have_allowed_more_progress(&self) -> Vec<String> {
        self.0.tokens_that_would_have_allowed_more_progress()
    }

    #[napi(namespace = "parse_error")]
    pub fn to_error_report(&self, source_id: String, source: String, with_color: bool) -> String {
        self.0.to_error_report(&source_id, &source, with_color)
    }
}

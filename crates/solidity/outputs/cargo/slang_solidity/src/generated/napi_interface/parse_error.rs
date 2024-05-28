// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

// NAPI-exposed functions have to accept owned values.
#![allow(clippy::needless_pass_by_value)]

use napi_derive::napi;

use crate::expose_diagnostic_trait_interface;
use crate::napi_interface::RustParseError;

#[napi(namespace = "parse_error")]
#[derive(PartialEq, Clone)]
pub struct ParseError(RustParseError);

impl From<RustParseError> for ParseError {
    fn from(value: RustParseError) -> Self {
        Self(value)
    }
}

expose_diagnostic_trait_interface!("parse_error", ParseError);

// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use napi::bindgen_prelude::*;
use napi_derive::napi;
use serde::Serialize;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize)]
#[napi]
pub enum TokenKind {
    XXX,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize)]
#[napi]
pub enum RuleKind {
    _SEQUENCE,
    _DELIMITEDBY,
    _TERMINATEDBY,
    XXX,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Serialize)]
#[napi]
pub enum ProductionKind {
    XXX,
}

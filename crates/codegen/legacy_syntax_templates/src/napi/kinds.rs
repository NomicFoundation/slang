// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use napi::bindgen_prelude::*;
use napi_derive::napi;
use serde::Serialize;

#[derive(
    Debug,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Serialize,
    strum_macros::EnumString,
    strum_macros::AsRefStr,
    strum_macros::Display,
)]
#[napi(namespace = "legacy")]
pub enum TokenKind {
    SKIPPED,
    XXX,
}

#[derive(
    Debug,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Serialize,
    strum_macros::EnumString,
    strum_macros::AsRefStr,
    strum_macros::Display,
)]
#[napi(namespace = "legacy")]
pub enum RuleKind {
    XXX,
}

#[derive(
    Debug,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Serialize,
    strum_macros::EnumString,
    strum_macros::AsRefStr,
    strum_macros::Display,
)]
#[napi(namespace = "legacy")]
pub enum ProductionKind {
    XXX,
}

#[cfg(feature = "slang_napi_interfaces")]
use napi_derive::napi;

#[derive(
    Debug,
    Eq,
    Ord,
    PartialEq,
    PartialOrd,
    serde::Serialize,
    strum_macros::AsRefStr,
    strum_macros::Display,
    strum_macros::EnumString,
)]
#[cfg_attr( feature = "slang_napi_interfaces", /* derives `Clone` and `Copy` */ napi(string_enum, namespace = "kinds") )]
#[cfg_attr(not(feature = "slang_napi_interfaces"), derive(Clone, Copy))]
pub enum TokenKind {
    SKIPPED,
    XXX,
}

#[derive(
    Debug,
    Eq,
    Ord,
    PartialEq,
    PartialOrd,
    serde::Serialize,
    strum_macros::AsRefStr,
    strum_macros::Display,
    strum_macros::EnumString,
)]
#[cfg_attr( feature = "slang_napi_interfaces", /* derives `Clone` and `Copy` */ napi(string_enum, namespace = "kinds") )]
#[cfg_attr(not(feature = "slang_napi_interfaces"), derive(Clone, Copy))]
pub enum RuleKind {
    XXX,
}

impl RuleKind {
    pub fn is_trivia(&self) -> bool {
        unreachable!("Expanded by the template")
    }
}

#[derive(
    Debug,
    Eq,
    Ord,
    PartialEq,
    PartialOrd,
    serde::Serialize,
    strum_macros::AsRefStr,
    strum_macros::Display,
    strum_macros::EnumString,
)]
#[cfg_attr( feature = "slang_napi_interfaces", /* derives `Clone` and `Copy` */ napi(string_enum, namespace = "kinds") )]
#[cfg_attr(not(feature = "slang_napi_interfaces"), derive(Clone, Copy))]
pub enum ProductionKind {
    XXX,
}

/// The lexical context of the scanner.
#[derive(strum_macros::FromRepr)]
#[cfg_attr(feature = "slang_napi_interfaces", /* derives `Clone` and `Copy` */ napi(string_enum, namespace = "language"))]
#[cfg_attr(not(feature = "slang_napi_interfaces"), derive(Clone, Copy))]
pub enum LexicalContext {
    XXX,
}

/// Marker trait for type-level [`LexicalContext`] variants.
pub trait IsLexicalContext {
    /// Returns a run-time [`LexicalContext`] value.
    fn value() -> LexicalContext;
}

#[allow(non_snake_case)]
pub mod LexicalContextType {}

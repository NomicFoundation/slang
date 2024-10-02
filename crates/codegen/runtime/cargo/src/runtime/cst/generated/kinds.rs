// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

#[cfg(feature = "__private_napi_interfaces")]
use napi_derive::napi;

// This needs to stay in sync with the wit-bindgen output
#[repr(u8)]
#[derive(
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    serde::Serialize,
    strum_macros::AsRefStr,
    strum_macros::Display,
    strum_macros::EnumString,
    strum_macros::IntoStaticStr,
)]
#[cfg_attr(feature = "__private_napi_interfaces", /* derives `Clone` and `Copy` */ napi(string_enum, namespace = "cst"))]
#[cfg_attr(not(feature = "__private_napi_interfaces"), derive(Clone, Copy))]
pub enum NonterminalKind {
    Stub1,
    Stub2,
    Stub3,
}

impl crate::cst::NonterminalKindExtensions for NonterminalKind {}

// This needs to stay in sync with the wit-bindgen output
#[repr(u8)]
#[derive(
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    serde::Serialize,
    strum_macros::AsRefStr,
    strum_macros::Display,
    strum_macros::EnumString,
    strum_macros::IntoStaticStr,
)]
#[strum(serialize_all = "snake_case")]
#[cfg_attr(feature = "__private_napi_interfaces", /* derives `Clone` and `Copy` */ napi(string_enum, namespace = "cst"))]
#[cfg_attr(not(feature = "__private_napi_interfaces"), derive(Clone, Copy))]
pub enum EdgeLabel {
    // Built-in:
    Item,
    Variant,
    Separator,
    Operand,
    LeftOperand,
    RightOperand,
    LeadingTrivia,
    TrailingTrivia,

    // Generated:
    Stub1,
    Stub2,
    Stub3,
}

impl crate::cst::EdgeLabelExtensions for EdgeLabel {}

// This needs to stay in sync with the wit-bindgen output
#[repr(u8)]
#[derive(
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    serde::Serialize,
    strum_macros::AsRefStr,
    strum_macros::Display,
    strum_macros::EnumString,
    strum_macros::IntoStaticStr,
)]
#[cfg_attr(feature = "__private_napi_interfaces", /* derives `Clone` and `Copy` */ napi(string_enum, namespace = "cst"))]
#[cfg_attr(not(feature = "__private_napi_interfaces"), derive(Clone, Copy))]
#[allow(clippy::upper_case_acronyms)]
pub enum TerminalKind {
    // Built-in:
    UNRECOGNIZED,
    MISSING,

    // Generated:
    Stub1,
    Stub2,
    Stub3,
}

impl crate::cst::TerminalKindExtensions for TerminalKind {
    fn is_trivia(&self) -> bool {
        false
    }

    fn is_valid(&self) -> bool {
        !matches!(self, Self::UNRECOGNIZED | Self::MISSING)
    }
}

/// The lexical context of the scanner.
// This needs to stay in sync with the wit-bindgen output
#[repr(u8)]
#[derive(strum_macros::FromRepr, Clone, Copy)]
pub(crate) enum LexicalContext {
    Stub1,
    Stub2,
    Stub3,
}

/// Marker trait for type-level [`LexicalContext`] variants.
pub(crate) trait IsLexicalContext {
    /// Returns a run-time [`LexicalContext`] value.
    fn value() -> LexicalContext;
}

#[allow(non_snake_case)]
pub(crate) mod LexicalContextType {}

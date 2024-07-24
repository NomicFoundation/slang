// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

#[cfg(feature = "__private_napi_interfaces")]
use napi_derive::napi;

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
#[cfg_attr(feature = "__private_napi_interfaces", /* derives `Clone` and `Copy` */ napi(string_enum, namespace = "kinds"))]
#[cfg_attr(not(feature = "__private_napi_interfaces"), derive(Clone, Copy))]
pub enum NonterminalKind {
    Stub1,
    Stub2,
    Stub3,
}

impl metaslang_cst::NonterminalKind for NonterminalKind {}

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
#[cfg_attr(feature = "__private_napi_interfaces", /* derives `Clone` and `Copy` */ napi(string_enum, namespace = "kinds"))]
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

impl metaslang_cst::EdgeLabel for EdgeLabel {}

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
#[cfg_attr(feature = "__private_napi_interfaces", /* derives `Clone` and `Copy` */ napi(string_enum, namespace = "kinds"))]
#[cfg_attr(not(feature = "__private_napi_interfaces"), derive(Clone, Copy))]
pub enum TerminalKind {
    // Built-in:
    UNRECOGNIZED,
    MISSING,

    // Generated:
    Stub1,
    Stub2,
    Stub3,
}

impl metaslang_cst::TerminalKind for TerminalKind {
    fn is_trivia(&self) -> bool {
        false
    }

    fn is_valid(&self) -> bool {
        !matches!(self, Self::UNRECOGNIZED | Self::MISSING)
    }
}

/// The lexical context of the scanner.
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

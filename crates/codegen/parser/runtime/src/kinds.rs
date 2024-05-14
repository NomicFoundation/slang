#[cfg(feature = "slang_napi_interfaces")]
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
)]
#[cfg_attr( feature = "slang_napi_interfaces", /* derives `Clone` and `Copy` */ napi(string_enum, namespace = "kinds") )]
#[cfg_attr(not(feature = "slang_napi_interfaces"), derive(Clone, Copy))]
pub enum TokenKind {
    SKIPPED,
}

impl metaslang_cst::TerminalKind for TokenKind {
    fn is_trivia(&self) -> bool {
        unreachable!("Expanded by the template")
    }
}

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
)]
#[cfg_attr( feature = "slang_napi_interfaces", /* derives `Clone` and `Copy` */ napi(string_enum, namespace = "kinds") )]
#[cfg_attr(not(feature = "slang_napi_interfaces"), derive(Clone, Copy))]
pub enum RuleKind {
    _DUMMY, // a hack to aavoid a linting error
}

impl metaslang_cst::NonTerminalKind for RuleKind {}

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
)]
#[cfg_attr(feature = "slang_napi_interfaces", /* derives `Clone` and `Copy` */ napi(string_enum, namespace = "kinds"))]
#[cfg_attr(not(feature = "slang_napi_interfaces"), derive(Clone, Copy))]
pub enum NodeLabel {
    // Built-in labels
    // _SLANG_INTERNAL_RESERVED_NODE_LABELS_ (keep in sync)
    Item,
    Variant,
    Separator,
    Operand,
    LeftOperand,
    RightOperand,
}

impl metaslang_cst::EdgeKind for NodeLabel {}

/// The lexical context of the scanner.
#[derive(strum_macros::FromRepr, Clone, Copy)]
pub(crate) enum LexicalContext {
    // Expanded by the template engine
}

/// Marker trait for type-level [`LexicalContext`] variants.
pub(crate) trait IsLexicalContext {
    /// Returns a run-time [`LexicalContext`] value.
    fn value() -> LexicalContext;
}

#[allow(non_snake_case)]
pub(crate) mod LexicalContextType {}

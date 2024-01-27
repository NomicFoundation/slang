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
#[strum(use_phf)]
#[cfg_attr( feature = "slang_napi_interfaces", /* derives `Clone` and `Copy` */ napi(string_enum, namespace = "kinds") )]
#[cfg_attr(not(feature = "slang_napi_interfaces"), derive(Clone, Copy))]
pub enum TokenKind {
    SKIPPED,
    // Used for testing ithis crate, this is generated in the client code
    Identifier,
    X,
    Y,
    Z,
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
#[strum(use_phf)]
#[cfg_attr( feature = "slang_napi_interfaces", /* derives `Clone` and `Copy` */ napi(string_enum, namespace = "kinds") )]
#[cfg_attr(not(feature = "slang_napi_interfaces"), derive(Clone, Copy))]
pub enum RuleKind {
    LeadingTrivia,
    TrailingTrivia,
    // Used for testing ithis crate, this is generated in the client code
    A,
    B,
    C,
    D,
}

impl RuleKind {
    pub fn is_trivia(&self) -> bool {
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
#[strum(use_phf)]
#[cfg_attr(feature = "slang_napi_interfaces", /* derives `Clone` and `Copy` */ napi(string_enum, namespace = "kinds"))]
#[cfg_attr(not(feature = "slang_napi_interfaces"), derive(Clone, Copy))]
pub enum FieldName {
    // Built-in fields
    // _SLANG_INTERNAL_RESERVED_NODE_FIELD_NAMES_ (keep in sync)
    Item,
    Variant,
    Separator,
    Operand,
    LeftOperand,
    RightOperand,
    // Used for testing ithis crate, this is generated in the client code
    N1,
    N2,
    N3,
    N4,
}

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

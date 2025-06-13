// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

/// The lexical context of the scanner.
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

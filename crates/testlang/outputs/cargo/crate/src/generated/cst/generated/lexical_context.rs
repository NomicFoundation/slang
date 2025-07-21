// This file is generated automatically by infrastructure scripts (crates/codegen/runner/src/main.rs). Please don't edit by hand.

/// The lexical context of the scanner.
#[repr(u8)]
#[derive(strum_macros::FromRepr, Clone, Copy)]
pub(crate) enum LexicalContext {
    Default,
    Tree,
}

/// Marker trait for type-level [`LexicalContext`] variants.
pub(crate) trait IsLexicalContext {
    /// Returns a run-time [`LexicalContext`] value.
    fn value() -> LexicalContext;
}

#[allow(non_snake_case)]
pub(crate) mod LexicalContextType {
    pub struct Default;

    impl super::IsLexicalContext for Default {
        fn value() -> super::LexicalContext {
            super::LexicalContext::Default
        }
    }
    pub struct Tree;

    impl super::IsLexicalContext for Tree {
        fn value() -> super::LexicalContext {
            super::LexicalContext::Tree
        }
    }
}

// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

/// The lexical context of the scanner.
#[repr(u8)]
#[derive(strum_macros::FromRepr, Clone, Copy)]
pub(crate) enum LexicalContext {
    Default,
    Pragma,
    Yul,
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
    pub struct Pragma;

    impl super::IsLexicalContext for Pragma {
        fn value() -> super::LexicalContext {
            super::LexicalContext::Pragma
        }
    }
    pub struct Yul;

    impl super::IsLexicalContext for Yul {
        fn value() -> super::LexicalContext {
            super::LexicalContext::Yul
        }
    }
}

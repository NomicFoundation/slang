// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

#[allow(clippy::doc_markdown)]
#[allow(clippy::doc_link_with_quotes)]
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
    Clone,
    Copy,
)]
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

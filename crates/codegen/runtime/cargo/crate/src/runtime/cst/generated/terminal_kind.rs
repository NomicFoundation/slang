// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

/// Represents different kinds of terminal nodes in the syntax tree.
/// These are leaf nodes that represent actual tokens in the source code.
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
#[allow(clippy::doc_markdown)]
#[allow(clippy::doc_link_with_quotes)]
#[repr(u8)]
pub enum TerminalKind {
    /// This terminal is created when the parser encounters an unexpected part of the input,
    /// and it cannot recognize it as any valid syntax in this position in the grammar.
    UNRECOGNIZED,
    /// This terminal is created when the parser is expecting a certain terminal but does not find it.
    /// Adding the missing input in this position may allow the parser to produce a valid tree there.
    MISSING,

    Stub1,
    Stub2,
    Stub3,
}

impl crate::cst::TerminalKindExtensions for TerminalKind {}
